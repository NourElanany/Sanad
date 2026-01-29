-- Enhanced Tafsir System Migration
-- Adds comprehensive Tafsir management with source credibility and comparison features

-- Add new columns to tafsir_sources table
ALTER TABLE tafsir_sources 
ADD COLUMN IF NOT EXISTS credibility_score DECIMAL(3,1) DEFAULT 5.0 CHECK (credibility_score >= 0.0 AND credibility_score <= 10.0),
ADD COLUMN IF NOT EXISTS scholarly_authentication VARCHAR(50) DEFAULT 'unverified' CHECK (scholarly_authentication IN ('highly_authenticated', 'authenticated', 'verified', 'unverified')),
ADD COLUMN IF NOT EXISTS source_type VARCHAR(50) DEFAULT 'contemporary' CHECK (source_type IN ('classical', 'contemporary', 'linguistic', 'thematic', 'sectarian')),
ADD COLUMN IF NOT EXISTS publication_year INTEGER,
ADD COLUMN IF NOT EXISTS methodology TEXT,
ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
ADD COLUMN IF NOT EXISTS is_active BOOLEAN DEFAULT TRUE;

-- Add new columns to tafsir table
ALTER TABLE tafsir 
ADD COLUMN IF NOT EXISTS word_count INTEGER DEFAULT 0,
ADD COLUMN IF NOT EXISTS themes TEXT[] DEFAULT '{}',
ADD COLUMN IF NOT EXISTS cross_references TEXT[] DEFAULT '{}',
ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW();

-- Update existing records with calculated word counts
UPDATE tafsir SET word_count = array_length(string_to_array(text, ' '), 1) WHERE word_count = 0;

-- Create indexes for better performance on new columns
CREATE INDEX IF NOT EXISTS idx_tafsir_sources_credibility ON tafsir_sources(credibility_score DESC);
CREATE INDEX IF NOT EXISTS idx_tafsir_sources_authentication ON tafsir_sources(scholarly_authentication);
CREATE INDEX IF NOT EXISTS idx_tafsir_sources_type ON tafsir_sources(source_type);
CREATE INDEX IF NOT EXISTS idx_tafsir_sources_active ON tafsir_sources(is_active) WHERE is_active = true;
CREATE INDEX IF NOT EXISTS idx_tafsir_word_count ON tafsir(word_count);
CREATE INDEX IF NOT EXISTS idx_tafsir_themes ON tafsir USING gin(themes);
CREATE INDEX IF NOT EXISTS idx_tafsir_cross_references ON tafsir USING gin(cross_references);
CREATE INDEX IF NOT EXISTS idx_tafsir_updated_at ON tafsir(updated_at);

-- Create trigger for updating updated_at on tafsir_sources
CREATE TRIGGER update_tafsir_sources_updated_at 
    BEFORE UPDATE ON tafsir_sources 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- Create trigger for updating updated_at on tafsir
CREATE TRIGGER update_tafsir_updated_at 
    BEFORE UPDATE ON tafsir 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- Insert enhanced sample Tafsir sources with credibility scores
INSERT INTO tafsir_sources (
    id, name, author, language, description, credibility_score, scholarly_authentication, 
    source_type, publication_year, methodology, created_at, updated_at, is_active
) VALUES 
(
    uuid_generate_v4(),
    'تفسير ابن كثير',
    'ابن كثير',
    'ar',
    'Classical Quranic exegesis by Ibn Kathir, one of the most respected traditional commentaries',
    9.5,
    'highly_authenticated',
    'classical',
    1365,
    'Traditional exegetical methodology combining Quran, Hadith, and scholarly consensus',
    NOW(),
    NOW(),
    true
),
(
    uuid_generate_v4(),
    'تفسير الطبري',
    'الطبري',
    'ar',
    'Comprehensive classical commentary by Al-Tabari, foundational work in Quranic exegesis',
    9.8,
    'highly_authenticated',
    'classical',
    923,
    'Historical-critical approach with extensive use of early Islamic sources',
    NOW(),
    NOW(),
    true
),
(
    uuid_generate_v4(),
    'تفسير القرطبي',
    'القرطبي',
    'ar',
    'Jurisprudential commentary focusing on legal implications of Quranic verses',
    9.2,
    'highly_authenticated',
    'classical',
    1273,
    'Jurisprudential methodology emphasizing legal rulings and practical applications',
    NOW(),
    NOW(),
    true
),
(
    uuid_generate_v4(),
    'في ظلال القرآن',
    'سيد قطب',
    'ar',
    'Modern thematic commentary emphasizing spiritual and social dimensions',
    7.5,
    'authenticated',
    'contemporary',
    1966,
    'Thematic approach focusing on spiritual, social, and political dimensions',
    NOW(),
    NOW(),
    true
),
(
    uuid_generate_v4(),
    'التحرير والتنوير',
    'ابن عاشور',
    'ar',
    'Comprehensive modern commentary combining classical and contemporary approaches',
    8.8,
    'authenticated',
    'contemporary',
    1984,
    'Balanced methodology combining linguistic analysis, historical context, and modern insights',
    NOW(),
    NOW(),
    true
),
(
    uuid_generate_v4(),
    'معاني القرآن',
    'الفراء',
    'ar',
    'Early linguistic commentary focusing on Arabic grammar and word meanings',
    8.5,
    'highly_authenticated',
    'linguistic',
    822,
    'Linguistic methodology emphasizing Arabic grammar, syntax, and etymology',
    NOW(),
    NOW(),
    true
);

-- Insert sample Tafsir entries with themes and cross-references
-- Note: In production, these would be loaded from authoritative sources
DO $$
DECLARE
    ibn_kathir_id UUID;
    tabari_id UUID;
    qurtubi_id UUID;
BEGIN
    -- Get source IDs
    SELECT id INTO ibn_kathir_id FROM tafsir_sources WHERE author = 'ابن كثير' LIMIT 1;
    SELECT id INTO tabari_id FROM tafsir_sources WHERE author = 'الطبري' LIMIT 1;
    SELECT id INTO qurtubi_id FROM tafsir_sources WHERE author = 'القرطبي' LIMIT 1;
    
    -- Insert sample Tafsir for Al-Fatiha (1:1) - Bismillah
    IF ibn_kathir_id IS NOT NULL THEN
        INSERT INTO tafsir (
            id, surah_number, ayah_number, source_id, text, text_hash, word_count, 
            themes, cross_references, created_at, updated_at
        ) VALUES (
            uuid_generate_v4(),
            1, 1, ibn_kathir_id,
            'البسملة افتتاح كل أمر ذي بال، وهي تتضمن الاستعانة بالله والتبرك باسمه العظيم. والرحمن الرحيم صفتان من صفات الله تعالى تدلان على سعة رحمته وعمومها.',
            encode(sha256('البسملة افتتاح كل أمر ذي بال، وهي تتضمن الاستعانة بالله والتبرك باسمه العظيم. والرحمن الرحيم صفتان من صفات الله تعالى تدلان على سعة رحمته وعمومها.'::bytea), 'hex'),
            28,
            ARRAY['Tawhid', 'Divine Names', 'Mercy', 'Blessing'],
            ARRAY['17:110', '27:30', '55:1'],
            NOW(),
            NOW()
        );
    END IF;
    
    IF tabari_id IS NOT NULL THEN
        INSERT INTO tafsir (
            id, surah_number, ayah_number, source_id, text, text_hash, word_count, 
            themes, cross_references, created_at, updated_at
        ) VALUES (
            uuid_generate_v4(),
            1, 1, tabari_id,
            'اختلف العلماء في البسملة هل هي آية من الفاتحة أم لا. والصحيح أنها آية مستقلة أنزلت للفصل بين السور. وفيها تعليم للمؤمنين أن يبدؤوا أعمالهم باسم الله.',
            encode(sha256('اختلف العلماء في البسملة هل هي آية من الفاتحة أم لا. والصحيح أنها آية مستقلة أنزلت للفصل بين السور. وفيها تعليم للمؤمنين أن يبدؤوا أعمالهم باسم الله.'::bytea), 'hex'),
            32,
            ARRAY['Scholarly Differences', 'Quranic Structure', 'Divine Guidance'],
            ARRAY['9:1', '11:41'],
            NOW(),
            NOW()
        );
    END IF;
    
    IF qurtubi_id IS NOT NULL THEN
        INSERT INTO tafsir (
            id, surah_number, ayah_number, source_id, text, text_hash, word_count, 
            themes, cross_references, created_at, updated_at
        ) VALUES (
            uuid_generate_v4(),
            1, 1, qurtubi_id,
            'البسملة مشروعة في ابتداء كل عمل مباح، وهي واجبة في الصلاة عند الشافعية، مستحبة عند المالكية والحنابلة. وفيها إقرار بالربوبية والألوهية.',
            encode(sha256('البسملة مشروعة في ابتداء كل عمل مباح، وهي واجبة في الصلاة عند الشافعية، مستحبة عند المالكية والحنابلة. وفيها إقرار بالربوبية والألوهية.'::bytea), 'hex'),
            26,
            ARRAY['Jurisprudence', 'Prayer Rules', 'Schools of Thought', 'Tawhid'],
            ARRAY['2:21', '4:1'],
            NOW(),
            NOW()
        );
    END IF;
END $$;

-- Create view for Tafsir with source information (for easier querying)
CREATE OR REPLACE VIEW tafsir_with_sources AS
SELECT 
    t.id,
    t.surah_number,
    t.ayah_number,
    t.text,
    t.word_count,
    t.themes,
    t.cross_references,
    t.created_at as tafsir_created_at,
    t.updated_at as tafsir_updated_at,
    ts.id as source_id,
    ts.name as source_name,
    ts.author,
    ts.language,
    ts.description,
    ts.credibility_score,
    ts.scholarly_authentication,
    ts.source_type,
    ts.publication_year,
    ts.methodology,
    ts.created_at as source_created_at,
    ts.updated_at as source_updated_at
FROM tafsir t
JOIN tafsir_sources ts ON t.source_id = ts.id
WHERE ts.is_active = true;

-- Create function to calculate Tafsir coverage statistics
CREATE OR REPLACE FUNCTION get_tafsir_coverage_stats()
RETURNS TABLE (
    surah_number INTEGER,
    total_ayahs INTEGER,
    covered_ayahs BIGINT,
    coverage_percentage DECIMAL(5,2),
    source_count BIGINT,
    avg_credibility DECIMAL(3,1)
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        s.number as surah_number,
        s.number_of_ayahs as total_ayahs,
        COALESCE(coverage.covered_ayahs, 0) as covered_ayahs,
        CASE 
            WHEN s.number_of_ayahs > 0 THEN 
                ROUND((COALESCE(coverage.covered_ayahs, 0)::DECIMAL / s.number_of_ayahs) * 100, 2)
            ELSE 0
        END as coverage_percentage,
        COALESCE(coverage.source_count, 0) as source_count,
        COALESCE(coverage.avg_credibility, 0) as avg_credibility
    FROM surahs s
    LEFT JOIN (
        SELECT 
            t.surah_number,
            COUNT(DISTINCT t.ayah_number) as covered_ayahs,
            COUNT(DISTINCT t.source_id) as source_count,
            ROUND(AVG(ts.credibility_score), 1) as avg_credibility
        FROM tafsir t
        JOIN tafsir_sources ts ON t.source_id = ts.id
        WHERE ts.is_active = true
        GROUP BY t.surah_number
    ) coverage ON s.number = coverage.surah_number
    ORDER BY s.number;
END;
$$ LANGUAGE plpgsql;

-- Create function to get Tafsir comparison data
CREATE OR REPLACE FUNCTION compare_tafsir_sources(
    p_surah_number INTEGER,
    p_ayah_number INTEGER,
    p_source_ids UUID[] DEFAULT NULL
)
RETURNS TABLE (
    source_id UUID,
    source_name TEXT,
    author TEXT,
    credibility_score DECIMAL(3,1),
    source_type VARCHAR(50),
    tafsir_text TEXT,
    word_count INTEGER,
    themes TEXT[],
    cross_references TEXT[]
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        ts.id as source_id,
        ts.name as source_name,
        ts.author,
        ts.credibility_score,
        ts.source_type,
        t.text as tafsir_text,
        t.word_count,
        t.themes,
        t.cross_references
    FROM tafsir t
    JOIN tafsir_sources ts ON t.source_id = ts.id
    WHERE t.surah_number = p_surah_number 
      AND t.ayah_number = p_ayah_number
      AND ts.is_active = true
      AND (p_source_ids IS NULL OR ts.id = ANY(p_source_ids))
    ORDER BY ts.credibility_score DESC, ts.name;
END;
$$ LANGUAGE plpgsql;

-- Add comments for documentation
COMMENT ON TABLE tafsir_sources IS 'Enhanced Tafsir sources with credibility scoring and scholarly authentication';
COMMENT ON COLUMN tafsir_sources.credibility_score IS 'Credibility score from 0.0 to 10.0 based on scholarly authentication and source quality';
COMMENT ON COLUMN tafsir_sources.scholarly_authentication IS 'Level of scholarly authentication: highly_authenticated, authenticated, verified, unverified';
COMMENT ON COLUMN tafsir_sources.source_type IS 'Type of Tafsir source: classical, contemporary, linguistic, thematic, sectarian';

COMMENT ON TABLE tafsir IS 'Enhanced Tafsir entries with thematic tagging and cross-references';
COMMENT ON COLUMN tafsir.word_count IS 'Number of words in the Tafsir text for reading time estimation';
COMMENT ON COLUMN tafsir.themes IS 'Array of thematic tags for categorization and analysis';
COMMENT ON COLUMN tafsir.cross_references IS 'Array of cross-references to other Quranic verses or Hadith';

COMMENT ON VIEW tafsir_with_sources IS 'Convenient view joining Tafsir entries with their source information';
COMMENT ON FUNCTION get_tafsir_coverage_stats() IS 'Returns coverage statistics for Tafsir across all Surahs';
COMMENT ON FUNCTION compare_tafsir_sources(INTEGER, INTEGER, UUID[]) IS 'Returns comparison data for Tafsir sources on a specific verse';