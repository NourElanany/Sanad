use crate::models::*;
use crate::planning_algorithms::PlanningAlgorithms;
use crate::repository::KhatmaRepository;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use tracing::info;

/// Response for plan updates
#[derive(Debug, Clone, serde::Serialize)]
pub struct PlanUpdate {
    pub updated_plan: Option<KhatmaPlan>,
    pub adjustment_made: bool,
    pub next_portion: Option<DailyPortion>,
    pub encouragement_message: String,
}

/// Smart Khatma Service implementing interactive planning algorithms
pub struct SmartKhatmaService {
    repository: KhatmaRepository,
}

impl SmartKhatmaService {
    pub fn new(repository: KhatmaRepository) -> Self {
        Self { repository }
    }

    /// Create a new adaptive khatma plan
    pub async fn create_khatma_plan(
        &self,
        user_id: Uuid,
        request: CreateKhatmaPlanRequest,
    ) -> Result<KhatmaPlan> {
        info!("Creating new khatma plan for user: {}", user_id);

        // Get user's reading history to calculate reading speed
        let reading_sessions = self.repository.get_user_reading_sessions(user_id).await?;
        let reading_speed = PlanningAlgorithms::calculate_reading_speed(&reading_sessions);

        info!("Calculated reading speed: {:.1} WPM for user: {}", reading_speed, user_id);

        // Create adaptive plan using algorithms
        let plan = PlanningAlgorithms::create_adaptive_plan(
            user_id,
            request.target_date,
            &request.preferences,
            reading_speed,
        )?;

        // Save plan to database
        let saved_plan = self.repository.create_khatma_plan(&plan).await?;
        
        // Update user statistics
        self.update_user_reading_statistics(user_id, &reading_sessions).await?;

        info!("Successfully created khatma plan: {} for user: {}", saved_plan.id, user_id);
        Ok(saved_plan)
    }

    /// Update reading progress for a khatma plan
    pub async fn update_progress(
        &self,
        user_id: Uuid,
        request: UpdateProgressRequest,
    ) -> Result<PlanUpdate> {
        info!("Updating progress for plan: {}", request.khatma_plan_id);

        // Get current plan
        let mut plan = self.repository.get_khatma_plan(request.khatma_plan_id).await?;
        
        // Verify ownership
        if plan.user_id != user_id {
            return Err(anyhow!("Unauthorized access to khatma plan"));
        }

        // Record the reading session
        self.repository.create_reading_session(&request.reading_session).await?;

        // Update plan progress
        let new_progress = self.calculate_progress(&plan, &request.reading_session).await?;
        plan.current_progress = new_progress;
        plan.updated_at = Utc::now();

        // Check if plan needs adjustment
        let needs_adjustment = self.should_adjust_plan(&plan).await?;
        
        if needs_adjustment {
            // Recalculate daily portions
            let updated_plan = self.adjust_plan_automatically(&plan).await?;
            self.repository.update_khatma_plan(&updated_plan).await?;
            
            Ok(PlanUpdate {
                updated_plan: Some(updated_plan),
                adjustment_made: true,
                next_portion: self.get_next_daily_portion(&plan).await?,
                encouragement_message: self.generate_encouragement_message(&plan).await?,
            })
        } else {
            self.repository.update_khatma_plan(&plan).await?;
            
            Ok(PlanUpdate {
                updated_plan: Some(plan.clone()),
                adjustment_made: false,
                next_portion: self.get_next_daily_portion(&plan).await?,
                encouragement_message: self.generate_encouragement_message(&plan).await?,
            })
        }
    }

    /// Get smart reminders for a user
    pub async fn get_smart_reminders(&self, user_id: Uuid) -> Result<Vec<SmartReminder>> {
        info!("Generating smart reminders for user: {}", user_id);

        // Get user's active plans
        let active_plans = self.repository.get_user_khatma_plans(user_id, Some(KhatmaStatus::Active)).await?;
        
        if active_plans.is_empty() {
            return Ok(vec![]);
        }

        let mut reminders = Vec::new();

        for plan in &active_plans {
            // Check if user is behind schedule
            if self.is_behind_schedule(plan).await? {
                let portion = DailyPortion {
                    date: Utc::now(),
                    surah_start: 1,
                    ayah_start: 1,
                    surah_end: 1,
                    ayah_end: 7,
                    estimated_minutes: 30,
                    word_count: 100,
                    completed: false,
                    actual_reading_time: None,
                    completion_date: None,
                };

                reminders.push(SmartReminder {
                    id: Uuid::new_v4(),
                    user_id,
                    khatma_plan_id: plan.id,
                    suggested_time: Utc::now(),
                    duration_minutes: 30,
                    portion,
                    confidence_score: 0.8,
                    reasoning: "You're a bit behind schedule. A short reading session now can help you catch up!".to_string(),
                    created_at: Utc::now(),
                });
            }
        }

        Ok(reminders)
    }

    /// Generate comprehensive statistics for a completed khatma
    pub async fn generate_statistics(&self, khatma_plan_id: Uuid) -> Result<KhatmaStatistics> {
        info!("Generating statistics for khatma: {}", khatma_plan_id);

        let plan = self.repository.get_khatma_plan(khatma_plan_id).await?;
        let reading_sessions = self.repository.get_plan_reading_sessions(khatma_plan_id).await?;

        if plan.status != KhatmaStatus::Completed {
            return Err(anyhow!("Cannot generate statistics for incomplete khatma"));
        }

        let total_reading_time = reading_sessions.iter()
            .filter_map(|s| s.duration_minutes)
            .sum::<i32>();

        let consistency_score = self.calculate_consistency_score(&reading_sessions);
        let reading_speed_improvement = self.calculate_speed_improvement(&reading_sessions);

        Ok(KhatmaStatistics {
            khatma_plan_id,
            completion_date: plan.updated_at,
            planned_duration_days: (plan.target_date - plan.start_date).num_days() as i32,
            actual_duration_days: (plan.updated_at - plan.start_date).num_days() as i32,
            total_reading_time_minutes: total_reading_time,
            average_daily_reading_minutes: if (plan.updated_at - plan.start_date).num_days() > 0 {
                total_reading_time as f64 / (plan.updated_at - plan.start_date).num_days() as f64
            } else {
                0.0
            },
            consistency_score,
            reading_speed_improvement,
            portions_completed_on_time: 0, // Would calculate from actual data
            portions_completed_late: 0,    // Would calculate from actual data
            portions_skipped: 0,           // Would calculate from actual data
            achievements: vec![],          // Would populate with actual achievements
        })
    }

    /// Get active khatma plans for a user
    pub async fn get_user_active_plans(&self, user_id: Uuid) -> Result<Vec<KhatmaPlan>> {
        self.repository.get_user_khatma_plans(user_id, Some(KhatmaStatus::Active)).await
    }

    // Helper methods
    async fn update_user_reading_statistics(&self, _user_id: Uuid, _sessions: &[ReadingSession]) -> Result<()> {
        // Implementation for updating user statistics
        Ok(())
    }

    async fn calculate_progress(&self, _plan: &KhatmaPlan, _session: &ReadingSession) -> Result<f64> {
        // Implementation for calculating progress
        Ok(0.0)
    }

    async fn should_adjust_plan(&self, _plan: &KhatmaPlan) -> Result<bool> {
        // Implementation for checking if plan needs adjustment
        Ok(false)
    }

    async fn adjust_plan_automatically(&self, plan: &KhatmaPlan) -> Result<KhatmaPlan> {
        // Implementation for automatic plan adjustment
        Ok(plan.clone())
    }

    async fn get_next_daily_portion(&self, _plan: &KhatmaPlan) -> Result<Option<DailyPortion>> {
        // Implementation for getting next daily portion
        Ok(None)
    }

    async fn generate_encouragement_message(&self, _plan: &KhatmaPlan) -> Result<String> {
        // Implementation for generating encouragement messages
        Ok("Keep up the great work!".to_string())
    }

    async fn is_behind_schedule(&self, _plan: &KhatmaPlan) -> Result<bool> {
        // Implementation for checking if behind schedule
        Ok(false)
    }

    fn calculate_consistency_score(&self, _sessions: &[ReadingSession]) -> f64 {
        // Implementation for calculating consistency score
        0.8
    }

    fn calculate_speed_improvement(&self, _sessions: &[ReadingSession]) -> f64 {
        // Implementation for calculating speed improvement
        0.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository::MockKhatmaRepository;

    #[tokio::test]
    async fn test_create_khatma_plan() {
        let repository = MockKhatmaRepository::new();
        let service = SmartKhatmaService::new(repository);
        
        let user_id = Uuid::new_v4();
        let request = CreateKhatmaPlanRequest {
            target_date: Utc::now() + chrono::Duration::days(30),
            preferences: KhatmaPreferences {
                target_completion_days: Some(30),
                daily_reading_time_minutes: Some(60),
                preferred_reading_times: vec![],
                adaptive_scheduling: true,
                reminder_settings: ReminderSettings {
                    enabled: true,
                    advance_minutes: 15,
                    smart_timing: true,
                    missed_reading_reminder: true,
                    progress_updates: true,
                },
                difficulty_preference: DifficultyPreference::Medium,
            },
        };

        let result = service.create_khatma_plan(user_id, request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_smart_reminders() {
        let repository = MockKhatmaRepository::new();
        let service = SmartKhatmaService::new(repository);
        
        let user_id = Uuid::new_v4();
        let result = service.get_smart_reminders(user_id).await;
        assert!(result.is_ok());
    }
}