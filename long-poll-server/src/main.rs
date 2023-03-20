use axum::{
    extract::{Query},
    handler::{Handler},
    routing::{get, },
    response::IntoResponse,
    Json, Router,
};
use serde::Deserialize;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;
use chat::Config;
use error::Error;
use axum::extract::State;

mod chat;
mod error;

#[derive(Clone)]
struct AppState {
    data: Arc<Mutex<Vec<Config>>>,
}


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    std::env::set_var("RUST_LOG", "rust_long_polling=info");
    env_logger::init();

    let mut db:Vec<Config> = vec![];
    let ctx = AppState {
        data: Arc::new(Mutex::new(db))
    };

    let app = Router::new()
        .route(
            "/messages",
            get(handler_find_messages)
            .post(handler_create_message),
        )
        // .or(handler_404.into_service())
        .with_state(ctx);

    log::info!("Starting server on 0.0.0.0:8080");
    axum::Server::bind(
        &"0.0.0.0:8080"
            .parse()
            .expect("parsing server's bind address"),
    )
    .serve(app.into_make_service())
    .await
    .expect("running server");

    Ok(())
}

#[derive(Clone, Debug, Deserialize)]
struct CreateMessage {
    body: String,
}

#[derive(Deserialize)]
struct FindMessagesQueryParameters {
    after: Option<usize>,
}

async fn handler_404() -> impl IntoResponse {
    Error::NotFound("Route not found".to_string())
}

// create message
async fn handler_create_message(
    State(state): State<AppState>,
    Json(input): Json<CreateMessage>,
) -> Result<Json<Config>, Error> {

    let configs = &mut *state.data.lock().await;

    let created_at = chrono::Utc::now();
    let id: usize = configs.len();

    let config = Config{
        id, 
        created_at
    };

    configs.push(Config{
        id, 
        created_at
    });

    Ok(config.into())
}

#[axum_macros::debug_handler]
async fn handler_find_messages(
    State(state): State<AppState>,
    query_params: Query<FindMessagesQueryParameters>,
) -> Result<Json<Vec<Config>>, Error> {
    let sleep_for = Duration::from_secs(1);

    let configs_clone = state.data.clone();

    loop {
        let configs = &*configs_clone.lock().await;

        let config = match query_params.after {
            Some(i) => {
                if i > configs.len() {
                    Vec::new()
                }else{
                    configs[i..].to_vec()
                }
            },
            None => {
                configs.to_vec()
            }
        };

        if config.len() != 0 {
            return Ok(config.into());
        }

        tokio::time::sleep(sleep_for).await;
    }

    Ok(Vec::new().into())
}
