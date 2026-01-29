use axum::{
    extract::Request,
    http::HeaderName,
    middleware::Next,
    response::Response,
};
use shared::utils::generate_request_id;
use tower::{Layer, Service};
use std::task::{Context, Poll};

/// Request ID layer that adds a unique request ID to each request
#[derive(Clone)]
pub struct RequestIdLayer;

impl RequestIdLayer {
    pub fn new() -> Self {
        Self
    }
}

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestIdService { inner }
    }
}

/// Request ID service
#[derive(Clone)]
pub struct RequestIdService<S> {
    inner: S,
}

impl<S> Service<Request> for RequestIdService<S>
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

    fn call(&mut self, mut req: Request) -> Self::Future {
        // Generate or extract request ID
        let request_id = req
            .headers()
            .get("x-request-id")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(generate_request_id);

        // Add request ID to headers
        req.headers_mut().insert(
            HeaderName::from_static("x-request-id"),
            request_id.parse().unwrap(),
        );

        let mut inner = self.inner.clone();

        Box::pin(async move {
            let mut response = inner.call(req).await?;
            
            // Add request ID to response headers
            response.headers_mut().insert(
                HeaderName::from_static("x-request-id"),
                request_id.parse().unwrap(),
            );

            Ok(response)
        })
    }
}