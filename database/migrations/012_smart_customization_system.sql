-- Smart Customization System Migration
-- This migration creates tables for intelligent user behavior analysis and personalized recommendations

-- Create custom types for the customization system
CREATE TYPE activity_type AS ENUM (
    'quran_reading',
    'hadith_study', 
    'dhikr_reminders',
    'prayer_reminders',
    'islamic_stories',
    'learning',
    'reflection'
);

CREATE TYPE content_type AS ENUM (
    'quran_verses',
    'hadith_narrations',
    'islamic_stories',
    'tafsir',
    'dhikr',
    'duas',
    'islamic_history',
    'fiqh',
    'aqeedah',
    'seerah'
);

CREATE TYPE content_length AS ENUM (
    'short',
    'medium', 
    'long',
    'extended'
);

CREATE TYPE interaction_style AS ENUM (
    'casual',
    'structured',
    'intensive',
    'social',
    'independent'
);

CREATE TYPE motivation_trigger AS ENUM (
    'progress',
    'community',
    'reminders',
    'challenges',
    'rewards',
    'spiritual',
    'knowledge'
);

CREATE TYPE learning_style AS ENUM (
    'visual',
    'auditory',
    'reading',
    'kinesthetic',
    'mixed'
);

CREATE TYPE difficulty_level AS ENUM (
    'beginner',
    'intermediate',
    'advanced',
    'scholar',
    'adaptive'
);

CREATE TYPE islamic_season AS ENUM (
    'ramadan',
    'dhul_hijjah',
    'muharram',
    'rajab',
    'shaban',
    'laylat_al_qadr',
    'eid_al_fitr',
    'eid_al_adha',
    'ashura',
    'mawlid',
    'isra_miraj',
    'regular'
);

CREATE TYPE reminder_type AS ENUM (
    'prayer',
    'dhikr',
    'quran_reading',
    'charity',
    'fasting',
    'reflection',
    'learning',
    'community'
);

CREATE TYPE recommendation_category AS ENUM (
    'daily_reading',
    'seasonal',
    'learning',
    'spiritual',
    'community',
    'personal',
    'trending',
    'continuation',
    'discovery'
);

CREATE TYPE personalization_factor AS ENUM (
    'historical_response',
    'current_context',
    'user_mood',
    'activity_pattern',
    'seasonal_context',
    'personal_goals',
    'progress_status',
    'social_context'
);

CREATE TYPE message_tone AS ENUM (
    'gentle',
    'motivational',
    'formal',
    'friendly',
    'urgent',
    'reflective'
);

CREATE TYPE message_length AS ENUM (
    'brief',
    'short',
    'medium',
    'detailed'
);

CREATE TYPE reminder_response AS ENUM (
    'ignored',
    'dismissed',
    'postponed',
    'acknowledged',
    'acted',
    'completed'
);

CREATE TYPE recurrence_frequency AS ENUM (
    'daily',
    'weekly',
    'monthly',
    'yearly',
    'custom'
);

CREATE TYPE preference_type AS ENUM (
    'reading_time',
    'content_type',
    'notification_timing',
    'session_duration',
    'difficulty_level',
    'language_preference',
    'interaction_style',
    'motivation_trigger',
    'seasonal_pattern'
);

CREATE TYPE learning_source AS ENUM (
    'user_behavior',
    'explicit_feedback',
    'interaction_pattern',
    'response_rate',
    'completion_rate',
    'time_analysis',
    'contextual_clues'
);

CREATE TYPE validation_status AS ENUM (
    'pending',
    'confirmed',
    'rejected',
    'uncertain',
    'expired'
);

CREATE TYPE urgency_level AS ENUM (
    'low',
    'normal',
    'high',
    'critical'
);

CREATE TYPE flexibility_level AS ENUM (
    'rigid',
    'limited',
    'moderate',
    'flexible',
    'very_flexible'
);

CREATE TYPE period_type AS ENUM (
    'daily',
    'weekly',
    'monthly',
    'quarterly',
    'yearly',
    'custom'
);

CREATE TYPE insight_type AS ENUM (
    'pattern',
    'preference',
    'opportunity',
    'trend',
    'anomaly',
    'achievement'
);

CREATE TYPE priority AS ENUM (
    'low',
    'medium',
    'high',
    'critical'
);

-- User Behavior Profiles table
CREATE TABLE user_behavior_profiles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE,
    
    -- Reading patterns
    preferred_reading_times JSONB NOT NULL DEFAULT '[]',
    average_session_duration INTEGER NOT NULL DEFAULT 30,
    reading_consistency_score DECIMAL(3,2) NOT NULL DEFAULT 0.5 CHECK (reading_consistency_score >= 0 AND reading_consistency_score <= 1),
    preferred_content_types JSONB NOT NULL DEFAULT '[]',
    
    -- Interaction patterns
    notification_response_rate DECIMAL(3,2) NOT NULL DEFAULT 0.5 CHECK (notification_response_rate >= 0 AND notification_response_rate <= 1),
    preferred_notification_times JSONB NOT NULL DEFAULT '[]',
    engagement_patterns JSONB NOT NULL DEFAULT '{}',
    
    -- Learning preferences
    learning_style learning_style NOT NULL DEFAULT 'mixed',
    difficulty_preference difficulty_level NOT NULL DEFAULT 'intermediate',
    language_preferences TEXT[] NOT NULL DEFAULT ARRAY['ar', 'en'],
    
    -- Seasonal and contextual patterns
    seasonal_preferences JSONB NOT NULL DEFAULT '{}',
    location_based_preferences JSONB,
    
    -- Adaptive metrics
    adaptation_score DECIMAL(3,2) NOT NULL DEFAULT 0.5 CHECK (adaptation_score >= 0 AND adaptation_score <= 1),
    satisfaction_score DECIMAL(3,2) NOT NULL DEFAULT 0.5 CHECK (satisfaction_score >= 0 AND satisfaction_score <= 1),
    
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Personalized Recommendations table
CREATE TABLE personalized_recommendations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    content_type content_type NOT NULL,
    content_id TEXT NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    recommendation_score DECIMAL(3,2) NOT NULL CHECK (recommendation_score >= 0 AND recommendation_score <= 1),
    reasoning TEXT NOT NULL,
    
    -- Recommendation metadata
    estimated_duration INTEGER NOT NULL, -- minutes
    difficulty_level difficulty_level NOT NULL,
    tags TEXT[] NOT NULL DEFAULT '{}',
    category recommendation_category NOT NULL,
    
    -- Tracking
    presented_at TIMESTAMP WITH TIME ZONE,
    interacted_at TIMESTAMP WITH TIME ZONE,
    completed_at TIMESTAMP WITH TIME ZONE,
    user_rating DECIMAL(2,1) CHECK (user_rating >= 1.0 AND user_rating <= 5.0),
    feedback TEXT,
    
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE,
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Adaptive Reminders table
CREATE TABLE adaptive_reminders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    reminder_type reminder_type NOT NULL,
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    
    -- Smart timing
    suggested_time TIMESTAMP WITH TIME ZONE NOT NULL,
    optimal_time_window JSONB NOT NULL DEFAULT '{}',
    adaptation_confidence DECIMAL(3,2) NOT NULL CHECK (adaptation_confidence >= 0 AND adaptation_confidence <= 1),
    
    -- Personalization
    personalization_factors personalization_factor[] NOT NULL DEFAULT '{}',
    content_customization JSONB NOT NULL DEFAULT '{}',
    
    -- Tracking and learning
    response_prediction DECIMAL(3,2) NOT NULL CHECK (response_prediction >= 0 AND response_prediction <= 1),
    actual_response JSONB,
    effectiveness_score DECIMAL(3,2) CHECK (effectiveness_score >= 0 AND effectiveness_score <= 1),
    
    -- Scheduling
    is_recurring BOOLEAN NOT NULL DEFAULT FALSE,
    recurrence_pattern JSONB,
    next_occurrence TIMESTAMP WITH TIME ZONE,
    
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Preference Learning Records table
CREATE TABLE preference_learning_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    preference_type preference_type NOT NULL,
    old_value JSONB NOT NULL,
    new_value JSONB NOT NULL,
    confidence_score DECIMAL(3,2) NOT NULL CHECK (confidence_score >= 0 AND confidence_score <= 1),
    learning_source learning_source NOT NULL,
    validation_status validation_status NOT NULL DEFAULT 'pending',
    impact_score DECIMAL(3,2) NOT NULL CHECK (impact_score >= 0 AND impact_score <= 1),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- User Interactions table (for behavior analysis)
CREATE TABLE user_interactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    interaction_type TEXT NOT NULL, -- 'positive', 'negative', 'neutral'
    activity_type activity_type,
    content_type content_type,
    content_id TEXT,
    duration_seconds INTEGER,
    context JSONB DEFAULT '{}',
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Content Interactions table (for content preference analysis)
CREATE TABLE content_interactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    content_type content_type NOT NULL,
    content_id TEXT NOT NULL,
    interaction_type TEXT NOT NULL, -- 'view', 'like', 'share', 'complete', 'skip'
    duration_seconds INTEGER,
    completed BOOLEAN DEFAULT FALSE,
    user_rating DECIMAL(2,1) CHECK (user_rating >= 1.0 AND user_rating <= 5.0),
    feedback TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Customization Analytics table
CREATE TABLE customization_analytics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    analysis_period_start TIMESTAMP WITH TIME ZONE NOT NULL,
    analysis_period_end TIMESTAMP WITH TIME ZONE NOT NULL,
    period_type period_type NOT NULL,
    
    -- Effectiveness metrics
    recommendation_accuracy DECIMAL(3,2) NOT NULL CHECK (recommendation_accuracy >= 0 AND recommendation_accuracy <= 1),
    reminder_effectiveness DECIMAL(3,2) NOT NULL CHECK (reminder_effectiveness >= 0 AND reminder_effectiveness <= 1),
    personalization_score DECIMAL(3,2) NOT NULL CHECK (personalization_score >= 0 AND personalization_score <= 1),
    
    -- Engagement metrics
    engagement_improvement DECIMAL(4,2) NOT NULL DEFAULT 0,
    satisfaction_trend DECIMAL(4,2) NOT NULL DEFAULT 0,
    retention_impact DECIMAL(3,2) NOT NULL DEFAULT 0,
    
    -- Learning metrics
    preference_stability DECIMAL(3,2) NOT NULL CHECK (preference_stability >= 0 AND preference_stability <= 1),
    adaptation_speed DECIMAL(3,2) NOT NULL CHECK (adaptation_speed >= 0 AND adaptation_speed <= 1),
    prediction_accuracy DECIMAL(3,2) NOT NULL CHECK (prediction_accuracy >= 0 AND prediction_accuracy <= 1),
    
    -- Content metrics
    content_diversity DECIMAL(3,2) NOT NULL CHECK (content_diversity >= 0 AND content_diversity <= 1),
    content_relevance DECIMAL(3,2) NOT NULL CHECK (content_relevance >= 0 AND content_relevance <= 1),
    completion_rate_improvement DECIMAL(4,2) NOT NULL DEFAULT 0,
    
    generated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

-- Create indexes for performance
CREATE INDEX idx_user_behavior_profiles_user_id ON user_behavior_profiles(user_id);
CREATE INDEX idx_personalized_recommendations_user_id ON personalized_recommendations(user_id);
CREATE INDEX idx_personalized_recommendations_created_at ON personalized_recommendations(created_at);
CREATE INDEX idx_personalized_recommendations_category ON personalized_recommendations(category);
CREATE INDEX idx_adaptive_reminders_user_id ON adaptive_reminders(user_id);
CREATE INDEX idx_adaptive_reminders_suggested_time ON adaptive_reminders(suggested_time);
CREATE INDEX idx_adaptive_reminders_reminder_type ON adaptive_reminders(reminder_type);
CREATE INDEX idx_preference_learning_records_user_id ON preference_learning_records(user_id);
CREATE INDEX idx_preference_learning_records_preference_type ON preference_learning_records(preference_type);
CREATE INDEX idx_user_interactions_user_id ON user_interactions(user_id);
CREATE INDEX idx_user_interactions_timestamp ON user_interactions(timestamp);
CREATE INDEX idx_user_interactions_activity_type ON user_interactions(activity_type);
CREATE INDEX idx_content_interactions_user_id ON content_interactions(user_id);
CREATE INDEX idx_content_interactions_content_type ON content_interactions(content_type);
CREATE INDEX idx_content_interactions_created_at ON content_interactions(created_at);
CREATE INDEX idx_customization_analytics_user_id ON customization_analytics(user_id);
CREATE INDEX idx_customization_analytics_period ON customization_analytics(analysis_period_start, analysis_period_end);

-- Create triggers for updating timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_user_behavior_profiles_updated_at 
    BEFORE UPDATE ON user_behavior_profiles 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_adaptive_reminders_updated_at 
    BEFORE UPDATE ON adaptive_reminders 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert some default behavior profiles for testing
INSERT INTO user_behavior_profiles (
    user_id,
    preferred_reading_times,
    preferred_content_types,
    engagement_patterns,
    seasonal_preferences
) VALUES (
    '00000000-0000-0000-0000-000000000001',
    '[
        {
            "start_time": "05:30:00",
            "end_time": "07:00:00",
            "activity_type": "quran_reading",
            "preference_strength": 0.8,
            "days_of_week": [1, 2, 3, 4, 5],
            "success_rate": 0.7
        },
        {
            "start_time": "20:00:00",
            "end_time": "21:30:00",
            "activity_type": "dhikr_reminders",
            "preference_strength": 0.6,
            "days_of_week": [0, 1, 2, 3, 4, 5, 6],
            "success_rate": 0.6
        }
    ]',
    '[
        {
            "content_type": "quran_verses",
            "preference_weight": 0.9,
            "interaction_frequency": 0.8,
            "completion_rate": 0.7
        },
        {
            "content_type": "hadith_narrations",
            "preference_weight": 0.7,
            "interaction_frequency": 0.5,
            "completion_rate": 0.6
        },
        {
            "content_type": "dhikr",
            "preference_weight": 0.8,
            "interaction_frequency": 0.6,
            "completion_rate": 0.8
        }
    ]',
    '{
        "peak_engagement_hours": [6, 7, 8, 19, 20, 21],
        "peak_engagement_days": [5, 6],
        "average_session_length": 35,
        "preferred_content_length": "medium",
        "interaction_style": "structured",
        "motivation_triggers": ["progress", "spiritual", "reminders"]
    }',
    '{
        "ramadan": {
            "season": "ramadan",
            "content_focus": ["quran_verses", "dhikr", "duas"],
            "activity_increase": 1.5,
            "preferred_reminders": ["quran_reading", "dhikr", "reflection"],
            "special_interests": ["night_prayers", "quran_completion", "charity"]
        }
    }'
);

-- Add some sample recommendations
INSERT INTO personalized_recommendations (
    user_id,
    content_type,
    content_id,
    title,
    description,
    recommendation_score,
    reasoning,
    estimated_duration,
    difficulty_level,
    tags,
    category
) VALUES (
    '00000000-0000-0000-0000-000000000001',
    'quran_verses',
    'surah_2_1_10',
    'سورة البقرة - الآيات 1-10',
    'قراءة مباركة من بداية سورة البقرة مع التدبر في معانيها العظيمة',
    0.92,
    'مناسبة لوقت القراءة المفضل صباحاً، ومستوى المستخدم المتوسط',
    25,
    'intermediate',
    ARRAY['morning_reading', 'quran', 'reflection'],
    'daily_reading'
),
(
    '00000000-0000-0000-0000-000000000001',
    'dhikr',
    'morning_adhkar',
    'أذكار الصباح',
    'أذكار الصباح المباركة لبداية يوم مليء بالبركة والخير',
    0.88,
    'يتماشى مع تفضيل المستخدم للأذكار في الصباح',
    15,
    'beginner',
    ARRAY['morning', 'dhikr', 'daily'],
    'spiritual'
);

-- Add some sample adaptive reminders
INSERT INTO adaptive_reminders (
    user_id,
    reminder_type,
    title,
    message,
    suggested_time,
    optimal_time_window,
    adaptation_confidence,
    personalization_factors,
    content_customization,
    response_prediction
) VALUES (
    '00000000-0000-0000-0000-000000000001',
    'quran_reading',
    'وقت القراءة المبارك',
    'حان وقت قراءة القرآن الكريم. ابدأ يومك بالبركة والهداية.',
    NOW() + INTERVAL '1 day' + INTERVAL '5 hours 30 minutes',
    '{
        "start_time": "05:30:00",
        "end_time": "07:00:00", 
        "preferred_time": "06:00:00",
        "flexibility_minutes": 30
    }',
    0.85,
    ARRAY['historical_response', 'activity_pattern', 'personal_goals'],
    '{
        "language": "ar",
        "tone": "gentle",
        "length": "short",
        "include_verse": true,
        "include_hadith": false,
        "include_motivation": true,
        "personalized_elements": ["user_name", "progress"]
    }',
    0.78
),
(
    '00000000-0000-0000-0000-000000000001',
    'dhikr',
    'أذكار المساء',
    'لا تنس أذكار المساء المباركة. اختتم يومك بذكر الله والاستغفار.',
    NOW() + INTERVAL '1 day' + INTERVAL '20 hours',
    '{
        "start_time": "19:30:00",
        "end_time": "21:00:00",
        "preferred_time": "20:00:00", 
        "flexibility_minutes": 45
    }',
    0.82,
    ARRAY['historical_response', 'seasonal_context', 'current_context'],
    '{
        "language": "ar",
        "tone": "reflective",
        "length": "medium",
        "include_verse": false,
        "include_hadith": true,
        "include_motivation": true,
        "personalized_elements": ["time_of_day", "consistency_praise"]
    }',
    0.73
);

COMMENT ON TABLE user_behavior_profiles IS 'Stores comprehensive user behavior profiles for smart customization';
COMMENT ON TABLE personalized_recommendations IS 'Stores AI-generated personalized content recommendations';
COMMENT ON TABLE adaptive_reminders IS 'Stores intelligent reminders that adapt to user habits and preferences';
COMMENT ON TABLE preference_learning_records IS 'Tracks learned user preferences and their validation status';
COMMENT ON TABLE user_interactions IS 'Records all user interactions for behavior analysis';
COMMENT ON TABLE content_interactions IS 'Records user interactions with specific content for preference learning';
COMMENT ON TABLE customization_analytics IS 'Stores analytics and metrics for customization effectiveness';