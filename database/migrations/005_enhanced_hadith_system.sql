-- Enhanced Hadith System Migration
-- This migration enhances the existing hadith system with advanced features
-- including improved data models, better indexing, and additional metadata

-- First, let's add missing columns to existing tables

-- Add missing columns to hadith_books table
ALTER TABLE hadith_books ADD COLUMN IF NOT EXISTS author_arabic_name VARCHAR(100);
ALTER TABLE hadith_books ADD COLUMN IF NOT EXISTS compilation_year INTEGER;
ALTER TABLE hadith_books ADD COLUMN IF NOT EXISTS total_hadiths INTEGER DEFAULT 0;
ALTER TABLE hadith_books ADD COLUMN IF NOT EXISTS book_type VARCHAR(20) DEFAULT 'sahih' 
    CHECK (book_type IN ('sahih', 'sunan', 'musnad', 'mujam', 'mustadrak', 'jami'));
ALTER TABLE hadith_books ADD COLUMN IF NOT EXISTS authenticity_level VARCHAR(20) DEFAULT 'high'
    CHECK (authenticity_level IN ('highest', 'high', 'moderate', 'variable'));
ALTER TABLE hadith_books ADD COLUMN IF NOT EXISTS language VARCHAR(10) DEFAULT 'ar';
ALTER TABLE hadith_books ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW();

-- Add missing columns to hadiths table
ALTER TABLE hadiths ADD COLUMN IF NOT EXISTS chapter_number INTEGER;
ALTER TABLE hadiths ADD COLUMN IF NOT EXISTS hadith_number_in_chapter INTEGER;
ALTER TABLE hadiths ADD COLUMN IF NOT EXISTS word_count INTEGER DEFAULT 0;
ALTER TABLE hadiths ADD COLUMN IF NOT EXISTS themes TEXT[] DEFAULT '{}';
ALTER TABLE hadiths ADD COLUMN IF NOT EXISTS keywords TEXT[] DEFAULT '{}';
ALTER TABLE hadiths ADD COLUMN IF NOT EXISTS language VARCHAR(10) DEFAULT 'ar';
ALTER TABLE hadiths ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW();

-- Create the Sanad (Chain of Narration) table
CREATE TABLE IF NOT EXISTS sanad (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    hadith_id UUID NOT NULL REFERENCES hadiths(id) ON DELETE CASCADE,
    chain_text TEXT NOT NULL,
    chain_hash VARCHAR(64) NOT NULL, -- SHA-256 hash for integrity verification
    narrators TEXT[] NOT NULL DEFAULT '{}', -- Ordered list of narrators
    chain_grade VARCHAR(20) NOT NULL DEFAULT 'sahih'
        CHECK (chain_grade IN ('sahih', 'hasan', 'daif', 'munqati', 'mursal')),
    chain_analysis TEXT, -- Scholarly analysis of the chain
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create the Scholars table
CREATE TABLE IF NOT EXISTS scholars (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(200) NOT NULL,
    arabic_name VARCHAR(200) NOT NULL,
    birth_year INTEGER,
    death_year INTEGER,
    biography TEXT,
    specialization TEXT[] DEFAULT '{}', -- Areas of expertise
    credibility_score DECIMAL(3,1) DEFAULT 5.0 CHECK (credibility_score >= 0.0 AND credibility_score <= 10.0),
    scholarly_authentication VARCHAR(30) NOT NULL DEFAULT 'verified'
        CHECK (scholarly_authentication IN ('highly_authenticated', 'authenticated', 'verified', 'unverified')),
    school_of_thought VARCHAR(100), -- Madhab or scholarly approach
    major_works TEXT[] DEFAULT '{}', -- List of major scholarly works
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create the Hadith Chapters table
CREATE TABLE IF NOT EXISTS hadith_chapters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    book_id UUID NOT NULL REFERENCES hadith_books(id) ON DELETE CASCADE,
    chapter_number INTEGER NOT NULL,
    title VARCHAR(300) NOT NULL,
    arabic_title VARCHAR(300) NOT NULL,
    description TEXT,
    hadith_count INTEGER DEFAULT 0,
    themes TEXT[] DEFAULT '{}', -- Thematic tags for the chapter
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(book_id, chapter_number)
);

-- Enhance the hadith_explanations table
ALTER TABLE hadith_explanations ADD COLUMN IF NOT EXISTS scholar_id UUID REFERENCES scholars(id);
ALTER TABLE hadith_explanations ADD COLUMN IF NOT EXISTS explanation_hash VARCHAR(64);
ALTER TABLE hadith_explanations ADD COLUMN IF NOT EXISTS word_count INTEGER DEFAULT 0;
ALTER TABLE hadith_explanations ADD COLUMN IF NOT EXISTS key_points TEXT[] DEFAULT '{}';
ALTER TABLE hadith_explanations ADD COLUMN IF NOT EXISTS related_verses TEXT[] DEFAULT '{}';
ALTER TABLE hadith_explanations ADD COLUMN IF NOT EXISTS related_hadiths TEXT[] DEFAULT '{}';
ALTER TABLE hadith_explanations ADD COLUMN IF NOT EXISTS updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW();

-- Create indexes for better performance

-- Hadith indexes
CREATE INDEX IF NOT EXISTS idx_hadiths_word_count ON hadiths(word_count);
CREATE INDEX IF NOT EXISTS idx_hadiths_themes ON hadiths USING gin(themes);
CREATE INDEX IF NOT EXISTS idx_hadiths_keywords ON hadiths USING gin(keywords);
CREATE INDEX IF NOT EXISTS idx_hadiths_language ON hadiths(language);
CREATE INDEX IF NOT EXISTS idx_hadiths_chapter_number ON hadiths(chapter_number);
CREATE INDEX IF NOT EXISTS idx_hadiths_updated_at ON hadiths(updated_at);

-- Sanad indexes
CREATE INDEX IF NOT EXISTS idx_sanad_hadith_id ON sanad(hadith_id);
CREATE INDEX IF NOT EXISTS idx_sanad_chain_grade ON sanad(chain_grade);
CREATE INDEX IF NOT EXISTS idx_sanad_narrators ON sanad USING gin(narrators);
CREATE INDEX IF NOT EXISTS idx_sanad_chain_text_search ON sanad USING gin(to_tsvector('arabic', chain_text));

-- Scholar indexes
CREATE INDEX IF NOT EXISTS idx_scholars_name ON scholars(name);
CREATE INDEX IF NOT EXISTS idx_scholars_arabic_name ON scholars(arabic_name);
CREATE INDEX IF NOT EXISTS idx_scholars_credibility_score ON scholars(credibility_score);
CREATE INDEX IF NOT EXISTS idx_scholars_authentication ON scholars(scholarly_authentication);
CREATE INDEX IF NOT EXISTS idx_scholars_specialization ON scholars USING gin(specialization);
CREATE INDEX IF NOT EXISTS idx_scholars_death_year ON scholars(death_year);

-- Hadith book indexes
CREATE INDEX IF NOT EXISTS idx_hadith_books_book_type ON hadith_books(book_type);
CREATE INDEX IF NOT EXISTS idx_hadith_books_authenticity_level ON hadith_books(authenticity_level);
CREATE INDEX IF NOT EXISTS idx_hadith_books_compilation_year ON hadith_books(compilation_year);
CREATE INDEX IF NOT EXISTS idx_hadith_books_total_hadiths ON hadith_books(total_hadiths);

-- Hadith chapter indexes
CREATE INDEX IF NOT EXISTS idx_hadith_chapters_book_id ON hadith_chapters(book_id);
CREATE INDEX IF NOT EXISTS idx_hadith_chapters_chapter_number ON hadith_chapters(chapter_number);
CREATE INDEX IF NOT EXISTS idx_hadith_chapters_themes ON hadith_chapters USING gin(themes);
CREATE INDEX IF NOT EXISTS idx_hadith_chapters_title_search ON hadith_chapters USING gin(to_tsvector('arabic', title || ' ' || arabic_title));

-- Hadith explanation indexes
CREATE INDEX IF NOT EXISTS idx_hadith_explanations_scholar_id ON hadith_explanations(scholar_id);
CREATE INDEX IF NOT EXISTS idx_hadith_explanations_word_count ON hadith_explanations(word_count);
CREATE INDEX IF NOT EXISTS idx_hadith_explanations_key_points ON hadith_explanations USING gin(key_points);
CREATE INDEX IF NOT EXISTS idx_hadith_explanations_related_verses ON hadith_explanations USING gin(related_verses);
CREATE INDEX IF NOT EXISTS idx_hadith_explanations_related_hadiths ON hadith_explanations USING gin(related_hadiths);

-- Create composite indexes for common query patterns
CREATE INDEX IF NOT EXISTS idx_hadiths_book_grade ON hadiths(book_id, grade);
CREATE INDEX IF NOT EXISTS idx_hadiths_grade_language ON hadiths(grade, language);
CREATE INDEX IF NOT EXISTS idx_sanad_hadith_grade ON sanad(hadith_id, chain_grade);

-- Create full-text search indexes for Arabic content
CREATE INDEX IF NOT EXISTS idx_hadiths_full_text_arabic ON hadiths USING gin(
    to_tsvector('arabic', text || ' ' || narrator || ' ' || chapter)
);

CREATE INDEX IF NOT EXISTS idx_hadith_explanations_full_text_arabic ON hadith_explanations USING gin(
    to_tsvector('arabic', explanation)
);

-- Create functions for data integrity and maintenance

-- Function to update hadith word count
CREATE OR REPLACE FUNCTION update_hadith_word_count()
RETURNS TRIGGER AS $$
BEGIN
    NEW.word_count = array_length(string_to_array(trim(NEW.text), ' '), 1);
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to update explanation word count
CREATE OR REPLACE FUNCTION update_explanation_word_count()
RETURNS TRIGGER AS $$
BEGIN
    NEW.word_count = array_length(string_to_array(trim(NEW.explanation), ' '), 1);
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to update book hadith count
CREATE OR REPLACE FUNCTION update_book_hadith_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE hadith_books 
        SET total_hadiths = total_hadiths + 1,
            updated_at = NOW()
        WHERE id = NEW.book_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE hadith_books 
        SET total_hadiths = total_hadiths - 1,
            updated_at = NOW()
        WHERE id = OLD.book_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Function to update chapter hadith count
CREATE OR REPLACE FUNCTION update_chapter_hadith_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' AND NEW.chapter_number IS NOT NULL THEN
        UPDATE hadith_chapters 
        SET hadith_count = hadith_count + 1
        WHERE book_id = NEW.book_id AND chapter_number = NEW.chapter_number;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' AND OLD.chapter_number IS NOT NULL THEN
        UPDATE hadith_chapters 
        SET hadith_count = hadith_count - 1
        WHERE book_id = OLD.book_id AND chapter_number = OLD.chapter_number;
        RETURN OLD;
    ELSIF TG_OP = 'UPDATE' THEN
        -- Handle chapter change
        IF OLD.chapter_number IS NOT NULL AND OLD.chapter_number != NEW.chapter_number THEN
            UPDATE hadith_chapters 
            SET hadith_count = hadith_count - 1
            WHERE book_id = OLD.book_id AND chapter_number = OLD.chapter_number;
        END IF;
        IF NEW.chapter_number IS NOT NULL AND OLD.chapter_number != NEW.chapter_number THEN
            UPDATE hadith_chapters 
            SET hadith_count = hadith_count + 1
            WHERE book_id = NEW.book_id AND chapter_number = NEW.chapter_number;
        END IF;
        RETURN NEW;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create triggers

-- Trigger for hadith word count
DROP TRIGGER IF EXISTS trigger_update_hadith_word_count ON hadiths;
CREATE TRIGGER trigger_update_hadith_word_count
    BEFORE INSERT OR UPDATE OF text ON hadiths
    FOR EACH ROW
    EXECUTE FUNCTION update_hadith_word_count();

-- Trigger for explanation word count
DROP TRIGGER IF EXISTS trigger_update_explanation_word_count ON hadith_explanations;
CREATE TRIGGER trigger_update_explanation_word_count
    BEFORE INSERT OR UPDATE OF explanation ON hadith_explanations
    FOR EACH ROW
    EXECUTE FUNCTION update_explanation_word_count();

-- Trigger for book hadith count
DROP TRIGGER IF EXISTS trigger_update_book_hadith_count ON hadiths;
CREATE TRIGGER trigger_update_book_hadith_count
    AFTER INSERT OR DELETE ON hadiths
    FOR EACH ROW
    EXECUTE FUNCTION update_book_hadith_count();

-- Trigger for chapter hadith count
DROP TRIGGER IF EXISTS trigger_update_chapter_hadith_count ON hadiths;
CREATE TRIGGER trigger_update_chapter_hadith_count
    AFTER INSERT OR UPDATE OR DELETE ON hadiths
    FOR EACH ROW
    EXECUTE FUNCTION update_chapter_hadith_count();

-- Create views for common queries

-- View for hadiths with their book and chapter information
CREATE OR REPLACE VIEW hadith_details AS
SELECT 
    h.id,
    h.hadith_number,
    h.text,
    h.text_hash,
    h.narrator,
    h.grade,
    h.word_count,
    h.themes,
    h.keywords,
    h.language,
    h.created_at,
    h.updated_at,
    hb.name as book_name,
    hb.arabic_name as book_arabic_name,
    hb.author as book_author,
    hb.book_type,
    hb.authenticity_level,
    hc.title as chapter_title,
    hc.arabic_title as chapter_arabic_title,
    s.chain_text,
    s.narrators,
    s.chain_grade
FROM hadiths h
LEFT JOIN hadith_books hb ON h.book_id = hb.id
LEFT JOIN hadith_chapters hc ON h.book_id = hc.book_id AND h.chapter_number = hc.chapter_number
LEFT JOIN sanad s ON h.id = s.hadith_id;

-- View for hadith explanations with scholar information
CREATE OR REPLACE VIEW hadith_explanations_with_scholars AS
SELECT 
    he.id,
    he.hadith_id,
    he.explanation,
    he.explanation_hash,
    he.word_count,
    he.key_points,
    he.related_verses,
    he.related_hadiths,
    he.language,
    he.created_at,
    he.updated_at,
    sc.name as scholar_name,
    sc.arabic_name as scholar_arabic_name,
    sc.credibility_score,
    sc.scholarly_authentication,
    sc.school_of_thought,
    sc.specialization
FROM hadith_explanations he
LEFT JOIN scholars sc ON he.scholar_id = sc.id;

-- View for book statistics
CREATE OR REPLACE VIEW hadith_book_statistics AS
SELECT 
    hb.id,
    hb.name,
    hb.arabic_name,
    hb.author,
    hb.book_type,
    hb.authenticity_level,
    hb.total_hadiths,
    COUNT(DISTINCT hc.id) as chapter_count,
    COUNT(CASE WHEN h.grade = 'Sahih' THEN 1 END) as sahih_count,
    COUNT(CASE WHEN h.grade = 'Hasan' THEN 1 END) as hasan_count,
    COUNT(CASE WHEN h.grade = 'Daif' THEN 1 END) as daif_count,
    COUNT(CASE WHEN h.grade = 'Mawdu' THEN 1 END) as mawdu_count,
    AVG(h.word_count) as avg_hadith_length
FROM hadith_books hb
LEFT JOIN hadith_chapters hc ON hb.id = hc.book_id
LEFT JOIN hadiths h ON hb.id = h.book_id
GROUP BY hb.id, hb.name, hb.arabic_name, hb.author, hb.book_type, hb.authenticity_level, hb.total_hadiths;

-- Insert some sample scholars
INSERT INTO scholars (name, arabic_name, birth_year, death_year, credibility_score, scholarly_authentication, school_of_thought, specialization, major_works) VALUES
('Imam Al-Bukhari', 'الإمام البخاري', 810, 870, 10.0, 'highly_authenticated', 'Ahl al-Hadith', ARRAY['Hadith Science', 'Islamic Jurisprudence'], ARRAY['Sahih al-Bukhari', 'Al-Adab al-Mufrad']),
('Imam Muslim', 'الإمام مسلم', 815, 875, 10.0, 'highly_authenticated', 'Ahl al-Hadith', ARRAY['Hadith Science'], ARRAY['Sahih Muslim']),
('Imam Abu Dawud', 'الإمام أبو داود', 817, 889, 9.5, 'highly_authenticated', 'Ahl al-Hadith', ARRAY['Hadith Science', 'Fiqh'], ARRAY['Sunan Abu Dawud']),
('Imam At-Tirmidhi', 'الإمام الترمذي', 824, 892, 9.5, 'highly_authenticated', 'Ahl al-Hadith', ARRAY['Hadith Science'], ARRAY['Jami at-Tirmidhi']),
('Imam Ibn Majah', 'الإمام ابن ماجه', 824, 887, 9.0, 'highly_authenticated', 'Ahl al-Hadith', ARRAY['Hadith Science'], ARRAY['Sunan Ibn Majah']),
('Imam An-Nasai', 'الإمام النسائي', 829, 915, 9.5, 'highly_authenticated', 'Ahl al-Hadith', ARRAY['Hadith Science'], ARRAY['Sunan an-Nasai'])
ON CONFLICT DO NOTHING;

-- Update existing hadith books with enhanced information
UPDATE hadith_books SET 
    author_arabic_name = 'الإمام البخاري',
    compilation_year = 846,
    book_type = 'sahih',
    authenticity_level = 'highest',
    language = 'ar'
WHERE name = 'Sahih Bukhari';

UPDATE hadith_books SET 
    author_arabic_name = 'الإمام مسلم',
    compilation_year = 875,
    book_type = 'sahih',
    authenticity_level = 'highest',
    language = 'ar'
WHERE name = 'Sahih Muslim';

UPDATE hadith_books SET 
    author_arabic_name = 'الإمام الترمذي',
    compilation_year = 884,
    book_type = 'jami',
    authenticity_level = 'high',
    language = 'ar'
WHERE name = 'Jami at-Tirmidhi';

-- Add comments for documentation
COMMENT ON TABLE sanad IS 'Chain of narration (Sanad) for each Hadith with integrity verification';
COMMENT ON TABLE scholars IS 'Islamic scholars who provided Hadith explanations and commentary';
COMMENT ON TABLE hadith_chapters IS 'Chapters within Hadith books for better organization';

COMMENT ON COLUMN sanad.chain_hash IS 'SHA-256 hash for verifying chain text integrity';
COMMENT ON COLUMN sanad.narrators IS 'Ordered array of narrators in the chain of transmission';
COMMENT ON COLUMN sanad.chain_grade IS 'Authenticity grade of the chain of narration';

COMMENT ON COLUMN scholars.credibility_score IS 'Scholar credibility score from 0.0 to 10.0';
COMMENT ON COLUMN scholars.scholarly_authentication IS 'Level of scholarly authentication and verification';
COMMENT ON COLUMN scholars.specialization IS 'Array of scholarly specialization areas';

COMMENT ON COLUMN hadiths.themes IS 'Thematic tags for categorization and search optimization';
COMMENT ON COLUMN hadiths.keywords IS 'Keywords extracted for enhanced search functionality';
COMMENT ON COLUMN hadiths.word_count IS 'Number of words in the Hadith text for reading time estimation';

COMMENT ON VIEW hadith_details IS 'Comprehensive view of Hadiths with book, chapter, and chain information';
COMMENT ON VIEW hadith_explanations_with_scholars IS 'Hadith explanations joined with scholar information';
COMMENT ON VIEW hadith_book_statistics IS 'Statistical summary of Hadith books including grade distribution';