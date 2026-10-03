use tokio::sync::mpsc::error::TryRecvError;
use tokio::time::{sleep, Duration};
use std::collections::HashMap;
use std::sync::Arc;

use super::message::MpscChannel;


fn CheckForNewChannels(
    channelsArc: Arc<tokio::sync::Mutex<Vec<MpscChannel>>>,
    activeChannels: &mut HashMap<usize, MpscChannel>,
) {
    let rt = tokio::runtime::Handle::current();
    rt.block_on(async {
        log::info!("[CheckForNewChannels] Awaiting channels");
        while activeChannels.is_empty() {
            {
                let mut data = channelsArc.lock().await;
                while let Some(ch) = data.pop() {
                    log::info!("[CheckForNewChannels] Adding channel id {}", ch.id);
                    activeChannels.insert(ch.id, ch);
                }
            }
            //if no channels present wait for 50ms outside of lock to prevent contant locking
            if activeChannels.is_empty() {
                sleep(Duration::from_millis(50)).await;
            }
        }
    });
}
//blocking thread and not async since work/ processing should happen here
pub fn ProcessingThread(channels_arc: Arc<tokio::sync::Mutex<Vec<MpscChannel>>>) {
    log::info!("ProcessingThread started");

    let mut active_channels = HashMap::<usize, MpscChannel>::new();

    loop {
        CheckForNewChannels(channels_arc.clone(), &mut active_channels);
        let mut removed_channels = Vec::<usize>::new();

        //adding threadpool here would be good (finish implementing scheduler) 
        while !active_channels.is_empty() {
            for (i, ref mut channel) in active_channels.iter_mut() {
                let res = channel.rx.try_recv();
                match res {
                    Ok(msg) => {
                        //process message here
                        //right now works as loopback by passing messages back to connection that sent them
                        let result = channel.tx.try_send(msg);
                        if let Some(err) = result.err()
                        {
                            log::error!("[ProcessingThread] Failed to send message. Id {}, error:{}", i, err);
                        }
                    }
                    Err(TryRecvError::Empty) => continue,
                    Err(TryRecvError::Disconnected) => {
                        log::info!("[ProcessingThread] Channel closed connection:{}", i);
                        removed_channels.push(*i);
                    }
                }
            }
            //clear disconnected channels
            while let Some(i) = removed_channels.pop() {
                active_channels.remove(&i);
            }
        }
    }
}