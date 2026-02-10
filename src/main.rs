mod limiter;

use axum::{routing::get, Router};
use limiter::middleware::{rate_limit, SharedBucket};
use limiter::token_bucket::TokenBucket;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let bucket: SharedBucket = Arc::new(Mutex::new(
        TokenBucket::new(10.0, 5.0),
    ));

    let app = Router::new()
        .route("/", get(|| async { "OK" }))
        .layer(axum::middleware::from_fn_with_state(
            bucket.clone(),
            rate_limit,
        ))
        .with_state(bucket);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("🚀 Server running on {}", addr);

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
