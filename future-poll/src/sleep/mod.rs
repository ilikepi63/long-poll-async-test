use std::time::Duration;
use tokio::time::sleep as tokio_sleep;

pub async fn sleep(duration: Duration) -> () {
    tokio_sleep(duration).await;
}
