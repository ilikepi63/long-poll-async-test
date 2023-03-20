use interval::interval_channel;
use std::time::Duration;
use long_poll::long_poll_channel;
use tokio::sync::mpsc::{self, Receiver};

mod interval; 
mod long_poll;

pub fn merge_streams<T>(mut rec_1: Receiver<T>, mut rec_2: Receiver<()>) -> Receiver<Option<T>>
    where T: std::fmt::Debug + Clone + std::marker::Send + 'static
{

    let (tx, mut rx) = mpsc::channel(100);
    let (tx_1, mut rx_1) = mpsc::channel(100); 

    let tx_copy = tx.clone();

    tokio::spawn(async move {
        while let Some(msg) = rec_1.recv().await {
            tx.send(Some(msg)).await.unwrap();
        } 
    });

    tokio::spawn(async move {
        while let Some(msg) = rec_2.recv().await {
            tx_copy.send(None).await.unwrap();
        } 
    });

    tokio::spawn(async move {

        let mut current_message: Option<T> = None; 

        while let Some(msg) = rx.recv().await {

            if let Some(m) = msg {
                current_message = Some(m);
            }

            let this_message = current_message.clone();

            tx_1.send(this_message).await.unwrap();
        } 
    });

    rx_1

}

#[tokio::main]
async fn main() {
    
    let mut receiver_1 = interval_channel(Duration::from_secs(10));
    let mut receiver_2  = long_poll_channel("http://localhost:8080/messages".to_string(), 0);

    let mut merged_streams = merge_streams(receiver_2, receiver_1);

    while let Some(msg) = merged_streams.recv().await {
        println!("Iterating an interval my bro! {:?}", msg);
    } 

}
