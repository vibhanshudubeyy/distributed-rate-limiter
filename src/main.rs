mod limiter;

use axum::{routing::get, Router};
use limiter::middleware::{rate_limit, SharedBucket};
use limiter::token_bucket::TokenBucket;
use std::sync::Arc;
use tokio::sync::Mutex;

async fn main(){
    let bucket: SharedBucket = Arc::new(Mutex::new(TokenBucket::new(10.0, 5.0)));

    let app = Router::new()
        .route("/",get(|| async { "OK" }))
        .layer(axum::middleware::from_fn_with_state(bucket.clone(), rate_limit))
        .with_state(bucket);


    print("Server running on http://localhost:3000");

    axum::Server::bind(&"0.0.0.3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}