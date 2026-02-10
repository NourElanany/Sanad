// Qibla API clients module
//
// This module provides clients for various Qibla direction APIs

mod aladhan_qibla_client;
mod islamic_finder_qibla_client;
mod manager;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod property_tests;

pub use aladhan_qibla_client::AladhanQiblaClient;
pub use islamic_finder_qibla_client::IslamicFinderQiblaClient;
pub use manager::QiblaApiManager;
