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

pub mod models;
pub mod service;

// Re-export main types
pub use models::*;
pub use service::ApiIntegrationService;
