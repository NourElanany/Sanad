-- Sanad Islamic Application Database Schema
-- This script creates the initial database structure

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_active_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    is_active BOOLEAN DEFAULT TRUE
);

-- User preferences table
CREATE TABLE user_preferences (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    language VARCHAR(10) DEFAULT 'ar',
    preferred_tafsir TEXT[] DEFAULT '{}',
    prayer_calculation_method VARCHAR(50) DEFAULT 'MuslimWorldLeague',
    notification_settings JSONB DEFAULT '{}',
    display_settings JSONB DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Quran surahs table
CREATE TABLE surahs (
    number INTEGER PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    arabic_name VARCHAR(100) NOT NULL,
    english_name VARCHAR(100) NOT NULL,
    revelation_type VARCHAR(20) NOT NULL CHECK (revelation_type IN ('meccan', 'medinan')),
    number_of_ayahs INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Quran ayahs table
CREATE TABLE ayahs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    surah_number INTEGER NOT NULL REFERENCES surahs(number),
    ayah_number INTEGER NOT NULL,
    text TEXT NOT NULL,
    text_hash VARCHAR(64) NOT NULL, -- SHA-256 hash for integrity verification
    juz INTEGER NOT NULL,
    page INTEGER NOT NULL,
    ruku INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(surah_number, ayah_number)
);

-- Tafsir sources table
CREATE TABLE tafsir_sources (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    author VARCHAR(100) NOT NULL,
    language VARCHAR(10) DEFAULT 'ar',
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Tafsir table
CREATE TABLE tafsir (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    surah_number INTEGER NOT NULL,
    ayah_number INTEGER NOT NULL,
    source_id UUID NOT NULL REFERENCES tafsir_sources(id),
    text TEXT NOT NULL,
    text_hash VARCHAR(64) NOT NULL, -- SHA-256 hash for integrity verification
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    FOREIGN KEY (surah_number, ayah_number) REFERENCES ayahs(surah_number, ayah_number)
);

-- Hadith books table
CREATE TABLE hadith_books (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    arabic_name VARCHAR(100) NOT NULL,
    author VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Hadith table
CREATE TABLE hadiths (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    book_id UUID NOT NULL REFERENCES hadith_books(id),
    hadith_number VARCHAR(20) NOT NULL,
    chapter VARCHAR(200),
    text TEXT NOT NULL,
    text_hash VARCHAR(64) NOT NULL, -- SHA-256 hash for integrity verification
    narrator VARCHAR(200),
    chain TEXT[], -- Array of narrators in the chain
    grade VARCHAR(20) NOT NULL CHECK (grade IN ('Sahih', 'Hasan', 'Daif', 'Mawdu')),
    source VARCHAR(100) NOT NULL,
    tags TEXT[] DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Hadith explanations table
CREATE TABLE hadith_explanations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    hadith_id UUID NOT NULL REFERENCES hadiths(id),
    scholar VARCHAR(100) NOT NULL,
    explanation TEXT NOT NULL,
    language VARCHAR(10) DEFAULT 'ar',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Islamic stories table
CREATE TABLE stories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(200) NOT NULL,
    category VARCHAR(50) NOT NULL,
    content TEXT NOT NULL,
    content_hash VARCHAR(64) NOT NULL, -- SHA-256 hash for integrity verification
    characters TEXT[] DEFAULT '{}',
    lessons TEXT[] DEFAULT '{}',
    sources TEXT[] DEFAULT '{}',
    language VARCHAR(10) DEFAULT 'ar',
    tags TEXT[] DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Islamic events table
CREATE TABLE islamic_events (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(200) NOT NULL,
    description TEXT,
    hijri_month INTEGER NOT NULL CHECK (hijri_month BETWEEN 1 AND 12),
    hijri_day INTEGER NOT NULL CHECK (hijri_day BETWEEN 1 AND 30),
    event_type VARCHAR(50) NOT NULL,
    is_recurring BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- User bookmarks table
CREATE TABLE bookmarks (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content_type VARCHAR(20) NOT NULL CHECK (content_type IN ('quran', 'hadith', 'tafsir', 'story')),
    content_id UUID NOT NULL,
    title VARCHAR(200) NOT NULL,
    notes TEXT,
    folder VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Reading progress table
CREATE TABLE reading_progress (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    content_type VARCHAR(20) NOT NULL,
    content_id UUID NOT NULL,
    progress_percentage DECIMAL(5,2) DEFAULT 0.00,
    last_position JSONB, -- Store position info (surah, ayah, etc.)
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, content_type, content_id)
);

-- Khatma plans table
CREATE TABLE khatma_plans (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    target_date DATE NOT NULL,
    daily_portions JSONB NOT NULL, -- Array of daily reading portions
    estimated_reading_time INTEGER, -- in minutes
    adaptive_schedule BOOLEAN DEFAULT TRUE,
    current_progress DECIMAL(5,2) DEFAULT 0.00,
    status VARCHAR(20) DEFAULT 'active' CHECK (status IN ('active', 'completed', 'paused')),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Reading sessions table (for tracking actual reading)
CREATE TABLE reading_sessions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    khatma_plan_id UUID REFERENCES khatma_plans(id) ON DELETE SET NULL,
    surah_start INTEGER NOT NULL,
    ayah_start INTEGER NOT NULL,
    surah_end INTEGER NOT NULL,
    ayah_end INTEGER NOT NULL,
    reading_time_minutes INTEGER,
    session_date DATE DEFAULT CURRENT_DATE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Audio recordings table (for recitation analysis)
CREATE TABLE audio_recordings (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    surah_number INTEGER NOT NULL,
    ayah_start INTEGER NOT NULL,
    ayah_end INTEGER NOT NULL,
    file_path VARCHAR(500) NOT NULL,
    file_size INTEGER,
    duration_seconds INTEGER,
    analysis_results JSONB, -- Store analysis results
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- User notifications table
CREATE TABLE user_notifications (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL,
    title VARCHAR(200) NOT NULL,
    message TEXT NOT NULL,
    is_read BOOLEAN DEFAULT FALSE,
    scheduled_for TIMESTAMP WITH TIME ZONE,
    sent_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_ayahs_surah_number ON ayahs(surah_number);
CREATE INDEX idx_ayahs_text_search ON ayahs USING gin(to_tsvector('arabic', text));
CREATE INDEX idx_hadiths_text_search ON hadiths USING gin(to_tsvector('arabic', text));
CREATE INDEX idx_hadiths_grade ON hadiths(grade);
CREATE INDEX idx_hadiths_book_id ON hadiths(book_id);
CREATE INDEX idx_stories_category ON stories(category);
CREATE INDEX idx_stories_text_search ON stories USING gin(to_tsvector('arabic', content));
CREATE INDEX idx_bookmarks_user_id ON bookmarks(user_id);
CREATE INDEX idx_reading_progress_user_id ON reading_progress(user_id);
CREATE INDEX idx_khatma_plans_user_id ON khatma_plans(user_id);
CREATE INDEX idx_reading_sessions_user_id ON reading_sessions(user_id);
CREATE INDEX idx_reading_sessions_date ON reading_sessions(session_date);
CREATE INDEX idx_audio_recordings_user_id ON audio_recordings(user_id);
CREATE INDEX idx_user_notifications_user_id ON user_notifications(user_id);
CREATE INDEX idx_user_notifications_scheduled ON user_notifications(scheduled_for) WHERE scheduled_for IS NOT NULL;

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply updated_at triggers to relevant tables
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_user_preferences_updated_at BEFORE UPDATE ON user_preferences FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_stories_updated_at BEFORE UPDATE ON stories FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_reading_progress_updated_at BEFORE UPDATE ON reading_progress FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_khatma_plans_updated_at BEFORE UPDATE ON khatma_plans FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();