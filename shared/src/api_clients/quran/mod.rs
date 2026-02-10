//! Quran API clients module
//!
//! This module provides implementations for various Quran APIs including:
//! - Quran.com / Quran Foundation API
//! - AlQuran Cloud API
//! - Tanzil.net API
//! - EveryAyah.com API (for audio recitations)

pub mod quran_com_client;
pub mod alquran_cloud_client;
pub mod tanzil_client;
pub mod everyayah_client;
pub mod manager;

#[cfg(test)]
mod property_tests;

#[cfg(test)]
mod tests;

pub use quran_com_client::QuranComClient;
pub use alquran_cloud_client::AlquranCloudClient;
pub use tanzil_client::TanzilClient;
pub use everyayah_client::EveryayahClient;
pub use manager::QuranApiManager;
