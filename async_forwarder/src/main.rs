#![allow(non_snake_case)]

use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener};


use clap::Parser;
mod forwarding;
use forwarding::{ForwardingThread, SelectorMode};

mod processing;
use processing::ProcessingThread;

pub mod message;
use message::Message;
use message::MPSC_CHANNEL_SIZE;
use message::MpscChannel;


//use threadpool::ThreadPool;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    //address to listen on to accept connections
    #[arg(short = 'a', long)]
    address: Option<String>,

    // address to connect to - indicates forwarding mode
    #[arg(short = 'r', long)]
    remote: Option<String>,

    // selector implementation to use in forwarding mode
    #[arg(long, value_enum, default_value = "multiple")]
    selector: SelectorMode,
}

struct Scheduler {
    activeConnections: Arc<tokio::sync::Mutex<HashMap<usize, MpscChannel>>>,
    workerCount: usize,
    threadConnections: Vec<usize>,
    threadMsgsRunningAverage: Vec<usize>,
}

impl Scheduler {
    fn new(wCount: usize) -> Scheduler {
        Scheduler {
            activeConnections: Arc::new(tokio::sync::Mutex::new(
                HashMap::<usize, MpscChannel>::new(),
            )),
            workerCount: wCount,
            threadConnections: Vec::<usize>::new(),
            threadMsgsRunningAverage: Vec::<usize>::new(),
        }
    }
    fn ReportLoading(&mut self, id: usize, msgCount: usize, numCons: usize) {}
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
    //Not specifying a address option will make it listen on loopback
    let conString: String = if args.address.is_some() {
        args.address.unwrap()
    } else {
        log::info!("Listening on default address 127.0.0.1:8080");
        "127.0.0.1:8080".to_owned()
    };

    let disCon = conString.clone();
    let listener = TcpListener::bind(conString).await?;
    let chans = Arc::new(tokio::sync::Mutex::new(Vec::<MpscChannel>::new()));

    let chansClone = Arc::clone(&chans);

    //if remote address was specified it will run in forwarding mode,
    //make a connection and pass messages back and forth
    // Otherwise processing thread will be started to process messages
    if args.remote.is_some() {
        tokio::spawn(async move {
            ForwardingThread(chansClone, args.remote.unwrap(), args.selector).await;
        });
    } else {
        tokio::task::spawn_blocking(move || {
            ProcessingThread(chansClone);
        });
    }

    let mut connectionId: usize = 0;

    log::info!("Waiting for connections on {} ...", disCon);
    loop {
        let (mut socket, _) = listener.accept().await?;
        let channelVect = Arc::clone(&chans);
        connectionId += 1;
        tokio::spawn(async move {
            log::info!("Accepting connection id {:?}", connectionId);

            // create TX and RX channels and push them into vector where processing or forwarding threads can see them
            let (inTx, mut inRx) = tokio::sync::mpsc::channel::<Message>(MPSC_CHANNEL_SIZE);
            let (outTx, outRx) = tokio::sync::mpsc::channel::<Message>(MPSC_CHANNEL_SIZE);
            {
                let mut channels = channelVect.lock().await;
                (*channels).push(MpscChannel::new(connectionId, 0, inTx, outRx));
            }

            // read data from the socket and write the data back.
            loop {
                let mut msg = Message::new(connectionId);

                let sockFut = socket.read(&mut *msg.buf);
                let channelFut = inRx.recv();

                //select! waits for both mpsc channel and socket futures and returns when first completes
                tokio::select! {
                    res1 = sockFut =>
                    {
                        match res1 {
                            Ok(0) => {
                                log::info!("Connection closed id {} closed",connectionId );
                                break}, //connection closed
                            Ok(n) => {
                                msg.size = n;
                                outTx.send(msg).await.unwrap()
                            },
                            Err(e) => {
                                log::error!("Connection id {}: failed to read from socket; err = {:?}",connectionId, e);
                            break
                            },
                        }
                    }
                    res2 = channelFut =>
                    {
                        match res2
                        {
                            Some(rxMsg) => {
                                if let Err(e) = socket.write_all(&rxMsg.buf[0..rxMsg.size]).await {
                                    log::error!("Connection id {}: failed to write to socket; err = {:?}",connectionId, e);
                                    break;
                                }
                            }
                            None => {
                                log::info!("Connection closed id {}: Channel closed",connectionId );
                            }
                        }
                    }
                }
            } //loop
        });
    }
}
