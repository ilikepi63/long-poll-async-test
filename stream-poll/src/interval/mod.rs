use tokio::sync::mpsc::{self, Receiver};
use std::time::Duration;
use tokio::{task, time};



pub fn interval_channel(interval_time:Duration) -> Receiver<()> {

    let (tx, mut rx) = mpsc::channel(100);

    tokio::spawn(async move {
        
        let mut interval = time::interval(interval_time);

        loop {
            interval.tick().await;

            // TODO: remove the unwrap here
            tx.send(()).await.unwrap();
        }
    });

    rx
}