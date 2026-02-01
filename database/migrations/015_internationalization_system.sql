-- Migration: Internationalization System
-- Description: Create tables for managing multilingual support, user language preferences, and translation metadata

-- User language preferences table
CREATE TABLE user_language_preferences (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    primary_language VARCHAR(10) NOT NULL DEFAULT 'ar',
    fallback_languages JSONB NOT NULL DEFAULT '["en"]',
    quran_translation_languages JSONB NOT NULL DEFAULT '["en"]',
    interface_language VARCHAR(10) NOT NULL DEFAULT 'ar',
    content_language_preferences JSONB NOT NULL DEFAULT '{}',
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Translation quality metrics table
CREATE TABLE translation_quality (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    language VARCHAR(10) NOT NULL,
    namespace VARCHAR(100) NOT NULL,
    completion_percentage REAL NOT NULL DEFAULT 0.0,
    accuracy_score REAL NOT NULL DEFAULT 0.0,
    consistency_score REAL NOT NULL DEFAULT 0.0,
    last_reviewed TIMESTAMP WITH TIME ZONE,
    reviewer_notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(language, namespace)
);

-- Available translations for content table
CREATE TABLE available_translations (
    content_id UUID PRIMARY KEY,
    content_type VARCHAR(50) NOT NULL,
    available_languages JSONB NOT NULL DEFAULT '[]',
    default_language VARCHAR(10) NOT NULL DEFAULT 'ar',
    quality_scores JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Language pack metadata table
CREATE TABLE language_pack_metadata (
    language VARCHAR(10) PRIMARY KEY,
    version VARCHAR(20) NOT NULL DEFAULT '1.0.0',
    contributors JSONB NOT NULL DEFAULT '[]',
    completion_percentage REAL NOT NULL DEFAULT 0.0,
    last_updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    quality_score REAL NOT NULL DEFAULT 0.0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Translation usage statistics table
CREATE TABLE translation_usage_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    translation_key VARCHAR(255) NOT NULL,
    namespace VARCHAR(100) NOT NULL,
    language VARCHAR(10) NOT NULL,
    usage_count BIGINT NOT NULL DEFAULT 0,
    last_used TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(translation_key, namespace, language)
);

-- Language detection logs table (for analytics)
CREATE TABLE language_detection_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    detected_language VARCHAR(10) NOT NULL,
    confidence_score REAL NOT NULL,
    detection_method VARCHAR(50) NOT NULL, -- 'text', 'headers', 'manual'
    input_text TEXT,
    user_agent TEXT,
    ip_address INET,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Content translation mappings table
CREATE TABLE content_translations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content_id UUID NOT NULL,
    content_type VARCHAR(50) NOT NULL,
    language VARCHAR(10) NOT NULL,
    translated_title TEXT,
    translated_content TEXT,
    translator_id UUID REFERENCES users(id) ON DELETE SET NULL,
    translation_status VARCHAR(20) NOT NULL DEFAULT 'draft', -- 'draft', 'review', 'approved', 'published'
    quality_score REAL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    UNIQUE(content_id, language)
);

-- Indexes for performance
CREATE INDEX idx_user_language_preferences_primary_language ON user_language_preferences(primary_language);
CREATE INDEX idx_user_language_preferences_interface_language ON user_language_preferences(interface_language);
CREATE INDEX idx_translation_quality_language ON translation_quality(language);
CREATE INDEX idx_translation_quality_namespace ON translation_quality(namespace);
CREATE INDEX idx_available_translations_content_type ON available_translations(content_type);
CREATE INDEX idx_available_translations_default_language ON available_translations(default_language);
CREATE INDEX idx_translation_usage_stats_language ON translation_usage_stats(language);
CREATE INDEX idx_translation_usage_stats_namespace ON translation_usage_stats(namespace);
CREATE INDEX idx_translation_usage_stats_last_used ON translation_usage_stats(last_used);
CREATE INDEX idx_language_detection_logs_detected_language ON language_detection_logs(detected_language);
CREATE INDEX idx_language_detection_logs_user_id ON language_detection_logs(user_id);
CREATE INDEX idx_language_detection_logs_created_at ON language_detection_logs(created_at);
CREATE INDEX idx_content_translations_content_id ON content_translations(content_id);
CREATE INDEX idx_content_translations_language ON content_translations(language);
CREATE INDEX idx_content_translations_status ON content_translations(translation_status);

-- Functions for automatic timestamp updates
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers for automatic timestamp updates
CREATE TRIGGER update_user_language_preferences_updated_at 
    BEFORE UPDATE ON user_language_preferences 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_translation_quality_updated_at 
    BEFORE UPDATE ON translation_quality 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_available_translations_updated_at 
    BEFORE UPDATE ON available_translations 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_content_translations_updated_at 
    BEFORE UPDATE ON content_translations 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert default language pack metadata for supported languages
INSERT INTO language_pack_metadata (language, version, contributors, completion_percentage, quality_score) VALUES
('ar', '1.0.0', '["System", "Native Speakers"]', 100.0, 1.0),
('en', '1.0.0', '["System", "Translators"]', 95.0, 0.95),
('fr', '1.0.0', '["System", "Translators"]', 80.0, 0.8),
('es', '1.0.0', '["System", "Translators"]', 80.0, 0.8),
('tr', '1.0.0', '["System", "Translators"]', 75.0, 0.75),
('ur', '1.0.0', '["System", "Native Speakers"]', 90.0, 0.9),
('id', '1.0.0', '["System", "Translators"]', 70.0, 0.7),
('ms', '1.0.0', '["System", "Translators"]', 70.0, 0.7),
('bn', '1.0.0', '["System", "Translators"]', 65.0, 0.65),
('fa', '1.0.0', '["System", "Native Speakers"]', 85.0, 0.85);

-- Insert default translation quality metrics for common namespaces
INSERT INTO translation_quality (language, namespace, completion_percentage, accuracy_score, consistency_score) VALUES
-- Arabic (complete reference)
('ar', 'common', 100.0, 1.0, 1.0),
('ar', 'prayers', 100.0, 1.0, 1.0),
('ar', 'quran', 100.0, 1.0, 1.0),
('ar', 'hadith', 100.0, 1.0, 1.0),
('ar', 'calendar', 100.0, 1.0, 1.0),
('ar', 'navigation', 100.0, 1.0, 1.0),

-- English (high quality translations)
('en', 'common', 95.0, 0.95, 0.9),
('en', 'prayers', 98.0, 0.98, 0.95),
('en', 'quran', 100.0, 1.0, 1.0),
('en', 'hadith', 95.0, 0.95, 0.9),
('en', 'calendar', 90.0, 0.9, 0.85),
('en', 'navigation', 95.0, 0.95, 0.9),

-- Urdu (good quality for Islamic content)
('ur', 'common', 90.0, 0.9, 0.85),
('ur', 'prayers', 95.0, 0.95, 0.9),
('ur', 'quran', 98.0, 0.98, 0.95),
('ur', 'hadith', 92.0, 0.92, 0.88),
('ur', 'calendar', 85.0, 0.85, 0.8),
('ur', 'navigation', 88.0, 0.88, 0.83);

-- Comments for documentation
COMMENT ON TABLE user_language_preferences IS 'Stores user language preferences for interface and content';
COMMENT ON TABLE translation_quality IS 'Tracks quality metrics for translations in different languages and namespaces';
COMMENT ON TABLE available_translations IS 'Maps content to available translation languages';
COMMENT ON TABLE language_pack_metadata IS 'Metadata about language packs including version and quality information';
COMMENT ON TABLE translation_usage_stats IS 'Statistics about translation key usage for analytics';
COMMENT ON TABLE language_detection_logs IS 'Logs of automatic language detection for analytics and improvement';
COMMENT ON TABLE content_translations IS 'Stores translated content for different languages';

COMMENT ON COLUMN user_language_preferences.primary_language IS 'Primary language for content display';
COMMENT ON COLUMN user_language_preferences.fallback_languages IS 'Array of fallback languages in order of preference';
COMMENT ON COLUMN user_language_preferences.quran_translation_languages IS 'Preferred languages for Quran translations';
COMMENT ON COLUMN user_language_preferences.interface_language IS 'Language for UI elements';
COMMENT ON COLUMN user_language_preferences.content_language_preferences IS 'Per-content-type language preferences';

COMMENT ON COLUMN translation_quality.completion_percentage IS 'Percentage of keys translated in this namespace';
COMMENT ON COLUMN translation_quality.accuracy_score IS 'Accuracy score from 0.0 to 1.0';
COMMENT ON COLUMN translation_quality.consistency_score IS 'Consistency score from 0.0 to 1.0';

COMMENT ON COLUMN available_translations.quality_scores IS 'Quality scores for each available language';
COMMENT ON COLUMN content_translations.translation_status IS 'Status: draft, review, approved, published';
COMMENT ON COLUMN content_translations.quality_score IS 'Quality score for this specific translation';