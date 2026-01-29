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

-- Create indexes for better performance
CREATE INDEX idx_ayahs_surah_number ON ayahs(surah_number);
CREATE INDEX idx_ayahs_text_search ON ayahs USING gin(to_tsvector('arabic', text));
CREATE INDEX idx_tafsir_surah_ayah ON tafsir(surah_number, ayah_number);
CREATE INDEX idx_tafsir_source_id ON tafsir(source_id);

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