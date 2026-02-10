//! Request Context Management
//!
//! This module provides utilities for managing request context including
//! correlation IDs that can be propagated through the entire request lifecycle.

use std::sync::Arc;
use tokio::task_local;

/// Request context containing correlation ID and other metadata
#[derive(Debug, Clone)]
pub struct RequestContext {
    /// Correlation ID for tracking requests across services
    pub correlation_id: String,
    /// Optional user ID for authenticated requests
    pub user_id: Option<String>,
    /// Request start time for performance tracking
    pub start_time: std::time::Instant,
}

impl RequestContext {
    /// Create a new request context with a correlation ID
    pub fn new(correlation_id: String) -> Self {
        Self {
            correlation_id,
            user_id: None,
            start_time: std::time::Instant::now(),
        }
    }

    /// Create a new request context with a generated correlation ID
    pub fn generate() -> Self {
        Self::new(uuid::Uuid::new_v4().to_string())
    }

    /// Set the user ID for this request
    pub fn with_user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
        self
    }

    /// Get the correlation ID
    pub fn correlation_id(&self) -> &str {
        &self.correlation_id
    }

    /// Get the elapsed time since request start
    pub fn elapsed(&self) -> std::time::Duration {
        self.start_time.elapsed()
    }
}

// Task-local storage for request context
task_local! {
    pub static REQUEST_CONTEXT: Arc<RequestContext>;
}

/// Get the current request context if available
pub fn current_context() -> Option<Arc<RequestContext>> {
    REQUEST_CONTEXT.try_with(|ctx| ctx.clone()).ok()
}

/// Get the current correlation ID if available
pub fn current_correlation_id() -> Option<String> {
    current_context().map(|ctx| ctx.correlation_id.clone())
}

/// Run a future with a request context
pub async fn with_context<F, T>(context: RequestContext, future: F) -> T
where
    F: std::future::Future<Output = T>,
{
    REQUEST_CONTEXT.scope(Arc::new(context), future).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_context_creation() {
        let ctx = RequestContext::new("test-id".to_string());
        assert_eq!(ctx.correlation_id(), "test-id");
        assert!(ctx.user_id.is_none());
    }

    #[test]
    fn test_request_context_with_user_id() {
        let ctx = RequestContext::new("test-id".to_string())
            .with_user_id("user-123".to_string());
        assert_eq!(ctx.correlation_id(), "test-id");
        assert_eq!(ctx.user_id, Some("user-123".to_string()));
    }

    #[test]
    fn test_request_context_generate() {
        let ctx = RequestContext::generate();
        assert!(!ctx.correlation_id().is_empty());
    }

    #[tokio::test]
    async fn test_with_context() {
        let ctx = RequestContext::new("test-id".to_string());
        
        let result = with_context(ctx, async {
            let current = current_correlation_id();
            assert_eq!(current, Some("test-id".to_string()));
            "success"
        }).await;
        
        assert_eq!(result, "success");
    }

    #[tokio::test]
    async fn test_current_context_outside_scope() {
        let ctx = current_context();
        assert!(ctx.is_none());
    }
}
