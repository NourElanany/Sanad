//! Tafsir API clients
//!
//! This module provides clients for various Tafsir (Quran interpretation) APIs.

mod quran_com_tafsir_client;
mod manager;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod property_tests;

pub use quran_com_tafsir_client::QuranComTafsirClient;
pub use manager::{TafsirApiManager, OrganizedTafsirResponse};
