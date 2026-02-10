//! API Integration Service
//! 
//! This service provides a unified interface for integrating with multiple official Islamic APIs
//! including Quran, Hadith, Prayer Times, Tafsir, Calendar, Qibla, and AI/NLP services.
//! 
//! The service implements:
//! - API client abstraction with fallback mechanisms
//! - Intelligent caching with TTL strategies
//! - Rate limiting per API
//! - Health monitoring
//! - Comprehensive error handling
//! - Request/response logging with correlation IDs
//! - CORS support
//! - Security headers
//! - Request timeouts

pub mod models;
pub mod service;
pub mod handlers;
pub mod middleware;
pub mod config;
pub mod request_context;
pub mod observability;

#[cfg(test)]
mod property_tests;

#[cfg(test)]
mod integration_tests;

#[cfg(test)]
mod tests;

// Re-export main types
pub use models::*;
pub use service::ApiIntegrationService;
pub use handlers::{create_router, AppState};
pub use config::{load_config, load_config_from_default_location};
pub use request_context::{RequestContext, current_context, current_correlation_id, with_context};
