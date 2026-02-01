-- Enhanced Translation System Migration
-- Description: Enhance the existing translations table with quality management and source tracking

-- Add new columns to existing translations table
ALTER TABLE translations 
ADD COLUMN IF NOT EXISTS text_hash VARCHAR(64),
ADD COLUMN IF NOT EXISTS quality_score REAL DEFAULT 0.0,
ADD COLUMN IF NOT EXISTS approval_status VARCHAR(20) DEFAULT 'pending',
ADD COLUMN IF NOT EXISTS source_reference TEXT,
ADD COLUMN IF NOT EXISTS methodology TEXT,
ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW();

-- Create translation sources table
CREATE TABLE IF NOT EXISTS translation_sources (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL,
    translator VARCHAR(100) NOT NULL,
    language VARCHAR(10) NOT NULL,
    description TEXT,
    methodology TEXT,
    source_reference TEXT,
    quality_score REAL DEFAULT 0.0,
    approval_status VARCHAR(20) DEFAULT 'pending',
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(translator, language)
);

-- Create translation quality metrics table
CREATE TABLE IF NOT EXISTS translation_quality_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    translation_id UUID REFERENCES translations(id) ON DELETE CASCADE,
    source_id UUID REFERENCES translation_sources(id) ON DELETE CASCADE,
    accuracy_score REAL DEFAULT 0.0,
    fluency_score REAL DEFAULT 0.0,
    consistency_score REAL DEFAULT 0.0,
    completeness_score REAL DEFAULT 0.0,
    overall_score REAL DEFAULT 0.0,
    reviewer_id UUID, -- References users table when available
    review_notes TEXT,
    reviewed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create translation usage analytics table
CREATE TABLE IF NOT EXISTS translation_usage_analytics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    translation_id UUID REFERENCES translations(id) ON DELETE CASCADE,
    user_id UUID, -- References users table when available
    access_count INTEGER DEFAULT 1,
    last_accessed TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    user_rating REAL, -- User rating 1-5
    user_feedback TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(translation_id, user_id)
);

-- Create indexes for better performance
CREATE INDEX IF NOT EXISTS idx_translations_text_hash ON translations(text_hash);
CREATE INDEX IF NOT EXISTS idx_translations_quality_score ON translations(quality_score);
CREATE INDEX IF NOT EXISTS idx_translations_approval_status ON translations(approval_status);
CREATE INDEX IF NOT EXISTS idx_translations_updated_at ON translations(updated_at);

CREATE INDEX IF NOT EXISTS idx_translation_sources_language ON translation_sources(language);
CREATE INDEX IF NOT EXISTS idx_translation_sources_translator ON translation_sources(translator);
CREATE INDEX IF NOT EXISTS idx_translation_sources_quality_score ON translation_sources(quality_score);
CREATE INDEX IF NOT EXISTS idx_translation_sources_approval_status ON translation_sources(approval_status);
CREATE INDEX IF NOT EXISTS idx_translation_sources_is_active ON translation_sources(is_active);

CREATE INDEX IF NOT EXISTS idx_translation_quality_metrics_translation_id ON translation_quality_metrics(translation_id);
CREATE INDEX IF NOT EXISTS idx_translation_quality_metrics_source_id ON translation_quality_metrics(source_id);
CREATE INDEX IF NOT EXISTS idx_translation_quality_metrics_overall_score ON translation_quality_metrics(overall_score);

CREATE INDEX IF NOT EXISTS idx_translation_usage_analytics_translation_id ON translation_usage_analytics(translation_id);
CREATE INDEX IF NOT EXISTS idx_translation_usage_analytics_user_id ON translation_usage_analytics(user_id);
CREATE INDEX IF NOT EXISTS idx_translation_usage_analytics_last_accessed ON translation_usage_analytics(last_accessed);

-- Create triggers for automatic timestamp updates
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_translations_updated_at 
    BEFORE UPDATE ON translations 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_translation_sources_updated_at 
    BEFORE UPDATE ON translation_sources 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_translation_quality_metrics_updated_at 
    BEFORE UPDATE ON translation_quality_metrics 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_translation_usage_analytics_updated_at 
    BEFORE UPDATE ON translation_usage_analytics 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Update existing translations with text hashes
UPDATE translations 
SET text_hash = encode(sha256(text::bytea), 'hex')
WHERE text_hash IS NULL;

-- Insert default translation sources for existing translations
INSERT INTO translation_sources (name, translator, language, description, quality_score, approval_status)
SELECT DISTINCT 
    translator as name,
    translator,
    language,
    'Legacy translation source' as description,
    CASE 
        WHEN translator ILIKE '%sahih%' THEN 9.0
        WHEN translator ILIKE '%pickthall%' THEN 8.5
        WHEN translator ILIKE '%yusuf%' THEN 8.5
        WHEN translator ILIKE '%shakir%' THEN 8.0
        WHEN translator ILIKE '%arberry%' THEN 8.0
        ELSE 7.0
    END as quality_score,
    'approved' as approval_status
FROM translations
WHERE NOT EXISTS (
    SELECT 1 FROM translation_sources ts 
    WHERE ts.translator = translations.translator 
    AND ts.language = translations.language
);

-- Update existing translations with quality scores based on translator reputation
UPDATE translations 
SET quality_score = CASE 
    WHEN translator ILIKE '%sahih%' THEN 9.0
    WHEN translator ILIKE '%pickthall%' THEN 8.5
    WHEN translator ILIKE '%yusuf%' THEN 8.5
    WHEN translator ILIKE '%shakir%' THEN 8.0
    WHEN translator ILIKE '%arberry%' THEN 8.0
    ELSE 7.0
END,
approval_status = 'approved'
WHERE quality_score = 0.0;

-- Insert sample high-quality translation sources
INSERT INTO translation_sources (name, translator, language, description, methodology, source_reference, quality_score, approval_status) VALUES
('Sahih International', 'Sahih International', 'en', 'Modern English translation with contemporary language', 'Contemporary scholarly approach with emphasis on clarity', 'https://sahihinternational.com', 9.0, 'verified'),
('The Noble Quran', 'Muhammad Taqi-ud-Din al-Hilali and Muhammad Muhsin Khan', 'en', 'Translation with extensive footnotes and commentary', 'Traditional approach with detailed explanations', 'Darussalam Publishers', 8.8, 'verified'),
('The Clear Quran', 'Dr. Mustafa Khattab', 'en', 'Easy-to-understand modern English translation', 'Contemporary approach focusing on clarity and accessibility', 'Book of Signs Foundation', 8.7, 'verified'),
('Pickthall Translation', 'Mohammed Marmaduke Pickthall', 'en', 'Classic English translation by a British Muslim convert', 'Early 20th century scholarly approach', 'Hyderabad: Government Central Press', 8.5, 'approved'),
('Yusuf Ali Translation', 'Abdullah Yusuf Ali', 'en', 'Widely used translation with extensive commentary', 'Classical approach with poetic language', 'Lahore: Sh. Muhammad Ashraf', 8.5, 'approved'),
('Arberry Translation', 'Arthur John Arberry', 'en', 'Academic translation focusing on literary style', 'Academic approach emphasizing Arabic literary beauty', 'Oxford University Press', 8.0, 'approved'),
('Shakir Translation', 'M. H. Shakir', 'en', 'Simple and direct English translation', 'Straightforward approach with clear language', 'Tahrike Tarsile Quran', 8.0, 'approved');

-- Insert sample French translations
INSERT INTO translation_sources (name, translator, language, description, methodology, quality_score, approval_status) VALUES
('Traduction Hamidullah', 'Muhammad Hamidullah', 'fr', 'Référence française de la traduction du Coran', 'Approche académique rigoureuse', 8.5, 'verified'),
('Traduction Boubakeur', 'Si Hamza Boubakeur', 'fr', 'Traduction avec commentaires explicatifs', 'Approche traditionnelle avec explications', 8.0, 'approved');

-- Insert sample Spanish translations
INSERT INTO translation_sources (name, translator, language, description, methodology, quality_score, approval_status) VALUES
('Traducción Cortés', 'Julio Cortés', 'es', 'Traducción académica española del Corán', 'Enfoque académico riguroso', 8.2, 'verified'),
('Traducción Isa García', 'Isa García', 'es', 'Traducción contemporánea en español', 'Enfoque moderno y accesible', 7.8, 'approved');

-- Insert sample Urdu translations
INSERT INTO translation_sources (name, translator, language, description, methodology, quality_score, approval_status) VALUES
('Kanz-ul-Iman', 'Ahmed Raza Khan Barelvi', 'ur', 'Classic Urdu translation', 'Traditional Sunni approach', 8.3, 'approved'),
('Tafheem-ul-Quran', 'Abul Ala Maududi', 'ur', 'Translation with detailed commentary', 'Modern interpretive approach', 8.5, 'verified');

-- Comments for documentation
COMMENT ON TABLE translation_sources IS 'Manages translation sources with quality metrics and approval status';
COMMENT ON TABLE translation_quality_metrics IS 'Detailed quality assessment metrics for translations';
COMMENT ON TABLE translation_usage_analytics IS 'Analytics data for translation usage and user feedback';

COMMENT ON COLUMN translations.text_hash IS 'SHA-256 hash of the translation text for integrity verification';
COMMENT ON COLUMN translations.quality_score IS 'Overall quality score from 0.0 to 10.0';
COMMENT ON COLUMN translations.approval_status IS 'Approval status: pending, approved, verified, rejected';
COMMENT ON COLUMN translations.source_reference IS 'Reference to the original source or publication';
COMMENT ON COLUMN translations.methodology IS 'Description of the translation methodology used';

-- Create view for approved translations with source information
CREATE OR REPLACE VIEW approved_translations_with_sources AS
SELECT 
    t.id,
    t.surah_number,
    t.ayah_number,
    t.language,
    t.translator,
    t.text,
    t.text_hash,
    t.quality_score,
    t.approval_status,
    t.source_reference,
    t.methodology,
    t.created_at,
    t.updated_at,
    ts.name as source_name,
    ts.description as source_description,
    ts.quality_score as source_quality_score,
    ts.is_active as source_is_active
FROM translations t
LEFT JOIN translation_sources ts ON t.translator = ts.translator AND t.language = ts.language
WHERE t.approval_status IN ('approved', 'verified')
AND (ts.is_active IS NULL OR ts.is_active = true);

-- Create view for translation statistics
CREATE OR REPLACE VIEW translation_statistics AS
SELECT 
    language,
    COUNT(*) as total_translations,
    COUNT(CASE WHEN approval_status IN ('approved', 'verified') THEN 1 END) as approved_translations,
    AVG(quality_score) as average_quality,
    COUNT(DISTINCT translator) as translator_count,
    MIN(created_at) as first_translation_date,
    MAX(updated_at) as last_update_date
FROM translations
GROUP BY language
ORDER BY total_translations DESC;

-- Grant necessary permissions (adjust as needed for your setup)
-- GRANT SELECT, INSERT, UPDATE, DELETE ON translations TO quran_service_user;
-- GRANT SELECT, INSERT, UPDATE, DELETE ON translation_sources TO quran_service_user;
-- GRANT SELECT, INSERT, UPDATE, DELETE ON translation_quality_metrics TO quran_service_user;
-- GRANT SELECT, INSERT, UPDATE, DELETE ON translation_usage_analytics TO quran_service_user;
-- GRANT SELECT ON approved_translations_with_sources TO quran_service_user;
-- GRANT SELECT ON translation_statistics TO quran_service_user;