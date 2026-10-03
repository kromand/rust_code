
use tokio::sync::mpsc;
use std::pin::Pin;

use core::num;
use std::io::Error;
use std::{backtrace, thread, time};
use std::sync::Arc;

use async_std::channel::Receiver;
use tokio::join;
use tokio::try_join;
use tokio::runtime::Handle;
use tokio::time::{sleep,Duration};

use tokio::sync::{Mutex, Semaphore, Notify, RwLock};

//barrier
use tokio::sync::{Barrier,BarrierWaitResult};
//use tokio::time::{sleep, Duration};
async fn barrier_example(barrier: Arc<Barrier>, notify: Arc<Notify>) ->BarrierWaitResult
{
    print!("Waiting for barrier");
   let wait_result = barrier.wait().await;
   print!("after barrier");
   if wait_result.is_leader(){
    notify.notify_one();
   }
                wait_result
}

fn blocking_call() ->String
{
    thread::sleep(time::Duration::from_secs(5));
    "blocking_call done".to_string()
}

async fn a_call(id:i32)  {
    sleep(Duration::from_secs(1)).await;
    println!("async call id {}",id);
}

async fn tokio_sample()
{
    let blocking_handle = tokio::task::spawn_blocking(blocking_call);
    let mut async_vec = Vec::new();
    for id in 0..10
    {
        async_vec.push(tokio::spawn(a_call(id)));
    }
    for handle in async_vec
    {
        handle.await.unwrap();
    }
    
    let result = blocking_handle.await.unwrap();
    println!("async call id {}",result);

    //tokio mutex
    let resource = 5;
    let mtx = Mutex::new(resource);
    //can't pass it directly to threads, need Arc
    let remote_arc = Arc::new(mtx);
    let mut curr_resurce = remote_arc.lock().await;
    *curr_resurce = 7;

    // iterating through touples
    for (name, id) in [("zyta",234),("tytus",45)]
    {
        println!("{} {} example, iterating through touples", name,id);
    }

    //semaphore example
    let num =4;
    let sm = Semaphore::new(num);
            let _sm_arc = Arc::new(sm);

    //notify
    //receiver: call notified.await() transmitter: call notify_one() or notify)_waiters()
    let t = Notify::new();
    let arc_t = Arc::new(t);
    arc_t.notify_one();
    //*******************************************barrier
    let tot_cans = 12;
    let barrier = Arc::new(Barrier::new(tot_cans));
    let notify = Arc::new(Notify::new());
    //to start sending batches
    notify.notify_one();
    let mut task_handles = Vec::new();
    for can_count in 0..60
    {
        if can_count % 12 ==0
        {
            notify.notified().await;
            //give barrier some time to close
            sleep(Duration::from_millis(1)).await;
        }
        task_handles.push(tokio::spawn(barrier_example(barrier.clone(), notify.clone())));

    }
    let mut num_leaders = 0;
    for handle in task_handles
    {
        let wait_result = handle.await.unwrap();
        if wait_result.is_leader(){
            num_leaders += 1;
        }
    }
    print!("Tot num leaders {}", num_leaders);
    //rwlock
    let rwLock = Arc::new(RwLock::new(5));
    {
        let reader = rwLock.read().await;
    }
    {
        let mut writer = rwLock.write().await;
        *writer = 6;

    }
    //channels
    //one shot single producer single consumer
    //mpsc multi producer single consumer
    //watch channel single producer multiple consumers
    //broadcast - multiple producer multiple consumer
}
//tokio::spawn_blocking(hello_task());

async fn sum(num: u64) -> u64 
{
    //100000000 ~ 600 ms
    (1..=num).sum()

}

async fn sum_loop(num: u64) -> u64 
{
    //100000000 ~ 600 ms
    let mut sum = 0;
    for t in (1..=num)
    {
        sum += t;
    }
    sum
}

async fn WaitLoop(num: u64, mp:f64)
{
    use async_std::task;
    for c in 1..num
    {
        println!("{} sec sleep count: {}",mp, c);
        tokio::time::sleep(Duration::from_millis((1000 as f64 * mp) as u64)).await;
    }
}

async fn HalfSecLoop(num: u64)
{
    use async_std::task;
    for c in 1..num
    {
        println!("Half sec sleep count: {}",c);
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
}

async fn ChannelReceive(  rx: &mut tokio::sync::mpsc::Receiver<u64>) -> Result<usize, std::io::Error>
{
    let res = rx.recv().await;

    match res
    {
        Some(k) =>  Ok(k as usize),
        None => Ok(0),
    }
}

                /* 
                FuturesUnordered
//let futures =
//            vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];

//        trpl::join_all(futures).await;

                let v: Vec<Pin<Box<dyn futures::Future<Output = ()>>>> = vec![sockFut, ft1];
                futures::future::select_all(v).await;

                    //sa::assert_impl_all!(Channels:Send);
use futures::stream::FuturesUnordered;
use std::future::Future;
use std::pin::Pin;

    let mut tasks = FuturesUnordered::<Pin<Box<dyn Future<Output = Option<Message>>>>>::new();
        
    // let future: Pin<Box<dyn Future<Output = Option<Message>>>> =
     //   Box::pin(cons.get_mut(0).unwrap().rx.recv());
    futs.push(Pin<Box<dyn Future<Output = Option<Message>>>> =
        Box::pin(cons.get_mut(0).unwrap().rx.recv()));


                        let rt = tokio::runtime::Handle::current();
                rt.block_on(async {
                    /*
                    let msg = activeConnections.first_mut().unwrap().rx.recv().await;
                    match msg {
                        Some(m) => {
                            activeConnections.first().unwrap().tx.send(m).await.unwrap();
                        }
                        None => {
                            println!("[ProcessingThread] Channel closed");
                        }
                    }
                    */
                });
                */
