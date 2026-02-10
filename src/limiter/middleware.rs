use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::token_bucket::TokenBucket;

pub type SharedBucket = Arc<Mutex<TokenBucket>>;

pub async fn rate_limit(
    State(bucket): State<SharedBucket>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let mut bucket = bucket.lock().await;

    if bucket.allow(1.0) {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::TOO_MANY_REQUESTS)
    }
}
