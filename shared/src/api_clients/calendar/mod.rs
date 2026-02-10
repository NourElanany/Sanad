//! Calendar API Clients
//!
//! This module provides clients for various Islamic Calendar APIs with fallback support.

mod aladhan_calendar_client;
mod islamic_finder_calendar_client;
mod manager;

pub use aladhan_calendar_client::AladhanCalendarClient;
pub use islamic_finder_calendar_client::IslamicFinderCalendarClient;
pub use manager::CalendarApiManager;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod property_tests;
