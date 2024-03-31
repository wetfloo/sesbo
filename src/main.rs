mod cli;
mod response;
mod util;

use axum::extract::Path;
use axum::routing::get;
use clap::Parser;
use cli::CliArgs;
use response::err::{KvReadError, KvWriteError};
use response::ok::{KvReadOk, KvWriteOk};
use std::collections::HashMap;

use axum::{routing::post, Router};
use once_cell::sync::Lazy;
use util::{alphanumeric, MAX_PAYLOAD_LEN};

static KVSTORE: Lazy<tokio::sync::RwLock<HashMap<String, String>>> =
    Lazy::new(|| tokio::sync::RwLock::new(HashMap::new()));

#[tokio::main]
async fn main() {
    let args = CliArgs::parse();

    let app = Router::new()
        .route("/", post(handle_insert))
        .route("/", get("Try POSTing to / instead"))
        .route("/:key", get(handle_read));

    let listener = tokio::net::TcpListener::bind((args.address, args.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_read(Path(key): Path<String>) -> Result<KvReadOk, KvReadError> {
    let guard = KVSTORE.read().await;
    let value = guard.get(&key);
    match value {
        Some(v) => Ok(KvReadOk(v.to_owned())),
        None => Err(KvReadError::NotFound { key }),
    }
}

async fn handle_insert(value: String) -> Result<KvWriteOk, KvWriteError> {
    let len = value.len();
    if len > MAX_PAYLOAD_LEN {
        return Err(KvWriteError::SizeTooBig(len));
    }

    let mut map = KVSTORE.write().await;

    loop {
        let key = alphanumeric(6);
        if map.contains_key(&key) {
            continue;
        }

        map.insert(key.clone(), value);
        break Ok(KvWriteOk(key));
    }
}
