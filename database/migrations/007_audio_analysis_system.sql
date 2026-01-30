-- Audio Analysis System Migration
-- This migration creates tables for the audio processing and Quran recitation correction system

-- Reciters table for storing information about Quranic reciters
CREATE TABLE IF NOT EXISTS reciters (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    arabic_name VARCHAR(255) NOT NULL,
    biography TEXT,
    recitation_style VARCHAR(50) NOT NULL DEFAULT 'Hafs',
    is_reference BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Audio recordings table for storing user and reference recordings
CREATE TABLE IF NOT EXISTS audio_recordings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    reciter_id UUID REFERENCES reciters(id) ON DELETE SET NULL,
    surah_number SMALLINT NOT NULL CHECK (surah_number >= 1 AND surah_number <= 114),
    ayah_start SMALLINT NOT NULL CHECK (ayah_start >= 1),
    ayah_end SMALLINT NOT NULL CHECK (ayah_end >= ayah_start),
    format VARCHAR(10) NOT NULL DEFAULT 'wav',
    sample_rate INTEGER NOT NULL DEFAULT 44100,
    duration_seconds DECIMAL(10,3) NOT NULL,
    file_path TEXT NOT NULL,
    file_size_bytes BIGINT NOT NULL,
    file_hash VARCHAR(64), -- SHA-256 hash for integrity verification
    is_reference BOOLEAN NOT NULL DEFAULT false,
    quality_score DECIMAL(3,2) DEFAULT 0.0 CHECK (quality_score >= 0.0 AND quality_score <= 1.0),
    verified BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Reference recordings table for linking reciters to specific ayah recordings
CREATE TABLE IF NOT EXISTS reference_recordings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    reciter_id UUID NOT NULL REFERENCES reciters(id) ON DELETE CASCADE,
    audio_recording_id UUID NOT NULL REFERENCES audio_recordings(id) ON DELETE CASCADE,
    surah_number SMALLINT NOT NULL CHECK (surah_number >= 1 AND surah_number <= 114),
    ayah_number SMALLINT NOT NULL CHECK (ayah_number >= 1),
    quality_score DECIMAL(3,2) NOT NULL DEFAULT 0.8 CHECK (quality_score >= 0.0 AND quality_score <= 1.0),
    verified BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(reciter_id, surah_number, ayah_number)
);

-- Recitation analyses table for storing analysis results
CREATE TABLE IF NOT EXISTS recitation_analyses (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_recording_id UUID NOT NULL REFERENCES audio_recordings(id) ON DELETE CASCADE,
    reference_recording_id UUID REFERENCES reference_recordings(id) ON DELETE SET NULL,
    overall_score DECIMAL(3,2) NOT NULL CHECK (overall_score >= 0.0 AND overall_score <= 1.0),
    tajweed_accuracy DECIMAL(3,2) NOT NULL CHECK (tajweed_accuracy >= 0.0 AND tajweed_accuracy <= 1.0),
    pronunciation_accuracy DECIMAL(3,2) NOT NULL CHECK (pronunciation_accuracy >= 0.0 AND pronunciation_accuracy <= 1.0),
    timing_accuracy DECIMAL(3,2) NOT NULL CHECK (timing_accuracy >= 0.0 AND timing_accuracy <= 1.0),
    fluency_score DECIMAL(3,2) DEFAULT 0.0 CHECK (fluency_score >= 0.0 AND fluency_score <= 1.0),
    clarity_score DECIMAL(3,2) DEFAULT 0.0 CHECK (clarity_score >= 0.0 AND clarity_score <= 1.0),
    rhythm_score DECIMAL(3,2) DEFAULT 0.0 CHECK (rhythm_score >= 0.0 AND rhythm_score <= 1.0),
    analysis_duration_ms INTEGER NOT NULL,
    analyzed_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Tajweed errors table for storing detected errors in recitation
CREATE TABLE IF NOT EXISTS tajweed_errors (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    analysis_id UUID NOT NULL REFERENCES recitation_analyses(id) ON DELETE CASCADE,
    error_type VARCHAR(50) NOT NULL, -- 'Ghunnah', 'Qalqalah', 'Madd', 'Idgham', 'Ikhfa', 'Pronunciation', 'Timing'
    start_time DECIMAL(10,3) NOT NULL, -- Start time in seconds
    end_time DECIMAL(10,3) NOT NULL,   -- End time in seconds
    severity VARCHAR(20) NOT NULL DEFAULT 'Moderate', -- 'Minor', 'Moderate', 'Major'
    description TEXT NOT NULL,
    correction_suggestion TEXT NOT NULL,
    reference_audio_path TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Audio spectra table for storing spectral analysis data
CREATE TABLE IF NOT EXISTS audio_spectra (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    recording_id UUID NOT NULL REFERENCES audio_recordings(id) ON DELETE CASCADE,
    sample_rate INTEGER NOT NULL,
    window_size INTEGER NOT NULL,
    frequencies DECIMAL[] NOT NULL, -- Array of frequency bins
    magnitudes DECIMAL[] NOT NULL,  -- Array of magnitude values
    spectral_centroid DECIMAL(10,3),
    spectral_rolloff DECIMAL(10,3),
    zero_crossing_rate DECIMAL(6,4),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Audio comparisons table for storing comparison results
CREATE TABLE IF NOT EXISTS audio_comparisons (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_recording_id UUID NOT NULL REFERENCES audio_recordings(id) ON DELETE CASCADE,
    reference_recording_id UUID NOT NULL REFERENCES audio_recordings(id) ON DELETE CASCADE,
    similarity_score DECIMAL(3,2) NOT NULL CHECK (similarity_score >= 0.0 AND similarity_score <= 1.0),
    frequency_correlation DECIMAL(3,2) NOT NULL CHECK (frequency_correlation >= 0.0 AND frequency_correlation <= 1.0),
    timing_correlation DECIMAL(3,2) NOT NULL CHECK (timing_correlation >= 0.0 AND timing_correlation <= 1.0),
    spectral_distance DECIMAL(10,6) NOT NULL,
    comparison_type VARCHAR(50) NOT NULL DEFAULT 'comprehensive',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Recommendations table for storing improvement suggestions
CREATE TABLE IF NOT EXISTS recommendations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    analysis_id UUID NOT NULL REFERENCES recitation_analyses(id) ON DELETE CASCADE,
    category VARCHAR(50) NOT NULL, -- 'Pronunciation', 'Timing', 'Tajweed', 'Fluency', 'General'
    priority VARCHAR(20) NOT NULL DEFAULT 'Medium', -- 'High', 'Medium', 'Low'
    description TEXT NOT NULL,
    specific_advice TEXT NOT NULL,
    practice_exercises TEXT[],
    reference_materials TEXT[],
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- User progress tracking table
CREATE TABLE IF NOT EXISTS user_progress (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    surah_number SMALLINT NOT NULL CHECK (surah_number >= 1 AND surah_number <= 114),
    ayah_number SMALLINT NOT NULL CHECK (ayah_number >= 1),
    best_score DECIMAL(3,2) NOT NULL DEFAULT 0.0 CHECK (best_score >= 0.0 AND best_score <= 1.0),
    attempts_count INTEGER NOT NULL DEFAULT 0,
    last_attempt_at TIMESTAMP WITH TIME ZONE,
    mastery_level VARCHAR(20) NOT NULL DEFAULT 'Beginner', -- 'Beginner', 'Intermediate', 'Advanced', 'Master'
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(user_id, surah_number, ayah_number)
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_audio_recordings_user_id ON audio_recordings(user_id);
CREATE INDEX IF NOT EXISTS idx_audio_recordings_surah_ayah ON audio_recordings(surah_number, ayah_start, ayah_end);
CREATE INDEX IF NOT EXISTS idx_audio_recordings_is_reference ON audio_recordings(is_reference);
CREATE INDEX IF NOT EXISTS idx_reference_recordings_surah_ayah ON reference_recordings(surah_number, ayah_number);
CREATE INDEX IF NOT EXISTS idx_reference_recordings_reciter ON reference_recordings(reciter_id);
CREATE INDEX IF NOT EXISTS idx_recitation_analyses_user_recording ON recitation_analyses(user_recording_id);
CREATE INDEX IF NOT EXISTS idx_tajweed_errors_analysis ON tajweed_errors(analysis_id);
CREATE INDEX IF NOT EXISTS idx_tajweed_errors_type ON tajweed_errors(error_type);
CREATE INDEX IF NOT EXISTS idx_audio_spectra_recording ON audio_spectra(recording_id);
CREATE INDEX IF NOT EXISTS idx_audio_comparisons_user_recording ON audio_comparisons(user_recording_id);
CREATE INDEX IF NOT EXISTS idx_recommendations_analysis ON recommendations(analysis_id);
CREATE INDEX IF NOT EXISTS idx_user_progress_user_id ON user_progress(user_id);
CREATE INDEX IF NOT EXISTS idx_user_progress_surah ON user_progress(surah_number);

-- Insert default reciters
INSERT INTO reciters (name, arabic_name, recitation_style, is_reference) VALUES
('Abdul Rahman Al-Sudais', 'عبد الرحمن السديس', 'Hafs', true),
('Saad Al-Ghamdi', 'سعد الغامدي', 'Hafs', true),
('Mishary Rashid Alafasy', 'مشاري راشد العفاسي', 'Hafs', true),
('Maher Al Mueaqly', 'ماهر المعيقلي', 'Hafs', true),
('Ahmed ibn Ali al-Ajamy', 'أحمد بن علي العجمي', 'Hafs', true),
('Yasser Al-Dosari', 'ياسر الدوسري', 'Hafs', true),
('Nasser Al Qatami', 'ناصر القطامي', 'Hafs', true),
('Warsh Recitation', 'رواية ورش', 'Warsh', true)
ON CONFLICT DO NOTHING;

-- Create trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply the trigger to relevant tables
CREATE TRIGGER update_reciters_updated_at BEFORE UPDATE ON reciters FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_audio_recordings_updated_at BEFORE UPDATE ON audio_recordings FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_user_progress_updated_at BEFORE UPDATE ON user_progress FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Add comments for documentation
COMMENT ON TABLE reciters IS 'Stores information about Quranic reciters for reference recordings';
COMMENT ON TABLE audio_recordings IS 'Stores all audio recordings including user recordings and reference recordings';
COMMENT ON TABLE reference_recordings IS 'Links reciters to specific ayah recordings for comparison purposes';
COMMENT ON TABLE recitation_analyses IS 'Stores detailed analysis results of user recitations';
COMMENT ON TABLE tajweed_errors IS 'Stores specific Tajweed errors detected in recitations';
COMMENT ON TABLE audio_spectra IS 'Stores spectral analysis data for audio recordings';
COMMENT ON TABLE audio_comparisons IS 'Stores comparison results between user and reference recordings';
COMMENT ON TABLE recommendations IS 'Stores personalized recommendations for improving recitation';
COMMENT ON TABLE user_progress IS 'Tracks user progress and mastery levels for different ayahs';

COMMENT ON COLUMN audio_recordings.file_hash IS 'SHA-256 hash for verifying file integrity';
COMMENT ON COLUMN audio_recordings.quality_score IS 'Quality score from 0.0 to 1.0 based on audio analysis';
COMMENT ON COLUMN recitation_analyses.analysis_duration_ms IS 'Time taken to complete the analysis in milliseconds';
COMMENT ON COLUMN tajweed_errors.start_time IS 'Start time of the error in seconds from beginning of recording';
COMMENT ON COLUMN tajweed_errors.end_time IS 'End time of the error in seconds from beginning of recording';