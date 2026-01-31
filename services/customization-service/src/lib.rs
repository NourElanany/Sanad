pub mod models;
pub mod service;
pub mod repository;
pub mod handlers;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod property_tests;

#[cfg(test)]
mod integration_tests;

pub use models::*;
pub use service::SmartCustomizationService;
pub use repository::CustomizationRepository;
pub use handlers::CustomizationHandlers;