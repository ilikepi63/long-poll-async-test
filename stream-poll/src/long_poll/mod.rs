use tokio::sync::mpsc::{self, Receiver};
use std::time::Duration;
use tokio::{task, time};
use reqwest::{Client, Response};


pub fn long_poll_channel(url:String, after: usize) -> Receiver<String> {

    let (tx, rx) = mpsc::channel(100);

    let client = Client::new();

    tokio::spawn(async move {
        let mut i = after;

        loop {
            if let Ok(res) = client.get(url.to_string())
                .query(&[("after", i.to_string())])
                .send()
                .await {
                    let response = res.text()
                    .await.unwrap();

                    tx.send(response).await.unwrap();
                }

            i += 1;
        }
    });

    rx
}