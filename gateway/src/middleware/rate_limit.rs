use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use shared::{ApiResponse, utils::RateLimiter};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tower::{Layer, Service};
use std::task::{Context, Poll};

/// Rate limiting layer
#[derive(Clone)]
pub struct RateLimitLayer {
    max_requests: usize,
}

impl RateLimitLayer {
    pub fn new(max_requests_per_minute: u32) -> Self {
        Self {
            max_requests: max_requests_per_minute as usize,
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService {
            inner,
            limiter: Arc::new(Mutex::new(RateLimiter::new(self.max_requests, 60))),
        }
    }
}

/// Rate limiting service
#[derive(Clone)]
pub struct RateLimitService<S> {
    inner: S,
    limiter: Arc<Mutex<RateLimiter>>,
}

impl<S> Service<Request> for RateLimitService<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let limiter = self.limiter.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Extract client IP or use a default key
            let client_key = req
                .headers()
                .get("x-forwarded-for")
                .and_then(|h| h.to_str().ok())
                .or_else(|| {
                    req.headers()
                        .get("x-real-ip")
                        .and_then(|h| h.to_str().ok())
                })
                .unwrap_or("unknown")
                .to_string();

            // Check rate limit
            let allowed = {
                let mut limiter = limiter.lock().unwrap();
                limiter.is_allowed(&client_key)
            };

            if !allowed {
                let error_response = ApiResponse::<()>::error(
                    "Rate limit exceeded. Please try again later.".to_string()
                );
                return Ok((StatusCode::TOO_MANY_REQUESTS, axum::Json(error_response)).into_response());
            }

            // Continue with the request
            inner.call(req).await
        })
    }
}