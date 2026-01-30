-- Smart Khatma System Migration
-- This migration creates tables for the interactive Khatma planning system

-- Khatma plans table
CREATE TABLE IF NOT EXISTS khatma_plans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    target_date TIMESTAMPTZ NOT NULL,
    start_date TIMESTAMPTZ NOT NULL,
    estimated_reading_time INTEGER NOT NULL, -- minutes per day
    adaptive_schedule BOOLEAN NOT NULL DEFAULT true,
    current_progress DECIMAL(5,2) NOT NULL DEFAULT 0.0, -- percentage 0.00 to 100.00
    reading_speed_wpm DECIMAL(6,2) NOT NULL DEFAULT 150.0, -- words per minute
    status TEXT NOT NULL DEFAULT 'Active', -- JSON serialized KhatmaStatus
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT khatma_plans_user_id_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT khatma_plans_progress_check CHECK (current_progress >= 0.0 AND current_progress <= 100.0),
    CONSTRAINT khatma_plans_speed_check CHECK (reading_speed_wpm > 0.0),
    CONSTRAINT khatma_plans_dates_check CHECK (target_date > start_date)
);

-- Daily portions table
CREATE TABLE IF NOT EXISTS daily_portions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    khatma_plan_id UUID NOT NULL,
    date TIMESTAMPTZ NOT NULL,
    surah_start SMALLINT NOT NULL CHECK (surah_start >= 1 AND surah_start <= 114),
    ayah_start INTEGER NOT NULL CHECK (ayah_start >= 1),
    surah_end SMALLINT NOT NULL CHECK (surah_end >= 1 AND surah_end <= 114),
    ayah_end INTEGER NOT NULL CHECK (ayah_end >= 1),
    estimated_minutes INTEGER NOT NULL CHECK (estimated_minutes > 0),
    word_count INTEGER NOT NULL CHECK (word_count > 0),
    completed BOOLEAN NOT NULL DEFAULT false,
    actual_reading_time INTEGER, -- actual time spent in minutes
    completion_date TIMESTAMPTZ,
    
    CONSTRAINT daily_portions_plan_fk FOREIGN KEY (khatma_plan_id) REFERENCES khatma_plans(id) ON DELETE CASCADE,
    CONSTRAINT daily_portions_surah_order_check CHECK (
        surah_start < surah_end OR (surah_start = surah_end AND ayah_start <= ayah_end)
    ),
    CONSTRAINT daily_portions_completion_check CHECK (
        (completed = false AND completion_date IS NULL) OR
        (completed = true AND completion_date IS NOT NULL)
    )
);

-- Preferred reading times table
CREATE TABLE IF NOT EXISTS preferred_reading_times (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    khatma_plan_id UUID NOT NULL,
    time TIME NOT NULL,
    duration_minutes INTEGER NOT NULL CHECK (duration_minutes > 0),
    priority TEXT NOT NULL, -- JSON serialized ReadingTimePriority
    days_of_week TEXT NOT NULL, -- JSON array of day numbers (0=Sunday, 1=Monday, etc.)
    
    CONSTRAINT preferred_times_plan_fk FOREIGN KEY (khatma_plan_id) REFERENCES khatma_plans(id) ON DELETE CASCADE
);

-- Reading sessions table
CREATE TABLE IF NOT EXISTS reading_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    khatma_plan_id UUID NOT NULL,
    surah_start SMALLINT NOT NULL CHECK (surah_start >= 1 AND surah_start <= 114),
    ayah_start INTEGER NOT NULL CHECK (ayah_start >= 1),
    surah_end SMALLINT NOT NULL CHECK (surah_end >= 1 AND surah_end <= 114),
    ayah_end INTEGER NOT NULL CHECK (ayah_end >= 1),
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ,
    duration_minutes INTEGER,
    word_count INTEGER NOT NULL CHECK (word_count > 0),
    reading_speed_wpm DECIMAL(6,2), -- calculated words per minute
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT reading_sessions_user_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT reading_sessions_plan_fk FOREIGN KEY (khatma_plan_id) REFERENCES khatma_plans(id) ON DELETE CASCADE,
    CONSTRAINT reading_sessions_time_check CHECK (end_time IS NULL OR end_time > start_time),
    CONSTRAINT reading_sessions_duration_check CHECK (
        (end_time IS NULL AND duration_minutes IS NULL) OR
        (end_time IS NOT NULL AND duration_minutes IS NOT NULL AND duration_minutes > 0)
    ),
    CONSTRAINT reading_sessions_speed_check CHECK (reading_speed_wpm IS NULL OR reading_speed_wpm > 0.0)
);

-- Reading statistics table
CREATE TABLE IF NOT EXISTS reading_statistics (
    user_id UUID PRIMARY KEY,
    average_reading_speed_wpm DECIMAL(6,2) NOT NULL DEFAULT 150.0,
    total_reading_time_minutes INTEGER NOT NULL DEFAULT 0,
    completed_khatmas INTEGER NOT NULL DEFAULT 0,
    reading_consistency_score DECIMAL(3,2) NOT NULL DEFAULT 0.0, -- 0.00 to 1.00
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT reading_stats_user_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT reading_stats_speed_check CHECK (average_reading_speed_wpm > 0.0),
    CONSTRAINT reading_stats_time_check CHECK (total_reading_time_minutes >= 0),
    CONSTRAINT reading_stats_khatmas_check CHECK (completed_khatmas >= 0),
    CONSTRAINT reading_stats_consistency_check CHECK (reading_consistency_score >= 0.0 AND reading_consistency_score <= 1.0)
);

-- Plan adjustments log table
CREATE TABLE IF NOT EXISTS plan_adjustments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    khatma_plan_id UUID NOT NULL,
    new_target_date TIMESTAMPTZ,
    new_daily_time_minutes INTEGER,
    reason TEXT NOT NULL, -- JSON serialized AdjustmentReason
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT plan_adjustments_plan_fk FOREIGN KEY (khatma_plan_id) REFERENCES khatma_plans(id) ON DELETE CASCADE,
    CONSTRAINT plan_adjustments_time_check CHECK (new_daily_time_minutes IS NULL OR new_daily_time_minutes > 0)
);

-- Smart reminders table
CREATE TABLE IF NOT EXISTS smart_reminders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    khatma_plan_id UUID NOT NULL,
    suggested_time TIMESTAMPTZ NOT NULL,
    duration_minutes INTEGER NOT NULL CHECK (duration_minutes > 0),
    confidence_score DECIMAL(3,2) NOT NULL CHECK (confidence_score >= 0.0 AND confidence_score <= 1.0),
    reasoning TEXT NOT NULL,
    sent BOOLEAN NOT NULL DEFAULT false,
    responded BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT smart_reminders_user_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT smart_reminders_plan_fk FOREIGN KEY (khatma_plan_id) REFERENCES khatma_plans(id) ON DELETE CASCADE
);

-- Achievements table for gamification
CREATE TABLE IF NOT EXISTS khatma_achievements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    khatma_plan_id UUID,
    achievement_id TEXT NOT NULL,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    category TEXT NOT NULL, -- JSON serialized AchievementCategory
    earned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    CONSTRAINT khatma_achievements_user_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    CONSTRAINT khatma_achievements_plan_fk FOREIGN KEY (khatma_plan_id) REFERENCES khatma_plans(id) ON DELETE SET NULL,
    CONSTRAINT khatma_achievements_unique UNIQUE (user_id, achievement_id, khatma_plan_id)
);

-- Indexes for performance optimization
CREATE INDEX IF NOT EXISTS idx_khatma_plans_user_id ON khatma_plans(user_id);
CREATE INDEX IF NOT EXISTS idx_khatma_plans_status ON khatma_plans(status);
CREATE INDEX IF NOT EXISTS idx_khatma_plans_target_date ON khatma_plans(target_date);

CREATE INDEX IF NOT EXISTS idx_daily_portions_plan_id ON daily_portions(khatma_plan_id);
CREATE INDEX IF NOT EXISTS idx_daily_portions_date ON daily_portions(date);
CREATE INDEX IF NOT EXISTS idx_daily_portions_completed ON daily_portions(completed);

CREATE INDEX IF NOT EXISTS idx_reading_sessions_user_id ON reading_sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_reading_sessions_plan_id ON reading_sessions(khatma_plan_id);
CREATE INDEX IF NOT EXISTS idx_reading_sessions_start_time ON reading_sessions(start_time);

CREATE INDEX IF NOT EXISTS idx_smart_reminders_user_id ON smart_reminders(user_id);
CREATE INDEX IF NOT EXISTS idx_smart_reminders_suggested_time ON smart_reminders(suggested_time);
CREATE INDEX IF NOT EXISTS idx_smart_reminders_sent ON smart_reminders(sent);

CREATE INDEX IF NOT EXISTS idx_khatma_achievements_user_id ON khatma_achievements(user_id);
CREATE INDEX IF NOT EXISTS idx_khatma_achievements_earned_at ON khatma_achievements(earned_at);

-- Triggers for automatic updates
CREATE OR REPLACE FUNCTION update_khatma_plan_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_khatma_plan_timestamp
    BEFORE UPDATE ON khatma_plans
    FOR EACH ROW
    EXECUTE FUNCTION update_khatma_plan_timestamp();

-- Function to automatically update plan progress when portions are completed
CREATE OR REPLACE FUNCTION update_plan_progress()
RETURNS TRIGGER AS $$
DECLARE
    total_portions INTEGER;
    completed_portions INTEGER;
    new_progress DECIMAL(5,2);
BEGIN
    -- Get total and completed portions for the plan
    SELECT COUNT(*) INTO total_portions
    FROM daily_portions
    WHERE khatma_plan_id = NEW.khatma_plan_id;
    
    SELECT COUNT(*) INTO completed_portions
    FROM daily_portions
    WHERE khatma_plan_id = NEW.khatma_plan_id AND completed = true;
    
    -- Calculate new progress percentage
    IF total_portions > 0 THEN
        new_progress = (completed_portions::DECIMAL / total_portions::DECIMAL) * 100.0;
        
        -- Update the khatma plan progress
        UPDATE khatma_plans
        SET current_progress = new_progress,
            updated_at = NOW()
        WHERE id = NEW.khatma_plan_id;
        
        -- If 100% complete, mark plan as completed
        IF new_progress >= 100.0 THEN
            UPDATE khatma_plans
            SET status = '"Completed"',
                updated_at = NOW()
            WHERE id = NEW.khatma_plan_id;
        END IF;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_plan_progress
    AFTER UPDATE OF completed ON daily_portions
    FOR EACH ROW
    WHEN (OLD.completed = false AND NEW.completed = true)
    EXECUTE FUNCTION update_plan_progress();

-- Function to calculate reading speed from session data
CREATE OR REPLACE FUNCTION calculate_reading_speed()
RETURNS TRIGGER AS $$
BEGIN
    -- Calculate reading speed if duration and word count are available
    IF NEW.duration_minutes IS NOT NULL AND NEW.duration_minutes > 0 AND NEW.word_count > 0 THEN
        NEW.reading_speed_wpm = (NEW.word_count::DECIMAL / NEW.duration_minutes::DECIMAL) * 60.0;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_calculate_reading_speed
    BEFORE INSERT OR UPDATE ON reading_sessions
    FOR EACH ROW
    EXECUTE FUNCTION calculate_reading_speed();

-- Sample data for testing (optional)
-- INSERT INTO khatma_plans (user_id, target_date, start_date, estimated_reading_time, adaptive_schedule)
-- VALUES (
--     (SELECT id FROM users LIMIT 1),
--     NOW() + INTERVAL '30 days',
--     NOW(),
--     60,
--     true
-- );

-- Comments for documentation
COMMENT ON TABLE khatma_plans IS 'Interactive Khatma plans with adaptive scheduling';
COMMENT ON TABLE daily_portions IS 'Daily reading portions for each Khatma plan';
COMMENT ON TABLE preferred_reading_times IS 'User preferred times for Quran reading';
COMMENT ON TABLE reading_sessions IS 'Individual reading sessions tracking';
COMMENT ON TABLE reading_statistics IS 'User reading statistics and patterns';
COMMENT ON TABLE plan_adjustments IS 'Log of manual and automatic plan adjustments';
COMMENT ON TABLE smart_reminders IS 'AI-generated smart reading reminders';
COMMENT ON TABLE khatma_achievements IS 'Gamification achievements for completed Khatmas';

COMMENT ON COLUMN khatma_plans.adaptive_schedule IS 'Whether the plan automatically adjusts based on user progress';
COMMENT ON COLUMN khatma_plans.reading_speed_wpm IS 'User calculated reading speed in words per minute';
COMMENT ON COLUMN daily_portions.word_count IS 'Estimated word count for this portion';
COMMENT ON COLUMN reading_sessions.reading_speed_wpm IS 'Calculated reading speed for this session';
COMMENT ON COLUMN smart_reminders.confidence_score IS 'AI confidence in this suggestion (0.0 to 1.0)';
COMMENT ON COLUMN reading_statistics.reading_consistency_score IS 'User consistency score (0.0 to 1.0)';