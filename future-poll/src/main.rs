use long_poll::long_poll_channel;
use sleep::sleep;
use std::time::Duration;

mod long_poll;
mod sleep;

#[tokio::main]
async fn main() {
    let mut current_config: Option<String> = None;
    let mut i = 0;

    loop {
        let fut_1 = sleep(Duration::from_secs(10));
        let fut_2 = long_poll_channel("http://localhost:8080/messages".to_string(), i);

        tokio::select! {
            _ = fut_1 => {}
            config = fut_2 => {
                if let Some(c) = config {
                    current_config = Some(c.clone());
                }
            }
        }

        if current_config.is_some(){
            println!("We are going to do something with the config here");
            let cc = current_config.as_ref().unwrap();
            println!("Config: {}", cc);
        }

        i += 1;

        // otherwise we just skip and wait again if the value is none
    }
}
