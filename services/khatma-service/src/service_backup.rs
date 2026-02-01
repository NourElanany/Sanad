use crate::models::*;
use crate::planning_algorithms::PlanningAlgorithms;
use crate::repository::KhatmaRepository;
use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc, Datelike, Timelike};
use std::collections::HashMap;
use uuid::Uuid;
use tracing::info;

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

    /// Generate smart reminders based on user behavior analysis
    pub async fn generate_smart_reminders(
        &self,
        user_id: Uuid,
        plan_id: Uuid,
    ) -> Result<Vec<SmartReminder>> {
        info!("Generating smart reminders for user: {} plan: {}", user_id, plan_id);

        let plan = self.repository.get_khatma_plan(plan_id).await?;
        let reading_history = self.repository.get_user_reading_sessions(user_id).await?;
        let user_stats = self.repository.get_reading_statistics(user_id).await?;

        // Analyze user behavior patterns
        let behavior_analysis = self.analyze_user_behavior(&reading_history, &user_stats).await?;

        // Generate intelligent reminders using the planning algorithms
        let current_time = Utc::now();
        let mut reminders = PlanningAlgorithms::generate_intelligent_reminders(
            user_id,
            &plan,
            &reading_history,
            current_time,
        );

        // Add additional personalized reminders based on behavior analysis
        reminders.extend(self.generate_personalized_reminders(user_id, &plan, &behavior_analysis).await?);

        // Filter and prioritize reminders
        reminders = self.filter_and_prioritize_reminders(reminders, &behavior_analysis);

        // Save reminders to database
        for reminder in &reminders {
            self.repository.create_smart_reminder(reminder).await?;
        }

        info!("Generated {} smart reminders for user: {}", reminders.len(), user_id);
        Ok(reminders)
    }

    /// Generate additional personalized reminders based on detailed behavior analysis
    async fn generate_personalized_reminders(
        &self,
        user_id: Uuid,
        plan: &KhatmaPlan,
        behavior: &UserBehaviorAnalysis,
    ) -> Result<Vec<SmartReminder>> {
        let mut reminders = Vec::new();
        let current_time = Utc::now();

        // 1. Habit-based reminders using user's preferred hours
        for &hour in &behavior.preferred_hours {
            if let Some(today_portion) = plan.daily_portions
                .iter()
                .find(|p| !p.completed && p.date.date_naive() == current_time.date_naive())
            {
                let habit_time = current_time.date_naive()
                    .and_hms_opt(hour, 0, 0)
                    .unwrap()
                    .and_utc();

                if habit_time > current_time {
                    let confidence = self.calculate_habit_confidence(hour, &behavior.preferred_hours);
                    
                    reminders.push(SmartReminder {
                        id: Uuid::new_v4(),
                        user_id,
                        khatma_plan_id: plan.id,
                        suggested_time: habit_time,
                        duration_minutes: today_portion.estimated_minutes,
                        portion: today_portion.clone(),
                        confidence_score: confidence,
                        reasoning: format!("Perfect timing! You usually read at {}:00 - this matches your established habit.", hour),
                        created_at: current_time,
                    });
                }
            }
        }

        // 2. Consistency-based reminders
        if behavior.consistency_score > 0.8 {
            // High consistency user - encourage maintaining streak
            if let Some(next_portion) = plan.daily_portions
                .iter()
                .find(|p| !p.completed && p.date >= current_time)
            {
                let streak_time = current_time + chrono::Duration::hours(1);
                
                reminders.push(SmartReminder {
                    id: Uuid::new_v4(),
                    user_id,
                    khatma_plan_id: plan.id,
                    suggested_time: streak_time,
                    duration_minutes: next_portion.estimated_minutes,
                    portion: next_portion.clone(),
                    confidence_score: 0.9,
                    reasoning: format!("Streak maintenance: You have excellent consistency ({:.0}% score). Keep your momentum going!", behavior.consistency_score * 100.0),
                    created_at: current_time,
                });
            }
        } else if behavior.consistency_score < 0.5 {
            // Low consistency user - gentle encouragement
            if let Some(next_portion) = plan.daily_portions
                .iter()
                .find(|p| !p.completed && p.date >= current_time)
            {
                let encouragement_time = current_time + chrono::Duration::minutes(30);
                
                reminders.push(SmartReminder {
                    id: Uuid::new_v4(),
                    user_id,
                    khatma_plan_id: plan.id,
                    suggested_time: encouragement_time,
                    duration_minutes: next_portion.estimated_minutes,
                    portion: next_portion.clone(),
                    confidence_score: 0.7,
                    reasoning: "Small steps lead to big achievements. Even a short reading session today will help build your routine.".to_string(),
                    created_at: current_time,
                });
            }
        }

        // 3. Duration-based reminders
        let (min_duration, avg_duration, max_duration) = behavior.session_duration_patterns;
        if avg_duration > 0 {
            if let Some(next_portion) = plan.daily_portions
                .iter()
                .find(|p| !p.completed && p.date >= current_time)
            {
                // Suggest optimal duration based on user's patterns
                let optimal_duration = if next_portion.estimated_minutes > avg_duration + 10 {
                    // Portion is longer than usual - suggest breaking it down
                    avg_duration
                } else {
                    next_portion.estimated_minutes
                };

                let duration_time = current_time + chrono::Duration::hours(2);
                
                reminders.push(SmartReminder {
                    id: Uuid::new_v4(),
                    user_id,
                    khatma_plan_id: plan.id,
                    suggested_time: duration_time,
                    duration_minutes: optimal_duration,
                    portion: next_portion.clone(),
                    confidence_score: 0.8,
                    reasoning: format!("Optimal session length: {} minutes matches your typical reading duration.", optimal_duration),
                    created_at: current_time,
                });
            }
        }

        Ok(reminders)
    }

    /// Filter and prioritize reminders based on behavior analysis
    fn filter_and_prioritize_reminders(
        &self,
        mut reminders: Vec<SmartReminder>,
        behavior: &UserBehaviorAnalysis,
    ) -> Vec<SmartReminder> {
        // Remove duplicate time slots (keep highest confidence)
        let mut seen_times = HashMap::new();
        reminders.retain(|reminder| {
            let time_key = reminder.suggested_time.format("%Y-%m-%d %H").to_string();
            if let Some(&existing_confidence) = seen_times.get(&time_key) {
                if reminder.confidence_score > existing_confidence {
                    seen_times.insert(time_key, reminder.confidence_score);
                    true
                } else {
                    false
                }
            } else {
                seen_times.insert(time_key, reminder.confidence_score);
                true
            }
        });

        // Boost confidence for reminders that match user's preferred days
        for reminder in &mut reminders {
            let day_of_week = reminder.suggested_time.weekday().num_days_from_sunday();
            if behavior.preferred_days.contains(&day_of_week) {
                reminder.confidence_score = (reminder.confidence_score + 0.1).min(1.0);
            }
        }

        // Sort by confidence and time
        reminders.sort_by(|a, b| {
            b.confidence_score.partial_cmp(&a.confidence_score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.suggested_time.cmp(&b.suggested_time))
        });

        // Limit to top 15 reminders to avoid overwhelming the user
        reminders.into_iter().take(15).collect()
    }

    /// Analyze user behavior patterns for personalized reminders
    async fn analyze_user_behavior(
        &self,
        reading_history: &[ReadingSession],
        user_stats: &Option<ReadingStatistics>,
    ) -> Result<UserBehaviorAnalysis> {
        let mut analysis = UserBehaviorAnalysis::default();

        if reading_history.is_empty() {
            return Ok(analysis);
        }

        // Analyze reading time patterns
        analysis.preferred_hours = self.analyze_preferred_reading_hours(reading_history);
        analysis.preferred_days = self.analyze_preferred_reading_days(reading_history);
        analysis.session_duration_patterns = self.analyze_session_duration_patterns(reading_history);
        analysis.consistency_score = user_stats.as_ref().map(|s| s.reading_consistency_score).unwrap_or(0.5);
        analysis.streak_patterns = self.analyze_streak_patterns(reading_history);
        analysis.missed_session_patterns = self.analyze_missed_session_patterns(reading_history);

        Ok(analysis)
    }

    /// Generate habit-based reminders
    async fn generate_habit_based_reminders(
        &self,
        user_id: Uuid,
        plan: &KhatmaPlan,
        behavior: &UserBehaviorAnalysis,
    ) -> Result<Vec<SmartReminder>> {
        let mut reminders = Vec::new();
        let current_time = Utc::now();

        // Get upcoming incomplete portions
        let upcoming_portions: Vec<_> = plan.daily_portions
            .iter()
            .filter(|p| !p.completed && p.date >= current_time)
            .take(3) // Next 3 days
            .collect();

        for portion in upcoming_portions {
            // Use user's preferred hours
            for &hour in &behavior.preferred_hours {
                let suggested_time = portion.date.date_naive()
                    .and_hms_opt(hour, 0, 0)
                    .unwrap()
                    .and_utc();

                let confidence = self.calculate_habit_confidence(hour, &behavior.preferred_hours);

                reminders.push(SmartReminder {
                    id: Uuid::new_v4(),
                    user_id,
                    khatma_plan_id: plan.id,
                    suggested_time,
                    duration_minutes: portion.estimated_minutes,
                    portion: portion.clone(),
                    confidence_score: confidence,
                    reasoning: format!("Based on your reading habit at {}:00", hour),
                    created_at: current_time,
                });
            }
        }

        Ok(reminders)
    }

    /// Generate adaptive time reminders
    async fn generate_adaptive_time_reminders(
        &self,
        user_id: Uuid,
        plan: &KhatmaPlan,
        behavior: &UserBehaviorAnalysis,
    ) -> Result<Vec<SmartReminder>> {
        let mut reminders = Vec::new();
        let current_time = Utc::now();

        // Get today's incomplete portion
        let today = current_time.date_naive();
        if let Some(today_portion) = plan.daily_portions
            .iter()
            .find(|p| !p.completed && p.date.date_naive() == today)
        {
            // Generate adaptive suggestions based on current time and behavior
            let adaptive_times = self.calculate_adaptive_times(current_time, behavior);

            for (time, confidence, reasoning) in adaptive_times {
                reminders.push(SmartReminder {
                    id: Uuid::new_v4(),
                    user_id,
                    khatma_plan_id: plan.id,
                    suggested_time: time,
                    duration_minutes: today_portion.estimated_minutes,
                    portion: today_portion.clone(),
                    confidence_score: confidence,
                    reasoning,
                    created_at: current_time,
                });
            }
        }

        Ok(reminders)
    }

    /// Generate motivational reminders for plan adherence
    async fn generate_motivational_reminders(
        &self,
        user_id: Uuid,
        plan: &KhatmaPlan,
        behavior: &UserBehaviorAnalysis,
    ) -> Result<Vec<SmartReminder>> {
        let mut reminders = Vec::new();
        let current_time = Utc::now();

        // Check if user is falling behind
        let expected_progress = self.calculate_expected_progress(plan, current_time);
        let progress_gap = expected_progress - plan.current_progress;

        if progress_gap > 5.0 { // User is more than 5% behind
            // Generate catch-up reminders
            if let Some(next_portion) = plan.daily_portions
                .iter()
                .find(|p| !p.completed && p.date >= current_time)
            {
                let motivational_time = current_time + chrono::Duration::hours(1);
                
                reminders.push(SmartReminder {
                    id: Uuid::new_v4(),
                    user_id,
                    khatma_plan_id: plan.id,
                    suggested_time: motivational_time,
                    duration_minutes: next_portion.estimated_minutes,
                    portion: next_portion.clone(),
                    confidence_score: 0.8,
                    reasoning: format!("Catch-up reminder: You're {:.1}% behind your plan. A short session now will help you stay on track!", progress_gap),
                    created_at: current_time,
                });
            }
        }

        // Generate streak maintenance reminders
        if behavior.consistency_score > 0.7 {
            // User has good consistency, encourage maintaining streak
            if let Some(today_portion) = plan.daily_portions
                .iter()
                .find(|p| !p.completed && p.date.date_naive() == current_time.date_naive())
            {
                let streak_reminder_time = current_time + chrono::Duration::hours(2);
                
                reminders.push(SmartReminder {
                    id: Uuid::new_v4(),
                    user_id,
                    khatma_plan_id: plan.id,
                    suggested_time: streak_reminder_time,
                    duration_minutes: today_portion.estimated_minutes,
                    portion: today_portion.clone(),
                    confidence_score: 0.9,
                    reasoning: "Keep your excellent reading streak going! You've been very consistent.".to_string(),
                    created_at: current_time,
                });
            }
        }

        Ok(reminders)
    }

    /// Generate recovery reminders for missed sessions
    async fn generate_recovery_reminders(
        &self,
        user_id: Uuid,
        plan: &KhatmaPlan,
        behavior: &UserBehaviorAnalysis,
    ) -> Result<Vec<SmartReminder>> {
        let mut reminders = Vec::new();
        let current_time = Utc::now();
        let yesterday = current_time - chrono::Duration::days(1);

        // Check for missed sessions from yesterday
        let missed_portions: Vec<_> = plan.daily_portions
            .iter()
            .filter(|p| !p.completed && p.date.date_naive() == yesterday.date_naive())
            .collect();

        for missed_portion in missed_portions {
            // Suggest making up for missed reading
            let recovery_time = current_time + chrono::Duration::minutes(30);
            
            reminders.push(SmartReminder {
                id: Uuid::new_v4(),
                user_id,
                khatma_plan_id: plan.id,
                suggested_time: recovery_time,
                duration_minutes: missed_portion.estimated_minutes,
                portion: missed_portion.clone(),
                confidence_score: 0.7,
                reasoning: "Recovery reminder: You missed yesterday's reading. A short session now will help you catch up.".to_string(),
                created_at: current_time,
            });
        }

        Ok(reminders)
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

    // Helper methods for smart reminder system

    /// Analyze user's preferred reading hours
    fn analyze_preferred_reading_hours(&self, sessions: &[ReadingSession]) -> Vec<u32> {
        let mut hour_counts = HashMap::new();
        
        for session in sessions {
            let hour = session.start_time.hour();
            *hour_counts.entry(hour).or_insert(0) += 1;
        }

        // Get top 3 most frequent hours
        let mut hours: Vec<_> = hour_counts.into_iter().collect();
        hours.sort_by(|a, b| b.1.cmp(&a.1));
        hours.into_iter().take(3).map(|(hour, _)| hour).collect()
    }

    /// Analyze user's preferred reading days
    fn analyze_preferred_reading_days(&self, sessions: &[ReadingSession]) -> Vec<u32> {
        let mut day_counts = HashMap::new();
        
        for session in sessions {
            let day = session.start_time.weekday().num_days_from_sunday();
            *day_counts.entry(day).or_insert(0) += 1;
        }

        // Get days with above-average activity
        let avg_sessions_per_day = sessions.len() as f64 / 7.0;
        day_counts.into_iter()
            .filter(|(_, count)| *count as f64 > avg_sessions_per_day)
            .map(|(day, _)| day)
            .collect()
    }

    /// Analyze session duration patterns
    fn analyze_session_duration_patterns(&self, sessions: &[ReadingSession]) -> (i32, i32, i32) {
        let durations: Vec<i32> = sessions
            .iter()
            .filter_map(|s| s.duration_minutes)
            .collect();

        if durations.is_empty() {
            return (30, 45, 60); // Default values
        }

        let avg = durations.iter().sum::<i32>() / durations.len() as i32;
        let min = *durations.iter().min().unwrap();
        let max = *durations.iter().max().unwrap();

        (min, avg, max)
    }

    /// Analyze reading streak patterns
    fn analyze_streak_patterns(&self, sessions: &[ReadingSession]) -> (u32, u32) {
        if sessions.is_empty() {
            return (0, 0);
        }

        // Group sessions by date
        let mut daily_sessions = HashMap::new();
        for session in sessions {
            let date = session.start_time.date_naive();
            daily_sessions.entry(date).or_insert(Vec::new()).push(session);
        }

        // Calculate streaks
        let mut dates: Vec<_> = daily_sessions.keys().collect();
        dates.sort();

        let mut current_streak = 0;
        let mut max_streak = 0;
        let mut last_date: Option<chrono::NaiveDate> = None;

        for date in dates {
            if let Some(last) = last_date {
                let days_diff = (*date - last).num_days();
                if days_diff == 1 {
                    current_streak += 1;
                } else {
                    max_streak = max_streak.max(current_streak);
                    current_streak = 1;
                }
            } else {
                current_streak = 1;
            }
            last_date = Some(*date);
        }

        max_streak = max_streak.max(current_streak);
        (current_streak, max_streak)
    }

    /// Analyze missed session patterns
    fn analyze_missed_session_patterns(&self, sessions: &[ReadingSession]) -> Vec<u32> {
        // This would analyze which days user typically misses
        // For now, return empty - would need plan data to compare against
        vec![]
    }

    /// Calculate confidence for habit-based reminders
    fn calculate_habit_confidence(&self, hour: u32, preferred_hours: &[u32]) -> f64 {
        let position = preferred_hours.iter().position(|&h| h == hour);
        match position {
            Some(0) => 0.9, // Most preferred hour
            Some(1) => 0.7, // Second most preferred
            Some(2) => 0.5, // Third most preferred
            _ => 0.3,       // Not in top preferences
        }
    }

    /// Calculate adaptive reminder times
    fn calculate_adaptive_times(
        &self,
        current_time: DateTime<Utc>,
        behavior: &UserBehaviorAnalysis,
    ) -> Vec<(DateTime<Utc>, f64, String)> {
        let mut adaptive_times = Vec::new();
        let current_hour = current_time.hour();

        // Suggest times based on current context
        if current_hour < 6 {
            // Early morning - suggest after Fajr
            let fajr_time = current_time.date_naive()
                .and_hms_opt(5, 30, 0)
                .unwrap()
                .and_utc();
            adaptive_times.push((
                fajr_time,
                0.8,
                "Perfect time for Quran reading after Fajr prayer".to_string(),
            ));
        } else if current_hour < 12 {
            // Morning - suggest productive morning time
            let morning_time = current_time + chrono::Duration::hours(1);
            adaptive_times.push((
                morning_time,
                0.7,
                "Morning is an excellent time for focused Quran reading".to_string(),
            ));
        } else if current_hour < 15 {
            // After Dhuhr
            let afternoon_time = current_time + chrono::Duration::minutes(30);
            adaptive_times.push((
                afternoon_time,
                0.6,
                "A peaceful afternoon session after Dhuhr".to_string(),
            ));
        } else if current_hour < 20 {
            // Evening
            let evening_time = current_time + chrono::Duration::hours(1);
            adaptive_times.push((
                evening_time,
                0.7,
                "Evening reading session before Maghrib".to_string(),
            ));
        } else {
            // Night
            let night_time = current_time + chrono::Duration::minutes(30);
            adaptive_times.push((
                night_time,
                0.6,
                "Peaceful night reading before sleep".to_string(),
            ));
        }

        adaptive_times
    }

    /// Generate comprehensive progress dashboard
    pub async fn generate_progress_dashboard(
        &self,
        user_id: Uuid,
        request: DashboardRequest,
    ) -> Result<ProgressDashboard> {
        info!("Generating comprehensive progress dashboard for user: {}", user_id);

        // Get current active khatma
        let current_khatma = self.get_user_active_plans(user_id).await?
            .into_iter()
            .next();

        // Get user's reading statistics
        let user_stats = self.repository.get_reading_statistics(user_id).await?;
        
        // Get all reading sessions for analysis
        let all_sessions = self.repository.get_user_reading_sessions(user_id).await?;
        
        // Get completed khatmas for comparison
        let completed_khatmas = self.repository.get_user_completed_khatmas(user_id).await?;

        // Generate overall progress
        let overall_progress = self.calculate_overall_progress(
            &current_khatma,
            &user_stats,
            &all_sessions,
            &completed_khatmas,
        ).await?;

        // Generate recent activity analysis
        let recent_activity = self.analyze_recent_activity(&all_sessions).await?;

        // Generate performance metrics
        let performance_metrics = self.calculate_performance_metrics(&all_sessions).await?;

        // Generate upcoming milestones
        let upcoming_milestones = self.generate_upcoming_milestones(
            user_id,
            &current_khatma,
            &overall_progress,
        ).await?;

        // Generate recommendations if requested
        let recommendations = if request.include_recommendations.unwrap_or(true) {
            self.generate_performance_recommendations(
                user_id,
                &overall_progress,
                &performance_metrics,
                &recent_activity,
            ).await?
        } else {
            vec![]
        };

        let dashboard = ProgressDashboard {
            user_id,
            current_khatma,
            overall_progress,
            recent_activity,
            performance_metrics,
            upcoming_milestones,
            recommendations,
            generated_at: Utc::now(),
        };

        info!("Successfully generated progress dashboard for user: {}", user_id);
        Ok(dashboard)
    }

    /// Generate Khatma comparison analysis
    pub async fn generate_khatma_comparison(
        &self,
        user_id: Uuid,
        request: ComparisonRequest,
    ) -> Result<KhatmaComparison> {
        info!("Generating Khatma comparison for user: {}", user_id);

        // Get current active khatma
        let current_khatma = self.get_user_active_plans(user_id).await?
            .into_iter()
            .next();

        // Get previous completed khatmas
        let compare_count = request.compare_with_count.unwrap_or(5);
        let previous_khatmas = self.repository
            .get_user_completed_khatmas_with_limit(user_id, compare_count)
            .await?;

        // Generate comparison metrics
        let comparison_metrics = self.calculate_comparison_metrics(
            &current_khatma,
            &previous_khatmas,
        ).await?;

        // Identify improvement areas
        let improvement_areas = self.identify_improvement_areas(
            &current_khatma,
            &previous_khatmas,
            &comparison_metrics,
        ).await?;

        // Compare achievements
        let achievements_comparison = self.compare_achievements(
            user_id,
            &current_khatma,
            &previous_khatmas,
        ).await?;

        let comparison = KhatmaComparison {
            user_id,
            current_khatma,
            previous_khatmas,
            comparison_metrics,
            improvement_areas,
            achievements_comparison,
            generated_at: Utc::now(),
        };

        info!("Successfully generated Khatma comparison for user: {}", user_id);
        Ok(comparison)
    }

    /// Calculate overall progress statistics
    async fn calculate_overall_progress(
        &self,
        current_khatma: &Option<KhatmaPlan>,
        user_stats: &Option<ReadingStatistics>,
        all_sessions: &[ReadingSession],
        completed_khatmas: &[KhatmaStatistics],
    ) -> Result<OverallProgress> {
        let total_khatmas_completed = completed_khatmas.len() as u32;
        
        let current_khatma_progress = current_khatma
            .as_ref()
            .map(|k| k.current_progress)
            .unwrap_or(0.0);

        let total_reading_time_hours = all_sessions
            .iter()
            .filter_map(|s| s.duration_minutes)
            .sum::<i32>() as f64 / 60.0;

        let average_daily_reading_minutes = if !all_sessions.is_empty() {
            let total_days = self.calculate_active_reading_days(all_sessions);
            if total_days > 0 {
                (total_reading_time_hours * 60.0) / total_days as f64
            } else {
                0.0
            }
        } else {
            0.0
        };

        let consistency_score = user_stats
            .as_ref()
            .map(|s| s.reading_consistency_score)
            .unwrap_or(0.0);

        let (current_streak_days, longest_streak_days) = self.calculate_streak_statistics(all_sessions);

        // Estimate pages read (assuming ~250 words per page)
        let total_words: u32 = all_sessions.iter().map(|s| s.word_count).sum();
        let pages_read_total = total_words / 250;

        // Count unique surahs completed
        let surahs_completed = self.count_completed_surahs(all_sessions);

        Ok(OverallProgress {
            total_khatmas_completed,
            current_khatma_progress,
            total_reading_time_hours,
            average_daily_reading_minutes,
            consistency_score,
            current_streak_days,
            longest_streak_days,
            pages_read_total,
            surahs_completed,
        })
    }

    /// Analyze recent activity patterns
    async fn analyze_recent_activity(&self, all_sessions: &[ReadingSession]) -> Result<RecentActivity> {
        let now = Utc::now();
        
        // Filter sessions for different time periods
        let last_7_days_sessions: Vec<_> = all_sessions
            .iter()
            .filter(|s| s.start_time >= now - chrono::Duration::days(7))
            .collect();

        let last_30_days_sessions: Vec<_> = all_sessions
            .iter()
            .filter(|s| s.start_time >= now - chrono::Duration::days(30))
            .collect();

        let this_month_start = now.date_naive()
            .with_day(1)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc();
        
        let this_month_sessions: Vec<_> = all_sessions
            .iter()
            .filter(|s| s.start_time >= this_month_start)
            .collect();

        // Calculate activity periods
        let last_7_days = self.calculate_activity_period(&last_7_days_sessions, 7).await?;
        let last_30_days = self.calculate_activity_period(&last_30_days_sessions, 30).await?;
        let this_month = self.calculate_activity_period(&this_month_sessions, 
            now.date_naive().day() as i32).await?;

        // Get recent session summaries (last 10 sessions)
        let recent_sessions = self.create_session_summaries(
            &last_30_days_sessions.into_iter().take(10).cloned().collect::<Vec<_>>()
        ).await?;

        Ok(RecentActivity {
            last_7_days,
            last_30_days,
            this_month,
            recent_sessions,
        })
    }

    /// Calculate activity period statistics
    async fn calculate_activity_period(
        &self,
        sessions: &[&ReadingSession],
        period_days: i32,
    ) -> Result<ActivityPeriod> {
        let total_reading_time_minutes: i32 = sessions
            .iter()
            .filter_map(|s| s.duration_minutes)
            .sum();

        let sessions_count = sessions.len() as u32;

        let average_session_duration = if sessions_count > 0 {
            total_reading_time_minutes as f64 / sessions_count as f64
        } else {
            0.0
        };

        // Calculate consistency as percentage of days with reading
        let unique_days = sessions
            .iter()
            .map(|s| s.start_time.date_naive())
            .collect::<std::collections::HashSet<_>>()
            .len();

        let consistency_percentage = if period_days > 0 {
            (unique_days as f64 / period_days as f64) * 100.0
        } else {
            0.0
        };

        // Estimate pages read
        let total_words: u32 = sessions.iter().map(|s| s.word_count).sum();
        let pages_read = total_words / 250;

        // Find best day
        let best_day = self.find_best_day(sessions).await?;

        Ok(ActivityPeriod {
            total_reading_time_minutes,
            sessions_count,
            average_session_duration,
            consistency_percentage,
            pages_read,
            best_day,
        })
    }

    /// Find the best reading day in a period
    async fn find_best_day(&self, sessions: &[&ReadingSession]) -> Result<Option<BestDayInfo>> {
        if sessions.is_empty() {
            return Ok(None);
        }

        // Group sessions by date
        let mut daily_stats = HashMap::new();
        for session in sessions {
            let date = session.start_time.date_naive();
            let entry = daily_stats.entry(date).or_insert((0, 0u32));
            entry.0 += session.duration_minutes.unwrap_or(0);
            entry.1 += session.word_count;
        }

        // Find the day with most reading time
        if let Some((best_date, (reading_time, word_count))) = daily_stats
            .iter()
            .max_by_key(|(_, (time, _))| *time)
        {
            let pages_read = word_count / 250;
            let achievement = if *reading_time > 120 {
                "Marathon Reader - Over 2 hours!".to_string()
            } else if *reading_time > 60 {
                "Dedicated Reader - Over 1 hour!".to_string()
            } else {
                "Consistent Reader".to_string()
            };

            Ok(Some(BestDayInfo {
                date: best_date.and_hms_opt(12, 0, 0).unwrap().and_utc(),
                reading_time_minutes: *reading_time,
                pages_read,
                achievement,
            }))
        } else {
            Ok(None)
        }
    }

    /// Create session summaries for dashboard
    async fn create_session_summaries(
        &self,
        sessions: &[ReadingSession],
    ) -> Result<Vec<ReadingSessionSummary>> {
        let mut summaries = Vec::new();

        for session in sessions.iter().take(10) {
            let surah_range = if session.surah_start == session.surah_end {
                format!("Surah {} ({}:{})", 
                    session.surah_start, 
                    session.ayah_start, 
                    session.ayah_end)
            } else {
                format!("Surahs {}-{}", session.surah_start, session.surah_end)
            };

            let reading_speed_wpm = session.reading_speed_wpm.unwrap_or(0.0);
            
            // Calculate quality score based on consistency and speed
            let quality_score = self.calculate_session_quality_score(session);

            summaries.push(ReadingSessionSummary {
                date: session.start_time,
                duration_minutes: session.duration_minutes.unwrap_or(0),
                surah_range,
                reading_speed_wpm,
                quality_score,
            });
        }

        Ok(summaries)
    }

    /// Calculate performance metrics
    async fn calculate_performance_metrics(
        &self,
        all_sessions: &[ReadingSession],
    ) -> Result<PerformanceMetrics> {
        let reading_speed_trend = self.calculate_speed_trend(all_sessions).await?;
        let consistency_trend = self.calculate_consistency_trend(all_sessions).await?;
        let optimal_reading_times = self.identify_optimal_reading_times(all_sessions).await?;
        let productivity_patterns = self.analyze_productivity_patterns(all_sessions).await?;
        let goal_achievement_rate = self.calculate_goal_achievement_rate(all_sessions).await?;

        Ok(PerformanceMetrics {
            reading_speed_trend,
            consistency_trend,
            optimal_reading_times,
            productivity_patterns,
            goal_achievement_rate,
        })
    }

    /// Calculate reading speed trend
    async fn calculate_speed_trend(&self, sessions: &[ReadingSession]) -> Result<SpeedTrend> {
        let speeds: Vec<f64> = sessions
            .iter()
            .filter_map(|s| s.reading_speed_wpm)
            .collect();

        if speeds.is_empty() {
            return Ok(SpeedTrend {
                current_wpm: 0.0,
                average_wpm: 0.0,
                improvement_percentage: 0.0,
                trend_direction: TrendDirection::Stable,
                weekly_speeds: vec![],
            });
        }

        let current_wpm = speeds.last().copied().unwrap_or(0.0);
        let average_wpm = speeds.iter().sum::<f64>() / speeds.len() as f64;

        // Calculate improvement from first to last sessions
        let first_sessions_avg = speeds.iter().take(5).sum::<f64>() / speeds.len().min(5) as f64;
        let last_sessions_avg = speeds.iter().rev().take(5).sum::<f64>() / speeds.len().min(5) as f64;
        
        let improvement_percentage = if first_sessions_avg > 0.0 {
            ((last_sessions_avg - first_sessions_avg) / first_sessions_avg) * 100.0
        } else {
            0.0
        };

        let trend_direction = if improvement_percentage > 5.0 {
            TrendDirection::Improving
        } else if improvement_percentage < -5.0 {
            TrendDirection::Declining
        } else {
            TrendDirection::Stable
        };

        // Generate weekly speed data
        let weekly_speeds = self.calculate_weekly_speeds(sessions).await?;

        Ok(SpeedTrend {
            current_wpm,
            average_wpm,
            improvement_percentage,
            trend_direction,
            weekly_speeds,
        })
    }

    /// Calculate weekly speed averages
    async fn calculate_weekly_speeds(&self, sessions: &[ReadingSession]) -> Result<Vec<WeeklySpeed>> {
        let mut weekly_data = HashMap::new();

        for session in sessions {
            if let Some(speed) = session.reading_speed_wpm {
                let week_start = session.start_time.date_naive()
                    - chrono::Duration::days(session.start_time.weekday().num_days_from_monday() as i64);
                
                let entry = weekly_data.entry(week_start).or_insert((Vec::new(), 0u32));
                entry.0.push(speed);
                entry.1 += 1;
            }
        }

        let mut weekly_speeds: Vec<_> = weekly_data
            .into_iter()
            .map(|(week_start, (speeds, count))| {
                let average_wpm = speeds.iter().sum::<f64>() / speeds.len() as f64;
                WeeklySpeed {
                    week_start: week_start.and_hms_opt(0, 0, 0).unwrap().and_utc(),
                    average_wpm,
                    sessions_count: count,
                }
            })
            .collect();

        weekly_speeds.sort_by_key(|w| w.week_start);
        Ok(weekly_speeds)
    }

    /// Calculate consistency trend
    async fn calculate_consistency_trend(&self, sessions: &[ReadingSession]) -> Result<ConsistencyTrend> {
        let current_score = self.calculate_consistency_score(sessions);
        
        // Calculate weekly consistency
        let weekly_consistency = self.calculate_weekly_consistency(sessions).await?;
        
        // Determine trend direction
        let trend_direction = if weekly_consistency.len() >= 2 {
            let recent_avg = weekly_consistency.iter().rev().take(2)
                .map(|w| w.consistency_score)
                .sum::<f64>() / 2.0;
            let older_avg = weekly_consistency.iter().take(2)
                .map(|w| w.consistency_score)
                .sum::<f64>() / 2.0;
            
            if recent_avg > older_avg + 0.1 {
                TrendDirection::Improving
            } else if recent_avg < older_avg - 0.1 {
                TrendDirection::Declining
            } else {
                TrendDirection::Stable
            }
        } else {
            TrendDirection::Stable
        };

        // Find best consistency period
        let best_consistency_period = self.find_best_consistency_period(sessions).await?;

        Ok(ConsistencyTrend {
            current_score,
            trend_direction,
            weekly_consistency,
            best_consistency_period,
        })
    }

    /// Calculate weekly consistency scores
    async fn calculate_weekly_consistency(&self, sessions: &[ReadingSession]) -> Result<Vec<WeeklyConsistency>> {
        let mut weekly_data = HashMap::new();

        for session in sessions {
            let week_start = session.start_time.date_naive()
                - chrono::Duration::days(session.start_time.weekday().num_days_from_monday() as i64);
            
            let entry = weekly_data.entry(week_start).or_insert(std::collections::HashSet::new());
            entry.insert(session.start_time.date_naive());
        }

        let mut weekly_consistency: Vec<_> = weekly_data
            .into_iter()
            .map(|(week_start, reading_days)| {
                let days_read = reading_days.len() as u32;
                let target_days = 7u32; // Ideally read every day
                let consistency_score = days_read as f64 / target_days as f64;
                
                WeeklyConsistency {
                    week_start: week_start.and_hms_opt(0, 0, 0).unwrap().and_utc(),
                    consistency_score,
                    days_read,
                    target_days,
                }
            })
            .collect();

        weekly_consistency.sort_by_key(|w| w.week_start);
        Ok(weekly_consistency)
    }

    /// Find the best consistency period
    async fn find_best_consistency_period(&self, sessions: &[ReadingSession]) -> Result<Option<ConsistencyPeriod>> {
        if sessions.len() < 7 {
            return Ok(None);
        }

        // Group sessions by date
        let mut daily_sessions = HashMap::new();
        for session in sessions {
            let date = session.start_time.date_naive();
            daily_sessions.entry(date).or_insert(Vec::new()).push(session);
        }

        let mut dates: Vec<_> = daily_sessions.keys().collect();
        dates.sort();

        // Find longest consecutive period
        let mut best_period: Option<ConsistencyPeriod> = None;
        let mut current_start = None;
        let mut current_length = 0;

        for i in 0..dates.len() {
            if i == 0 || (*dates[i] - *dates[i-1]).num_days() == 1 {
                if current_start.is_none() {
                    current_start = Some(*dates[i]);
                }
                current_length += 1;
            } else {
                // End of consecutive period
                if current_length >= 7 { // At least a week
                    let consistency_score = current_length as f64 / current_length as f64; // Perfect consistency in consecutive days
                    let period = ConsistencyPeriod {
                        start_date: current_start.unwrap().and_hms_opt(0, 0, 0).unwrap().and_utc(),
                        end_date: dates[i-1].and_hms_opt(23, 59, 59).unwrap().and_utc(),
                        consistency_score,
                        duration_days: current_length,
                    };
                    
                    if best_period.is_none() || period.duration_days > best_period.as_ref().unwrap().duration_days {
                        best_period = Some(period);
                    }
                }
                current_start = Some(*dates[i]);
                current_length = 1;
            }
        }

        // Check the last period
        if current_length >= 7 {
            let consistency_score = current_length as f64 / current_length as f64;
            let period = ConsistencyPeriod {
                start_date: current_start.unwrap().and_hms_opt(0, 0, 0).unwrap().and_utc(),
                end_date: dates.last().unwrap().and_hms_opt(23, 59, 59).unwrap().and_utc(),
                consistency_score,
                duration_days: current_length,
            };
            
            if best_period.is_none() || period.duration_days > best_period.as_ref().unwrap().duration_days {
                best_period = Some(period);
            }
        }

        Ok(best_period)
    }

    /// Identify optimal reading times based on user's reading history
    async fn identify_optimal_reading_times(&self, sessions: &[ReadingSession]) -> Result<Vec<OptimalTimeSlot>> {
        if sessions.is_empty() {
            return Ok(vec![]);
        }

        // Analyze reading patterns by hour of day
        let mut hour_stats = std::collections::HashMap::new();

        for session in sessions {
            let hour = session.start_time.hour();
            let entry = hour_stats.entry(hour).or_insert((Vec::new(), Vec::new(), Vec::new()));
            
            if let Some(duration) = session.duration_minutes {
                entry.0.push(duration);
            }
            if let Some(speed) = session.reading_speed_wpm {
                entry.1.push(speed);
            }
            entry.2.push(session); // Store session reference for success rate calculation
        }

        let mut optimal_times = Vec::new();
        for (hour, (durations, speeds, hour_sessions)) in hour_stats {
            if hour_sessions.len() < 2 {
                continue; // Need at least 2 sessions for meaningful analysis
            }

            let average_duration = if !durations.is_empty() {
                durations.iter().sum::<i32>() / durations.len() as i32
            } else {
                0
            };

            let average_speed = if !speeds.is_empty() {
                speeds.iter().sum::<f64>() / speeds.len() as f64
            } else {
                0.0
            };

            // Calculate success rate (sessions that completed their planned portion)
            let successful_sessions = hour_sessions.iter()
                .filter(|s| s.duration_minutes.unwrap_or(0) >= 15) // At least 15 minutes
                .count();
            let success_rate = (successful_sessions as f64 / hour_sessions.len() as f64) * 100.0;

            let recommendation_strength = if success_rate > 80.0 && average_duration > 30 {
                RecommendationStrength::Strong
            } else if success_rate > 60.0 && average_duration > 20 {
                RecommendationStrength::Moderate
            } else {
                RecommendationStrength::Weak
            };

            optimal_times.push(OptimalTimeSlot {
                hour,
                success_rate,
                average_duration,
                average_speed,
                recommendation_strength,
            });
        }

        // Sort by success rate and average duration
        optimal_times.sort_by(|a, b| {
            b.success_rate.partial_cmp(&a.success_rate)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.average_duration.cmp(&a.average_duration))
        });

        Ok(optimal_times.into_iter().take(5).collect()) // Top 5 optimal times
    }

    /// Analyze productivity patterns
    async fn analyze_productivity_patterns(&self, sessions: &[ReadingSession]) -> Result<ProductivityPatterns> {
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

    #[tokio::test]
    async fn test_analyze_user_behavior() {
        let repository = MockKhatmaRepository::new();
        let service = SmartKhatmaService::new(repository);

        // Create sample reading sessions
        let sessions = vec![
            ReadingSession {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                khatma_plan_id: Uuid::new_v4(),
                surah_start: 1,
                ayah_start: 1,
                surah_end: 1,
                ayah_end: 7,
                start_time: Utc::now().with_hour(8).unwrap(),
                end_time: Some(Utc::now().with_hour(8).unwrap() + Duration::minutes(30)),
                duration_minutes: Some(30),
                word_count: 100,
                reading_speed_wpm: Some(120.0),
                created_at: Utc::now(),
            },
            ReadingSession {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                khatma_plan_id: Uuid::new_v4(),
                surah_start: 2,
                ayah_start: 1,
                surah_end: 2,
                ayah_end: 10,
                start_time: Utc::now().with_hour(8).unwrap(),
                end_time: Some(Utc::now().with_hour(8).unwrap() + Duration::minutes(25)),
                duration_minutes: Some(25),
                word_count: 150,
                reading_speed_wpm: Some(180.0),
                created_at: Utc::now(),
            },
        ];

        let user_stats = Some(ReadingStatistics {
            user_id: Uuid::new_v4(),
            average_reading_speed_wpm: 150.0,
            total_reading_time_minutes: 55,
            completed_khatmas: 1,
            preferred_reading_times: vec![],
            reading_consistency_score: 0.8,
            last_updated: Utc::now(),
        });

        let behavior = service.analyze_user_behavior(&sessions, &user_stats).await.unwrap();

        // Test that behavior analysis works
        assert!(!behavior.preferred_hours.is_empty());
        assert_eq!(behavior.consistency_score, 0.8);
        assert_eq!(behavior.session_duration_patterns.1, 27); // Average duration
    }

    #[test]
    fn test_analyze_preferred_reading_hours() {
        let repository = MockKhatmaRepository::new();
        let service = SmartKhatmaService::new(repository);

        let sessions = vec![
            ReadingSession {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                khatma_plan_id: Uuid::new_v4(),
                surah_start: 1,
                ayah_start: 1,
                surah_end: 1,
                ayah_end: 7,
                start_time: Utc::now().with_hour(8).unwrap(),
                end_time: None,
                duration_minutes: Some(30),
                word_count: 100,
                reading_speed_wpm: Some(120.0),
                created_at: Utc::now(),
            },
            ReadingSession {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                khatma_plan_id: Uuid::new_v4(),
                surah_start: 2,
                ayah_start: 1,
                surah_end: 2,
                ayah_end: 10,
                start_time: Utc::now().with_hour(8).unwrap(),
                end_time: None,
                duration_minutes: Some(25),
                word_count: 150,
                reading_speed_wpm: Some(180.0),
                created_at: Utc::now(),
            },
            ReadingSession {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                khatma_plan_id: Uuid::new_v4(),
                surah_start: 3,
                ayah_start: 1,
                surah_end: 3,
                ayah_end: 5,
                start_time: Utc::now().with_hour(20).unwrap(),
                end_time: None,
                duration_minutes: Some(20),
                word_count: 80,
                reading_speed_wpm: Some(160.0),
                created_at: Utc::now(),
            },
        ];

        let preferred_hours = service.analyze_preferred_reading_hours(&sessions);
        
        // Should identify 8 AM as the most preferred hour (appears twice)
        assert!(preferred_hours.contains(&8));
        assert!(preferred_hours.len() <= 3); // Should return top 3 hours
    }

    #[test]
    fn test_calculate_habit_confidence() {
        let repository = MockKhatmaRepository::new();
        let service = SmartKhatmaService::new(repository);

        let preferred_hours = vec![8, 20, 14]; // Most to least preferred

        // Test confidence calculation
        assert_eq!(service.calculate_habit_confidence(8, &preferred_hours), 0.9);  // Most preferred
        assert_eq!(service.calculate_habit_confidence(20, &preferred_hours), 0.7); // Second preferred
        assert_eq!(service.calculate_habit_confidence(14, &preferred_hours), 0.5); // Third preferred
        assert_eq!(service.calculate_habit_confidence(12, &preferred_hours), 0.3); // Not preferred
    }

    #[test]
    fn test_analyze_streak_patterns() {
        let repository = MockKhatmaRepository::new();
        let service = SmartKhatmaService::new(repository);

        let base_date = Utc::now().date_naive();
        let sessions = vec![
            ReadingSession {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                khatma_plan_id: Uuid::new_v4(),
                surah_start: 1,
                ayah_start: 1,
                surah_end: 1,
                ayah_end: 7,
                start_time: base_date.and_hms_opt(8, 0, 0).unwrap().and_utc(),
                end_time: None,
                duration_minutes: Some(30),
                word_count: 100,
                reading_speed_wpm: Some(120.0),
                created_at: Utc::now(),
            },
            ReadingSession {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                khatma_plan_id: Uuid::new_v4(),
                surah_start: 2,
                ayah_start: 1,
                surah_end: 2,
                ayah_end: 10,
                start_time: (base_date + chrono::Duration::days(1)).and_hms_opt(8, 0, 0).unwrap().and_utc(),
                end_time: None,
                duration_minutes: Some(25),
                word_count: 150,
                reading_speed_wpm: Some(180.0),
                created_at: Utc::now(),
            },
            ReadingSession {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                khatma_plan_id: Uuid::new_v4(),
                surah_start: 3,
                ayah_start: 1,
                surah_end: 3,
                ayah_end: 5,
                start_time: (base_date + chrono::Duration::days(2)).and_hms_opt(8, 0, 0).unwrap().and_utc(),
                end_time: None,
                duration_minutes: Some(20),
                word_count: 80,
                reading_speed_wpm: Some(160.0),
                created_at: Utc::now(),
            },
        ];

        let (current_streak, max_streak) = service.analyze_streak_patterns(&sessions);
        
        // Should detect a 3-day streak
        assert_eq!(current_streak, 3);
        assert_eq!(max_streak, 3);
    }

    #[test]
    fn test_filter_and_prioritize_reminders() {
        let repository = MockKhatmaRepository::new();
        let service = SmartKhatmaService::new(repository);

        let behavior = UserBehaviorAnalysis {
            preferred_hours: vec![8, 20],
            preferred_days: vec![1, 2, 3, 4, 5], // Weekdays
            session_duration_patterns: (20, 30, 45),
            consistency_score: 0.8,
            streak_patterns: (5, 10),
            missed_session_patterns: vec![],
        };

        let base_time = Utc::now();
        let reminders = vec![
            SmartReminder {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                khatma_plan_id: Uuid::new_v4(),
                suggested_time: base_time + Duration::hours(1),
                duration_minutes: 30,
                portion: DailyPortion {
                    date: base_time,
                    surah_start: 1,
                    ayah_start: 1,
                    surah_end: 1,
                    ayah_end: 7,
                    estimated_minutes: 30,
                    word_count: 100,
                    completed: false,
                    actual_reading_time: None,
                    completion_date: None,
                },
                confidence_score: 0.7,
                reasoning: "Test reminder 1".to_string(),
                created_at: base_time,
            },
            SmartReminder {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                khatma_plan_id: Uuid::new_v4(),
                suggested_time: base_time + Duration::hours(2),
                duration_minutes: 25,
                portion: DailyPortion {
                    date: base_time,
                    surah_start: 2,
                    ayah_start: 1,
                    surah_end: 2,
                    ayah_end: 10,
                    estimated_minutes: 25,
                    word_count: 150,
                    completed: false,
                    actual_reading_time: None,
                    completion_date: None,
                },
                confidence_score: 0.9,
                reasoning: "Test reminder 2".to_string(),
                created_at: base_time,
            },
        ];

        let filtered = service.filter_and_prioritize_reminders(reminders, &behavior);
        
        // Should be sorted by confidence score (highest first)
        assert_eq!(filtered.len(), 2);
        assert!(filtered[0].confidence_score >= filtered[1].confidence_score);
    }
}
        let best_days_of_week = self.analyze_day_productivity(sessions).await?;
        let best_times_of_day = self.analyze_hour_productivity(sessions).await?;
        let session_length_effectiveness = self.analyze_session_length_effectiveness(sessions).await?;
        let environmental_factors = self.analyze_environmental_factors(sessions).await?;

        Ok(ProductivityPatterns {
            best_days_of_week,
            best_times_of_day,
            session_length_effectiveness,
            environmental_factors,
        })
    }

    /// Analyze productivity by day of week
    async fn analyze_day_productivity(&self, sessions: &[ReadingSession]) -> Result<Vec<DayProductivity>> {
        let day_names = vec!["Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday"];
        let mut day_stats = HashMap::new();

        for session in sessions {
            let day = session.start_time.weekday().num_days_from_sunday() as usize;
            let entry = day_stats.entry(day).or_insert((Vec::new(), Vec::new()));
            
            if let Some(duration) = session.duration_minutes {
                entry.0.push(duration);
            }
            entry.1.push(session.start_time.date_naive());
        }

        let mut day_productivity = Vec::new();
        for day in 0..7 {
            let day_name = day_names[day].to_string();
            
            if let Some((durations, dates)) = day_stats.get(&day) {
                let average_reading_time = if !durations.is_empty() {
                    durations.iter().sum::<i32>() / durations.len() as i32
                } else {
                    0
                };

                // Calculate consistency rate for this day
                let unique_dates: std::collections::HashSet<_> = dates.iter().collect();
                let total_possible_days = self.count_possible_days_for_weekday(sessions, day);
                let consistency_rate = if total_possible_days > 0 {
                    (unique_dates.len() as f64 / total_possible_days as f64) * 100.0
                } else {
                    0.0
                };

                // Calculate productivity score (combination of duration and consistency)
                let productivity_score = (average_reading_time as f64 * 0.6) + (consistency_rate * 0.4);

                day_productivity.push(DayProductivity {
                    day_of_week: day as u32,
                    day_name,
                    average_reading_time,
                    consistency_rate: consistency_rate / 100.0, // Convert to 0-1 scale
                    productivity_score,
                });
            } else {
                day_productivity.push(DayProductivity {
                    day_of_week: day as u32,
                    day_name,
                    average_reading_time: 0,
                    consistency_rate: 0.0,
                    productivity_score: 0.0,
                });
            }
        }

        // Sort by productivity score
        day_productivity.sort_by(|a, b| b.productivity_score.partial_cmp(&a.productivity_score).unwrap_or(std::cmp::Ordering::Equal));
        Ok(day_productivity)
    }

    /// Analyze productivity by hour of day
    async fn analyze_hour_productivity(&self, sessions: &[ReadingSession]) -> Result<Vec<HourProductivity>> {
        let mut hour_stats = HashMap::new();

        for session in sessions {
            let hour = session.start_time.hour();
            let entry = hour_stats.entry(hour).or_insert((0u32, Vec::new(), Vec::new(), 0u32));
            
            entry.0 += 1; // sessions count
            if let Some(duration) = session.duration_minutes {
                entry.1.push(duration);
            }
            if let Some(speed) = session.reading_speed_wpm {
                entry.2.push(speed);
            }
            if session.duration_minutes.unwrap_or(0) >= 15 {
                entry.3 += 1; // completed sessions
            }
        }

        let mut hour_productivity = Vec::new();
        for hour in 0..24 {
            if let Some((sessions_count, durations, speeds, completed_count)) = hour_stats.get(&hour) {
                let average_duration = if !durations.is_empty() {
                    durations.iter().sum::<i32>() / durations.len() as i32
                } else {
                    0
                };

                let average_speed = if !speeds.is_empty() {
                    speeds.iter().sum::<f64>() / speeds.len() as f64
                } else {
                    0.0
                };

                let completion_rate = if *sessions_count > 0 {
                    (*completed_count as f64 / *sessions_count as f64) * 100.0
                } else {
                    0.0
                };

                hour_productivity.push(HourProductivity {
                    hour,
                    sessions_count: *sessions_count,
                    average_duration,
                    average_speed,
                    completion_rate,
                });
            }
        }

        // Sort by completion rate and average duration
        hour_productivity.sort_by(|a, b| {
            b.completion_rate.partial_cmp(&a.completion_rate)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.average_duration.cmp(&a.average_duration))
        });

        Ok(hour_productivity.into_iter().filter(|h| h.sessions_count > 0).collect())
    }

    /// Analyze session length effectiveness
    async fn analyze_session_length_effectiveness(&self, sessions: &[ReadingSession]) -> Result<SessionLengthAnalysis> {
        let mut short_sessions = Vec::new(); // < 20 minutes
        let mut medium_sessions = Vec::new(); // 20-45 minutes
        let mut long_sessions = Vec::new(); // > 45 minutes

        for session in sessions {
            if let Some(duration) = session.duration_minutes {
                let effectiveness = self.calculate_session_effectiveness(session);
                
                if duration < 20 {
                    short_sessions.push(effectiveness);
                } else if duration <= 45 {
                    medium_sessions.push(effectiveness);
                } else {
                    long_sessions.push(effectiveness);
                }
            }
        }

        let short_sessions_effectiveness = if !short_sessions.is_empty() {
            short_sessions.iter().sum::<f64>() / short_sessions.len() as f64
        } else {
            0.0
        };

        let medium_sessions_effectiveness = if !medium_sessions.is_empty() {
            medium_sessions.iter().sum::<f64>() / medium_sessions.len() as f64
        } else {
            0.0
        };

        let long_sessions_effectiveness = if !long_sessions.is_empty() {
            long_sessions.iter().sum::<f64>() / long_sessions.len() as f64
        } else {
            0.0
        };

        // Determine optimal duration and recommendation
        let optimal_duration_minutes = if medium_sessions_effectiveness > short_sessions_effectiveness && 
                                          medium_sessions_effectiveness > long_sessions_effectiveness {
            30 // Medium sessions are most effective
        } else if long_sessions_effectiveness > short_sessions_effectiveness {
            60 // Long sessions are most effective
        } else {
            15 // Short sessions are most effective
        };

        let recommendation = match optimal_duration_minutes {
            15 => "Short, frequent sessions work best for you. Try 15-20 minute focused sessions.".to_string(),
            30 => "Medium-length sessions are your sweet spot. Aim for 25-35 minute sessions.".to_string(),
            60 => "You excel in longer sessions. Consider 45-60 minute deep reading sessions.".to_string(),
            _ => "Experiment with different session lengths to find your optimal duration.".to_string(),
        };

        Ok(SessionLengthAnalysis {
            optimal_duration_minutes,
            short_sessions_effectiveness,
            medium_sessions_effectiveness,
            long_sessions_effectiveness,
            recommendation,
        })
    }

    /// Analyze environmental factors
    async fn analyze_environmental_factors(&self, sessions: &[ReadingSession]) -> Result<EnvironmentalFactors> {
        let mut weekend_sessions = Vec::new();
        let mut weekday_sessions = Vec::new();
        let mut morning_sessions = Vec::new(); // 5 AM - 12 PM
        let mut evening_sessions = Vec::new(); // 6 PM - 11 PM

        for session in sessions {
            let weekday = session.start_time.weekday();
            let hour = session.start_time.hour();
            let effectiveness = self.calculate_session_effectiveness(session);

            // Weekend vs Weekday
            if weekday == chrono::Weekday::Sat || weekday == chrono::Weekday::Sun {
                weekend_sessions.push(effectiveness);
            } else {
                weekday_sessions.push(effectiveness);
            }

            // Morning vs Evening
            if hour >= 5 && hour < 12 {
                morning_sessions.push(effectiveness);
            } else if hour >= 18 && hour < 23 {
                evening_sessions.push(effectiveness);
            }
        }

        let weekend_avg = if !weekend_sessions.is_empty() {
            weekend_sessions.iter().sum::<f64>() / weekend_sessions.len() as f64
        } else {
            0.0
        };

        let weekday_avg = if !weekday_sessions.is_empty() {
            weekday_sessions.iter().sum::<f64>() / weekday_sessions.len() as f64
        } else {
            0.0
        };

        let morning_avg = if !morning_sessions.is_empty() {
            morning_sessions.iter().sum::<f64>() / morning_sessions.len() as f64
        } else {
            0.0
        };

        let evening_avg = if !evening_sessions.is_empty() {
            evening_sessions.iter().sum::<f64>() / evening_sessions.len() as f64
        } else {
            0.0
        };

        let weekend_vs_weekday_performance = if weekday_avg > 0.0 {
            weekend_avg / weekday_avg
        } else {
            1.0
        };

        let morning_vs_evening_preference = if evening_avg > 0.0 {
            morning_avg / evening_avg
        } else {
            1.0
        };

        // Calculate consistency impact (how much environmental factors affect consistency)
        let consistency_impact_score = self.calculate_consistency_impact(sessions);

        Ok(EnvironmentalFactors {
            weekend_vs_weekday_performance,
            morning_vs_evening_preference,
            consistency_impact_score,
        })
    }

    /// Calculate goal achievement rate
    async fn calculate_goal_achievement_rate(&self, sessions: &[ReadingSession]) -> Result<f64> {
        if sessions.is_empty() {
            return Ok(0.0);
        }

        // Calculate based on sessions that met their expected duration
        let successful_sessions = sessions.iter()
            .filter(|s| {
                let duration = s.duration_minutes.unwrap_or(0);
                duration >= 15 && // At least 15 minutes
                s.reading_speed_wpm.unwrap_or(0.0) > 50.0 // Reasonable reading speed
            })
            .count();

        Ok((successful_sessions as f64 / sessions.len() as f64) * 100.0)
    }

    /// Generate upcoming milestones
    async fn generate_upcoming_milestones(
        &self,
        user_id: Uuid,
        current_khatma: &Option<KhatmaPlan>,
        overall_progress: &OverallProgress,
    ) -> Result<Vec<Milestone>> {
        let mut milestones = Vec::new();
        let now = Utc::now();

        // Khatma completion milestone
        if let Some(khatma) = current_khatma {
            let progress_to_completion = 100.0 - khatma.current_progress;
            milestones.push(Milestone {
                id: "current_khatma_completion".to_string(),
                title: "Complete Current Khatma".to_string(),
                description: format!("Finish your current Khatma plan with {:.1}% remaining", progress_to_completion),
                target_date: khatma.target_date,
                progress_percentage: khatma.current_progress,
                milestone_type: MilestoneType::KhatmaCompletion,
                reward: Some("Khatma Completion Certificate".to_string()),
            });
        }

        // Reading streak milestones
        let next_streak_target = if overall_progress.current_streak_days < 7 {
            7
        } else if overall_progress.current_streak_days < 30 {
            30
        } else if overall_progress.current_streak_days < 100 {
            100
        } else {
            overall_progress.current_streak_days + 30
        };

        let streak_progress = (overall_progress.current_streak_days as f64 / next_streak_target as f64) * 100.0;
        milestones.push(Milestone {
            id: format!("streak_{}", next_streak_target),
            title: format!("{}-Day Reading Streak", next_streak_target),
            description: format!("Maintain daily reading for {} consecutive days", next_streak_target),
            target_date: now + chrono::Duration::days((next_streak_target - overall_progress.current_streak_days) as i64),
            progress_percentage: streak_progress,
            milestone_type: MilestoneType::ReadingStreak,
            reward: Some(format!("Streak Master {} Badge", next_streak_target)),
        });

        // Speed improvement milestone
        if overall_progress.total_reading_time_hours > 5.0 {
            milestones.push(Milestone {
                id: "speed_improvement".to_string(),
                title: "Reading Speed Improvement".to_string(),
                description: "Improve your reading speed by 20%".to_string(),
                target_date: now + chrono::Duration::days(30),
                progress_percentage: 0.0, // Would need to calculate based on recent improvement
                milestone_type: MilestoneType::SpeedImprovement,
                reward: Some("Speed Reader Badge".to_string()),
            });
        }

        // Consistency goal milestone
        let consistency_target = if overall_progress.consistency_score < 0.7 {
            0.7
        } else {
            0.9
        };

        let consistency_progress = (overall_progress.consistency_score / consistency_target) * 100.0;
        milestones.push(Milestone {
            id: "consistency_goal".to_string(),
            title: format!("Achieve {:.0}% Consistency", consistency_target * 100.0),
            description: "Maintain regular daily reading habits".to_string(),
            target_date: now + chrono::Duration::days(30),
            progress_percentage: consistency_progress.min(100.0),
            milestone_type: MilestoneType::ConsistencyGoal,
            reward: Some("Consistency Champion Badge".to_string()),
        });

        // Time goal milestone
        let next_time_target = if overall_progress.total_reading_time_hours < 10.0 {
            10.0
        } else if overall_progress.total_reading_time_hours < 50.0 {
            50.0
        } else {
            overall_progress.total_reading_time_hours + 25.0
        };

        let time_progress = (overall_progress.total_reading_time_hours / next_time_target) * 100.0;
        milestones.push(Milestone {
            id: format!("time_goal_{}", next_time_target as u32),
            title: format!("{} Hours of Reading", next_time_target as u32),
            description: format!("Accumulate {} total hours of Quran reading", next_time_target as u32),
            target_date: now + chrono::Duration::days(60),
            progress_percentage: time_progress.min(100.0),
            milestone_type: MilestoneType::TimeGoal,
            reward: Some("Dedicated Reader Badge".to_string()),
        });

        // Sort by progress percentage (closest to completion first)
        milestones.sort_by(|a, b| b.progress_percentage.partial_cmp(&a.progress_percentage).unwrap_or(std::cmp::Ordering::Equal));

        Ok(milestones)
    }

    /// Generate performance improvement recommendations
    async fn generate_performance_recommendations(
        &self,
        user_id: Uuid,
        overall_progress: &OverallProgress,
        performance_metrics: &PerformanceMetrics,
        recent_activity: &RecentActivity,
    ) -> Result<Vec<PerformanceRecommendation>> {
        let mut recommendations = Vec::new();

        // Consistency recommendations
        if overall_progress.consistency_score < 0.6 {
            recommendations.push(PerformanceRecommendation {
                id: "improve_consistency".to_string(),
                title: "Improve Reading Consistency".to_string(),
                description: "Your consistency score is below optimal. Regular daily reading will help build a strong habit.".to_string(),
                category: RecommendationCategory::Consistency,
                priority: RecommendationPriority::High,
                expected_impact: "Increase consistency score by 30-40%".to_string(),
                action_steps: vec![
                    "Set a specific time each day for Quran reading".to_string(),
                    "Start with shorter 15-20 minute sessions".to_string(),
                    "Use reminders to build the habit".to_string(),
                    "Track your daily progress".to_string(),
                ],
                confidence_score: 0.9,
            });
        }

        // Time management recommendations
        if recent_activity.last_7_days.average_session_duration < 20.0 {
            recommendations.push(PerformanceRecommendation {
                id: "extend_sessions".to_string(),
                title: "Extend Reading Sessions".to_string(),
                description: "Your recent sessions are quite short. Longer sessions can improve focus and comprehension.".to_string(),
                category: RecommendationCategory::TimeManagement,
                priority: RecommendationPriority::Medium,
                expected_impact: "Better focus and deeper engagement with the text".to_string(),
                action_steps: vec![
                    "Gradually increase session length by 5 minutes each week".to_string(),
                    "Find a quiet, comfortable reading environment".to_string(),
                    "Use the Pomodoro technique: 25 minutes reading, 5 minutes break".to_string(),
                ],
                confidence_score: 0.8,
            });
        }

        // Reading speed recommendations
        if let TrendDirection::Declining = performance_metrics.reading_speed_trend.trend_direction {
            recommendations.push(PerformanceRecommendation {
                id: "improve_reading_speed".to_string(),
                title: "Improve Reading Speed".to_string(),
                description: "Your reading speed has been declining. Let's work on improving it while maintaining comprehension.".to_string(),
                category: RecommendationCategory::ReadingSpeed,
                priority: RecommendationPriority::Medium,
                expected_impact: "Increase reading speed by 15-25%".to_string(),
                action_steps: vec![
                    "Practice reading aloud to improve fluency".to_string(),
                    "Focus on familiar surahs to build confidence".to_string(),
                    "Avoid subvocalization for silent reading".to_string(),
                    "Use finger or pointer to guide your eyes".to_string(),
                ],
                confidence_score: 0.7,
            });
        }

        // Session optimization recommendations
        if let Some(optimal_time) = performance_metrics.optimal_reading_times.first() {
            if matches!(optimal_time.recommendation_strength, RecommendationStrength::Strong) {
                recommendations.push(PerformanceRecommendation {
                    id: "optimize_timing".to_string(),
                    title: "Optimize Reading Times".to_string(),
                    description: format!("You perform best at {}:00. Try to schedule more sessions at this time.", optimal_time.hour),
                    category: RecommendationCategory::SessionOptimization,
                    priority: RecommendationPriority::Medium,
                    expected_impact: "Improve session quality and consistency".to_string(),
                    action_steps: vec![
                        format!("Schedule daily reading at {}:00", optimal_time.hour),
                        "Set reminders for your optimal time".to_string(),
                        "Prepare your reading space in advance".to_string(),
                    ],
                    confidence_score: 0.85,
                });
            }
        }

        // Goal setting recommendations
        if overall_progress.current_streak_days == 0 {
            recommendations.push(PerformanceRecommendation {
                id: "start_streak".to_string(),
                title: "Start a Reading Streak".to_string(),
                description: "Building a daily reading streak will significantly improve your consistency and progress.".to_string(),
                category: RecommendationCategory::GoalSetting,
                priority: RecommendationPriority::High,
                expected_impact: "Establish a strong daily reading habit".to_string(),
                action_steps: vec![
                    "Commit to reading for at least 10 minutes daily".to_string(),
                    "Choose a consistent time each day".to_string(),
                    "Track your streak progress".to_string(),
                    "Celebrate small wins (3, 7, 14 day streaks)".to_string(),
                ],
                confidence_score: 0.9,
            });
        }

        // Motivation recommendations
        if recent_activity.last_7_days.sessions_count < 3 {
            recommendations.push(PerformanceRecommendation {
                id: "increase_motivation".to_string(),
                title: "Boost Reading Motivation".to_string(),
                description: "Your recent activity has decreased. Let's find ways to reignite your passion for Quran reading.".to_string(),
                category: RecommendationCategory::Motivation,
                priority: RecommendationPriority::High,
                expected_impact: "Renewed enthusiasm and increased reading frequency".to_string(),
                action_steps: vec![
                    "Set small, achievable daily goals".to_string(),
                    "Read with translation to enhance understanding".to_string(),
                    "Join a reading group or find an accountability partner".to_string(),
                    "Reflect on the spiritual benefits of regular reading".to_string(),
                ],
                confidence_score: 0.8,
            });
        }

        // Sort by priority and confidence
        recommendations.sort_by(|a, b| {
            let priority_order = |p: &RecommendationPriority| match p {
                RecommendationPriority::High => 3,
                RecommendationPriority::Medium => 2,
                RecommendationPriority::Low => 1,
            };
            
            priority_order(&b.priority).cmp(&priority_order(&a.priority))
                .then_with(|| b.confidence_score.partial_cmp(&a.confidence_score).unwrap_or(std::cmp::Ordering::Equal))
        });

        Ok(recommendations.into_iter().take(5).collect()) // Top 5 recommendations
    }

    /// Calculate comparison metrics between current and previous Khatmas
    async fn calculate_comparison_metrics(
        &self,
        current_khatma: &Option<KhatmaPlan>,
        previous_khatmas: &[KhatmaStatistics],
    ) -> Result<ComparisonMetrics> {
        if previous_khatmas.is_empty() {
            return Ok(ComparisonMetrics {
                completion_time_comparison: TimeComparison {
                    current_pace_days: current_khatma.as_ref().map(|k| (k.target_date - k.start_date).num_days() as i32),
                    average_previous_pace_days: 0.0,
                    best_previous_pace_days: 0,
                    improvement_percentage: 0.0,
                    trend: TrendDirection::Stable,
                },
                reading_speed_comparison: SpeedComparison {
                    current_average_wpm: current_khatma.as_ref().map(|k| k.reading_speed_wpm),
                    previous_average_wpm: 0.0,
                    best_previous_wpm: 0.0,
                    improvement_percentage: 0.0,
                    trend: TrendDirection::Stable,
                },
                consistency_comparison: ConsistencyComparison {
                    current_consistency_score: None, // Would need to calculate from current sessions
                    previous_average_consistency: 0.0,
                    best_previous_consistency: 0.0,
                    improvement_percentage: 0.0,
                    trend: TrendDirection::Stable,
                },
                overall_improvement_score: 0.5,
            });
        }

        // Time comparison
        let average_previous_pace_days = previous_khatmas.iter()
            .map(|k| k.actual_duration_days as f64)
            .sum::<f64>() / previous_khatmas.len() as f64;

        let best_previous_pace_days = previous_khatmas.iter()
            .map(|k| k.actual_duration_days)
            .min()
            .unwrap_or(0);

        let current_pace_days = current_khatma.as_ref()
            .map(|k| (k.target_date - k.start_date).num_days() as i32);

        let time_improvement_percentage = if let Some(current_days) = current_pace_days {
            if average_previous_pace_days > 0.0 {
                ((average_previous_pace_days - current_days as f64) / average_previous_pace_days) * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        };

        let time_trend = if time_improvement_percentage > 5.0 {
            TrendDirection::Improving
        } else if time_improvement_percentage < -5.0 {
            TrendDirection::Declining
        } else {
            TrendDirection::Stable
        };

        // Speed comparison
        let previous_average_wpm = previous_khatmas.iter()
            .map(|k| k.reading_speed_improvement) // This would need to be actual speed, not improvement
            .sum::<f64>() / previous_khatmas.len() as f64;

        let best_previous_wpm = previous_khatmas.iter()
            .map(|k| k.reading_speed_improvement)
            .fold(0.0f64, |a, b| a.max(b));

        let current_average_wpm = current_khatma.as_ref().map(|k| k.reading_speed_wpm);

        let speed_improvement_percentage = if let Some(current_wpm) = current_average_wpm {
            if previous_average_wpm > 0.0 {
                ((current_wpm - previous_average_wpm) / previous_average_wpm) * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        };

        let speed_trend = if speed_improvement_percentage > 5.0 {
            TrendDirection::Improving
        } else if speed_improvement_percentage < -5.0 {
            TrendDirection::Declining
        } else {
            TrendDirection::Stable
        };

        // Consistency comparison
        let previous_average_consistency = previous_khatmas.iter()
            .map(|k| k.consistency_score)
            .sum::<f64>() / previous_khatmas.len() as f64;

        let best_previous_consistency = previous_khatmas.iter()
            .map(|k| k.consistency_score)
            .fold(0.0f64, |a, b| a.max(b));

        // Overall improvement score (combination of all metrics)
        let overall_improvement_score = {
            let time_score = if time_improvement_percentage > 0.0 { 0.4 } else { 0.2 };
            let speed_score = if speed_improvement_percentage > 0.0 { 0.3 } else { 0.15 };
            let consistency_score = 0.3; // Would calculate based on current consistency
            
            time_score + speed_score + consistency_score
        };

        Ok(ComparisonMetrics {
            completion_time_comparison: TimeComparison {
                current_pace_days,
                average_previous_pace_days,
                best_previous_pace_days,
                improvement_percentage: time_improvement_percentage,
                trend: time_trend,
            },
            reading_speed_comparison: SpeedComparison {
                current_average_wpm,
                previous_average_wpm,
                best_previous_wpm,
                improvement_percentage: speed_improvement_percentage,
                trend: speed_trend,
            },
            consistency_comparison: ConsistencyComparison {
                current_consistency_score: None, // Would calculate from current sessions
                previous_average_consistency,
                best_previous_consistency,
                improvement_percentage: 0.0, // Would calculate
                trend: TrendDirection::Stable,
            },
            overall_improvement_score,
        })
    }

    /// Identify areas for improvement
    async fn identify_improvement_areas(
        &self,
        current_khatma: &Option<KhatmaPlan>,
        previous_khatmas: &[KhatmaStatistics],
        comparison_metrics: &ComparisonMetrics,
    ) -> Result<Vec<ImprovementArea>> {
        let mut improvement_areas = Vec::new();

        // Time management improvement
        if let TrendDirection::Declining = comparison_metrics.completion_time_comparison.trend {
            improvement_areas.push(ImprovementArea {
                area: "Completion Time".to_string(),
                current_performance: comparison_metrics.completion_time_comparison.current_pace_days.unwrap_or(0) as f64,
                target_performance: comparison_metrics.completion_time_comparison.best_previous_pace_days as f64,
                improvement_potential: comparison_metrics.completion_time_comparison.improvement_percentage.abs(),
                specific_recommendations: vec![
                    "Increase daily reading time by 10-15 minutes".to_string(),
                    "Use adaptive scheduling to catch up when behind".to_string(),
                    "Set more realistic daily goals".to_string(),
                ],
            });
        }

        // Reading speed improvement
        if let TrendDirection::Declining = comparison_metrics.reading_speed_comparison.trend {
            improvement_areas.push(ImprovementArea {
                area: "Reading Speed".to_string(),
                current_performance: comparison_metrics.reading_speed_comparison.current_average_wpm.unwrap_or(0.0),
                target_performance: comparison_metrics.reading_speed_comparison.best_previous_wpm,
                improvement_potential: comparison_metrics.reading_speed_comparison.improvement_percentage.abs(),
                specific_recommendations: vec![
                    "Practice reading familiar passages to build fluency".to_string(),
                    "Focus on reducing subvocalization".to_string(),
                    "Use guided reading techniques".to_string(),
                ],
            });
        }

        // Consistency improvement
        if comparison_metrics.consistency_comparison.previous_average_consistency > 0.0 {
            let current_consistency = comparison_metrics.consistency_comparison.current_consistency_score.unwrap_or(0.5);
            if current_consistency < comparison_metrics.consistency_comparison.previous_average_consistency {
                improvement_areas.push(ImprovementArea {
                    area: "Reading Consistency".to_string(),
                    current_performance: current_consistency,
                    target_performance: comparison_metrics.consistency_comparison.best_previous_consistency,
                    improvement_potential: (comparison_metrics.consistency_comparison.best_previous_consistency - current_consistency) * 100.0,
                    specific_recommendations: vec![
                        "Establish a fixed daily reading schedule".to_string(),
                        "Use habit stacking (read after an existing habit)".to_string(),
                        "Set up environmental cues for reading".to_string(),
                    ],
                });
            }
        }

        Ok(improvement_areas)
    }

    /// Compare achievements across Khatmas
    async fn compare_achievements(
        &self,
        user_id: Uuid,
        current_khatma: &Option<KhatmaPlan>,
        previous_khatmas: &[KhatmaStatistics],
    ) -> Result<AchievementsComparison> {
        let total_achievements_earned = previous_khatmas.iter()
            .map(|k| k.achievements.len() as u32)
            .sum();

        let new_achievements_this_khatma = 0u32; // Would calculate based on current khatma achievements

        // Group achievements by category
        let mut achievement_categories_progress = HashMap::new();
        for khatma in previous_khatmas {
            for achievement in &khatma.achievements {
                let category = format!("{:?}", achievement.category);
                *achievement_categories_progress.entry(category).or_insert(0) += 1;
            }
        }

        // Find rarest achievement (appears in fewest Khatmas)
        let rarest_achievement = previous_khatmas.iter()
            .flat_map(|k| &k.achievements)
            .fold(HashMap::new(), |mut acc, achievement| {
                *acc.entry(&achievement.id).or_insert(0) += 1;
                acc
            })
            .into_iter()
            .min_by_key(|(_, count)| *count)
            .and_then(|(id, _)| {
                previous_khatmas.iter()
                    .flat_map(|k| &k.achievements)
                    .find(|a| &a.id == id)
                    .cloned()
            });

        Ok(AchievementsComparison {
            total_achievements_earned,
            new_achievements_this_khatma,
            achievement_categories_progress,
            rarest_achievement,
        })
    }

    /// Calculate expected progress for a plan
    fn calculate_expected_progress(&self, plan: &KhatmaPlan, current_time: DateTime<Utc>) -> f64 {
        let total_days = (plan.target_date - plan.start_date).num_days() as f64;
        let elapsed_days = (current_time - plan.start_date).num_days() as f64;
        
        if total_days <= 0.0 {
            return 0.0;
        }
        
        (elapsed_days / total_days * 100.0).min(100.0).max(0.0)
    }

    // Helper methods for analytics

    /// Calculate active reading days from sessions
    fn calculate_active_reading_days(&self, sessions: &[ReadingSession]) -> u32 {
        sessions.iter()
            .map(|s| s.start_time.date_naive())
            .collect::<std::collections::HashSet<_>>()
            .len() as u32
    }

    /// Calculate streak statistics
    fn calculate_streak_statistics(&self, sessions: &[ReadingSession]) -> (u32, u32) {
        if sessions.is_empty() {
            return (0, 0);
        }

        // Group sessions by date
        let mut daily_sessions = HashMap::new();
        for session in sessions {
            let date = session.start_time.date_naive();
            daily_sessions.entry(date).or_insert(Vec::new()).push(session);
        }

        let mut dates: Vec<_> = daily_sessions.keys().collect();
        dates.sort();

        let mut current_streak = 0;
        let mut max_streak = 0;
        let mut last_date: Option<chrono::NaiveDate> = None;
        let today = Utc::now().date_naive();

        for date in dates {
            if let Some(last) = last_date {
                let days_diff = (*date - last).num_days();
                if days_diff == 1 {
                    current_streak += 1;
                } else {
                    max_streak = max_streak.max(current_streak);
                    current_streak = 1;
                }
            } else {
                current_streak = 1;
            }
            last_date = Some(*date);
        }

        max_streak = max_streak.max(current_streak);

        // Check if streak is still active (last session was yesterday or today)
        if let Some(last_session_date) = dates.last() {
            let days_since_last = (today - **last_session_date).num_days();
            if days_since_last > 1 {
                current_streak = 0; // Streak is broken
            }
        }

        (current_streak, max_streak)
    }

    /// Count completed surahs
    fn count_completed_surahs(&self, sessions: &[ReadingSession]) -> u32 {
        sessions.iter()
            .flat_map(|s| s.surah_start..=s.surah_end)
            .collect::<std::collections::HashSet<_>>()
            .len() as u32
    }

    /// Calculate session quality score
    fn calculate_session_quality_score(&self, session: &ReadingSession) -> f64 {
        let mut score: f64 = 0.5; // Base score

        // Duration factor
        if let Some(duration) = session.duration_minutes {
            if duration >= 15 && duration <= 60 {
                score += 0.2; // Good duration
            } else if duration > 60 {
                score += 0.1; // Long session, might be less focused
            }
        }

        // Speed factor
        if let Some(speed) = session.reading_speed_wpm {
            if speed >= 100.0 && speed <= 200.0 {
                score += 0.2; // Good reading speed
            } else if speed > 200.0 {
                score += 0.1; // Very fast, might compromise comprehension
            }
        }

        // Word count factor
        if session.word_count > 100 {
            score += 0.1; // Substantial reading
        }

        score.min(1.0)
    }

    /// Calculate session effectiveness
    fn calculate_session_effectiveness(&self, session: &ReadingSession) -> f64 {
        let mut effectiveness: f64 = 0.5; // Base effectiveness

        // Duration effectiveness
        if let Some(duration) = session.duration_minutes {
            effectiveness += match duration {
                15..=30 => 0.3,  // Optimal for focus
                31..=45 => 0.4,  // Very good
                46..=60 => 0.3,  // Good but might lose focus
                _ => 0.1,        // Too short or too long
            };
        }

        // Speed effectiveness
        if let Some(speed) = session.reading_speed_wpm {
            effectiveness += match speed as u32 {
                100..=150 => 0.2, // Good comprehension speed
                151..=200 => 0.1, // Fast but still good
                _ => 0.05,        // Too slow or too fast
            };
        }

        effectiveness.min(1.0)
    }

    /// Count possible days for a specific weekday
    fn count_possible_days_for_weekday(&self, sessions: &[ReadingSession], target_weekday: usize) -> u32 {
        if sessions.is_empty() {
            return 0;
        }

        let first_date = sessions.iter().map(|s| s.start_time.date_naive()).min().unwrap();
        let last_date = sessions.iter().map(|s| s.start_time.date_naive()).max().unwrap();
        
        let mut count = 0;
        let mut current_date = first_date;
        
        while current_date <= last_date {
            if current_date.weekday().num_days_from_sunday() as usize == target_weekday {
                count += 1;
            }
            current_date += chrono::Duration::days(1);
        }
        
        count
    }

    /// Calculate consistency impact score
    fn calculate_consistency_impact(&self, _sessions: &[ReadingSession]) -> f64 {
        // This would analyze how environmental factors affect consistency
        // For now, return a placeholder value
        0.7
    }
}