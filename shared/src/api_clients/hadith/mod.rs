//! Hadith API clients module
//!
//! This module provides implementations for various Hadith APIs including:
//! - Sunnah.com API (Primary)
//! - Hadith API
//! - Aladhan Hadith API

pub mod sunnah_com_client;
pub mod hadith_api_client;
pub mod aladhan_hadith_client;
pub mod manager;

#[cfg(test)]
mod property_tests;

#[cfg(test)]
mod tests;

pub use sunnah_com_client::SunnahComClient;
pub use hadith_api_client::HadithApiClientImpl;
pub use aladhan_hadith_client::AladhanHadithClient;
pub use manager::HadithApiManager;
