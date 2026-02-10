//! Prayer Times API Clients
//!
//! This module provides clients for various Prayer Times APIs with fallback support.

mod aladhan_prayer_client;
mod islamic_finder_prayer_client;
mod manager;

pub use aladhan_prayer_client::AladhanPrayerClient;
pub use islamic_finder_prayer_client::IslamicFinderPrayerClient;
pub use manager::PrayerTimesApiManager;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod property_tests;
