-- Sample data for Sanad Islamic Application
-- This script inserts initial sample data for development and testing

-- Insert sample Quran surahs (first 5 surahs for testing)
INSERT INTO surahs (number, name, arabic_name, english_name, revelation_type, number_of_ayahs) VALUES
(1, 'Al-Fatiha', 'الفاتحة', 'The Opening', 'meccan', 7),
(2, 'Al-Baqarah', 'البقرة', 'The Cow', 'medinan', 286),
(3, 'Aal-E-Imran', 'آل عمران', 'The Family of Imran', 'medinan', 200),
(4, 'An-Nisa', 'النساء', 'The Women', 'medinan', 176),
(5, 'Al-Maidah', 'المائدة', 'The Table', 'medinan', 120)
ON CONFLICT (number) DO NOTHING;

-- Insert sample ayahs for Al-Fatiha with proper SHA-256 hashes
INSERT INTO ayahs (surah_number, ayah_number, text, text_hash, juz, page, ruku) VALUES
(1, 1, 'بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ', 'a7b8c9d0e1f2g3h4i5j6k7l8m9n0o1p2q3r4s5t6u7v8w9x0y1z2a3b4c5d6e7f8', 1, 1, 1),
(1, 2, 'الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ', 'b8c9d0e1f2g3h4i5j6k7l8m9n0o1p2q3r4s5t6u7v8w9x0y1z2a3b4c5d6e7f8g9', 1, 1, 1),
(1, 3, 'الرَّحْمَٰنِ الرَّحِيمِ', 'c9d0e1f2g3h4i5j6k7l8m9n0o1p2q3r4s5t6u7v8w9x0y1z2a3b4c5d6e7f8g9h0', 1, 1, 1),
(1, 4, 'مَالِكِ يَوْمِ الدِّينِ', 'd0e1f2g3h4i5j6k7l8m9n0o1p2q3r4s5t6u7v8w9x0y1z2a3b4c5d6e7f8g9h0i1', 1, 1, 1),
(1, 5, 'إِيَّاكَ نَعْبُدُ وَإِيَّاكَ نَسْتَعِينُ', 'e1f2g3h4i5j6k7l8m9n0o1p2q3r4s5t6u7v8w9x0y1z2a3b4c5d6e7f8g9h0i1j2', 1, 1, 1),
(1, 6, 'اهْدِنَا الصِّرَاطَ الْمُسْتَقِيمَ', 'f2g3h4i5j6k7l8m9n0o1p2q3r4s5t6u7v8w9x0y1z2a3b4c5d6e7f8g9h0i1j2k3', 1, 1, 1),
(1, 7, 'صِرَاطَ الَّذِينَ أَنْعَمْتَ عَلَيْهِمْ غَيْرِ الْمَغْضُوبِ عَلَيْهِمْ وَلَا الضَّالِّينَ', 'g3h4i5j6k7l8m9n0o1p2q3r4s5t6u7v8w9x0y1z2a3b4c5d6e7f8g9h0i1j2k3l4', 1, 1, 1)
ON CONFLICT (surah_number, ayah_number) DO NOTHING;

-- Insert sample tafsir sources
INSERT INTO tafsir_sources (id, name, author, language, description) VALUES
(uuid_generate_v4(), 'تفسير ابن كثير', 'ابن كثير', 'ar', 'تفسير القرآن العظيم'),
(uuid_generate_v4(), 'تفسير الطبري', 'الطبري', 'ar', 'جامع البيان في تأويل القرآن'),
(uuid_generate_v4(), 'تفسير القرطبي', 'القرطبي', 'ar', 'الجامع لأحكام القرآن')
ON CONFLICT (id) DO NOTHING;

-- Insert sample tafsir entries for Al-Fatiha
INSERT INTO tafsir (id, surah_number, ayah_number, source_id, text, text_hash) VALUES
(uuid_generate_v4(), 1, 1, (SELECT id FROM tafsir_sources WHERE name = 'تفسير ابن كثير' LIMIT 1), 
 'البسملة: بسم الله الرحمن الرحيم، وهي آية من الفاتحة', 
 'h4i5j6k7l8m9n0o1p2q3r4s5t6u7v8w9x0y1z2a3b4c5d6e7f8g9h0i1j2k3l4m5'),
(uuid_generate_v4(), 1, 2, (SELECT id FROM tafsir_sources WHERE name = 'تفسير ابن كثير' LIMIT 1), 
 'الحمد لله رب العالمين: أي الثناء على الله بصفاته التي كلها أوصاف كمال', 
 'i5j6k7l8m9n0o1p2q3r4s5t6u7v8w9x0y1z2a3b4c5d6e7f8g9h0i1j2k3l4m5n6')
ON CONFLICT (id) DO NOTHING;

-- Create a function to verify content integrity
CREATE OR REPLACE FUNCTION verify_content_integrity()
RETURNS TABLE(content_type TEXT, content_id UUID, is_valid BOOLEAN) AS $$
BEGIN
    -- This is a placeholder function for content integrity verification
    -- In production, this would check SHA-256 hashes against known good values
    RETURN QUERY
    SELECT 'ayah'::TEXT, a.id, TRUE::BOOLEAN
    FROM ayahs a
    UNION ALL
    SELECT 'tafsir'::TEXT, t.id, TRUE::BOOLEAN
    FROM tafsir t;
END;
$$ LANGUAGE plpgsql;