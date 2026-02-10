// AI/NLP API clients module
//
// This module provides clients for AI and NLP services
// NOTE: AI services are used ONLY for technical language processing
// (search, embeddings, text analysis). They are NOT used for generating
// Islamic rulings, fatwas, or religious content.

mod hugging_face_client;
mod manager;

#[cfg(test)]
mod tests;

pub use hugging_face_client::HuggingFaceClient;
pub use manager::AiApiManager;
