use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;
use tracing::{info, error};

use crate::service::AudioAnalysisService;
use crate::models::*;
use crate::progress_tracker::*;
use crate::improvement_engine::*;
use crate::reward_system::*;

/// API handlers for the audio analysis service with tracking and improvement features
pub struct AudioAnalysisHandlers;

/// Request to initialize user progress
#[derive(Debug, Deserialize)]
pub struct InitializeProgressRequest {
    pub user_id: Uuid,
}

/// Request to update user progress
#[derive(Debug, Deserialize)]
pub struct UpdateProgressRequest {
    pub user_id: Uuid,
    pub surah: u8,
    pub ayah: u16,
    pub score: f64,
    pub session_duration_minutes: u32,
}

/// Request to create learning plan
#[derive(Debug, Deserialize)]
pub struct CreateLearningPlanRequest {
    pub user_id: Uuid,
    pub target_duration_weeks: u32,
    pub daily_practice_minutes: u32,
}

/// Request to predict progress
#[derive(Debug, Deserialize)]
pub struct PredictProgressRequest {
    pub user_id: Uuid,
    pub weeks_ahead: u32,
}

/// Query parameters for recommendations
#[derive(Debug, Deserialize)]
pub struct RecommendationQuery {
    pub session_count: Option<u32>,
}

impl AudioAnalysisHandlers {
    /// Create a new handlers instance
    pub fn new() -> Self {
        Self
    }
    
    /// Handle user progress initialization
    pub async fn handle_initialize_user_progress(
        service: Arc<AudioAnalysisService>,
        user_id: Uuid,
    ) -> Result<String, String> {
        match service.initialize_user_progress(user_id).await {
            Ok(_) => {
                info!("Initialized progress tracking for user: {}", user_id);
                Ok("Progress tracking initialized successfully".to_string())
            }
            Err(e) => {
                error!("Failed to initialize user progress: {}", e);
                Err(format!("Failed to initialize progress: {}", e))
            }
        }
    }
    
    /// Handle getting user progress
    pub async fn handle_get_user_progress(
        service: Arc<AudioAnalysisService>,
        user_id: Uuid,
    ) -> Result<UserProgressData, String> {
        match service.get_user_progress_data(user_id).await {
            Ok(progress) => Ok(progress),
            Err(e) => {
                error!("Failed to get user progress: {}", e);
                Err(format!("Failed to get progress: {}", e))
            }
        }
    }
    
    /// Handle updating user progress
    pub async fn handle_update_user_progress(
        service: Arc<AudioAnalysisService>,
        request: UpdateProgressRequest,
    ) -> Result<ProgressUpdate, String> {
        let errors = vec![]; // In a real implementation, errors would come from the analysis
        
        match service.update_user_progress(
            request.user_id,
            request.surah,
            request.ayah,
            request.score,
            &errors,
            request.session_duration_minutes,
        ).await {
            Ok(update) => Ok(update),
            Err(e) => {
                error!("Failed to update user progress: {}", e);
                Err(format!("Failed to update progress: {}", e))
            }
        }
    }
    
    /// Handle getting user dashboard
    pub async fn handle_get_user_dashboard(
        service: Arc<AudioAnalysisService>,
        user_id: Uuid,
    ) -> Result<UserDashboard, String> {
        match service.get_user_dashboard(user_id).await {
            Ok(dashboard) => Ok(dashboard),
            Err(e) => {
                error!("Failed to get user dashboard: {}", e);
                Err(format!("Failed to get dashboard: {}", e))
            }
        }
    }
    
    /// Handle getting personalized exercises
    pub async fn handle_get_personalized_exercises(
        service: Arc<AudioAnalysisService>,
        user_id: Uuid,
    ) -> Result<Vec<Exercise>, String> {
        match service.generate_personalized_exercises(user_id).await {
            Ok(exercises) => Ok(exercises),
            Err(e) => {
                error!("Failed to get personalized exercises: {}", e);
                Err(format!("Failed to get exercises: {}", e))
            }
        }
    }
    
    /// Handle getting improvement recommendations
    pub async fn handle_get_improvement_recommendations(
        service: Arc<AudioAnalysisService>,
        user_id: Uuid,
        session_count: u32,
    ) -> Result<Vec<ImprovementRecommendation>, String> {
        let recent_errors = vec![]; // Would get from recent sessions
        
        match service.generate_improvement_recommendations(user_id, &recent_errors, session_count).await {
            Ok(recommendations) => Ok(recommendations),
            Err(e) => {
                error!("Failed to get improvement recommendations: {}", e);
                Err(format!("Failed to get recommendations: {}", e))
            }
        }
    }
    
    /// Handle creating learning plan
    pub async fn handle_create_learning_plan(
        service: Arc<AudioAnalysisService>,
        request: CreateLearningPlanRequest,
    ) -> Result<LearningPlan, String> {
        match service.create_learning_plan(
            request.user_id,
            request.target_duration_weeks,
            request.daily_practice_minutes,
        ).await {
            Ok(plan) => Ok(plan),
            Err(e) => {
                error!("Failed to create learning plan: {}", e);
                Err(format!("Failed to create plan: {}", e))
            }
        }
    }
    
    /// Handle getting reward status
    pub async fn handle_get_user_reward_status(
        service: Arc<AudioAnalysisService>,
        user_id: Uuid,
    ) -> Result<UserRewardStatus, String> {
        match service.get_user_reward_status(user_id).await {
            Ok(status) => Ok(status),
            Err(e) => {
                error!("Failed to get user reward status: {}", e);
                Err(format!("Failed to get reward status: {}", e))
            }
        }
    }
    
    /// Handle getting daily goals
    pub async fn handle_get_daily_goals(
        service: Arc<AudioAnalysisService>,
        user_id: Uuid,
    ) -> Result<Vec<DailyGoal>, String> {
        match service.generate_daily_goals(user_id).await {
            Ok(goals) => Ok(goals),
            Err(e) => {
                error!("Failed to get daily goals: {}", e);
                Err(format!("Failed to get daily goals: {}", e))
            }
        }
    }
    
    /// Handle getting gamification status
    pub async fn handle_get_gamification_status(
        service: Arc<AudioAnalysisService>,
        user_id: Uuid,
    ) -> Result<GamificationStatus, String> {
        match service.get_gamification_status(user_id).await {
            Ok(status) => Ok(status),
            Err(e) => {
                error!("Failed to get gamification status: {}", e);
                Err(format!("Failed to get gamification status: {}", e))
            }
        }
    }
}