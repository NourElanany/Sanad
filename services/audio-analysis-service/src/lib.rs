pub mod audio_processor;
pub mod audio_recorder;
pub mod spectrum_analyzer;
pub mod reference_manager;
pub mod comparison_engine;
pub mod tajweed_analyzer;
pub mod scoring_system;
pub mod models;
pub mod service;

#[cfg(test)]
mod tests;

pub use service::AudioAnalysisService;