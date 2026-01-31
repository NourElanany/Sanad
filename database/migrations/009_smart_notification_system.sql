-- Smart Notification System for Islamic App
-- This migration creates tables for graduated notifications, dhikr reminders, and seasonal notifications

-- Notification types enum
CREATE TYPE notification_type AS ENUM (
    'prayer_reminder',
    'prayer_graduated',
    'sunnah_reminder',
    'nafl_reminder',
    'dhikr_reminder',
    'seasonal_reminder',
    'islamic_event',
    'khatma_reminder',
    'daily_verse'
);

-- Notification priority levels
CREATE TYPE notification_priority AS ENUM (
    'low',
    'medium',
    'high',
    'urgent'
);

-- Notification delivery status
CREATE TYPE notification_status AS ENUM (
    'pending',
    'sent',
    'delivered',
    'read',
    'dismissed',
    'failed'
);

-- Prayer names for graduated notifications
CREATE TYPE prayer_name AS ENUM (
    'fajr',
    'dhuhr',
    'asr',
    'maghrib',
    'isha'
);

-- Islamic seasons and special periods
CREATE TYPE islamic_season AS ENUM (
    'ramadan',
    'dhul_hijjah',
    'muharram',
    'rajab',
    'shaban',
    'laylat_al_qadr',
    'ashura',
    'mawlid',
    'isra_miraj'
);

-- Dhikr categories for time-appropriate reminders
CREATE TYPE dhikr_category AS ENUM (
    'morning',
    'evening',
    'after_prayer',
    'before_sleep',
    'after_wudu',
    'travel',
    'general'
);

-- Main notifications table
CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    notification_type notification_type NOT NULL,
    title VARCHAR(255) NOT NULL,
    body TEXT NOT NULL,
    priority notification_priority DEFAULT 'medium',
    status notification_status DEFAULT 'pending',
    
    -- Scheduling information
    scheduled_at TIMESTAMP WITH TIME ZONE NOT NULL,
    sent_at TIMESTAMP WITH TIME ZONE,
    delivered_at TIMESTAMP WITH TIME ZONE,
    read_at TIMESTAMP WITH TIME ZONE,
    
    -- Metadata for different notification types
    metadata JSONB DEFAULT '{}',
    
    -- Expiration and retry logic
    expires_at TIMESTAMP WITH TIME ZONE,
    retry_count INTEGER DEFAULT 0,
    max_retries INTEGER DEFAULT 3,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Prayer time notifications with graduated reminders
CREATE TABLE prayer_notifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    prayer_name prayer_name NOT NULL,
    prayer_time TIMESTAMP WITH TIME ZONE NOT NULL,
    
    -- Graduated notification settings
    enable_graduated BOOLEAN DEFAULT TRUE,
    reminder_intervals INTEGER[] DEFAULT '{30, 15, 5}', -- minutes before prayer
    
    -- Location context for prayer times
    latitude DECIMAL(10, 8),
    longitude DECIMAL(11, 8),
    timezone VARCHAR(50),
    
    -- Notification preferences
    enable_adhan BOOLEAN DEFAULT TRUE,
    enable_vibration BOOLEAN DEFAULT TRUE,
    custom_message TEXT,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Sunnah and Nafl reminders
CREATE TABLE sunnah_reminders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Sunnah details
    sunnah_name VARCHAR(255) NOT NULL,
    sunnah_description TEXT,
    sunnah_reference TEXT, -- Hadith or Quran reference
    
    -- Timing and frequency
    reminder_time TIME NOT NULL,
    frequency VARCHAR(20) DEFAULT 'daily', -- daily, weekly, monthly
    days_of_week INTEGER[], -- 0=Sunday, 1=Monday, etc.
    
    -- Notification settings
    is_active BOOLEAN DEFAULT TRUE,
    priority notification_priority DEFAULT 'medium',
    custom_message TEXT,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Islamic seasonal reminders
CREATE TABLE seasonal_reminders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Season information
    season islamic_season NOT NULL,
    event_name VARCHAR(255) NOT NULL,
    event_description TEXT,
    
    -- Timing (can be Hijri-based)
    hijri_month INTEGER, -- 1-12
    hijri_day INTEGER,   -- 1-30
    gregorian_date DATE, -- For fixed Gregorian dates
    
    -- Notification settings
    days_before_notification INTEGER DEFAULT 1,
    is_active BOOLEAN DEFAULT TRUE,
    priority notification_priority DEFAULT 'high',
    
    -- Content
    reminder_message TEXT,
    recommended_actions TEXT[],
    related_verses TEXT[],
    related_hadiths TEXT[],
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Dhikr reminders for time-appropriate notifications
CREATE TABLE dhikr_reminders (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Dhikr information
    dhikr_category dhikr_category NOT NULL,
    dhikr_text_arabic TEXT NOT NULL,
    dhikr_text_transliteration TEXT,
    dhikr_text_translation TEXT,
    dhikr_reference TEXT, -- Source reference
    
    -- Timing settings
    trigger_time TIME, -- For fixed time dhikr (morning/evening)
    trigger_after_prayer prayer_name, -- For post-prayer dhikr
    trigger_condition VARCHAR(100), -- Custom conditions
    
    -- Notification preferences
    is_active BOOLEAN DEFAULT TRUE,
    frequency VARCHAR(20) DEFAULT 'daily',
    priority notification_priority DEFAULT 'low',
    
    -- Repetition and tracking
    recommended_repetitions INTEGER DEFAULT 1,
    track_completion BOOLEAN DEFAULT FALSE,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- User notification preferences
CREATE TABLE user_notification_preferences (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Global notification settings
    notifications_enabled BOOLEAN DEFAULT TRUE,
    quiet_hours_start TIME DEFAULT '22:00',
    quiet_hours_end TIME DEFAULT '06:00',
    
    -- Prayer notification preferences
    prayer_notifications_enabled BOOLEAN DEFAULT TRUE,
    prayer_graduated_enabled BOOLEAN DEFAULT TRUE,
    prayer_reminder_intervals INTEGER[] DEFAULT '{30, 15, 5}',
    
    -- Sunnah and Nafl preferences
    sunnah_reminders_enabled BOOLEAN DEFAULT TRUE,
    nafl_reminders_enabled BOOLEAN DEFAULT TRUE,
    
    -- Dhikr preferences
    dhikr_reminders_enabled BOOLEAN DEFAULT TRUE,
    morning_dhikr_time TIME DEFAULT '06:00',
    evening_dhikr_time TIME DEFAULT '18:00',
    
    -- Seasonal preferences
    seasonal_reminders_enabled BOOLEAN DEFAULT TRUE,
    ramadan_reminders_enabled BOOLEAN DEFAULT TRUE,
    hajj_reminders_enabled BOOLEAN DEFAULT TRUE,
    
    -- Delivery preferences
    push_notifications BOOLEAN DEFAULT TRUE,
    email_notifications BOOLEAN DEFAULT FALSE,
    sms_notifications BOOLEAN DEFAULT FALSE,
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    UNIQUE(user_id)
);

-- Notification delivery log for tracking and analytics
CREATE TABLE notification_delivery_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    notification_id UUID NOT NULL REFERENCES notifications(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    
    -- Delivery details
    delivery_method VARCHAR(20) NOT NULL, -- push, email, sms
    delivery_status notification_status NOT NULL,
    delivery_attempt INTEGER DEFAULT 1,
    
    -- Response tracking
    opened_at TIMESTAMP WITH TIME ZONE,
    clicked_at TIMESTAMP WITH TIME ZONE,
    dismissed_at TIMESTAMP WITH TIME ZONE,
    
    -- Error information
    error_message TEXT,
    error_code VARCHAR(50),
    
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Pre-defined dhikr content for common times
CREATE TABLE default_dhikr_content (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    category dhikr_category NOT NULL,
    title VARCHAR(255) NOT NULL,
    arabic_text TEXT NOT NULL,
    transliteration TEXT,
    translation_en TEXT,
    translation_ar TEXT,
    reference TEXT,
    repetitions INTEGER DEFAULT 1,
    order_index INTEGER DEFAULT 0,
    is_active BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for performance
CREATE INDEX idx_notifications_user_id ON notifications(user_id);
CREATE INDEX idx_notifications_scheduled_at ON notifications(scheduled_at);
CREATE INDEX idx_notifications_status ON notifications(status);
CREATE INDEX idx_notifications_type ON notifications(notification_type);

CREATE INDEX idx_prayer_notifications_user_id ON prayer_notifications(user_id);
CREATE INDEX idx_prayer_notifications_prayer_time ON prayer_notifications(prayer_time);
CREATE INDEX idx_prayer_notifications_prayer_name ON prayer_notifications(prayer_name);

CREATE INDEX idx_sunnah_reminders_user_id ON sunnah_reminders(user_id);
CREATE INDEX idx_sunnah_reminders_time ON sunnah_reminders(reminder_time);
CREATE INDEX idx_sunnah_reminders_active ON sunnah_reminders(is_active);

CREATE INDEX idx_seasonal_reminders_user_id ON seasonal_reminders(user_id);
CREATE INDEX idx_seasonal_reminders_season ON seasonal_reminders(season);
CREATE INDEX idx_seasonal_reminders_active ON seasonal_reminders(is_active);

CREATE INDEX idx_dhikr_reminders_user_id ON dhikr_reminders(user_id);
CREATE INDEX idx_dhikr_reminders_category ON dhikr_reminders(dhikr_category);
CREATE INDEX idx_dhikr_reminders_active ON dhikr_reminders(is_active);

CREATE INDEX idx_delivery_log_notification_id ON notification_delivery_log(notification_id);
CREATE INDEX idx_delivery_log_user_id ON notification_delivery_log(user_id);
CREATE INDEX idx_delivery_log_created_at ON notification_delivery_log(created_at);

-- Apply updated_at triggers
CREATE TRIGGER update_notifications_updated_at 
    BEFORE UPDATE ON notifications 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_prayer_notifications_updated_at 
    BEFORE UPDATE ON prayer_notifications 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_sunnah_reminders_updated_at 
    BEFORE UPDATE ON sunnah_reminders 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_seasonal_reminders_updated_at 
    BEFORE UPDATE ON seasonal_reminders 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_dhikr_reminders_updated_at 
    BEFORE UPDATE ON dhikr_reminders 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_user_notification_preferences_updated_at 
    BEFORE UPDATE ON user_notification_preferences 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();