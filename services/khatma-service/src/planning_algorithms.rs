use crate::models::*;
use chrono::{DateTime, Utc, Duration, Datelike, Timelike};
use std::collections::HashMap;
use uuid::Uuid;
use anyhow::{Result, anyhow};

/// Core planning algorithms for interactive Khatma scheduling
pub struct PlanningAlgorithms;

impl PlanningAlgorithms {
    /// Calculate user's reading speed based on historical sessions
    pub fn calculate_reading_speed(sessions: &[ReadingSession]) -> f64 {
        if sessions.is_empty() {
            return 150.0; // Default average reading speed for Arabic text
        }

        let valid_sessions: Vec<_> = sessions
            .iter()
            .filter(|s| s.reading_speed_wpm.is_some() && s.reading_speed_wpm.unwrap() > 0.0)
            .collect();

        if valid_sessions.is_empty() {
            return 150.0;
        }

        // Calculate weighted average, giving more weight to recent sessions
        let total_weight: f64 = valid_sessions.len() as f64;
        let weighted_sum: f64 = valid_sessions
            .iter()
            .enumerate()
            .map(|(i, session)| {
                let weight = (i + 1) as f64 / total_weight; // More recent sessions get higher weight
                session.reading_speed_wpm.unwrap() * weight
            })
            .sum();

        weighted_sum / valid_sessions.len() as f64
    }

    /// Create an adaptive khatma plan based on user preferences and reading speed
    pub fn create_adaptive_plan(
        user_id: Uuid,
        target_date: DateTime<Utc>,
        preferences: &KhatmaPreferences,
        reading_speed_wpm: f64,
    ) -> Result<KhatmaPlan> {
        let start_date = Utc::now();
        let total_days = (target_date - start_date).num_days();
        
        if total_days <= 0 {
            return Err(anyhow!("Target date must be in the future"));
        }

        // Total words in Quran (approximately)
        const TOTAL_QURAN_WORDS: u32 = 77_430;
        
        // Calculate daily reading requirements
        let daily_word_target = TOTAL_QURAN_WORDS as f64 / total_days as f64;
        let daily_reading_time = if let Some(time) = preferences.daily_reading_time_minutes {
            time as f64
        } else {
            daily_word_target / reading_speed_wpm * 60.0 // Convert to minutes
        };

        // Adjust based on difficulty preference
        let (adjusted_daily_words, adjusted_time) = match preferences.difficulty_preference {
            DifficultyPreference::Easy => (daily_word_target * 0.8, daily_reading_time * 1.2),
            DifficultyPreference::Medium => (daily_word_target, daily_reading_time),
            DifficultyPreference::Hard => (daily_word_target * 1.2, daily_reading_time * 0.8),
            DifficultyPreference::Custom => (daily_word_target, daily_reading_time),
        };

        let daily_portions = Self::distribute_quran_portions(
            start_date,
            total_days as u32,
            adjusted_daily_words as u32,
            &preferences.preferred_reading_times,
        )?;

        Ok(KhatmaPlan {
            id: Uuid::new_v4(),
            user_id,
            target_date,
            start_date,
            daily_portions,
            estimated_reading_time: adjusted_time as i32,
            adaptive_schedule: preferences.adaptive_scheduling,
            current_progress: 0.0,
            reading_speed_wpm,
            preferred_reading_times: preferences.preferred_reading_times.clone(),
            status: KhatmaStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    /// Distribute Quran portions across days intelligently
    fn distribute_quran_portions(
        start_date: DateTime<Utc>,
        total_days: u32,
        daily_word_target: u32,
        preferred_times: &[PreferredReadingTime],
    ) -> Result<Vec<DailyPortion>> {
        let mut portions = Vec::new();
        let mut current_surah = 1u8;
        let mut current_ayah = 1u16;
        
        // Quran structure: [surah_number] -> (total_ayahs, approximate_words_per_ayah)
        let quran_structure = Self::get_quran_structure();
        
        for day in 0..total_days {
            let portion_date = start_date + Duration::days(day as i64);
            
            // Adjust daily target based on day of week and user preferences
            let adjusted_target = Self::adjust_daily_target(
                daily_word_target,
                portion_date,
                preferred_times,
            );
            
            let (end_surah, end_ayah, word_count) = Self::calculate_portion_end(
                current_surah,
                current_ayah,
                adjusted_target,
                &quran_structure,
            )?;
            
            let estimated_minutes = (word_count as f64 / 150.0 * 60.0) as i32; // Assuming 150 WPM
            
            portions.push(DailyPortion {
                date: portion_date,
                surah_start: current_surah,
                ayah_start: current_ayah,
                surah_end: end_surah,
                ayah_end: end_ayah,
                estimated_minutes,
                word_count,
                completed: false,
                actual_reading_time: None,
                completion_date: None,
            });
            
            // Move to next portion
            if end_surah == 114 && end_ayah == Self::get_surah_ayah_count(114) {
                break; // Completed Quran
            }
            
            current_surah = end_surah;
            current_ayah = end_ayah + 1;
            
            // Handle surah transitions
            if current_ayah > Self::get_surah_ayah_count(current_surah) {
                current_surah += 1;
                current_ayah = 1;
            }
        }
        
        Ok(portions)
    }

    /// Adjust daily reading target based on day of week and preferences
    fn adjust_daily_target(
        base_target: u32,
        date: DateTime<Utc>,
        preferred_times: &[PreferredReadingTime],
    ) -> u32 {
        let day_of_week = date.weekday().num_days_from_sunday() as u8;
        
        // Find if user has preferences for this day
        let day_preferences: Vec<_> = preferred_times
            .iter()
            .filter(|pref| pref.days_of_week.contains(&day_of_week))
            .collect();
        
        if day_preferences.is_empty() {
            return base_target;
        }
        
        // Calculate adjustment based on available time and priority
        let total_available_minutes: i32 = day_preferences
            .iter()
            .map(|pref| pref.duration_minutes)
            .sum();
        
        let high_priority_time: i32 = day_preferences
            .iter()
            .filter(|pref| matches!(pref.priority, ReadingTimePriority::High))
            .map(|pref| pref.duration_minutes)
            .sum();
        
        // Adjust target based on available time
        let adjustment_factor = if high_priority_time > 0 {
            1.2 // Increase target for high-priority days
        } else if total_available_minutes < 30 {
            0.7 // Decrease target for low-availability days
        } else {
            1.0
        };
        
        (base_target as f64 * adjustment_factor) as u32
    }

    /// Calculate the end position for a daily portion
    fn calculate_portion_end(
        start_surah: u8,
        start_ayah: u16,
        target_words: u32,
        quran_structure: &HashMap<u8, (u16, u32)>,
    ) -> Result<(u8, u16, u32)> {
        let mut current_surah = start_surah;
        let mut current_ayah = start_ayah;
        let mut accumulated_words = 0u32;
        
        while current_surah <= 114 && accumulated_words < target_words {
            let (total_ayahs, words_per_ayah) = quran_structure
                .get(&current_surah)
                .ok_or_else(|| anyhow!("Invalid surah number: {}", current_surah))?;
            
            let remaining_ayahs_in_surah = *total_ayahs - current_ayah + 1;
            let words_in_remaining_ayahs = remaining_ayahs_in_surah as u32 * words_per_ayah;
            
            if accumulated_words + words_in_remaining_ayahs <= target_words {
                // Include entire remaining surah
                accumulated_words += words_in_remaining_ayahs;
                current_surah += 1;
                current_ayah = 1;
            } else {
                // Partial surah inclusion
                let remaining_words = target_words - accumulated_words;
                let ayahs_needed = (remaining_words + words_per_ayah - 1) / words_per_ayah; // Ceiling division
                current_ayah += ayahs_needed as u16 - 1;
                accumulated_words = target_words;
                break;
            }
        }
        
        // Ensure we don't exceed surah boundaries
        if current_surah <= 114 {
            let (total_ayahs, _) = quran_structure.get(&current_surah).unwrap();
            if current_ayah > *total_ayahs {
                current_ayah = *total_ayahs;
            }
        } else {
            current_surah = 114;
            current_ayah = Self::get_surah_ayah_count(114);
        }
        
        Ok((current_surah, current_ayah, accumulated_words))
    }

    /// Automatically adjust plan when user falls behind or gets ahead
    pub fn adjust_plan_for_delay(
        plan: &mut KhatmaPlan,
        current_progress: f64,
        reading_sessions: &[ReadingSession],
    ) -> Result<Vec<String>> {
        let mut adjustments = Vec::new();
        let current_date = Utc::now();
        let days_elapsed = (current_date - plan.start_date).num_days();
        let total_days = (plan.target_date - plan.start_date).num_days();
        
        if days_elapsed <= 0 || total_days <= 0 {
            return Ok(adjustments);
        }
        
        let expected_progress = (days_elapsed as f64 / total_days as f64) * 100.0;
        let progress_difference = current_progress - expected_progress;
        
        // Update reading speed based on recent sessions
        if !reading_sessions.is_empty() {
            let new_speed = Self::calculate_reading_speed(reading_sessions);
            if (new_speed - plan.reading_speed_wpm).abs() > 10.0 {
                plan.reading_speed_wpm = new_speed;
                adjustments.push(format!("Updated reading speed to {:.1} WPM", new_speed));
            }
        }
        
        if progress_difference < -10.0 {
            // User is significantly behind schedule
            adjustments.extend(Self::handle_behind_schedule(plan, progress_difference)?);
        } else if progress_difference > 10.0 {
            // User is ahead of schedule
            adjustments.extend(Self::handle_ahead_of_schedule(plan, progress_difference)?);
        }
        
        plan.current_progress = current_progress;
        plan.updated_at = current_date;
        
        Ok(adjustments)
    }

    /// Handle when user is behind schedule
    fn handle_behind_schedule(plan: &mut KhatmaPlan, deficit: f64) -> Result<Vec<String>> {
        let mut adjustments = Vec::new();
        let remaining_days = (plan.target_date - Utc::now()).num_days();
        
        if remaining_days <= 0 {
            return Ok(vec!["Plan completion date has passed. Consider creating a new plan.".to_string()]);
        }
        
        // Strategy 1: Increase daily reading time
        let time_increase = (deficit.abs() / 100.0 * plan.estimated_reading_time as f64 * 0.3) as i32;
        plan.estimated_reading_time += time_increase;
        adjustments.push(format!("Increased daily reading time by {} minutes", time_increase));
        
        // Strategy 2: Redistribute remaining portions
        let incomplete_portions: Vec<_> = plan.daily_portions
            .iter()
            .filter(|p| !p.completed && p.date >= Utc::now())
            .collect();
        
        if !incomplete_portions.is_empty() {
            let total_remaining_words: u32 = incomplete_portions
                .iter()
                .map(|p| p.word_count)
                .sum();
            
            let words_per_day = total_remaining_words / remaining_days as u32;
            
            // Update future portions
            for portion in plan.daily_portions.iter_mut() {
                if !portion.completed && portion.date >= Utc::now() {
                    portion.word_count = words_per_day;
                    portion.estimated_minutes = (words_per_day as f64 / plan.reading_speed_wpm * 60.0) as i32;
                }
            }
            
            adjustments.push("Redistributed remaining portions to catch up".to_string());
        }
        
        Ok(adjustments)
    }

    /// Handle when user is ahead of schedule
    fn handle_ahead_of_schedule(plan: &mut KhatmaPlan, surplus: f64) -> Result<Vec<String>> {
        let mut adjustments = Vec::new();
        
        // Strategy 1: Reduce daily reading time to maintain consistency
        let time_reduction = (surplus / 100.0 * plan.estimated_reading_time as f64 * 0.2) as i32;
        plan.estimated_reading_time = (plan.estimated_reading_time - time_reduction).max(10);
        adjustments.push(format!("Reduced daily reading time by {} minutes for better consistency", time_reduction));
        
        // Strategy 2: Suggest earlier completion or more reflection time
        adjustments.push("Consider adding more reflection time or completing earlier than planned".to_string());
        
        Ok(adjustments)
    }

    /// Suggest optimal reading times based on user patterns and preferences
    pub fn suggest_reading_times(
        user_id: Uuid,
        plan: &KhatmaPlan,
        reading_history: &[ReadingSession],
    ) -> Vec<SmartReminder> {
        let mut suggestions = Vec::new();
        let current_date = Utc::now();
        
        // Analyze user's historical reading patterns
        let pattern_analysis = Self::analyze_reading_patterns(reading_history);
        
        // Get upcoming incomplete portions
        let upcoming_portions: Vec<_> = plan.daily_portions
            .iter()
            .filter(|p| !p.completed && p.date >= current_date.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc())
            .take(7) // Next 7 days
            .collect();
        
        for portion in upcoming_portions {
            let optimal_times = Self::calculate_optimal_times_for_day(
                portion,
                &plan.preferred_reading_times,
                &pattern_analysis,
            );
            
            for (time, confidence) in optimal_times {
                let suggested_datetime = portion.date.date_naive()
                    .and_time(time)
                    .and_utc();
                
                suggestions.push(SmartReminder {
                    id: Uuid::new_v4(),
                    user_id,
                    khatma_plan_id: plan.id,
                    suggested_time: suggested_datetime,
                    duration_minutes: portion.estimated_minutes,
                    portion: portion.clone(),
                    confidence_score: confidence,
                    reasoning: Self::generate_suggestion_reasoning(confidence, &pattern_analysis),
                    created_at: Utc::now(),
                });
            }
        }
        
        // Sort by confidence score and date
        suggestions.sort_by(|a, b| {
            b.confidence_score.partial_cmp(&a.confidence_score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.suggested_time.cmp(&b.suggested_time))
        });
        
        suggestions.into_iter().take(10).collect() // Return top 10 suggestions
    }

    /// Analyze user's reading patterns from history
    fn analyze_reading_patterns(sessions: &[ReadingSession]) -> HashMap<String, f64> {
        let mut patterns = HashMap::new();
        
        if sessions.is_empty() {
            return patterns;
        }
        
        // Analyze preferred hours
        let mut hour_counts = HashMap::new();
        let mut day_counts = HashMap::new();
        
        for session in sessions {
            let hour = session.start_time.hour();
            let day = session.start_time.weekday().num_days_from_sunday();
            
            *hour_counts.entry(hour).or_insert(0) += 1;
            *day_counts.entry(day).or_insert(0) += 1;
        }
        
        // Find most common reading hours
        if let Some((&best_hour, &count)) = hour_counts.iter().max_by_key(|(_, &count)| count) {
            patterns.insert("preferred_hour".to_string(), best_hour as f64);
            patterns.insert("hour_consistency".to_string(), count as f64 / sessions.len() as f64);
        }
        
        // Calculate average session duration
        let avg_duration: f64 = sessions
            .iter()
            .filter_map(|s| s.duration_minutes)
            .map(|d| d as f64)
            .sum::<f64>() / sessions.len() as f64;
        
        patterns.insert("avg_duration".to_string(), avg_duration);
        
        patterns
    }

    /// Calculate optimal times for a specific day
    fn calculate_optimal_times_for_day(
        portion: &DailyPortion,
        preferred_times: &[PreferredReadingTime],
        patterns: &HashMap<String, f64>,
    ) -> Vec<(chrono::NaiveTime, f64)> {
        let mut optimal_times = Vec::new();
        let day_of_week = portion.date.weekday().num_days_from_sunday() as u8;
        
        // Check user's explicit preferences for this day
        for pref_time in preferred_times {
            if pref_time.days_of_week.contains(&day_of_week) {
                let confidence = match pref_time.priority {
                    ReadingTimePriority::High => 0.9,
                    ReadingTimePriority::Medium => 0.7,
                    ReadingTimePriority::Low => 0.5,
                };
                optimal_times.push((pref_time.time, confidence));
            }
        }
        
        // Add suggestions based on historical patterns
        if let Some(&preferred_hour) = patterns.get("preferred_hour") {
            let time = chrono::NaiveTime::from_hms_opt(preferred_hour as u32, 0, 0).unwrap();
            let consistency = patterns.get("hour_consistency").unwrap_or(&0.5);
            optimal_times.push((time, *consistency));
        }
        
        // Add general optimal times if no specific preferences
        if optimal_times.is_empty() {
            // Default optimal times for Quran reading
            let default_times = vec![
                (chrono::NaiveTime::from_hms_opt(5, 30, 0).unwrap(), 0.8), // After Fajr
                (chrono::NaiveTime::from_hms_opt(8, 0, 0).unwrap(), 0.6),  // Morning
                (chrono::NaiveTime::from_hms_opt(14, 0, 0).unwrap(), 0.5), // After Dhuhr
                (chrono::NaiveTime::from_hms_opt(20, 0, 0).unwrap(), 0.7), // Evening
                (chrono::NaiveTime::from_hms_opt(22, 0, 0).unwrap(), 0.6), // Night
            ];
            optimal_times.extend(default_times);
        }
        
        optimal_times.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        optimal_times
    }

    /// Generate reasoning for suggestion
    fn generate_suggestion_reasoning(confidence: f64, patterns: &HashMap<String, f64>) -> String {
        if confidence > 0.8 {
            "Based on your preferred reading times and high consistency".to_string()
        } else if confidence > 0.6 {
            "Matches your historical reading patterns".to_string()
        } else if patterns.contains_key("preferred_hour") {
            "Similar to your usual reading time".to_string()
        } else {
            "Optimal time for Quran reading based on Islamic traditions".to_string()
        }
    }

    /// Get simplified Quran structure for calculations
    fn get_quran_structure() -> HashMap<u8, (u16, u32)> {
        // Simplified structure: surah_number -> (total_ayahs, avg_words_per_ayah)
        // This is a simplified version - in production, use exact word counts
        let mut structure = HashMap::new();
        
        // Some key surahs with approximate data
        structure.insert(1, (7, 4));      // Al-Fatiha
        structure.insert(2, (286, 25));   // Al-Baqarah
        structure.insert(3, (200, 20));   // Ali 'Imran
        structure.insert(4, (176, 22));   // An-Nisa
        structure.insert(5, (120, 18));   // Al-Ma'idah
        
        // For other surahs, use average estimates
        for surah in 6..=114 {
            let ayah_count = Self::get_surah_ayah_count(surah);
            let avg_words = if surah <= 9 { 20 } else if surah <= 50 { 15 } else { 8 };
            structure.insert(surah, (ayah_count, avg_words));
        }
        
        structure
    }

    /// Get ayah count for a surah (simplified version)
    fn get_surah_ayah_count(surah: u8) -> u16 {
        match surah {
            1 => 7, 2 => 286, 3 => 200, 4 => 176, 5 => 120,
            6 => 165, 7 => 206, 8 => 75, 9 => 129, 10 => 109,
            // ... (in production, include all 114 surahs)
            114 => 6,
            _ => 20, // Default estimate
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, NaiveTime};

    #[test]
    fn test_calculate_reading_speed() {
        let sessions = vec![
            ReadingSession {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                khatma_plan_id: Uuid::new_v4(),
                surah_start: 1,
                ayah_start: 1,
                surah_end: 1,
                ayah_end: 7,
                start_time: Utc::now(),
                end_time: Some(Utc::now() + Duration::minutes(10)),
                duration_minutes: Some(10),
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
                start_time: Utc::now(),
                end_time: Some(Utc::now() + Duration::minutes(15)),
                duration_minutes: Some(15),
                word_count: 200,
                reading_speed_wpm: Some(180.0),
                created_at: Utc::now(),
            },
        ];

        let speed = PlanningAlgorithms::calculate_reading_speed(&sessions);
        assert!(speed > 120.0 && speed < 180.0);
    }

    #[test]
    fn test_create_adaptive_plan() {
        let user_id = Uuid::new_v4();
        let target_date = Utc::now() + Duration::days(30);
        let preferences = KhatmaPreferences {
            target_completion_days: Some(30),
            daily_reading_time_minutes: Some(60),
            preferred_reading_times: vec![
                PreferredReadingTime {
                    time: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                    duration_minutes: 30,
                    priority: ReadingTimePriority::High,
                    days_of_week: vec![1, 2, 3, 4, 5], // Weekdays
                }
            ],
            adaptive_scheduling: true,
            reminder_settings: ReminderSettings {
                enabled: true,
                advance_minutes: 15,
                smart_timing: true,
                missed_reading_reminder: true,
                progress_updates: true,
            },
            difficulty_preference: DifficultyPreference::Medium,
        };

        let plan = PlanningAlgorithms::create_adaptive_plan(
            user_id,
            target_date,
            &preferences,
            150.0,
        ).unwrap();

        assert_eq!(plan.user_id, user_id);
        assert_eq!(plan.target_date, target_date);
        assert!(!plan.daily_portions.is_empty());
        assert_eq!(plan.status, KhatmaStatus::Active);
    }
}