use axum::{
    extract::{Multipart, Path, State},
    http::StatusCode,
    response::{Json, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use shared::{
    ApiResponse, AudioRecording, RecitationAnalysis, 
    AudioSpectrum, AudioComparisonResult, Reciter, ReferenceRecording
};
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;
use tracing::info;

mod audio_processor;
mod audio_recorder;
mod spectrum_analyzer;
mod reference_manager;
mod comparison_engine;
mod tajweed_analyzer;
mod scoring_system;
mod progress_tracker;
mod improvement_engine;
mod reward_system;
mod models;
mod service;

#[cfg(test)]
mod tests;

use models::*;
use service::AudioAnalysisService;

/// Global state for the audio analysis service
#[derive(Clone)]
struct AppState {
    service: Arc<AudioAnalysisService>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    
    info!("Starting Audio Analysis Service...");
    
    // Initialize the audio analysis service
    let service = AudioAnalysisService::new().await?;
    
    let app_state = AppState {
        service: Arc::new(service),
    };
    
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/record/start", post(start_recording))
        .route("/record/stop", post(stop_recording))
        .route("/upload", post(upload_audio))
        .route("/analyze/:recording_id", post(analyze_recording))
        .route("/compare", post(compare_recordings))
        .route("/reference/reciters", get(get_reciters))
        .route("/reference/recordings/:surah/:ayah", get(get_reference_recordings))
        .route("/spectrum/:recording_id", get(get_audio_spectrum))
        .route("/system/health", get(get_system_health))
        .with_state(app_state);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8088").await?;
    info!("Audio Analysis Service listening on 0.0.0.0:8088");
    
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health_check() -> Json<ApiResponse<HashMap<String, String>>> {
    let mut status = HashMap::new();
    status.insert("status".to_string(), "healthy".to_string());
    status.insert("service".to_string(), "audio-analysis-service".to_string());
    status.insert("version".to_string(), "0.1.0".to_string());
    status.insert("features".to_string(), "recording,analysis,comparison,reference_db".to_string());
    Json(ApiResponse::success(status))
}

async fn start_recording(
    State(state): State<AppState>,
    Json(request): Json<StartRecordingRequest>
) -> Response {
    info!("Recording start requested for Surah {} Ayah {}-{}", 
          request.surah_number, request.ayah_start, request.ayah_end);
    
    match state.service.start_recording_session(request).await {
        Ok(session) => Json(ApiResponse::success(session)).into_response(),
        Err(e) => {
            tracing::error!("Failed to start recording: {}", e);
            Json(ApiResponse::error(format!("Failed to start recording: {}", e))).into_response()
        }
    }
}

async fn stop_recording(
    State(state): State<AppState>,
    Json(session_id): Json<String>
) -> Response {
    info!("Recording stop requested for session: {}", session_id);
    
    match state.service.stop_recording_session(&session_id).await {
        Ok(recording) => Json(ApiResponse::success(recording)).into_response(),
        Err(e) => {
            tracing::error!("Failed to stop recording: {}", e);
            Json(ApiResponse::error(format!("Failed to stop recording: {}", e))).into_response()
        }
    }
}

async fn upload_audio(
    State(state): State<AppState>,
    mut multipart: Multipart
) -> Result<Json<ApiResponse<AudioRecording>>, StatusCode> {
    info!("Audio upload requested");
    
    let mut audio_data: Option<Vec<u8>> = None;
    let mut metadata: Option<AudioUploadMetadata> = None;
    
    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();
        
        match name.as_str() {
            "audio" => {
                let bytes = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                audio_data = Some(bytes.to_vec());
            }
            "metadata" => {
                let bytes = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
                metadata = Some(serde_json::from_slice(&bytes).map_err(|_| StatusCode::BAD_REQUEST)?);
            }
            _ => {}
        }
    }
    
    let audio_data = audio_data.ok_or(StatusCode::BAD_REQUEST)?;
    let metadata = metadata.ok_or(StatusCode::BAD_REQUEST)?;
    
    match state.service.upload_audio(audio_data, metadata).await {
        Ok(recording) => Ok(Json(ApiResponse::success(recording))),
        Err(e) => {
            tracing::error!("Failed to upload audio: {}", e);
            Ok(Json(ApiResponse::error(format!("Failed to upload audio: {}", e))))
        }
    }
}

async fn analyze_recording(
    State(state): State<AppState>,
    Path(recording_id): Path<Uuid>,
    Json(request): Json<AnalyzeRecordingRequest>
) -> Response {
    info!("Analysis requested for recording: {}", recording_id);
    
    match state.service.analyze_recording(recording_id, request).await {
        Ok(analysis) => Json(ApiResponse::success(analysis)).into_response(),
        Err(e) => {
            tracing::error!("Failed to analyze recording: {}", e);
            Json(ApiResponse::error(format!("Failed to analyze recording: {}", e))).into_response()
        }
    }
}

async fn compare_recordings(
    State(state): State<AppState>,
    Json(request): Json<CompareRecordingsRequest>
) -> Response {
    info!("Audio comparison requested");
    
    match state.service.compare_recordings(request).await {
        Ok(result) => Json(ApiResponse::success(result)).into_response(),
        Err(e) => {
            tracing::error!("Failed to compare recordings: {}", e);
            Json(ApiResponse::error(format!("Failed to compare recordings: {}", e))).into_response()
        }
    }
}

async fn get_reciters(State(state): State<AppState>) -> Response {
    info!("Reciters list requested");
    
    match state.service.get_all_reciters().await {
        Ok(reciters) => Json(ApiResponse::success(reciters)).into_response(),
        Err(e) => {
            tracing::error!("Failed to get reciters: {}", e);
            Json(ApiResponse::error(format!("Failed to get reciters: {}", e))).into_response()
        }
    }
}

async fn get_reference_recordings(
    State(state): State<AppState>,
    Path((surah, ayah)): Path<(u8, u16)>
) -> Response {
    info!("Reference recordings requested for Surah {} Ayah {}", surah, ayah);
    
    match state.service.get_reference_recordings(surah, ayah).await {
        Ok(recordings) => Json(ApiResponse::success(recordings)).into_response(),
        Err(e) => {
            tracing::error!("Failed to get reference recordings: {}", e);
            Json(ApiResponse::error(format!("Failed to get reference recordings: {}", e))).into_response()
        }
    }
}

async fn get_audio_spectrum(
    State(state): State<AppState>,
    Path(recording_id): Path<Uuid>
) -> Response {
    info!("Spectrum analysis requested for recording: {}", recording_id);
    
    match state.service.get_audio_spectrum(recording_id).await {
        Ok(spectrum) => Json(ApiResponse::success(spectrum)).into_response(),
        Err(e) => {
            tracing::error!("Failed to get audio spectrum: {}", e);
            Json(ApiResponse::error(format!("Failed to get audio spectrum: {}", e))).into_response()
        }
    }
}

async fn get_system_health(State(state): State<AppState>) -> Response {
    info!("System health check requested");
    
    match state.service.get_system_health().await {
        Ok(health) => Json(ApiResponse::success(health)).into_response(),
        Err(e) => {
            tracing::error!("Failed to get system health: {}", e);
            Json(ApiResponse::error(format!("Failed to get system health: {}", e)))
        }
    }
}