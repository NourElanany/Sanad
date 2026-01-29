pub mod models;
pub mod repository;
pub mod service;
pub mod handlers;

#[cfg(test)]
mod tests;

pub use models::*;
pub use repository::QuranRepository;
pub use service::QuranService;
pub use handlers::create_router;