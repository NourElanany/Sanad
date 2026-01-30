-- Islamic Stories System Migration
-- This migration creates comprehensive tables for Islamic stories with
-- integrity verification, categorization, and source authentication

-- Enable UUID extension if not already enabled
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create enum types for story categorization and metadata

-- Story categories enum
CREATE TYPE story_category AS ENUM (
    'prophets',
    'companions', 
    'righteous_predecessors',
    'historical_events',
    'moral_lessons',
    'miracles',
    'battles',
    'conversions',
    'women_in_islam',
    'children_stories'
);

-- Time periods in Islamic history
CREATE TYPE time_period AS ENUM (
    'pre_islamic',
    'prophetic_era',
    'rightly_guided_caliphs',
    'umayyad',
    'abbasid',
    'ottoman',
    'modern',
    'ancient_prophets'
);

-- Age groups for story targeting
CREATE TYPE age_group AS ENUM (
    'children',
    'teenagers',
    'young_adults',
    'adults',
    'all_ages'
);

-- Character types in Islamic stories
CREATE TYPE character_type AS ENUM (
    'prophet',
    'messenger',
    'companion',
    'righteous_person',
    'scholar',
    'ruler',
    'martyr',
    'convert',
    'historical_figure',
    'antagonist'
);

-- Character roles within stories
CREATE TYPE character_role AS ENUM (
    'protagonist',
    'supporting',
    'mentor',
    'antagonist',
    'witness',
    'narrator'
);

-- Importance levels of characters
CREATE TYPE importance_level AS ENUM (
    'primary',
    'secondary',
    'minor'
);

-- Types of lessons
CREATE TYPE lesson_type AS ENUM (
    'moral',
    'spiritual',
    'practical',
    'historical',
    'theological',
    'social'
);

-- Moral categories
CREATE TYPE moral_category AS ENUM (
    'patience',
    'gratitude',
    'justice',
    'mercy',
    'honesty',
    'courage',
    'humility',
    'forgiveness',
    'perseverance',
    'faith'
);

-- Source types for stories
CREATE TYPE source_type AS ENUM (
    'quran',
    'hadith',
    'historical_book',
    'biography',
    'tafsir',
    'scholarly_work'
);

-- Authenticity levels
CREATE TYPE authenticity_level AS ENUM (
    'authentic',
    'well_documented',
    'probable',
    'traditional',
    'educational'
);

-- Scholarly verification status
CREATE TYPE scholarly_verification AS ENUM (
    'verified',
    'under_review',
    'pending',
    'disputed'
);

-- Verification status for sources
CREATE TYPE verification_status AS ENUM (
    'verified',
    'unverified',
    'questionable'
);

-- Collection types
CREATE TYPE collection_type AS ENUM (
    'thematic',
    'chronological',
    'character_based',
    'age_specific',
    'educational'
);

-- Main stories table
CREATE TABLE stories (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(300) NOT NULL,
    arabic_title VARCHAR(300) NOT NULL,
    content TEXT NOT NULL,
    content_hash VARCHAR(64) NOT NULL, -- SHA-256 hash for integrity verification
    summary TEXT,
    category story_category NOT NULL,
    subcategory VARCHAR(100),
    time_period time_period,
    location VARCHAR(200),
    word_count INTEGER DEFAULT 0,
    estimated_reading_time INTEGER DEFAULT 0, -- in minutes
    age_group age_group NOT NULL,
    moral_lessons TEXT[] DEFAULT '{}',
    themes TEXT[] DEFAULT '{}',
    keywords TEXT[] DEFAULT '{}',
    language VARCHAR(10) DEFAULT 'ar',
    authenticity_level authenticity_level NOT NULL,
    scholarly_verification scholarly_verification DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Characters table
CREATE TABLE characters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(200) NOT NULL,
    arabic_name VARCHAR(200) NOT NULL,
    character_type character_type NOT NULL,
    description TEXT,
    historical_period time_period,
    birth_year INTEGER, -- Hijri year
    death_year INTEGER, -- Hijri year
    biography TEXT,
    virtues TEXT[] DEFAULT '{}',
    role_significance TEXT,
    related_stories_count INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Story-Character relationship table
CREATE TABLE story_characters (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    story_id UUID NOT NULL REFERENCES stories(id) ON DELETE CASCADE,
    character_id UUID NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    role_in_story character_role NOT NULL,
    importance_level importance_level NOT NULL,
    character_description_in_story TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(story_id, character_id)
);

-- Lessons table
CREATE TABLE lessons (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(300) NOT NULL,
    arabic_title VARCHAR(300) NOT NULL,
    description TEXT NOT NULL,
    lesson_type lesson_type NOT NULL,
    moral_category moral_category NOT NULL,
    practical_application TEXT,
    target_audience age_group[] DEFAULT '{all_ages}',
    related_verses TEXT[] DEFAULT '{}', -- Related Quranic verses
    related_hadiths TEXT[] DEFAULT '{}', -- Related Hadith references
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Story-Lesson relationship table
CREATE TABLE story_lessons (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    story_id UUID NOT NULL REFERENCES stories(id) ON DELETE CASCADE,
    lesson_id UUID NOT NULL REFERENCES lessons(id) ON DELETE CASCADE,
    relevance_score DECIMAL(3,1) DEFAULT 5.0 CHECK (relevance_score >= 0.0 AND relevance_score <= 10.0),
    explanation TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(story_id, lesson_id)
);

-- Story sources table for references and authentication
CREATE TABLE story_sources (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    story_id UUID NOT NULL REFERENCES stories(id) ON DELETE CASCADE,
    source_type source_type NOT NULL,
    source_name VARCHAR(300) NOT NULL,
    arabic_source_name VARCHAR(300) NOT NULL,
    author VARCHAR(200),
    reference VARCHAR(500) NOT NULL, -- Specific reference (verse, hadith number, page, etc.)
    authenticity_grade VARCHAR(50), -- For Hadith sources
    credibility_score DECIMAL(3,1) DEFAULT 5.0 CHECK (credibility_score >= 0.0 AND credibility_score <= 10.0),
    verification_status verification_status DEFAULT 'unverified',
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Story collections table
CREATE TABLE story_collections (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(300) NOT NULL,
    arabic_name VARCHAR(300) NOT NULL,
    description TEXT,
    collection_type collection_type NOT NULL,
    story_count INTEGER DEFAULT 0,
    target_age_group age_group,
    themes TEXT[] DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Story collection membership table
CREATE TABLE story_collection_members (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    collection_id UUID NOT NULL REFERENCES story_collections(id) ON DELETE CASCADE,
    story_id UUID NOT NULL REFERENCES stories(id) ON DELETE CASCADE,
    order_in_collection INTEGER NOT NULL,
    added_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(collection_id, story_id),
    UNIQUE(collection_id, order_in_collection)
);

-- Create indexes for better performance

-- Stories indexes
CREATE INDEX idx_stories_category ON stories(category);
CREATE INDEX idx_stories_age_group ON stories(age_group);
CREATE INDEX idx_stories_time_period ON stories(time_period);
CREATE INDEX idx_stories_authenticity_level ON stories(authenticity_level);
CREATE INDEX idx_stories_scholarly_verification ON stories(scholarly_verification);
CREATE INDEX idx_stories_word_count ON stories(word_count);
CREATE INDEX idx_stories_language ON stories(language);
CREATE INDEX idx_stories_themes ON stories USING gin(themes);
CREATE INDEX idx_stories_keywords ON stories USING gin(keywords);
CREATE INDEX idx_stories_moral_lessons ON stories USING gin(moral_lessons);
CREATE INDEX idx_stories_created_at ON stories(created_at);
CREATE INDEX idx_stories_updated_at ON stories(updated_at);

-- Characters indexes
CREATE INDEX idx_characters_name ON characters(name);
CREATE INDEX idx_characters_arabic_name ON characters(arabic_name);
CREATE INDEX idx_characters_character_type ON characters(character_type);
CREATE INDEX idx_characters_historical_period ON characters(historical_period);
CREATE INDEX idx_characters_birth_year ON characters(birth_year);
CREATE INDEX idx_characters_death_year ON characters(death_year);
CREATE INDEX idx_characters_virtues ON characters USING gin(virtues);
CREATE INDEX idx_characters_related_stories_count ON characters(related_stories_count);

-- Story-Character relationship indexes
CREATE INDEX idx_story_characters_story_id ON story_characters(story_id);
CREATE INDEX idx_story_characters_character_id ON story_characters(character_id);
CREATE INDEX idx_story_characters_role ON story_characters(role_in_story);
CREATE INDEX idx_story_characters_importance ON story_characters(importance_level);

-- Lessons indexes
CREATE INDEX idx_lessons_lesson_type ON lessons(lesson_type);
CREATE INDEX idx_lessons_moral_category ON lessons(moral_category);
CREATE INDEX idx_lessons_target_audience ON lessons USING gin(target_audience);
CREATE INDEX idx_lessons_related_verses ON lessons USING gin(related_verses);
CREATE INDEX idx_lessons_related_hadiths ON lessons USING gin(related_hadiths);

-- Story-Lesson relationship indexes
CREATE INDEX idx_story_lessons_story_id ON story_lessons(story_id);
CREATE INDEX idx_story_lessons_lesson_id ON story_lessons(lesson_id);
CREATE INDEX idx_story_lessons_relevance_score ON story_lessons(relevance_score);

-- Story sources indexes
CREATE INDEX idx_story_sources_story_id ON story_sources(story_id);
CREATE INDEX idx_story_sources_source_type ON story_sources(source_type);
CREATE INDEX idx_story_sources_credibility_score ON story_sources(credibility_score);
CREATE INDEX idx_story_sources_verification_status ON story_sources(verification_status);
CREATE INDEX idx_story_sources_author ON story_sources(author);

-- Story collections indexes
CREATE INDEX idx_story_collections_collection_type ON story_collections(collection_type);
CREATE INDEX idx_story_collections_target_age_group ON story_collections(target_age_group);
CREATE INDEX idx_story_collections_themes ON story_collections USING gin(themes);
CREATE INDEX idx_story_collections_story_count ON story_collections(story_count);

-- Collection membership indexes
CREATE INDEX idx_story_collection_members_collection_id ON story_collection_members(collection_id);
CREATE INDEX idx_story_collection_members_story_id ON story_collection_members(story_id);
CREATE INDEX idx_story_collection_members_order ON story_collection_members(order_in_collection);

-- Create composite indexes for common query patterns
CREATE INDEX idx_stories_category_age_group ON stories(category, age_group);
CREATE INDEX idx_stories_authenticity_verification ON stories(authenticity_level, scholarly_verification);
CREATE INDEX idx_characters_type_period ON characters(character_type, historical_period);
CREATE INDEX idx_story_sources_type_verification ON story_sources(source_type, verification_status);

-- Create full-text search indexes for Arabic content
CREATE INDEX idx_stories_full_text_arabic ON stories USING gin(
    to_tsvector('arabic', title || ' ' || arabic_title || ' ' || COALESCE(content, '') || ' ' || COALESCE(summary, ''))
);

CREATE INDEX idx_characters_full_text_arabic ON characters USING gin(
    to_tsvector('arabic', name || ' ' || arabic_name || ' ' || COALESCE(description, '') || ' ' || COALESCE(biography, ''))
);

CREATE INDEX idx_lessons_full_text_arabic ON lessons USING gin(
    to_tsvector('arabic', title || ' ' || arabic_title || ' ' || description)
);

-- Create functions for data integrity and maintenance

-- Function to update story word count and reading time
CREATE OR REPLACE FUNCTION update_story_metrics()
RETURNS TRIGGER AS $$
BEGIN
    NEW.word_count = array_length(string_to_array(trim(NEW.content), ' '), 1);
    NEW.estimated_reading_time = GREATEST(1, CEIL(NEW.word_count::DECIMAL / 200)); -- 200 words per minute
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Function to update character story count
CREATE OR REPLACE FUNCTION update_character_story_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE characters 
        SET related_stories_count = related_stories_count + 1,
            updated_at = NOW()
        WHERE id = NEW.character_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE characters 
        SET related_stories_count = related_stories_count - 1,
            updated_at = NOW()
        WHERE id = OLD.character_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Function to update collection story count
CREATE OR REPLACE FUNCTION update_collection_story_count()
RETURNS TRIGGER AS $$
BEGIN
    IF TG_OP = 'INSERT' THEN
        UPDATE story_collections 
        SET story_count = story_count + 1,
            updated_at = NOW()
        WHERE id = NEW.collection_id;
        RETURN NEW;
    ELSIF TG_OP = 'DELETE' THEN
        UPDATE story_collections 
        SET story_count = story_count - 1,
            updated_at = NOW()
        WHERE id = OLD.collection_id;
        RETURN OLD;
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Function to validate content hash integrity
CREATE OR REPLACE FUNCTION validate_content_hash()
RETURNS TRIGGER AS $$
BEGIN
    -- This would be implemented in the application layer for actual hash calculation
    -- Here we just ensure the hash field is not empty
    IF NEW.content_hash IS NULL OR LENGTH(NEW.content_hash) != 64 THEN
        RAISE EXCEPTION 'Invalid content hash: must be 64-character SHA-256 hash';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create triggers

-- Trigger for story metrics update
DROP TRIGGER IF EXISTS trigger_update_story_metrics ON stories;
CREATE TRIGGER trigger_update_story_metrics
    BEFORE INSERT OR UPDATE OF content ON stories
    FOR EACH ROW
    EXECUTE FUNCTION update_story_metrics();

-- Trigger for character story count
DROP TRIGGER IF EXISTS trigger_update_character_story_count ON story_characters;
CREATE TRIGGER trigger_update_character_story_count
    AFTER INSERT OR DELETE ON story_characters
    FOR EACH ROW
    EXECUTE FUNCTION update_character_story_count();

-- Trigger for collection story count
DROP TRIGGER IF EXISTS trigger_update_collection_story_count ON story_collection_members;
CREATE TRIGGER trigger_update_collection_story_count
    AFTER INSERT OR DELETE ON story_collection_members
    FOR EACH ROW
    EXECUTE FUNCTION update_collection_story_count();

-- Trigger for content hash validation
DROP TRIGGER IF EXISTS trigger_validate_content_hash ON stories;
CREATE TRIGGER trigger_validate_content_hash
    BEFORE INSERT OR UPDATE OF content_hash ON stories
    FOR EACH ROW
    EXECUTE FUNCTION validate_content_hash();

-- Create views for common queries

-- View for stories with their main character and primary lesson
CREATE OR REPLACE VIEW story_overview AS
SELECT 
    s.id,
    s.title,
    s.arabic_title,
    s.category,
    s.age_group,
    s.authenticity_level,
    s.word_count,
    s.estimated_reading_time,
    s.themes,
    s.created_at,
    -- Get primary character
    (SELECT c.name 
     FROM characters c 
     JOIN story_characters sc ON c.id = sc.character_id 
     WHERE sc.story_id = s.id AND sc.importance_level = 'primary' 
     LIMIT 1) as primary_character,
    -- Get primary lesson
    (SELECT l.title 
     FROM lessons l 
     JOIN story_lessons sl ON l.id = sl.lesson_id 
     WHERE sl.story_id = s.id 
     ORDER BY sl.relevance_score DESC 
     LIMIT 1) as primary_lesson,
    -- Count of sources
    (SELECT COUNT(*) FROM story_sources ss WHERE ss.story_id = s.id) as source_count
FROM stories s;

-- View for character details with their stories
CREATE OR REPLACE VIEW character_details AS
SELECT 
    c.id,
    c.name,
    c.arabic_name,
    c.character_type,
    c.historical_period,
    c.birth_year,
    c.death_year,
    c.virtues,
    c.related_stories_count,
    -- Get story categories this character appears in
    array_agg(DISTINCT s.category) as story_categories,
    -- Get most common role
    (SELECT sc.role_in_story 
     FROM story_characters sc 
     JOIN stories s ON sc.story_id = s.id 
     WHERE sc.character_id = c.id 
     GROUP BY sc.role_in_story 
     ORDER BY COUNT(*) DESC 
     LIMIT 1) as most_common_role
FROM characters c
LEFT JOIN story_characters sc ON c.id = sc.character_id
LEFT JOIN stories s ON sc.story_id = s.id
GROUP BY c.id, c.name, c.arabic_name, c.character_type, c.historical_period, 
         c.birth_year, c.death_year, c.virtues, c.related_stories_count;

-- View for lesson analytics
CREATE OR REPLACE VIEW lesson_analytics AS
SELECT 
    l.id,
    l.title,
    l.arabic_title,
    l.lesson_type,
    l.moral_category,
    l.target_audience,
    -- Count of stories teaching this lesson
    COUNT(sl.story_id) as story_count,
    -- Average relevance score
    AVG(sl.relevance_score) as avg_relevance_score,
    -- Most common story category for this lesson
    (SELECT s.category 
     FROM stories s 
     JOIN story_lessons sl2 ON s.id = sl2.story_id 
     WHERE sl2.lesson_id = l.id 
     GROUP BY s.category 
     ORDER BY COUNT(*) DESC 
     LIMIT 1) as most_common_category
FROM lessons l
LEFT JOIN story_lessons sl ON l.id = sl.lesson_id
GROUP BY l.id, l.title, l.arabic_title, l.lesson_type, l.moral_category, l.target_audience;

-- View for story statistics by category
CREATE OR REPLACE VIEW story_category_statistics AS
SELECT 
    category,
    COUNT(*) as total_stories,
    COUNT(CASE WHEN authenticity_level = 'authentic' THEN 1 END) as authentic_stories,
    COUNT(CASE WHEN authenticity_level = 'well_documented' THEN 1 END) as well_documented_stories,
    COUNT(CASE WHEN age_group = 'children' THEN 1 END) as children_stories,
    COUNT(CASE WHEN age_group = 'all_ages' THEN 1 END) as all_ages_stories,
    AVG(word_count) as avg_word_count,
    AVG(estimated_reading_time) as avg_reading_time,
    -- Most common themes
    (SELECT unnest(themes) as theme 
     FROM stories s2 
     WHERE s2.category = s.category 
     GROUP BY theme 
     ORDER BY COUNT(*) DESC 
     LIMIT 3) as top_themes
FROM stories s
GROUP BY category;

-- Insert sample data for testing and demonstration

-- Insert sample characters
INSERT INTO characters (name, arabic_name, character_type, historical_period, description, virtues) VALUES
('Prophet Muhammad', 'النبي محمد صلى الله عليه وسلم', 'prophet', 'prophetic_era', 'The final messenger of Allah', ARRAY['Honesty', 'Mercy', 'Justice', 'Patience']),
('Prophet Yusuf', 'النبي يوسف عليه السلام', 'prophet', 'ancient_prophets', 'Prophet known for his beauty and interpretation of dreams', ARRAY['Patience', 'Forgiveness', 'Wisdom']),
('Abu Bakr As-Siddiq', 'أبو بكر الصديق رضي الله عنه', 'companion', 'prophetic_era', 'The first Caliph and closest companion of Prophet Muhammad', ARRAY['Loyalty', 'Courage', 'Generosity']),
('Umar ibn Al-Khattab', 'عمر بن الخطاب رضي الله عنه', 'companion', 'rightly_guided_caliphs', 'The second Caliph known for his justice', ARRAY['Justice', 'Courage', 'Humility']),
('Aisha bint Abu Bakr', 'عائشة بنت أبي بكر رضي الله عنها', 'companion', 'prophetic_era', 'Wife of Prophet Muhammad and scholar', ARRAY['Knowledge', 'Teaching', 'Devotion']),
('Salahuddin Al-Ayyubi', 'صلاح الدين الأيوبي', 'ruler', 'abbasid', 'Kurdish Muslim leader who recaptured Jerusalem', ARRAY['Justice', 'Mercy', 'Military Strategy'])
ON CONFLICT DO NOTHING;

-- Insert sample lessons
INSERT INTO lessons (title, arabic_title, description, lesson_type, moral_category, target_audience, related_verses) VALUES
('The Importance of Patience', 'أهمية الصبر', 'Learning to be patient in times of difficulty and hardship', 'moral', 'patience', ARRAY['all_ages'], ARRAY['2:155', '3:200']),
('Forgiveness and Mercy', 'المغفرة والرحمة', 'The virtue of forgiving others and showing mercy', 'moral', 'forgiveness', ARRAY['teenagers', 'adults'], ARRAY['24:22', '42:40']),
('Trust in Allah', 'التوكل على الله', 'Having complete trust and reliance on Allah', 'spiritual', 'faith', ARRAY['all_ages'], ARRAY['65:3', '8:2']),
('Justice and Fairness', 'العدل والإنصاف', 'The importance of being just and fair in all dealings', 'moral', 'justice', ARRAY['young_adults', 'adults'], ARRAY['4:135', '5:8']),
('Honesty in Speech', 'الصدق في القول', 'The virtue of being truthful in all circumstances', 'moral', 'honesty', ARRAY['children', 'teenagers'], ARRAY['9:119', '33:70'])
ON CONFLICT DO NOTHING;

-- Insert sample story collections
INSERT INTO story_collections (name, arabic_name, description, collection_type, target_age_group, themes) VALUES
('Stories of the Prophets', 'قصص الأنبياء', 'Collection of authentic stories about the Prophets mentioned in the Quran', 'thematic', 'all_ages', ARRAY['Prophethood', 'Divine Guidance', 'Patience']),
('Companions of the Prophet', 'قصص الصحابة', 'Stories of the noble companions of Prophet Muhammad', 'thematic', 'teenagers', ARRAY['Loyalty', 'Sacrifice', 'Faith']),
('Children''s Islamic Stories', 'قصص إسلامية للأطفال', 'Educational stories suitable for young children', 'age_specific', 'children', ARRAY['Basic Values', 'Good Manners', 'Simple Lessons']),
('Women in Islam', 'نساء في الإسلام', 'Stories of remarkable women in Islamic history', 'thematic', 'all_ages', ARRAY['Female Role Models', 'Strength', 'Wisdom']),
('Historical Events', 'الأحداث التاريخية', 'Important events in Islamic history', 'chronological', 'adults', ARRAY['History', 'Lessons', 'Context'])
ON CONFLICT DO NOTHING;

-- Add comments for documentation
COMMENT ON TABLE stories IS 'Main table for Islamic stories with integrity verification and comprehensive metadata';
COMMENT ON TABLE characters IS 'Characters appearing in Islamic stories with biographical information';
COMMENT ON TABLE story_characters IS 'Many-to-many relationship between stories and characters with role information';
COMMENT ON TABLE lessons IS 'Moral and spiritual lessons derived from Islamic stories';
COMMENT ON TABLE story_lessons IS 'Many-to-many relationship between stories and lessons with relevance scoring';
COMMENT ON TABLE story_sources IS 'Sources and references for Islamic stories with authenticity verification';
COMMENT ON TABLE story_collections IS 'Collections or series of related stories';
COMMENT ON TABLE story_collection_members IS 'Membership of stories in collections with ordering';

COMMENT ON COLUMN stories.content_hash IS 'SHA-256 hash for verifying story content integrity';
COMMENT ON COLUMN stories.moral_lessons IS 'Array of key moral lessons from the story';
COMMENT ON COLUMN stories.themes IS 'Thematic tags for categorization and search';
COMMENT ON COLUMN stories.keywords IS 'Keywords extracted for enhanced search functionality';
COMMENT ON COLUMN stories.estimated_reading_time IS 'Estimated reading time in minutes based on word count';

COMMENT ON COLUMN characters.virtues IS 'Array of character virtues and positive qualities';
COMMENT ON COLUMN characters.related_stories_count IS 'Count of stories this character appears in';

COMMENT ON COLUMN story_sources.credibility_score IS 'Source credibility score from 0.0 to 10.0';
COMMENT ON COLUMN story_sources.authenticity_grade IS 'Authenticity grade for Hadith sources (sahih, hasan, etc.)';

COMMENT ON VIEW story_overview IS 'Comprehensive overview of stories with primary character and lesson';
COMMENT ON VIEW character_details IS 'Character information with story statistics and common roles';
COMMENT ON VIEW lesson_analytics IS 'Analytics for lessons including story count and relevance scores';
COMMENT ON VIEW story_category_statistics IS 'Statistical summary of stories by category';