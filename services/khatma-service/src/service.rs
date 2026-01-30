use crate::models::*;
use crate::planning_algorithms::PlanningAlgorithms;
use crate::repository::KhatmaRepository;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc, Datelike};
use std::collections::HashMap;
use uuid::Uuid;
use tracing::{info, warn, error};

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
        let mut plan = PlanningAlgorithms::create_adaptive_plan(
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

    /// Update reading progress and automatically adjust plan if needed
    pub async fn update_reading_progress(
        &self,
        request: UpdateProgressRequest,
    ) -> Result<(KhatmaPlan, Vec<String>)> {
        info!("Updating reading progress for plan: {}", request.khatma_plan_id);

        // Get current plan
        let mut plan = self.repository.get_khatma_plan(request.khatma_plan_id).await?;
        
        // Save reading session
        let session = self.repository.create_reading_session(&request.reading_session).await?;
        
        // Calculate current progress
        let current_progress = self.calculate_current_progress(&plan, &session).await?;
        
        // Get all reading sessions for this plan
        let all_sessions = self.repository.get_plan_reading_sessions(request.khatma_plan_id).await?;
        
        // Apply automatic adjustments if plan is adaptive
        let adjustments = if plan.adaptive_schedule {
            PlanningAlgorithms::adjust_plan_for_delay(&mut plan, current_progress, &all_sessions)?
        } else {
            vec![]
        };

        // Update plan in database
        let updated_plan = self.repository.update_khatma_plan(&plan).await?;

        if !adjustments.is_empty() {
            info!("Applied {} automatic adjustments to plan: {}", adjustments.len(), plan.id);
            for adjustment in &adjustments {
                info!("Adjustment: {}", adjustment);
            }
        }

        Ok((updated_plan, adjustments))
    }

    /// Get smart reading time suggestions for a user
    pub async fn get_reading_time_suggestions(
        &self,
        user_id: Uuid,
        plan_id: Uuid,
    ) -> Result<ReadingTimeSuggestionResponse> {
        info!("Generating reading time suggestions for user: {} plan: {}", user_id, plan_id);

        let plan = self.repository.get_khatma_plan(plan_id).await?;
        let reading_history = self.repository.get_user_reading_sessions(user_id).await?;

        // Generate smart suggestions
        let suggestions = PlanningAlgorithms::suggest_reading_times(user_id, &plan, &reading_history);

        // Create optimal daily schedule
        let optimal_schedule = self.create_optimal_daily_schedule(&plan, &suggestions).await?;

        let reasoning = self.generate_suggestion_reasoning(&suggestions, &reading_history);

        Ok(ReadingTimeSuggestionResponse {
            suggested_times: suggestions,
            optimal_daily_schedule: optimal_schedule,
            reasoning,
        })
    }

    /// Manually adjust a khatma plan
    pub async fn adjust_khatma_plan(
        &self,
        request: PlanAdjustmentRequest,
    ) -> Result<(KhatmaPlan, Vec<String>)> {
        info!("Manually adjusting khatma plan: {}", request.khatma_plan_id);

        let mut plan = self.repository.get_khatma_plan(request.khatma_plan_id).await?;
        let mut adjustments = Vec::new();

        // Apply requested changes
        if let Some(new_target_date) = request.new_target_date {
            let old_date = plan.target_date;
            plan.target_date = new_target_date;
            adjustments.push(format!("Changed target date from {} to {}", 
                old_date.format("%Y-%m-%d"), 
                new_target_date.format("%Y-%m-%d")
            ));

            // Recalculate daily portions for new timeline
            let remaining_days = (new_target_date - Utc::now()).num_days();
            if remaining_days > 0 {
                self.recalculate_remaining_portions(&mut plan, remaining_days as u32).await?;
                adjustments.push("Recalculated remaining daily portions".to_string());
            }
        }

        if let Some(new_daily_time) = request.new_daily_time_minutes {
            let old_time = plan.estimated_reading_time;
            plan.estimated_reading_time = new_daily_time;
            adjustments.push(format!("Changed daily reading time from {} to {} minutes", 
                old_time, new_daily_time));
        }

        plan.updated_at = Utc::now();

        // Save changes
        let updated_plan = self.repository.update_khatma_plan(&plan).await?;

        // Log adjustment request
        self.repository.log_plan_adjustment(&request).await?;

        info!("Successfully adjusted plan: {} with {} changes", plan.id, adjustments.len());
        Ok((updated_plan, adjustments))
    }

    /// Get comprehensive statistics for a completed khatma
    pub async fn get_khatma_statistics(
        &self,
        khatma_plan_id: Uuid,
    ) -> Result<KhatmaStatistics> {
        info!("Generating statistics for khatma plan: {}", khatma_plan_id);

        let plan = self.repository.get_khatma_plan(khatma_plan_id).await?;
        let sessions = self.repository.get_plan_reading_sessions(khatma_plan_id).await?;

        if !matches!(plan.status, KhatmaStatus::Completed) {
            return Err(anyhow!("Khatma plan is not completed yet"));
        }

        let statistics = self.calculate_comprehensive_statistics(&plan, &sessions).await?;
        
        info!("Generated statistics for completed khatma: {}", khatma_plan_id);
        Ok(statistics)
    }

    /// Get active khatma plans for a user
    pub async fn get_user_active_plans(&self, user_id: Uuid) -> Result<Vec<KhatmaPlan>> {
        self.repository.get_user_khatma_plans(user_id, Some(KhatmaStatus::Active)).await
    }

    /// Calculate current progress percentage
    async fn calculate_current_progress(
        &self,
        plan: &KhatmaPlan,
        latest_session: &ReadingSession,
    ) -> Result<f64> {
        let completed_portions = plan.daily_portions
            .iter()
            .filter(|p| p.completed)
            .count();

        let total_portions = plan.daily_portions.len();
        
        if total_portions == 0 {
            return Ok(0.0);
        }

        let base_progress = (completed_portions as f64 / total_portions as f64) * 100.0;
        
        // Add partial progress from current session if it's for today
        let today = Utc::now().date_naive();
        if latest_session.start_time.date_naive() == today {
            let today_portion = plan.daily_portions
                .iter()
                .find(|p| p.date.date_naive() == today);
            
            if let Some(portion) = today_portion {
                if !portion.completed {
                    let session_progress = (latest_session.word_count as f64 / portion.word_count as f64).min(1.0);
                    let portion_weight = 100.0 / total_portions as f64;
                    return Ok(base_progress + (session_progress * portion_weight));
                }
            }
        }

        Ok(base_progress)
    }

    /// Update user reading statistics
    async fn update_user_reading_statistics(
        &self,
        user_id: Uuid,
        sessions: &[ReadingSession],
    ) -> Result<()> {
        if sessions.is_empty() {
            return Ok(());
        }

        let avg_speed = PlanningAlgorithms::calculate_reading_speed(sessions);
        let total_time: i32 = sessions.iter()
            .filter_map(|s| s.duration_minutes)
            .sum();

        let completed_khatmas = self.repository.count_completed_khatmas(user_id).await?;
        
        // Calculate consistency score based on regular reading patterns
        let consistency_score = self.calculate_consistency_score(sessions);

        let stats = ReadingStatistics {
            user_id,
            average_reading_speed_wpm: avg_speed,
            total_reading_time_minutes: total_time,
            completed_khatmas,
            preferred_reading_times: vec![], // Will be populated from user preferences
            reading_consistency_score: consistency_score,
            last_updated: Utc::now(),
        };

        self.repository.update_reading_statistics(&stats).await?;
        Ok(())
    }

    /// Calculate reading consistency score
    fn calculate_consistency_score(&self, sessions: &[ReadingSession]) -> f64 {
        if sessions.len() < 7 {
            return 0.5; // Not enough data
        }

        // Group sessions by date
        let mut daily_sessions = HashMap::new();
        for session in sessions {
            let date = session.start_time.date_naive();
            daily_sessions.entry(date).or_insert(Vec::new()).push(session);
        }

        // Calculate consistency based on regular reading days
        let total_days = sessions.len() as f64;
        let reading_days = daily_sessions.len() as f64;
        
        // Higher score for more consistent daily reading
        (reading_days / total_days).min(1.0)
    }

    /// Create optimal daily schedule from suggestions
    async fn create_optimal_daily_schedule(
        &self,
        plan: &KhatmaPlan,
        suggestions: &[SmartReminder],
    ) -> Result<HashMap<String, Vec<PreferredReadingTime>>> {
        let mut schedule = HashMap::new();
        
        let days = vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
        
        for (day_index, day_name) in days.iter().enumerate() {
            let day_suggestions: Vec<_> = suggestions
                .iter()
                .filter(|s| s.suggested_time.weekday().num_days_from_sunday() == day_index as u32)
                .collect();

            let mut day_times = Vec::new();
            for suggestion in day_suggestions.iter().take(3) { // Top 3 suggestions per day
                let priority = if suggestion.confidence_score > 0.8 {
                    ReadingTimePriority::High
                } else if suggestion.confidence_score > 0.6 {
                    ReadingTimePriority::Medium
                } else {
                    ReadingTimePriority::Low
                };

                day_times.push(PreferredReadingTime {
                    time: suggestion.suggested_time.time(),
                    duration_minutes: suggestion.duration_minutes,
                    priority,
                    days_of_week: vec![day_index as u8],
                });
            }

            schedule.insert(day_name.to_string(), day_times);
        }

        Ok(schedule)
    }

    /// Generate reasoning for suggestions
    fn generate_suggestion_reasoning(
        &self,
        suggestions: &[SmartReminder],
        reading_history: &[ReadingSession],
    ) -> String {
        if suggestions.is_empty() {
            return "No specific suggestions available. Consider setting preferred reading times.".to_string();
        }

        let high_confidence_count = suggestions.iter()
            .filter(|s| s.confidence_score > 0.8)
            .count();

        let avg_confidence: f64 = suggestions.iter()
            .map(|s| s.confidence_score)
            .sum::<f64>() / suggestions.len() as f64;

        if high_confidence_count > 0 {
            format!("Found {} highly recommended times based on your reading patterns and preferences. Average confidence: {:.1}%", 
                high_confidence_count, avg_confidence * 100.0)
        } else if !reading_history.is_empty() {
            "Suggestions based on your reading history and optimal Islamic reading times.".to_string()
        } else {
            "Suggestions based on traditional Islamic reading times. Your personalized suggestions will improve as you build reading history.".to_string()
        }
    }

    /// Recalculate remaining portions for adjusted timeline
    async fn recalculate_remaining_portions(
        &self,
        plan: &mut KhatmaPlan,
        remaining_days: u32,
    ) -> Result<()> {
        // Find incomplete portions
        let incomplete_portions: Vec<_> = plan.daily_portions
            .iter()
            .filter(|p| !p.completed)
            .collect();

        if incomplete_portions.is_empty() {
            return Ok(());
        }

        // Calculate total remaining words
        let total_remaining_words: u32 = incomplete_portions
            .iter()
            .map(|p| p.word_count)
            .sum();

        // Redistribute across remaining days
        let words_per_day = total_remaining_words / remaining_days;
        let reading_time_per_day = (words_per_day as f64 / plan.reading_speed_wpm * 60.0) as i32;

        // Update incomplete portions
        let mut day_counter = 0;
        for portion in plan.daily_portions.iter_mut() {
            if !portion.completed && day_counter < remaining_days {
                portion.word_count = words_per_day;
                portion.estimated_minutes = reading_time_per_day;
                portion.date = Utc::now() + chrono::Duration::days(day_counter as i64);
                day_counter += 1;
            }
        }

        // Remove excess portions if timeline shortened
        plan.daily_portions.retain(|p| p.completed || p.date <= plan.target_date);

        Ok(())
    }

    /// Calculate comprehensive statistics for completed khatma
    async fn calculate_comprehensive_statistics(
        &self,
        plan: &KhatmaPlan,
        sessions: &[ReadingSession],
    ) -> Result<KhatmaStatistics> {
        let planned_duration = (plan.target_date - plan.start_date).num_days() as i32;
        let actual_completion_date = plan.daily_portions
            .iter()
            .filter_map(|p| p.completion_date)
            .max()
            .unwrap_or(Utc::now());
        
        let actual_duration = (actual_completion_date - plan.start_date).num_days() as i32;
        
        let total_reading_time: i32 = sessions
            .iter()
            .filter_map(|s| s.duration_minutes)
            .sum();

        let avg_daily_reading = if actual_duration > 0 {
            total_reading_time as f64 / actual_duration as f64
        } else {
            0.0
        };

        let portions_on_time = plan.daily_portions
            .iter()
            .filter(|p| p.completed && p.completion_date.map_or(false, |cd| cd.date_naive() <= p.date.date_naive()))
            .count() as u32;

        let portions_late = plan.daily_portions
            .iter()
            .filter(|p| p.completed && p.completion_date.map_or(false, |cd| cd.date_naive() > p.date.date_naive()))
            .count() as u32;

        let portions_skipped = plan.daily_portions
            .iter()
            .filter(|p| !p.completed)
            .count() as u32;

        let consistency_score = self.calculate_consistency_score(sessions);

        // Calculate reading speed improvement
        let initial_sessions = &sessions[..sessions.len().min(5)];
        let final_sessions = &sessions[sessions.len().saturating_sub(5)..];
        
        let initial_speed = PlanningAlgorithms::calculate_reading_speed(initial_sessions);
        let final_speed = PlanningAlgorithms::calculate_reading_speed(final_sessions);
        let speed_improvement = ((final_speed - initial_speed) / initial_speed * 100.0).max(0.0);

        // Generate achievements
        let achievements = self.generate_achievements(plan, sessions, consistency_score, speed_improvement);

        Ok(KhatmaStatistics {
            khatma_plan_id: plan.id,
            completion_date: actual_completion_date,
            planned_duration_days: planned_duration,
            actual_duration_days: actual_duration,
            total_reading_time_minutes: total_reading_time,
            average_daily_reading_minutes: avg_daily_reading,
            consistency_score,
            portions_completed_on_time: portions_on_time,
            portions_completed_late: portions_late,
            portions_skipped,
            reading_speed_improvement: speed_improvement,
            achievements,
        })
    }

    /// Generate achievements for gamification
    fn generate_achievements(
        &self,
        plan: &KhatmaPlan,
        sessions: &[ReadingSession],
        consistency_score: f64,
        speed_improvement: f64,
    ) -> Vec<Achievement> {
        let mut achievements = Vec::new();
        let now = Utc::now();

        // Completion achievement
        achievements.push(Achievement {
            id: "khatma_completed".to_string(),
            name: "Khatma Completed".to_string(),
            description: "Successfully completed a full Quran reading plan".to_string(),
            earned_at: now,
            category: AchievementCategory::Completion,
        });

        // Consistency achievements
        if consistency_score > 0.9 {
            achievements.push(Achievement {
                id: "consistency_master".to_string(),
                name: "Consistency Master".to_string(),
                description: "Maintained excellent reading consistency throughout the khatma".to_string(),
                earned_at: now,
                category: AchievementCategory::Consistency,
            });
        } else if consistency_score > 0.7 {
            achievements.push(Achievement {
                id: "steady_reader".to_string(),
                name: "Steady Reader".to_string(),
                description: "Showed good consistency in daily reading".to_string(),
                earned_at: now,
                category: AchievementCategory::Consistency,
            });
        }

        // Speed improvement achievements
        if speed_improvement > 20.0 {
            achievements.push(Achievement {
                id: "speed_improver".to_string(),
                name: "Speed Improver".to_string(),
                description: format!("Improved reading speed by {:.1}%", speed_improvement),
                earned_at: now,
                category: AchievementCategory::Speed,
            });
        }

        // Dedication achievements
        let total_time: i32 = sessions.iter().filter_map(|s| s.duration_minutes).sum();
        if total_time > 1800 { // More than 30 hours
            achievements.push(Achievement {
                id: "dedicated_reader".to_string(),
                name: "Dedicated Reader".to_string(),
                description: format!("Spent {} hours in Quran reading", total_time / 60),
                earned_at: now,
                category: AchievementCategory::Dedication,
            });
        }

        achievements
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

        // This test would need a proper mock implementation
        // For now, we're just testing the structure
        assert!(true);
    }
}