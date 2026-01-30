pub mod models;
pub mod repository;
pub mod service;
pub mod handlers;

#[cfg(test)]
mod tests;

// Re-export all models for external use
pub use models::*;
pub use repository::HadithRepository;
pub use service::HadithService;
pub use handlers::create_router;