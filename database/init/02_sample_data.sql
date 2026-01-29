-- Sample data for Sanad Islamic Application
-- This script inserts initial sample data for development and testing

-- Insert sample Quran surahs (first 5 surahs for testing)
INSERT INTO surahs (number, name, arabic_name, english_name, revelation_type, number_of_ayahs) VALUES
(1, 'Al-Fatiha', 'الفاتحة', 'The Opening', 'meccan', 7),
(2, 'Al-Baqarah', 'البقرة', 'The Cow', 'medinan', 286),
(3, 'Aal-E-Imran', 'آل عمران', 'The Family of Imran', 'medinan', 200),
(4, 'An-Nisa', 'النساء', 'The Women', 'medinan', 176),
(5, 'Al-Maidah', 'المائدة', 'The Table', 'medinan', 120);

-- Insert sample ayahs for Al-Fatiha
INSERT INTO ayahs (surah_number, ayah_number, text, text_hash, juz, page, ruku) VALUES
(1, 1, 'بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ', 'a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6a7b8c9d0e1f2', 1, 1, 1),
(1, 2, 'الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ', 'b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6a7b8c9d0e1f2g3', 1, 1, 1),
(1, 3, 'الرَّحْمَٰنِ الرَّحِيمِ', 'c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6a7b8c9d0e1f2g3h4', 1, 1, 1),
(1, 4, 'مَالِكِ يَوْمِ الدِّينِ', 'd4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6a7b8c9d0e1f2g3h4i5', 1, 1, 1),
(1, 5, 'إِيَّاكَ نَعْبُدُ وَإِيَّاكَ نَسْتَعِينُ', 'e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6a7b8c9d0e1f2g3h4i5j6', 1, 1, 1),
(1, 6, 'اهْدِنَا الصِّرَاطَ الْمُسْتَقِيمَ', 'f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6a7b8c9d0e1f2g3h4i5j6k7', 1, 1, 1),
(1, 7, 'صِرَاطَ الَّذِينَ أَنْعَمْتَ عَلَيْهِمْ غَيْرِ الْمَغْضُوبِ عَلَيْهِمْ وَلَا الضَّالِّينَ', 'g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6a7b8c9d0e1f2g3h4i5j6k7l8', 1, 1, 1);

-- Insert sample tafsir sources
INSERT INTO tafsir_sources (id, name, author, language, description) VALUES
(uuid_generate_v4(), 'تفسير ابن كثير', 'ابن كثير', 'ar', 'تفسير القرآن العظيم'),
(uuid_generate_v4(), 'تفسير الطبري', 'الطبري', 'ar', 'جامع البيان في تأويل القرآن'),
(uuid_generate_v4(), 'تفسير القرطبي', 'القرطبي', 'ar', 'الجامع لأحكام القرآن');

-- Insert sample hadith books
INSERT INTO hadith_books (id, name, arabic_name, author, description) VALUES
(uuid_generate_v4(), 'Sahih Bukhari', 'صحيح البخاري', 'الإمام البخاري', 'أصح كتاب بعد كتاب الله'),
(uuid_generate_v4(), 'Sahih Muslim', 'صحيح مسلم', 'الإمام مسلم', 'ثاني أصح الكتب بعد البخاري'),
(uuid_generate_v4(), 'Sunan Abu Dawood', 'سنن أبي داود', 'أبو داود', 'من كتب السنن المعتبرة'),
(uuid_generate_v4(), 'Jami at-Tirmidhi', 'جامع الترمذي', 'الترمذي', 'من كتب السنن المشهورة');

-- Insert sample hadiths
INSERT INTO hadiths (id, book_id, hadith_number, chapter, text, text_hash, narrator, chain, grade, source, tags) VALUES
(uuid_generate_v4(), (SELECT id FROM hadith_books WHERE name = 'Sahih Bukhari' LIMIT 1), '1', 'كتاب بدء الوحي', 'إنما الأعمال بالنيات وإنما لكل امرئ ما نوى', 'h1i2j3k4l5m6n7o8p9q0r1s2t3u4v5w6x7y8z9a0b1c2d3e4f5g6h7i8j9k0l1m2', 'عمر بن الخطاب', ARRAY['عمر بن الخطاب', 'علقمة بن وقاص', 'محمد بن إبراهيم'], 'Sahih', 'البخاري', ARRAY['النية', 'الأعمال']),
(uuid_generate_v4(), (SELECT id FROM hadith_books WHERE name = 'Sahih Muslim' LIMIT 1), '1', 'كتاب الإيمان', 'بني الإسلام على خمس: شهادة أن لا إله إلا الله وأن محمداً رسول الله، وإقام الصلاة، وإيتاء الزكاة، وحج البيت، وصوم رمضان', 'i2j3k4l5m6n7o8p9q0r1s2t3u4v5w6x7y8z9a0b1c2d3e4f5g6h7i8j9k0l1m2n3', 'عبد الله بن عمر', ARRAY['عبد الله بن عمر', 'نافع', 'عبيد الله بن عمر'], 'Sahih', 'مسلم', ARRAY['أركان الإسلام', 'الإيمان']);

-- Insert sample Islamic stories
INSERT INTO stories (id, title, category, content, content_hash, characters, lessons, sources, language, tags) VALUES
(uuid_generate_v4(), 'قصة آدم عليه السلام', 'قصص الأنبياء', 'خلق الله آدم من طين ونفخ فيه من روحه...', 'j3k4l5m6n7o8p9q0r1s2t3u4v5w6x7y8z9a0b1c2d3e4f5g6h7i8j9k0l1m2n3o4', ARRAY['آدم', 'حواء', 'إبليس'], ARRAY['طاعة الله', 'التوبة'], ARRAY['القرآن الكريم'], 'ar', ARRAY['أنبياء', 'خلق']),
(uuid_generate_v4(), 'قصة أبي بكر الصديق', 'قصص الصحابة', 'كان أبو بكر أول من آمن من الرجال...', 'k4l5m6n7o8p9q0r1s2t3u4v5w6x7y8z9a0b1c2d3e4f5g6h7i8j9k0l1m2n3o4p5', ARRAY['أبو بكر', 'النبي محمد'], ARRAY['الصداقة', 'الإيمان'], ARRAY['السيرة النبوية'], 'ar', ARRAY['صحابة', 'إيمان']);

-- Insert sample Islamic events
INSERT INTO islamic_events (id, name, description, hijri_month, hijri_day, event_type, is_recurring) VALUES
(uuid_generate_v4(), 'عيد الفطر', 'عيد المسلمين بعد انتهاء شهر رمضان', 10, 1, 'Eid', true),
(uuid_generate_v4(), 'عيد الأضحى', 'عيد الأضحى المبارك', 12, 10, 'Eid', true),
(uuid_generate_v4(), 'بداية شهر رمضان', 'بداية شهر الصيام المبارك', 9, 1, 'HolyMonth', true),
(uuid_generate_v4(), 'ليلة القدر', 'ليلة القدر خير من ألف شهر', 9, 27, 'ImportantDay', true),
(uuid_generate_v4(), 'يوم عرفة', 'يوم الحج الأكبر', 12, 9, 'ImportantDay', true);

-- Insert sample admin user (password: admin123)
INSERT INTO users (id, username, email, password_hash, is_active) VALUES
(uuid_generate_v4(), 'admin', 'admin@sanad.app', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewdBPj/RK.PJ/..G', true);

-- Insert user preferences for admin user
INSERT INTO user_preferences (user_id, language, preferred_tafsir, prayer_calculation_method, notification_settings, display_settings) VALUES
((SELECT id FROM users WHERE username = 'admin' LIMIT 1), 'ar', ARRAY['تفسير ابن كثير', 'تفسير الطبري'], 'MuslimWorldLeague', 
'{"prayer_reminders": true, "prayer_reminder_minutes": 15, "islamic_events": true, "khatma_reminders": true, "daily_verse": true}',
'{"theme": "light", "font_size": "medium", "arabic_font": "Uthmanic", "translation_font": "Arial"}');

-- Insert sample bookmark for admin user
INSERT INTO bookmarks (user_id, content_type, content_id, title, notes, folder) VALUES
((SELECT id FROM users WHERE username = 'admin' LIMIT 1), 'quran', (SELECT id FROM ayahs WHERE surah_number = 1 AND ayah_number = 1 LIMIT 1), 'البسملة', 'بداية كل سورة', 'المفضلة');

-- Insert sample reading progress for admin user
INSERT INTO reading_progress (user_id, content_type, content_id, progress_percentage, last_position) VALUES
((SELECT id FROM users WHERE username = 'admin' LIMIT 1), 'quran', (SELECT id FROM surahs WHERE number = 1 LIMIT 1), 100.00, '{"surah": 1, "ayah": 7, "completed": true}');

-- Insert sample khatma plan for admin user
INSERT INTO khatma_plans (user_id, target_date, daily_portions, estimated_reading_time, current_progress) VALUES
((SELECT id FROM users WHERE username = 'admin' LIMIT 1), CURRENT_DATE + INTERVAL '30 days', 
'[{"date": "2024-01-01", "surah_start": 1, "ayah_start": 1, "surah_end": 1, "ayah_end": 7, "estimated_minutes": 5}]', 
30, 5.00);

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
    SELECT 'hadith'::TEXT, h.id, TRUE::BOOLEAN
    FROM hadiths h
    UNION ALL
    SELECT 'story'::TEXT, s.id, TRUE::BOOLEAN
    FROM stories s;
END;
$$ LANGUAGE plpgsql;