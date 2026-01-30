use anyhow::{Result, Context};
use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use shared::TajweedErrorType;

use crate::progress_tracker::{
    UserProgressData, Achievement, AchievementCategory, Badge, BadgeRarity,
    StreakReward, MilestoneReward, MasteryLevel
};

/// Comprehensive reward and motivation system
pub struct RewardSystem {
    /// Achievement definitions
    achievements: HashMap<String, AchievementDefinition>,
    /// Badge definitions
    badges: HashMap<String, BadgeDefinition>,
    /// Streak reward definitions
    streak_rewards: Vec<StreakRewardDefinition>,
    /// Level system configuration
    level_system: LevelSystemConfig,
}

/// Definition of an achievement
#[derive(Debug, Clone)]
pub struct AchievementDefinition {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: AchievementCategory,
    pub points: u32,
    pub badge_id: Option<String>,
    pub requirements: AchievementRequirements,
    pub is_hidden: bool, // Hidden until unlocked
    pub is_repeatable: bool,
}

/// Requirements for unlocking an achievement
#[derive(Debug, Clone)]
pub enum AchievementRequirements {
    PracticeStreak { days: u32 },
    ScoreThreshold { score: f64, attempts: u32 },
    MasteryLevel { level: MasteryLevel, skills: Vec<TajweedErrorType> },
    TotalPracticeTime { minutes: u32 },
    ErrorReduction { error_type: TajweedErrorType, reduction_percent: f64 },
    ConsecutiveSessions { sessions: u32, min_score: f64 },
    SpecialMilestone { milestone_id: String },
    Combination { requirements: Vec<AchievementRequirements> },
}

/// Badge definition
#[derive(Debug, Clone)]
pub struct BadgeDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub rarity: BadgeRarity,
    pub unlock_message: String,
}

/// Streak reward definition
#[derive(Debug, Clone)]
pub struct StreakRewardDefinition {
    pub days: u32,
    pub reward_type: RewardType,
    pub value: u32,
    pub description: String,
    pub badge_id: Option<String>,
}

/// Types of rewards
#[derive(Debug, Clone, Serialize)]
pub enum RewardType {
    ExperiencePoints,
    Badge,
    Title,
    SpecialFeature,
    Encouragement,
}

/// Level system configuration
#[derive(Debug, Clone)]
pub struct LevelSystemConfig {
    pub base_exp_per_level: u32,
    pub exp_multiplier: f64,
    pub max_level: u32,
    pub level_titles: HashMap<u32, String>,
}

/// User's current reward status
#[derive(Debug, Clone, Serialize)]
pub struct UserRewardStatus {
    pub user_id: Uuid,
    pub total_experience: u32,
    pub current_level: u32,
    pub level_progress: f64, // 0.0 to 1.0
    pub level_title: String,
    pub earned_achievements: Vec<Achievement>,
    pub earned_badges: Vec<Badge>,
    pub active_streaks: Vec<ActiveStreak>,
    pub next_rewards: Vec<UpcomingReward>,
    pub lifetime_stats: LifetimeStats,
}

/// Active streak information
#[derive(Debug, Clone, Serialize)]
pub struct ActiveStreak {
    pub streak_type: StreakType,
    pub current_count: u32,
    pub best_count: u32,
    pub next_reward_at: u32,
    pub started_at: DateTime<Utc>,
}

/// Types of streaks
#[derive(Debug, Clone, Serialize)]
pub enum StreakType {
    DailyPractice,
    WeeklyGoal,
    ImprovementStreak,
    PerfectSessions,
}

/// Upcoming reward information
#[derive(Debug, Clone, Serialize)]
pub struct UpcomingReward {
    pub reward_id: String,
    pub title: String,
    pub description: String,
    pub progress: f64, // 0.0 to 1.0
    pub estimated_unlock: Option<DateTime<Utc>>,
    pub reward_type: RewardType,
}

/// Lifetime statistics for motivation
#[derive(Debug, Clone, Serialize)]
pub struct LifetimeStats {
    pub total_practice_sessions: u32,
    pub total_practice_minutes: u32,
    pub total_recordings: u32,
    pub best_streak_days: u32,
    pub ayahs_mastered: u32,
    pub perfect_sessions: u32,
    pub improvement_milestones: u32,
    pub rank_among_peers: Option<u32>, // Percentile ranking
}

/// Motivational message system
#[derive(Debug, Clone, Serialize)]
pub struct MotivationalMessage {
    pub message_id: String,
    pub title: String,
    pub content: String,
    pub message_type: MessageType,
    pub trigger_condition: MessageTrigger,
    pub expires_at: Option<DateTime<Utc>>,
}

/// Types of motivational messages
#[derive(Debug, Clone, Serialize)]
pub enum MessageType {
    Encouragement,
    Celebration,
    Challenge,
    Reminder,
    Tip,
}

/// Conditions that trigger messages
#[derive(Debug, Clone, Serialize)]
pub enum MessageTrigger {
    LowPerformance,
    HighPerformance,
    StreakBreak,
    NewAchievement,
    LongAbsence,
    Milestone,
}

/// Gamification elements
#[derive(Debug, Clone, Serialize)]
pub struct GamificationStatus {
    pub user_id: Uuid,
    pub current_challenges: Vec<Challenge>,
    pub leaderboard_position: Option<LeaderboardPosition>,
    pub seasonal_events: Vec<SeasonalEvent>,
    pub daily_goals: Vec<DailyGoal>,
    pub weekly_goals: Vec<WeeklyGoal>,
}

/// Challenge system
#[derive(Debug, Clone, Serialize)]
pub struct Challenge {
    pub challenge_id: String,
    pub title: String,
    pub description: String,
    pub difficulty: ChallengeDifficulty,
    pub target_value: f64,
    pub current_progress: f64,
    pub reward_points: u32,
    pub reward_badge: Option<String>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
    pub completed: bool,
}

/// Challenge difficulty levels
#[derive(Debug, Clone, Serialize)]
pub enum ChallengeDifficulty {
    Easy,
    Medium,
    Hard,
    Expert,
    Legendary,
}

/// Leaderboard position
#[derive(Debug, Clone, Serialize)]
pub struct LeaderboardPosition {
    pub category: LeaderboardCategory,
    pub position: u32,
    pub total_participants: u32,
    pub score: f64,
    pub percentile: f64,
}

/// Leaderboard categories
#[derive(Debug, Clone, Serialize)]
pub enum LeaderboardCategory {
    OverallScore,
    ImprovementRate,
    ConsistencyScore,
    StreakLength,
    AyahsMastered,
}

/// Seasonal events
#[derive(Debug, Clone, Serialize)]
pub struct SeasonalEvent {
    pub event_id: String,
    pub title: String,
    pub description: String,
    pub bonus_multiplier: f64,
    pub special_rewards: Vec<String>,
    pub starts_at: DateTime<Utc>,
    pub ends_at: DateTime<Utc>,
}

/// Daily goals
#[derive(Debug, Clone, Serialize)]
pub struct DailyGoal {
    pub goal_id: String,
    pub description: String,
    pub target_value: f64,
    pub current_progress: f64,
    pub reward_points: u32,
    pub completed: bool,
    pub date: DateTime<Utc>,
}

/// Weekly goals
#[derive(Debug, Clone, Serialize)]
pub struct WeeklyGoal {
    pub goal_id: String,
    pub description: String,
    pub target_value: f64,
    pub current_progress: f64,
    pub reward_points: u32,
    pub reward_badge: Option<String>,
    pub completed: bool,
    pub week_start: DateTime<Utc>,
}

impl RewardSystem {
    /// Create a new reward system
    pub fn new() -> Self {
        let mut system = Self {
            achievements: HashMap::new(),
            badges: HashMap::new(),
            streak_rewards: Vec::new(),
            level_system: LevelSystemConfig::default(),
        };
        
        system.initialize_achievements();
        system.initialize_badges();
        system.initialize_streak_rewards();
        system
    }
    
    /// Check for new achievements and rewards
    pub fn check_rewards(&self, user_progress: &UserProgressData) -> Result<RewardUpdate> {
        let mut new_achievements = Vec::new();
        let mut new_badges = Vec::new();
        let mut experience_gained = 0;
        let mut messages = Vec::new();
        
        // Check for new achievements
        for (_, achievement_def) in &self.achievements {
            if self.is_achievement_unlocked(achievement_def, user_progress)? {
                // Check if user already has this achievement
                if !user_progress.achievements.iter().any(|a| a.id == achievement_def.id) {
                    let achievement = Achievement {
                        id: achievement_def.id.clone(),
                        title: achievement_def.title.clone(),
                        description: achievement_def.description.clone(),
                        category: achievement_def.category.clone(),
                        earned_at: Utc::now(),
                        progress: 1.0,
                    };
                    
                    new_achievements.push(achievement);
                    experience_gained += achievement_def.points;
                    
                    // Award badge if associated
                    if let Some(badge_id) = &achievement_def.badge_id {
                        if let Some(badge_def) = self.badges.get(badge_id) {
                            let badge = Badge {
                                id: badge_def.id.clone(),
                                name: badge_def.name.clone(),
                                description: badge_def.description.clone(),
                                icon: badge_def.icon.clone(),
                                rarity: badge_def.rarity.clone(),
                                earned_at: Utc::now(),
                            };
                            new_badges.push(badge);
                            
                            messages.push(MotivationalMessage {
                                message_id: format!("msg_{}", Uuid::new_v4()),
                                title: "New Achievement Unlocked!".to_string(),
                                content: format!("Congratulations! You've earned: {}", achievement_def.title),
                                message_type: MessageType::Celebration,
                                trigger_condition: MessageTrigger::NewAchievement,
                                expires_at: Some(Utc::now() + Duration::days(7)),
                            });
                        }
                    }
                }
            }
        }
        
        // Check streak rewards
        let streak_rewards = self.check_streak_rewards(user_progress)?;
        experience_gained += streak_rewards.iter().map(|r| 100).sum::<u32>(); // Fixed value for now
        
        // Generate motivational messages
        let motivational_messages = self.generate_motivational_messages(user_progress)?;
        messages.extend(motivational_messages);
        
        Ok(RewardUpdate {
            new_achievements,
            new_badges,
            streak_rewards,
            experience_gained,
            level_up: self.check_level_up(user_progress, experience_gained),
            messages,
        })
    }
    
    /// Get user's current reward status
    pub fn get_user_reward_status(&self, user_progress: &UserProgressData) -> Result<UserRewardStatus> {
        let total_experience = self.calculate_total_experience(user_progress);
        let current_level = self.calculate_level(total_experience);
        let level_progress = self.calculate_level_progress(total_experience, current_level);
        let level_title = self.get_level_title(current_level);
        
        let active_streaks = self.get_active_streaks(user_progress);
        let next_rewards = self.get_next_rewards(user_progress)?;
        let lifetime_stats = self.calculate_lifetime_stats(user_progress);
        
        Ok(UserRewardStatus {
            user_id: user_progress.user_id,
            total_experience,
            current_level,
            level_progress,
            level_title,
            earned_achievements: user_progress.achievements.clone(),
            earned_badges: Vec::new(), // Would be loaded from database
            active_streaks,
            next_rewards,
            lifetime_stats,
        })
    }
    
    /// Generate daily goals for a user
    pub fn generate_daily_goals(&self, user_progress: &UserProgressData) -> Result<Vec<DailyGoal>> {
        let mut goals = Vec::new();
        let today = Utc::now();
        
        // Practice goal based on user's consistency
        let practice_target = if user_progress.overall_stats.consistency_score > 0.8 {
            30.0 // High consistency users get higher targets
        } else {
            15.0 // Lower targets for inconsistent users
        };
        
        goals.push(DailyGoal {
            goal_id: format!("daily_practice_{}", today.format("%Y%m%d")),
            description: format!("Practice for {} minutes today", practice_target),
            target_value: practice_target,
            current_progress: 0.0,
            reward_points: 50,
            completed: false,
            date: today,
        });
        
        // Score improvement goal
        let score_target = (user_progress.overall_stats.average_score + 0.05).min(1.0);
        goals.push(DailyGoal {
            goal_id: format!("score_improvement_{}", today.format("%Y%m%d")),
            description: format!("Achieve a score of {:.0}% or higher", score_target * 100.0),
            target_value: score_target,
            current_progress: 0.0,
            reward_points: 75,
            completed: false,
            date: today,
        });
        
        // Error reduction goal if user has weak points
        if let Some(weak_point) = user_progress.weak_points.first() {
            goals.push(DailyGoal {
                goal_id: format!("error_reduction_{}", today.format("%Y%m%d")),
                description: format!("Practice {} without errors", self.error_type_to_string(&weak_point.error_type)),
                target_value: 1.0,
                current_progress: 0.0,
                reward_points: 100,
                completed: false,
                date: today,
            });
        }
        
        Ok(goals)
    }
    
    /// Generate weekly goals for a user
    pub fn generate_weekly_goals(&self, user_progress: &UserProgressData) -> Result<Vec<WeeklyGoal>> {
        let mut goals = Vec::new();
        let week_start = Utc::now().date_naive().week(chrono::Weekday::Mon).first_day();
        let week_start = week_start.and_hms_opt(0, 0, 0).unwrap().and_utc();
        
        // Weekly practice time goal
        goals.push(WeeklyGoal {
            goal_id: format!("weekly_practice_{}", week_start.format("%Y%W")),
            description: "Practice for 3 hours this week".to_string(),
            target_value: 180.0, // 3 hours in minutes
            current_progress: 0.0,
            reward_points: 300,
            reward_badge: Some("consistent_learner".to_string()),
            completed: false,
            week_start,
        });
        
        // Master new ayahs goal
        goals.push(WeeklyGoal {
            goal_id: format!("master_ayahs_{}", week_start.format("%Y%W")),
            description: "Master 3 new ayahs this week".to_string(),
            target_value: 3.0,
            current_progress: 0.0,
            reward_points: 500,
            reward_badge: Some("ayah_master".to_string()),
            completed: false,
            week_start,
        });
        
        Ok(goals)
    }
    
    /// Generate challenges for a user
    pub fn generate_challenges(&self, user_progress: &UserProgressData) -> Result<Vec<Challenge>> {
        let mut challenges = Vec::new();
        let now = Utc::now();
        
        // Skill-based challenge
        match user_progress.skill_levels.overall_level.current_level {
            MasteryLevel::Beginner => {
                challenges.push(Challenge {
                    challenge_id: format!("beginner_challenge_{}", Uuid::new_v4()),
                    title: "First Steps".to_string(),
                    description: "Complete 10 practice sessions with 60% average score".to_string(),
                    difficulty: ChallengeDifficulty::Easy,
                    target_value: 10.0,
                    current_progress: 0.0,
                    reward_points: 200,
                    reward_badge: Some("first_steps".to_string()),
                    starts_at: now,
                    ends_at: now + Duration::weeks(2),
                    completed: false,
                });
            }
            MasteryLevel::Intermediate => {
                challenges.push(Challenge {
                    challenge_id: format!("intermediate_challenge_{}", Uuid::new_v4()),
                    title: "Tajweed Master".to_string(),
                    description: "Achieve 90% Tajweed accuracy in 5 consecutive sessions".to_string(),
                    difficulty: ChallengeDifficulty::Medium,
                    target_value: 5.0,
                    current_progress: 0.0,
                    reward_points: 500,
                    reward_badge: Some("tajweed_master".to_string()),
                    starts_at: now,
                    ends_at: now + Duration::weeks(3),
                    completed: false,
                });
            }
            _ => {}
        }
        
        // Streak challenge
        challenges.push(Challenge {
            challenge_id: format!("streak_challenge_{}", Uuid::new_v4()),
            title: "Consistency Champion".to_string(),
            description: "Maintain a 14-day practice streak".to_string(),
            difficulty: ChallengeDifficulty::Medium,
            target_value: 14.0,
            current_progress: user_progress.overall_stats.current_streak_days as f64,
            reward_points: 750,
            reward_badge: Some("consistency_champion".to_string()),
            starts_at: now,
            ends_at: now + Duration::weeks(4),
            completed: false,
        });
        
        Ok(challenges)
    }
    
    // Private helper methods
    
    fn initialize_achievements(&mut self) {
        // First recording achievement
        self.achievements.insert("first_recording".to_string(), AchievementDefinition {
            id: "first_recording".to_string(),
            title: "First Steps".to_string(),
            description: "Complete your first recitation recording".to_string(),
            category: AchievementCategory::Practice,
            points: 100,
            badge_id: Some("first_recording_badge".to_string()),
            requirements: AchievementRequirements::TotalPracticeTime { minutes: 1 },
            is_hidden: false,
            is_repeatable: false,
        });
        
        // Score achievements
        self.achievements.insert("good_score".to_string(), AchievementDefinition {
            id: "good_score".to_string(),
            title: "Good Recitation".to_string(),
            description: "Achieve a score of 70% or higher".to_string(),
            category: AchievementCategory::Improvement,
            points: 200,
            badge_id: Some("good_score_badge".to_string()),
            requirements: AchievementRequirements::ScoreThreshold { score: 0.7, attempts: 1 },
            is_hidden: false,
            is_repeatable: false,
        });
        
        self.achievements.insert("excellent_score".to_string(), AchievementDefinition {
            id: "excellent_score".to_string(),
            title: "Excellent Recitation".to_string(),
            description: "Achieve a score of 90% or higher".to_string(),
            category: AchievementCategory::Improvement,
            points: 500,
            badge_id: Some("excellent_score_badge".to_string()),
            requirements: AchievementRequirements::ScoreThreshold { score: 0.9, attempts: 1 },
            is_hidden: false,
            is_repeatable: false,
        });
        
        // Streak achievements
        self.achievements.insert("week_streak".to_string(), AchievementDefinition {
            id: "week_streak".to_string(),
            title: "Weekly Warrior".to_string(),
            description: "Practice for 7 consecutive days".to_string(),
            category: AchievementCategory::Consistency,
            points: 300,
            badge_id: Some("week_streak_badge".to_string()),
            requirements: AchievementRequirements::PracticeStreak { days: 7 },
            is_hidden: false,
            is_repeatable: true,
        });
        
        self.achievements.insert("month_streak".to_string(), AchievementDefinition {
            id: "month_streak".to_string(),
            title: "Monthly Master".to_string(),
            description: "Practice for 30 consecutive days".to_string(),
            category: AchievementCategory::Consistency,
            points: 1000,
            badge_id: Some("month_streak_badge".to_string()),
            requirements: AchievementRequirements::PracticeStreak { days: 30 },
            is_hidden: false,
            is_repeatable: true,
        });
        
        // Mastery achievements
        self.achievements.insert("tajweed_master".to_string(), AchievementDefinition {
            id: "tajweed_master".to_string(),
            title: "Tajweed Master".to_string(),
            description: "Achieve advanced level in Tajweed skills".to_string(),
            category: AchievementCategory::Mastery,
            points: 750,
            badge_id: Some("tajweed_master_badge".to_string()),
            requirements: AchievementRequirements::MasteryLevel { 
                level: MasteryLevel::Advanced, 
                skills: vec![TajweedErrorType::Ghunnah, TajweedErrorType::Madd, TajweedErrorType::Qalqalah] 
            },
            is_hidden: false,
            is_repeatable: false,
        });
        
        // Special achievements
        self.achievements.insert("perfectionist".to_string(), AchievementDefinition {
            id: "perfectionist".to_string(),
            title: "Perfectionist".to_string(),
            description: "Achieve 100% score in a recitation".to_string(),
            category: AchievementCategory::Special,
            points: 1000,
            badge_id: Some("perfectionist_badge".to_string()),
            requirements: AchievementRequirements::ScoreThreshold { score: 1.0, attempts: 1 },
            is_hidden: true,
            is_repeatable: false,
        });
    }
    
    fn initialize_badges(&mut self) {
        self.badges.insert("first_recording_badge".to_string(), BadgeDefinition {
            id: "first_recording_badge".to_string(),
            name: "First Recording".to_string(),
            description: "Completed first recitation recording".to_string(),
            icon: "🎤".to_string(),
            rarity: BadgeRarity::Common,
            unlock_message: "Welcome to your Quran recitation journey!".to_string(),
        });
        
        self.badges.insert("good_score_badge".to_string(), BadgeDefinition {
            id: "good_score_badge".to_string(),
            name: "Good Reciter".to_string(),
            description: "Achieved 70% score".to_string(),
            icon: "⭐".to_string(),
            rarity: BadgeRarity::Common,
            unlock_message: "Great progress! Keep improving!".to_string(),
        });
        
        self.badges.insert("excellent_score_badge".to_string(), BadgeDefinition {
            id: "excellent_score_badge".to_string(),
            name: "Excellent Reciter".to_string(),
            description: "Achieved 90% score".to_string(),
            icon: "🌟".to_string(),
            rarity: BadgeRarity::Rare,
            unlock_message: "Excellent recitation! You're becoming skilled!".to_string(),
        });
        
        self.badges.insert("week_streak_badge".to_string(), BadgeDefinition {
            id: "week_streak_badge".to_string(),
            name: "Weekly Warrior".to_string(),
            description: "7-day practice streak".to_string(),
            icon: "🔥".to_string(),
            rarity: BadgeRarity::Uncommon,
            unlock_message: "Consistency is key! Keep the streak going!".to_string(),
        });
        
        self.badges.insert("month_streak_badge".to_string(), BadgeDefinition {
            id: "month_streak_badge".to_string(),
            name: "Monthly Master".to_string(),
            description: "30-day practice streak".to_string(),
            icon: "🏆".to_string(),
            rarity: BadgeRarity::Epic,
            unlock_message: "Amazing dedication! You're truly committed!".to_string(),
        });
        
        self.badges.insert("tajweed_master_badge".to_string(), BadgeDefinition {
            id: "tajweed_master_badge".to_string(),
            name: "Tajweed Master".to_string(),
            description: "Advanced Tajweed skills".to_string(),
            icon: "📿".to_string(),
            rarity: BadgeRarity::Epic,
            unlock_message: "Mashallah! Your Tajweed skills are excellent!".to_string(),
        });
        
        self.badges.insert("perfectionist_badge".to_string(), BadgeDefinition {
            id: "perfectionist_badge".to_string(),
            name: "Perfectionist".to_string(),
            description: "Perfect recitation score".to_string(),
            icon: "💎".to_string(),
            rarity: BadgeRarity::Legendary,
            unlock_message: "Perfect! Your recitation is truly beautiful!".to_string(),
        });
    }
    
    fn initialize_streak_rewards(&mut self) {
        self.streak_rewards = vec![
            StreakRewardDefinition {
                days: 3,
                reward_type: RewardType::ExperiencePoints,
                value: 50,
                description: "3-day streak bonus".to_string(),
                badge_id: None,
            },
            StreakRewardDefinition {
                days: 7,
                reward_type: RewardType::Badge,
                value: 100,
                description: "Weekly streak achievement".to_string(),
                badge_id: Some("week_streak_badge".to_string()),
            },
            StreakRewardDefinition {
                days: 14,
                reward_type: RewardType::ExperiencePoints,
                value: 300,
                description: "Two-week streak bonus".to_string(),
                badge_id: None,
            },
            StreakRewardDefinition {
                days: 30,
                reward_type: RewardType::Badge,
                value: 500,
                description: "Monthly streak achievement".to_string(),
                badge_id: Some("month_streak_badge".to_string()),
            },
        ];
    }
    
    fn is_achievement_unlocked(&self, achievement: &AchievementDefinition, user_progress: &UserProgressData) -> Result<bool> {
        match &achievement.requirements {
            AchievementRequirements::PracticeStreak { days } => {
                Ok(user_progress.overall_stats.current_streak_days >= *days)
            }
            AchievementRequirements::ScoreThreshold { score, attempts: _ } => {
                Ok(user_progress.overall_stats.best_score >= *score)
            }
            AchievementRequirements::MasteryLevel { level, skills } => {
                // Check if user has reached the required mastery level in specified skills
                let overall_level_met = match (&user_progress.skill_levels.overall_level.current_level, level) {
                    (MasteryLevel::Expert, _) => true,
                    (MasteryLevel::Advanced, MasteryLevel::Advanced | MasteryLevel::Intermediate | MasteryLevel::Elementary | MasteryLevel::Beginner) => true,
                    (MasteryLevel::Intermediate, MasteryLevel::Intermediate | MasteryLevel::Elementary | MasteryLevel::Beginner) => true,
                    (MasteryLevel::Elementary, MasteryLevel::Elementary | MasteryLevel::Beginner) => true,
                    (MasteryLevel::Beginner, MasteryLevel::Beginner) => true,
                    _ => false,
                };
                
                // For now, just check overall level. In production, would check specific skills
                Ok(overall_level_met && skills.len() <= 3) // Simplified check
            }
            AchievementRequirements::TotalPracticeTime { minutes } => {
                Ok(user_progress.overall_stats.total_practice_time_minutes >= *minutes)
            }
            AchievementRequirements::ErrorReduction { error_type: _, reduction_percent: _ } => {
                // Would implement error reduction tracking
                Ok(false) // Placeholder
            }
            AchievementRequirements::ConsecutiveSessions { sessions: _, min_score: _ } => {
                // Would implement consecutive session tracking
                Ok(false) // Placeholder
            }
            AchievementRequirements::SpecialMilestone { milestone_id: _ } => {
                // Would check milestone completion
                Ok(false) // Placeholder
            }
            AchievementRequirements::Combination { requirements: _ } => {
                // Would check all requirements in combination
                Ok(false) // Placeholder
            }
        }
    }
    
    fn check_streak_rewards(&self, user_progress: &UserProgressData) -> Result<Vec<StreakReward>> {
        let mut rewards = Vec::new();
        let current_streak = user_progress.overall_stats.current_streak_days;
        
        for streak_def in &self.streak_rewards {
            if current_streak >= streak_def.days && current_streak % streak_def.days == 0 {
                rewards.push(StreakReward {
                    days: streak_def.days,
                    reward_type: format!("{:?}", streak_def.reward_type),
                    description: streak_def.description.clone(),
                    earned: true,
                });
            }
        }
        
        Ok(rewards)
    }
    
    fn generate_motivational_messages(&self, user_progress: &UserProgressData) -> Result<Vec<MotivationalMessage>> {
        let mut messages = Vec::new();
        
        // Check for low performance
        if user_progress.overall_stats.average_score < 0.5 {
            messages.push(MotivationalMessage {
                message_id: format!("msg_{}", Uuid::new_v4()),
                title: "Keep Going!".to_string(),
                content: "Every expert was once a beginner. Your dedication will pay off!".to_string(),
                message_type: MessageType::Encouragement,
                trigger_condition: MessageTrigger::LowPerformance,
                expires_at: Some(Utc::now() + Duration::days(3)),
            });
        }
        
        // Check for high performance
        if user_progress.overall_stats.average_score > 0.8 {
            messages.push(MotivationalMessage {
                message_id: format!("msg_{}", Uuid::new_v4()),
                title: "Excellent Work!".to_string(),
                content: "Your recitation is becoming beautiful! Keep up the excellent work!".to_string(),
                message_type: MessageType::Celebration,
                trigger_condition: MessageTrigger::HighPerformance,
                expires_at: Some(Utc::now() + Duration::days(5)),
            });
        }
        
        // Check for streak milestones
        if user_progress.overall_stats.current_streak_days > 0 && user_progress.overall_stats.current_streak_days % 7 == 0 {
            messages.push(MotivationalMessage {
                message_id: format!("msg_{}", Uuid::new_v4()),
                title: "Streak Milestone!".to_string(),
                content: format!("Amazing! You've practiced for {} days in a row!", user_progress.overall_stats.current_streak_days),
                message_type: MessageType::Celebration,
                trigger_condition: MessageTrigger::Milestone,
                expires_at: Some(Utc::now() + Duration::days(7)),
            });
        }
        
        Ok(messages)
    }
    
    fn calculate_total_experience(&self, user_progress: &UserProgressData) -> u32 {
        // Base experience from practice
        let practice_exp = user_progress.overall_stats.total_practice_time_minutes * 2;
        
        // Bonus experience from achievements
        let achievement_exp: u32 = user_progress.achievements.iter()
            .filter_map(|a| self.achievements.get(&a.id))
            .map(|def| def.points)
            .sum();
        
        // Score-based experience
        let score_exp = (user_progress.overall_stats.average_score * 1000.0) as u32;
        
        practice_exp + achievement_exp + score_exp
    }
    
    fn calculate_level(&self, total_experience: u32) -> u32 {
        let mut level = 1;
        let mut exp_needed = self.level_system.base_exp_per_level;
        let mut accumulated_exp = 0;
        
        while accumulated_exp + exp_needed <= total_experience && level < self.level_system.max_level {
            accumulated_exp += exp_needed;
            level += 1;
            exp_needed = (exp_needed as f64 * self.level_system.exp_multiplier) as u32;
        }
        
        level
    }
    
    fn calculate_level_progress(&self, total_experience: u32, current_level: u32) -> f64 {
        if current_level >= self.level_system.max_level {
            return 1.0;
        }
        
        let mut exp_for_current_level = 0;
        let mut exp_needed = self.level_system.base_exp_per_level;
        
        for _ in 1..current_level {
            exp_for_current_level += exp_needed;
            exp_needed = (exp_needed as f64 * self.level_system.exp_multiplier) as u32;
        }
        
        let exp_in_current_level = total_experience - exp_for_current_level;
        exp_in_current_level as f64 / exp_needed as f64
    }
    
    fn get_level_title(&self, level: u32) -> String {
        self.level_system.level_titles.get(&level)
            .cloned()
            .unwrap_or_else(|| format!("Level {}", level))
    }
    
    fn get_active_streaks(&self, user_progress: &UserProgressData) -> Vec<ActiveStreak> {
        vec![
            ActiveStreak {
                streak_type: StreakType::DailyPractice,
                current_count: user_progress.overall_stats.current_streak_days,
                best_count: user_progress.overall_stats.longest_streak_days,
                next_reward_at: self.get_next_streak_reward(user_progress.overall_stats.current_streak_days),
                started_at: Utc::now() - Duration::days(user_progress.overall_stats.current_streak_days as i64),
            }
        ]
    }
    
    fn get_next_streak_reward(&self, current_streak: u32) -> u32 {
        for streak_def in &self.streak_rewards {
            if streak_def.days > current_streak {
                return streak_def.days;
            }
        }
        current_streak + 30 // Default next milestone
    }
    
    fn get_next_rewards(&self, user_progress: &UserProgressData) -> Result<Vec<UpcomingReward>> {
        let mut upcoming = Vec::new();
        
        // Check achievements close to completion
        for (_, achievement_def) in &self.achievements {
            if !user_progress.achievements.iter().any(|a| a.id == achievement_def.id) {
                if let Some(progress) = self.calculate_achievement_progress(achievement_def, user_progress) {
                    if progress > 0.5 { // Show achievements that are more than 50% complete
                        upcoming.push(UpcomingReward {
                            reward_id: achievement_def.id.clone(),
                            title: achievement_def.title.clone(),
                            description: achievement_def.description.clone(),
                            progress,
                            estimated_unlock: None, // Would calculate based on current progress rate
                            reward_type: RewardType::Badge,
                        });
                    }
                }
            }
        }
        
        // Sort by progress (closest to completion first)
        upcoming.sort_by(|a, b| b.progress.partial_cmp(&a.progress).unwrap_or(std::cmp::Ordering::Equal));
        upcoming.truncate(5); // Show top 5
        
        Ok(upcoming)
    }
    
    fn calculate_achievement_progress(&self, achievement: &AchievementDefinition, user_progress: &UserProgressData) -> Option<f64> {
        match &achievement.requirements {
            AchievementRequirements::PracticeStreak { days } => {
                Some((user_progress.overall_stats.current_streak_days as f64 / *days as f64).min(1.0))
            }
            AchievementRequirements::ScoreThreshold { score, attempts: _ } => {
                Some((user_progress.overall_stats.best_score / score).min(1.0))
            }
            AchievementRequirements::TotalPracticeTime { minutes } => {
                Some((user_progress.overall_stats.total_practice_time_minutes as f64 / *minutes as f64).min(1.0))
            }
            _ => None, // Other types would need specific calculations
        }
    }
    
    fn calculate_lifetime_stats(&self, user_progress: &UserProgressData) -> LifetimeStats {
        LifetimeStats {
            total_practice_sessions: user_progress.practice_history.len() as u32,
            total_practice_minutes: user_progress.overall_stats.total_practice_time_minutes,
            total_recordings: user_progress.overall_stats.total_recordings,
            best_streak_days: user_progress.overall_stats.longest_streak_days,
            ayahs_mastered: user_progress.overall_stats.mastered_ayahs,
            perfect_sessions: 0, // Would calculate from practice history
            improvement_milestones: user_progress.achievements.len() as u32,
            rank_among_peers: None, // Would calculate from peer data
        }
    }
    
    fn check_level_up(&self, user_progress: &UserProgressData, experience_gained: u32) -> Option<u32> {
        let old_total_exp = self.calculate_total_experience(user_progress);
        let new_total_exp = old_total_exp + experience_gained;
        
        let old_level = self.calculate_level(old_total_exp);
        let new_level = self.calculate_level(new_total_exp);
        
        if new_level > old_level {
            Some(new_level)
        } else {
            None
        }
    }
    
    fn error_type_to_string(&self, error_type: &TajweedErrorType) -> &str {
        match error_type {
            TajweedErrorType::Ghunnah => "Ghunnah",
            TajweedErrorType::Qalqalah => "Qalqalah",
            TajweedErrorType::Madd => "Madd",
            TajweedErrorType::Idgham => "Idgham",
            TajweedErrorType::Ikhfa => "Ikhfa",
            TajweedErrorType::Pronunciation => "Pronunciation",
            TajweedErrorType::Timing => "Timing",
            TajweedErrorType::Other(_) => "Other",
        }
    }
}

/// Result of checking for rewards
#[derive(Debug, Clone, Serialize)]
pub struct RewardUpdate {
    pub new_achievements: Vec<Achievement>,
    pub new_badges: Vec<Badge>,
    pub streak_rewards: Vec<StreakReward>,
    pub experience_gained: u32,
    pub level_up: Option<u32>,
    pub messages: Vec<MotivationalMessage>,
}

impl Default for LevelSystemConfig {
    fn default() -> Self {
        let mut level_titles = HashMap::new();
        level_titles.insert(1, "Beginner Reciter".to_string());
        level_titles.insert(5, "Learning Student".to_string());
        level_titles.insert(10, "Dedicated Practitioner".to_string());
        level_titles.insert(15, "Skilled Reciter".to_string());
        level_titles.insert(20, "Advanced Student".to_string());
        level_titles.insert(25, "Tajweed Scholar".to_string());
        level_titles.insert(30, "Master Reciter".to_string());
        level_titles.insert(40, "Quran Expert".to_string());
        level_titles.insert(50, "Recitation Master".to_string());
        
        Self {
            base_exp_per_level: 1000,
            exp_multiplier: 1.2,
            max_level: 50,
            level_titles,
        }
    }
}

impl Default for RewardSystem {
    fn default() -> Self {
        Self::new()
    }
}