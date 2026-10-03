use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

use super::message::Message;
use super::message::MpscChannel;

use std::collections::HashMap;
use std::sync::Arc;

use futures::stream::FuturesUnordered;
use futures::StreamExt;

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum SelectorMode {
    Single,
    Multiple,
}

async fn AwaitIncommingChannels(
    v: Arc<tokio::sync::Mutex<Vec<MpscChannel>>>,
    remoteAddress: &String,
) -> (
    HashMap<i32, tokio::sync::mpsc::Receiver<Message>>,
    HashMap<i32, tokio::sync::mpsc::Sender<Message>>,
) {
    loop {
        let channels = {
            let mut data = v.lock().await;
            data.drain(..).collect::<Vec<_>>()
        };
        if !channels.is_empty() {
            let mut receivers = HashMap::with_capacity(channels.len());
            let mut senders = HashMap::with_capacity(channels.len());
            for channel in channels {
                let Ok(id) = i32::try_from(channel.id) else {
                    log::error!(
                        "AwaitIncommingChannels: unsupported connection id {}",
                        channel.id
                    );
                    continue;
                };
                receivers.insert(id, channel.rx);
                senders.insert(id, channel.tx);
            }
            if !receivers.is_empty() {
                log::info!(
                    "AwaitIncommingChannels: {} channel(s) ready. Connecting to {}",
                    receivers.len(),
                    remoteAddress
                );
                return (receivers, senders);
            }
        }
        sleep(Duration::from_millis(1)).await;
    }
}

async fn AwaitTargetConnection(remoteAddress: &str) -> Result<TcpStream, std::io::Error> {
    let mut res = TcpStream::connect(remoteAddress).await;
    while res.is_err() {
        log::info!("AwaitTargetConnection: Unable to connect. Retrying in .5 s");
        sleep(Duration::from_millis(500)).await;
        res = TcpStream::connect(remoteAddress).await;
    }
    res
}
/*
This function is polling all channel futures.
Issues: indentifying the future, removing when channel is closed and adding new ones
*/
struct SocketMessageState {
    header: [u8; Message::WIRE_HEADER_SIZE],
    header_filled: usize,
    current_message: Option<Message>,
    payload_filled: usize,
}

impl SocketMessageState {
    fn new() -> Self {
        Self {
            header: [0; Message::WIRE_HEADER_SIZE],
            header_filled: 0,
            current_message: None,
            payload_filled: 0,
        }
    }

    fn reading_header(&self) -> bool {
        self.current_message.is_none()
    }

    fn read_buffer(&mut self) -> &mut [u8] {
        if self.reading_header() {
            &mut self.header[self.header_filled..]
        } else {
            let message = self.current_message.as_mut().unwrap();
            &mut message.buf[self.payload_filled..message.size]
        }
    }
}

enum ChannelSelectEvent {
    SocketRead(Result<usize, std::io::Error>),
    ChannelReceive(Option<(i32, Option<Message>)>),
}

async fn HandleSocketRead(
    size: usize,
    senders: &HashMap<i32, tokio::sync::mpsc::Sender<Message>>,
    state: &mut SocketMessageState,
) -> bool {
    if state.reading_header() {
        state.header_filled += size;
        if state.header_filled < Message::WIRE_HEADER_SIZE {
            return true;
        }

        let (raw_connection_id, payload_size) = Message::decode_wire_header(&state.header);
        if payload_size > Message::MAX_PAYLOAD_SIZE {
            log::error!("SelectMultipleChannels: invalid message payload size {payload_size}");
            return false;
        }

        let msg = Message::new(raw_connection_id);
        let Ok(connection_id) = i32::try_from(msg.connection_id()) else {
            log::error!(
                "SelectMultipleChannels: invalid connection id {}",
                msg.connection_id()
            );
            return false;
        };
        if !senders.contains_key(&connection_id) {
            log::error!("SelectMultipleChannels: no sender for connection {connection_id}");
            return false;
        }

        let mut msg = msg;
        msg.size = payload_size;
        state.current_message = Some(msg);
        state.payload_filled = 0;
    } else {
        state.payload_filled += size;
    }

    let message_complete = state
        .current_message
        .as_ref()
        .is_some_and(|msg| state.payload_filled == msg.size);
    if message_complete {
        let Some(msg) = state.current_message.take() else {
            return false;
        };
        let Ok(connection_id) = i32::try_from(msg.connection_id()) else {
            log::error!(
                "SelectMultipleChannels: invalid connection id {}",
                msg.connection_id()
            );
            return false;
        };
        let Some(sender) = senders.get(&connection_id) else {
            log::error!("SelectMultipleChannels: no sender for connection {connection_id}");
            return false;
        };
        if sender.send(msg).await.is_err() {
            log::error!("SelectMultipleChannels: sender channel {connection_id} closed");
            return false;
        }
        state.header_filled = 0;
        state.payload_filled = 0;
    }
    true
}

/// Handles one event from the re-armed channel/socket selector.
///
/// Returns `false` when the selector should stop.
async fn HandleMultipleChannelEvent(
    event: ChannelSelectEvent,
    receivers: &mut HashMap<i32, tokio::sync::mpsc::Receiver<Message>>,
    senders: &HashMap<i32, tokio::sync::mpsc::Sender<Message>>,
    forwarding_socket: &mut TcpStream,
    state: &mut SocketMessageState,
) -> bool {
    match event {
        ChannelSelectEvent::SocketRead(Ok(0)) => {
            log::info!("SelectMultipleChannels: forwarding socket closed");
            false
        }
        ChannelSelectEvent::SocketRead(Ok(size)) => HandleSocketRead(size, senders, state).await,
        ChannelSelectEvent::SocketRead(Err(error)) => {
            log::error!("SelectMultipleChannels: failed to read from socket: {error}");
            false
        }
        ChannelSelectEvent::ChannelReceive(Some((id, Some(rx_msg)))) => {
            let Some(header) = rx_msg.encode_wire_header() else {
                log::error!("SelectMultipleChannels: invalid message from channel {id}");
                return false;
            };
            if let Err(error) = forwarding_socket.write_all(&header).await {
                log::error!("SelectMultipleChannels: failed to write message header: {error}");
                return false;
            }
            if let Err(error) = forwarding_socket
                .write_all(&rx_msg.buf[0..rx_msg.size])
                .await
            {
                log::error!("SelectMultipleChannels: failed to write to socket: {error}");
                return false;
            }
            true
        }
        ChannelSelectEvent::ChannelReceive(Some((id, None))) => {
            log::info!("SelectMultipleChannels: channel {id} closed");
            receivers.remove(&id);
            true
        }
        ChannelSelectEvent::ChannelReceive(None) => {
            log::info!("SelectMultipleChannels: no channel receivers remain");
            false
        }
    }
}

/// Selects between forwarding-socket reads and messages from all receivers.
///
/// Receiver futures are rebuilt after each event. Socket messages use an 8-byte
/// connection-ID and payload-length header followed by the payload.
pub async fn SelectMultipleChannels(
    receivers: &mut HashMap<i32, tokio::sync::mpsc::Receiver<Message>>,
    senders: &HashMap<i32, tokio::sync::mpsc::Sender<Message>>,
    forwarding_socket: &mut TcpStream,
) {
    let mut state = SocketMessageState::new();

    loop {
        let has_receivers = !receivers.is_empty();
        let event = {
            let mut future_stream: FuturesUnordered<_> = receivers
                .iter_mut()
                .map(|(id, recv)| {
                    let id = *id;
                    async move { (id, recv.recv().await) }
                })
                .collect();

            let read_buffer = state.read_buffer();
            tokio::select! {
                result = forwarding_socket.read(read_buffer) => {
                    ChannelSelectEvent::SocketRead(result)
                }
                result = future_stream.next(), if has_receivers => {
                    ChannelSelectEvent::ChannelReceive(result)
                }
            }
        };

        if !HandleMultipleChannelEvent(event, receivers, &senders, forwarding_socket, &mut state)
            .await
        {
            break;
        }
    }
}

/// Selects framed forwarding events using the single-selector implementation.
pub async fn SelectSingleChannel(
    receivers: &mut HashMap<i32, tokio::sync::mpsc::Receiver<Message>>,
    senders: &HashMap<i32, tokio::sync::mpsc::Sender<Message>>,
    forwarding_socket: &mut TcpStream,
) {
    let mut state = SocketMessageState::new();

    loop {
        let has_receivers = !receivers.is_empty();
        let event = {
            let mut future_stream: FuturesUnordered<_> = receivers
                .iter_mut()
                .map(|(id, receiver)| {
                    let id = *id;
                    async move { (id, receiver.recv().await) }
                })
                .collect();

            let read_buffer = state.read_buffer();
            tokio::select! {
                result = forwarding_socket.read(read_buffer) => {
                    ChannelSelectEvent::SocketRead(result)
                }
                channel = future_stream.next(), if has_receivers => {
                    ChannelSelectEvent::ChannelReceive(channel)
                }
            }
        };

        if !HandleMultipleChannelEvent(event, receivers, senders, forwarding_socket, &mut state)
            .await
        {
            break;
        }
    }
}

pub async fn ForwardingThread(
    v: Arc<tokio::sync::Mutex<Vec<MpscChannel>>>,
    remAddress: String,
    selector_mode: SelectorMode,
) {
    log::info!("ForwardingThread started. Awaiting connection ");
    let (mut receivers, senders) = AwaitIncommingChannels(v, &remAddress).await;

    while !receivers.is_empty() {
        let mut stream = AwaitTargetConnection(&remAddress).await.unwrap();
        match selector_mode {
            SelectorMode::Single => {
                SelectSingleChannel(&mut receivers, &senders, &mut stream).await;
            }
            SelectorMode::Multiple => {
                SelectMultipleChannels(&mut receivers, &senders, &mut stream).await;
            }
        }
    }
}
