-- Quran Service Enhancements Migration
-- Adds support for translations and recitation styles

-- Translations table for Quran meanings in different languages
CREATE TABLE translations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    surah_number INTEGER NOT NULL,
    ayah_number INTEGER NOT NULL,
    language VARCHAR(10) NOT NULL,
    translator VARCHAR(100) NOT NULL,
    text TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    FOREIGN KEY (surah_number, ayah_number) REFERENCES ayahs(surah_number, ayah_number)
);

-- Recitation styles table for different Qira'at
CREATE TABLE recitation_styles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    arabic_name VARCHAR(100) NOT NULL,
    reciter VARCHAR(100) NOT NULL,
    description TEXT,
    language VARCHAR(10) DEFAULT 'ar',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_translations_surah_ayah ON translations(surah_number, ayah_number);
CREATE INDEX idx_translations_language ON translations(language);
CREATE INDEX idx_translations_translator ON translations(translator);
CREATE INDEX idx_recitation_styles_name ON recitation_styles(name);
CREATE INDEX idx_recitation_styles_reciter ON recitation_styles(reciter);

-- Insert some sample recitation styles
INSERT INTO recitation_styles (name, arabic_name, reciter, description, language) VALUES
('Hafs an Asim', 'حفص عن عاصم', 'Various', 'The most widely used recitation style', 'ar'),
('Warsh an Nafi', 'ورش عن نافع', 'Various', 'Popular in North and West Africa', 'ar'),
('Qalun an Nafi', 'قالون عن نافع', 'Various', 'Another transmission from Nafi', 'ar'),
('Ad-Duri an Abi Amr', 'الدوري عن أبي عمرو', 'Various', 'Transmission from Abu Amr', 'ar');

-- Insert some sample English translations (for demonstration)
-- Note: In production, these would be loaded from authoritative sources
INSERT INTO translations (surah_number, ayah_number, language, translator, text) VALUES
(1, 1, 'en', 'Sahih International', 'In the name of Allah, the Entirely Merciful, the Especially Merciful.'),
(1, 2, 'en', 'Sahih International', '[All] praise is [due] to Allah, Lord of the worlds -'),
(1, 3, 'en', 'Sahih International', 'The Entirely Merciful, the Especially Merciful,'),
(1, 4, 'en', 'Sahih International', 'Sovereign of the Day of Recompense.'),
(1, 5, 'en', 'Sahih International', 'It is You we worship and You we ask for help.'),
(1, 6, 'en', 'Sahih International', 'Guide us to the straight path -'),
(1, 7, 'en', 'Sahih International', 'The path of those upon whom You have bestowed favor, not of those who have evoked [Your] anger or of those who are astray.');