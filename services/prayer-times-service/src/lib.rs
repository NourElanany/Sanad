pub mod models;
pub mod calculator;
pub mod hijri_calendar;
#[cfg(not(test))]
pub mod repository;
#[cfg(not(test))]
pub mod service;
pub mod notification_service;
#[cfg(not(test))]
pub mod handlers;

#[cfg(test)]
mod mock_repository;

#[cfg(test)]
mod property_tests;

#[cfg(test)]
mod debug_test;

#[cfg(test)]
mod hijri_calendar_tests;

#[cfg(test)]
mod hijri_property_tests;

#[cfg(test)]
mod hijri_round_trip_test;

#[cfg(test)]
mod notification_tests;

#[cfg(test)]
mod notification_property_tests;

pub use calculator::PrayerTimesCalculator;
#[cfg(not(test))]
pub use service::PrayerTimesService;
#[cfg(not(test))]
pub use repository::PrayerTimesRepository;