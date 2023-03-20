use reqwest::{Client};

pub async fn long_poll_channel(url: String, after: usize) -> Option<String> {
    let client = Client::new();

    match client
        .get(url.to_string())
        .query(&[("after", after.to_string())])
        .send().await
    {
        Ok(res) => match res.text().await {
            Ok(t) => return Some(t),
            Err(_) => return None,
        },
        Err(_) => return None,
    }
}
