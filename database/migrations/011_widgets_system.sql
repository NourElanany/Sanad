-- Migration: Interactive Widgets System
-- Description: Creates tables for the interactive widgets system including widget configurations, dashboards, and data caching

-- Widget types enum
CREATE TYPE widget_type AS ENUM (
    'next_prayer_time',
    'verse_of_the_day',
    'khatma_progress',
    'islamic_calendar',
    'dhikr_reminder',
    'quick_stats',
    'recent_activity',
    'notifications'
);

-- Widget size enum
CREATE TYPE widget_size AS ENUM (
    'small',   -- 1x1 grid
    'medium',  -- 2x1 or 1x2 grid
    'large',   -- 2x2 grid
    'wide',    -- 3x1 or 4x1 grid
    'tall'     -- 1x3 or 1x4 grid
);

-- Dhikr category enum (reused from notification service)
CREATE TYPE dhikr_category AS ENUM (
    'morning',
    'evening',
    'after_prayer',
    'before_sleep',
    'after_wudu',
    'travel',
    'general'
);

-- Main widgets table
CREATE TABLE widgets (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    widget_type widget_type NOT NULL,
    title VARCHAR(255) NOT NULL,
    is_enabled BOOLEAN NOT NULL DEFAULT true,
    layout JSONB NOT NULL, -- WidgetLayout as JSON
    configuration JSONB NOT NULL DEFAULT '{}', -- Widget-specific configuration
    refresh_interval_minutes INTEGER NOT NULL DEFAULT 15,
    last_updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Widget dashboards table
CREATE TABLE widget_dashboards (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT false,
    layout_config JSONB NOT NULL DEFAULT '{}', -- Grid layout configuration
    widgets JSONB NOT NULL DEFAULT '[]', -- Array of widget IDs
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Widget data cache table (for caching external service responses)
CREATE TABLE widget_data_cache (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cache_key VARCHAR(255) NOT NULL UNIQUE,
    widget_type widget_type NOT NULL,
    data JSONB NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Default dhikr content table (for dhikr reminder widgets)
CREATE TABLE default_dhikr_content (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    category dhikr_category NOT NULL,
    title VARCHAR(255) NOT NULL,
    arabic_text TEXT NOT NULL,
    transliteration TEXT,
    translation_en TEXT,
    translation_ar TEXT,
    reference VARCHAR(255), -- Source reference (e.g., "Sahih Bukhari 123")
    repetitions INTEGER NOT NULL DEFAULT 1,
    order_index INTEGER NOT NULL DEFAULT 0,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Widget refresh log table (for tracking refresh status and errors)
CREATE TABLE widget_refresh_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    widget_id UUID NOT NULL REFERENCES widgets(id) ON DELETE CASCADE,
    refresh_started_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    refresh_completed_at TIMESTAMP WITH TIME ZONE,
    refresh_status VARCHAR(50) NOT NULL DEFAULT 'in_progress', -- success, failed, in_progress
    error_message TEXT,
    data_size_bytes INTEGER,
    response_time_ms INTEGER,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_widgets_user_id ON widgets(user_id);
CREATE INDEX idx_widgets_type ON widgets(widget_type);
CREATE INDEX idx_widgets_enabled ON widgets(is_enabled);
CREATE INDEX idx_widgets_last_updated ON widgets(last_updated);
CREATE INDEX idx_widgets_refresh_needed ON widgets(last_updated, refresh_interval_minutes) WHERE is_enabled = true;

CREATE INDEX idx_widget_dashboards_user_id ON widget_dashboards(user_id);
CREATE INDEX idx_widget_dashboards_default ON widget_dashboards(user_id, is_default);

CREATE INDEX idx_widget_data_cache_key ON widget_data_cache(cache_key);
CREATE INDEX idx_widget_data_cache_expires ON widget_data_cache(expires_at);
CREATE INDEX idx_widget_data_cache_type ON widget_data_cache(widget_type);

CREATE INDEX idx_default_dhikr_category ON default_dhikr_content(category);
CREATE INDEX idx_default_dhikr_active ON default_dhikr_content(is_active);
CREATE INDEX idx_default_dhikr_order ON default_dhikr_content(category, order_index);

CREATE INDEX idx_widget_refresh_log_widget_id ON widget_refresh_log(widget_id);
CREATE INDEX idx_widget_refresh_log_status ON widget_refresh_log(refresh_status);
CREATE INDEX idx_widget_refresh_log_created ON widget_refresh_log(created_at);

-- Constraints
ALTER TABLE widget_dashboards ADD CONSTRAINT unique_user_default_dashboard 
    EXCLUDE (user_id WITH =) WHERE (is_default = true);

-- Triggers for updated_at timestamps
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_widgets_updated_at BEFORE UPDATE ON widgets
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_widget_dashboards_updated_at BEFORE UPDATE ON widget_dashboards
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_widget_data_cache_updated_at BEFORE UPDATE ON widget_data_cache
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- Insert default dhikr content
INSERT INTO default_dhikr_content (category, title, arabic_text, transliteration, translation_en, translation_ar, reference, repetitions, order_index) VALUES
-- Morning dhikr
('morning', 'Morning Sovereignty', 'أَصْبَحْنَا وَأَصْبَحَ الْمُلْكُ لِلَّهِ، وَالْحَمْدُ لِلَّهِ', 'Asbahna wa asbahal-mulku lillah, walhamdu lillah', 'We have reached the morning and with it Allah''s sovereignty, and praise is to Allah', 'أصبحنا وأصبح الملك لله، والحمد لله', 'Abu Dawud 5077', 1, 1),
('morning', 'Morning Protection', 'اللَّهُمَّ بِكَ أَصْبَحْنَا وَبِكَ أَمْسَيْنَا وَبِكَ نَحْيَا وَبِكَ نَمُوتُ وَإِلَيْكَ النُّشُورُ', 'Allahumma bika asbahna wa bika amsayna wa bika nahya wa bika namutu wa ilaykan-nushur', 'O Allah, by You we have reached the morning and by You we reach the evening, by You we live and by You we die, and to You is the resurrection', 'اللهم بك أصبحنا وبك أمسينا وبك نحيا وبك نموت وإليك النشور', 'Tirmidhi 3391', 1, 2),
('morning', 'Seeking Allah''s Protection', 'أَعُوذُ بِاللَّهِ مِنَ الشَّيْطَانِ الرَّجِيمِ', 'A''udhu billahi min ash-shaytanir-rajim', 'I seek refuge in Allah from Satan, the accursed', 'أعوذ بالله من الشيطان الرجيم', 'Quran', 3, 3),

-- Evening dhikr
('evening', 'Evening Sovereignty', 'أَمْسَيْنَا وَأَمْسَى الْمُلْكُ لِلَّهِ، وَالْحَمْدُ لِلَّهِ', 'Amsayna wa amsal-mulku lillah, walhamdu lillah', 'We have reached the evening and with it Allah''s sovereignty, and praise is to Allah', 'أمسينا وأمسى الملك لله، والحمد لله', 'Abu Dawud 5077', 1, 1),
('evening', 'Evening Protection', 'اللَّهُمَّ بِكَ أَمْسَيْنَا وَبِكَ أَصْبَحْنَا وَبِكَ نَحْيَا وَبِكَ نَمُوتُ وَإِلَيْكَ الْمَصِيرُ', 'Allahumma bika amsayna wa bika asbahna wa bika nahya wa bika namutu wa ilaykal-masir', 'O Allah, by You we have reached the evening and by You we reach the morning, by You we live and by You we die, and to You is the final destination', 'اللهم بك أمسينا وبك أصبحنا وبك نحيا وبك نموت وإليك المصير', 'Tirmidhi 3391', 1, 2),

-- After prayer dhikr
('after_prayer', 'Tasbih after Prayer', 'سُبْحَانَ اللَّهِ', 'Subhan Allah', 'Glory is to Allah', 'سبحان الله', 'Sahih Muslim 596', 33, 1),
('after_prayer', 'Tahmid after Prayer', 'الْحَمْدُ لِلَّهِ', 'Alhamdu lillah', 'Praise is to Allah', 'الحمد لله', 'Sahih Muslim 596', 33, 2),
('after_prayer', 'Takbir after Prayer', 'اللَّهُ أَكْبَرُ', 'Allahu akbar', 'Allah is the Greatest', 'الله أكبر', 'Sahih Muslim 596', 34, 3),
('after_prayer', 'Ayat al-Kursi', 'اللَّهُ لَا إِلَٰهَ إِلَّا هُوَ الْحَيُّ الْقَيُّومُ', 'Allahu la ilaha illa huwal-hayyul-qayyum', 'Allah - there is no deity except Him, the Ever-Living, the Sustainer of existence', 'الله لا إله إلا هو الحي القيوم', 'Quran 2:255', 1, 4),

-- Before sleep dhikr
('before_sleep', 'Before Sleep Protection', 'بِاسْمِكَ اللَّهُمَّ أَمُوتُ وَأَحْيَا', 'Bismika Allahumma amutu wa ahya', 'In Your name, O Allah, I die and I live', 'باسمك اللهم أموت وأحيا', 'Sahih Bukhari 6312', 1, 1),
('before_sleep', 'Seeking Forgiveness', 'أَسْتَغْفِرُ اللَّهَ الَّذِي لَا إِلَٰهَ إِلَّا هُوَ الْحَيَّ الْقَيُّومَ وَأَتُوبُ إِلَيْهِ', 'Astaghfirullaha alladhi la ilaha illa huwal-hayyul-qayyumu wa atubu ilayh', 'I seek forgiveness from Allah, besides whom there is no deity, the Ever-Living, the Sustainer, and I repent to Him', 'أستغفر الله الذي لا إله إلا هو الحي القيوم وأتوب إليه', 'Abu Dawud 1517', 3, 2),

-- After wudu dhikr
('after_wudu', 'Shahada after Wudu', 'أَشْهَدُ أَنْ لَا إِلَٰهَ إِلَّا اللَّهُ وَحْدَهُ لَا شَرِيكَ لَهُ، وَأَشْهَدُ أَنَّ مُحَمَّدًا عَبْدُهُ وَرَسُولُهُ', 'Ashhadu an la ilaha illallahu wahdahu la sharika lah, wa ashhadu anna Muhammadan abduhu wa rasuluh', 'I bear witness that there is no deity except Allah alone, without partner, and I bear witness that Muhammad is His servant and messenger', 'أشهد أن لا إله إلا الله وحده لا شريك له، وأشهد أن محمداً عبده ورسوله', 'Sahih Muslim 234', 1, 1),

-- Travel dhikr
('travel', 'Travel Takbir', 'اللَّهُ أَكْبَرُ اللَّهُ أَكْبَرُ اللَّهُ أَكْبَرُ', 'Allahu akbar, Allahu akbar, Allahu akbar', 'Allah is the Greatest, Allah is the Greatest, Allah is the Greatest', 'الله أكبر الله أكبر الله أكبر', 'Sahih Muslim 1342', 1, 1),
('travel', 'Travel Dua', 'سُبْحَانَ الَّذِي سَخَّرَ لَنَا هَٰذَا وَمَا كُنَّا لَهُ مُقْرِنِينَ', 'Subhanal-ladhi sakhkhara lana hadha wa ma kunna lahu muqrinin', 'Glory is to Him who has subjected this to us, and we could not have subdued it', 'سبحان الذي سخر لنا هذا وما كنا له مقرنين', 'Abu Dawud 2602', 1, 2),

-- General dhikr
('general', 'General Tasbih', 'سُبْحَانَ اللَّهِ وَبِحَمْدِهِ', 'Subhan Allah wa bihamdihi', 'Glory is to Allah and praise is to Him', 'سبحان الله وبحمده', 'Sahih Bukhari 6406', 100, 1),
('general', 'Istighfar', 'أَسْتَغْفِرُ اللَّهَ', 'Astaghfirullah', 'I seek forgiveness from Allah', 'أستغفر الله', 'Various Hadith', 100, 2),
('general', 'Salawat on Prophet', 'اللَّهُمَّ صَلِّ عَلَىٰ مُحَمَّدٍ وَعَلَىٰ آلِ مُحَمَّدٍ', 'Allahumma salli ala Muhammad wa ala ali Muhammad', 'O Allah, send prayers upon Muhammad and upon the family of Muhammad', 'اللهم صل على محمد وعلى آل محمد', 'Sahih Bukhari 3370', 10, 3);

-- Create function to clean expired cache entries
CREATE OR REPLACE FUNCTION clean_expired_widget_cache()
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER;
BEGIN
    DELETE FROM widget_data_cache WHERE expires_at < NOW();
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

-- Create function to get widgets needing refresh
CREATE OR REPLACE FUNCTION get_widgets_needing_refresh(limit_count INTEGER DEFAULT 100)
RETURNS TABLE (
    widget_id UUID,
    user_id UUID,
    widget_type widget_type,
    last_updated TIMESTAMP WITH TIME ZONE,
    refresh_interval_minutes INTEGER
) AS $$
BEGIN
    RETURN QUERY
    SELECT 
        w.id,
        w.user_id,
        w.widget_type,
        w.last_updated,
        w.refresh_interval_minutes
    FROM widgets w
    WHERE w.is_enabled = true 
    AND w.last_updated < NOW() - INTERVAL '1 minute' * w.refresh_interval_minutes
    ORDER BY w.last_updated ASC
    LIMIT limit_count;
END;
$$ LANGUAGE plpgsql;

-- Create function to update widget refresh status
CREATE OR REPLACE FUNCTION update_widget_refresh_status(
    p_widget_id UUID,
    p_status VARCHAR(50),
    p_error_message TEXT DEFAULT NULL,
    p_data_size_bytes INTEGER DEFAULT NULL,
    p_response_time_ms INTEGER DEFAULT NULL
)
RETURNS VOID AS $$
BEGIN
    -- Update the widget's last_updated timestamp
    UPDATE widgets 
    SET last_updated = NOW() 
    WHERE id = p_widget_id;
    
    -- Insert refresh log entry
    INSERT INTO widget_refresh_log (
        widget_id, 
        refresh_completed_at, 
        refresh_status, 
        error_message, 
        data_size_bytes, 
        response_time_ms
    ) VALUES (
        p_widget_id, 
        NOW(), 
        p_status, 
        p_error_message, 
        p_data_size_bytes, 
        p_response_time_ms
    );
END;
$$ LANGUAGE plpgsql;

-- Comments for documentation
COMMENT ON TABLE widgets IS 'Main table storing user widget configurations and metadata';
COMMENT ON TABLE widget_dashboards IS 'User dashboard layouts containing multiple widgets';
COMMENT ON TABLE widget_data_cache IS 'Cache table for storing widget data from external services';
COMMENT ON TABLE default_dhikr_content IS 'Default dhikr content for dhikr reminder widgets';
COMMENT ON TABLE widget_refresh_log IS 'Log table tracking widget refresh operations and performance';

COMMENT ON COLUMN widgets.layout IS 'JSON object containing widget position and size information';
COMMENT ON COLUMN widgets.configuration IS 'JSON object containing widget-specific configuration options';
COMMENT ON COLUMN widgets.refresh_interval_minutes IS 'How often the widget data should be refreshed in minutes';

COMMENT ON COLUMN widget_dashboards.layout_config IS 'JSON object containing grid layout configuration';
COMMENT ON COLUMN widget_dashboards.widgets IS 'JSON array of widget IDs in display order';

COMMENT ON FUNCTION clean_expired_widget_cache() IS 'Removes expired entries from widget data cache';
COMMENT ON FUNCTION get_widgets_needing_refresh(INTEGER) IS 'Returns widgets that need data refresh based on their interval';
COMMENT ON FUNCTION update_widget_refresh_status(UUID, VARCHAR, TEXT, INTEGER, INTEGER) IS 'Updates widget refresh status and logs the operation';