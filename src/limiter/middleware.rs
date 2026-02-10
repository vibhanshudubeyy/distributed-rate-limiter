use axum::{
    extract::State,
    Http:: {Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::token_bucket::TokenBucket;

pub type SharedBucket = Arc<Mutex<TokenBucket>>;

pub async fn rate_limit<B>(State(bucket): State<SharedBucket>, req: Request<B>, next: Next<B>) -> Result(Response, StatusCode) {

    let mut bucket = bucket.lock().await;
    if(bucket.allow(1.0)){
        Ok(next.run(req).await);
    } else {
        Err(StatusCode::TOO_MANY_REQUESTS);
    }
}