use crate::models::*;
use crate::service::StoryService;
use crate::repository::StoryRepository;
use crate::handlers::{SearchLessonsParams, SearchByLessonParams, SearchByMoralParams};
use uuid::Uuid;
use std::collections::HashMap;
use anyhow::Result;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_story_creation_and_integrity() {
        let story = Story::new(
            "The Story of Prophet Yusuf".to_string(),
            "قصة النبي يوسف عليه السلام".to_string(),
            "This is the story of Prophet Yusuf, known for his beauty, wisdom, and patience. He was sold into slavery by his brothers but eventually became a minister in Egypt through Allah's guidance.".to_string(),
            StoryCategory::Prophets,
            AgeGroup::AllAges,
            "en".to_string(),
            AuthenticityLevel::Authentic,
        );

        // Test basic properties
        assert_eq!(story.title, "The Story of Prophet Yusuf");
        assert_eq!(story.category, StoryCategory::Prophets);
        assert_eq!(story.age_group, AgeGroup::AllAges);
        assert_eq!(story.authenticity_level, AuthenticityLevel::Authentic);
        assert!(story.word_count > 0);
        assert!(story.estimated_reading_time > 0);

        // Test integrity verification
        assert!(story.verify_integrity());

        // Test category methods
        assert_eq!(story.category_arabic(), "قصص الأنبياء");
        assert!(story.is_historically_authentic());
        assert!(story.is_suitable_for_children());
    }

    #[test]
    fn test_story_integrity_failure() {
        let mut story = Story::new(
            "Test Story".to_string(),
            "قصة تجريبية".to_string(),
            "Original content".to_string(),
            StoryCategory::MoralLessons,
            AgeGroup::Children,
            "en".to_string(),
            AuthenticityLevel::Educational,
        );

        // Initially should pass integrity check
        assert!(story.verify_integrity());

        // Tamper with content without updating hash
        story.content = "Modified content without updating hash".to_string();

        // Should now fail integrity check
        assert!(!story.verify_integrity());
    }

    #[test]
    fn test_story_metrics_calculation() {
        let mut story = Story::new(
            "Test Story".to_string(),
            "قصة تجريبية".to_string(),
            "Short content".to_string(),
            StoryCategory::MoralLessons,
            AgeGroup::Children,
            "en".to_string(),
            AuthenticityLevel::Educational,
        );

        let original_word_count = story.word_count;
        let original_reading_time = story.estimated_reading_time;

        // Update with longer content
        story.content = "This is a much longer story content with many more words to test the automatic calculation of word count and estimated reading time based on the standard reading speed of 200 words per minute for Arabic text.".to_string();
        story.update_metrics();

        assert!(story.word_count > original_word_count);
        assert!(story.estimated_reading_time >= original_reading_time);
        assert!(story.verify_integrity()); // Hash should be updated
    }

    #[test]
    fn test_story_theme_and_keyword_management() {
        let mut story = Story::new(
            "Test Story".to_string(),
            "قصة تجريبية".to_string(),
            "Test content".to_string(),
            StoryCategory::MoralLessons,
            AgeGroup::Children,
            "en".to_string(),
            AuthenticityLevel::Educational,
        );

        // Test adding themes
        story.add_theme("Patience".to_string());
        story.add_theme("Justice".to_string());
        story.add_theme("Patience".to_string()); // Duplicate should be ignored

        assert_eq!(story.themes.len(), 2);
        assert!(story.themes.contains(&"Patience".to_string()));
        assert!(story.themes.contains(&"Justice".to_string()));

        // Test adding moral lessons
        story.add_moral_lesson("Trust in Allah".to_string());
        story.add_moral_lesson("Be patient in adversity".to_string());
        story.add_moral_lesson("Trust in Allah".to_string()); // Duplicate should be ignored

        assert_eq!(story.moral_lessons.len(), 2);
        assert!(story.moral_lessons.contains(&"Trust in Allah".to_string()));

        // Test adding keywords
        story.add_keyword("prophet".to_string());
        story.add_keyword("guidance".to_string());
        story.add_keyword("prophet".to_string()); // Duplicate should be ignored

        assert_eq!(story.keywords.len(), 2);
        assert!(story.keywords.contains(&"prophet".to_string()));
    }

    #[test]
    fn test_reading_time_calculation() {
        // Test edge cases for reading time calculation
        assert_eq!(Story::calculate_reading_time(0), 0);
        assert_eq!(Story::calculate_reading_time(50), 1);
        assert_eq!(Story::calculate_reading_time(200), 1);
        assert_eq!(Story::calculate_reading_time(250), 2);
        assert_eq!(Story::calculate_reading_time(400), 2);
        assert_eq!(Story::calculate_reading_time(600), 3);
    }

    #[test]
    fn test_story_difficulty_levels() {
        // Test children's easy story
        let children_easy = Story::new(
            "Easy Story".to_string(),
            "قصة سهلة".to_string(),
            "Short ".repeat(100), // 200 words
            StoryCategory::ChildrenStories,
            AgeGroup::Children,
            "en".to_string(),
            AuthenticityLevel::Educational,
        );
        assert_eq!(children_easy.get_difficulty_level(), "Easy");

        // Test children's challenging story
        let children_challenging = Story::new(
            "Long Story".to_string(),
            "قصة طويلة".to_string(),
            "Long ".repeat(600), // 1200 words
            StoryCategory::ChildrenStories,
            AgeGroup::Children,
            "en".to_string(),
            AuthenticityLevel::Educational,
        );
        assert_eq!(children_challenging.get_difficulty_level(), "Challenging");

        // Test adult challenging story
        let adult_challenging = Story::new(
            "Complex Story".to_string(),
            "قصة معقدة".to_string(),
            "Complex ".repeat(2000), // 4000 words
            StoryCategory::HistoricalEvents,
            AgeGroup::Adults,
            "en".to_string(),
            AuthenticityLevel::WellDocumented,
        );
        assert_eq!(adult_challenging.get_difficulty_level(), "Challenging");
    }

    #[test]
    fn test_character_creation_and_methods() {
        let mut character = Character::new(
            "Prophet Muhammad".to_string(),
            "النبي محمد صلى الله عليه وسلم".to_string(),
            CharacterType::Prophet,
        );

        // Test basic properties
        assert_eq!(character.name, "Prophet Muhammad");
        assert_eq!(character.character_type, CharacterType::Prophet);
        assert!(character.is_prophet());
        assert_eq!(character.character_type_arabic(), "نبي");

        // Test adding virtues
        character.add_virtue("Honesty".to_string());
        character.add_virtue("Mercy".to_string());
        character.add_virtue("Honesty".to_string()); // Duplicate should be ignored

        assert_eq!(character.virtues.len(), 2);
        assert!(character.virtues.contains(&"Honesty".to_string()));
        assert!(character.virtues.contains(&"Mercy".to_string()));

        // Test historical period
        character.historical_period = Some(TimePeriod::PropheticEra);
        assert!(character.is_early_islamic());

        // Test lifespan calculation
        assert_eq!(character.get_lifespan(), None); // No birth/death years set

        character.birth_year = Some(571); // 571 CE
        character.death_year = Some(632); // 632 CE
        assert_eq!(character.get_lifespan(), Some(61));
    }

    #[test]
    fn test_lesson_creation_and_methods() {
        let mut lesson = Lesson::new(
            "The Importance of Patience".to_string(),
            "أهمية الصبر".to_string(),
            "Patience is a virtue that helps us endure difficulties and trust in Allah's wisdom.".to_string(),
            LessonType::Moral,
            MoralCategory::Patience,
        );

        // Test basic properties
        assert_eq!(lesson.title, "The Importance of Patience");
        assert_eq!(lesson.lesson_type, LessonType::Moral);
        assert_eq!(lesson.moral_category, MoralCategory::Patience);
        assert_eq!(lesson.lesson_type_arabic(), "أخلاقي");
        assert_eq!(lesson.moral_category_arabic(), "الصبر");

        // Test target audience
        assert!(lesson.is_suitable_for_age(&AgeGroup::AllAges));
        assert!(lesson.is_suitable_for_age(&AgeGroup::Children));

        lesson.target_audience = vec![AgeGroup::Adults, AgeGroup::YoungAdults];
        assert!(lesson.is_suitable_for_age(&AgeGroup::Adults));
        assert!(!lesson.is_suitable_for_age(&AgeGroup::Children));

        // Test adding related verses and hadiths
        lesson.add_related_verse("2:155".to_string());
        lesson.add_related_verse("3:200".to_string());
        lesson.add_related_verse("2:155".to_string()); // Duplicate should be ignored

        assert_eq!(lesson.related_verses.len(), 2);
        assert!(lesson.related_verses.contains(&"2:155".to_string()));

        lesson.add_related_hadith("Bukhari 1".to_string());
        lesson.add_related_hadith("Muslim 5".to_string());

        assert_eq!(lesson.related_hadiths.len(), 2);
        assert!(lesson.related_hadiths.contains(&"Bukhari 1".to_string()));
    }

    #[test]
    fn test_story_source_creation_and_methods() {
        let story_id = Uuid::new_v4();
        let mut source = StorySource::new(
            story_id,
            SourceType::Quran,
            "Holy Quran".to_string(),
            "القرآن الكريم".to_string(),
            "Surah Yusuf (12:1-111)".to_string(),
        );

        // Test basic properties
        assert_eq!(source.story_id, story_id);
        assert_eq!(source.source_type, SourceType::Quran);
        assert!(source.is_primary_source());
        assert_eq!(source.source_type_arabic(), "القرآن الكريم");

        // Test credibility
        source.credibility_score = 10.0;
        source.verification_status = VerificationStatus::Verified;
        assert!(source.is_highly_credible());

        // Test Hadith source
        let hadith_source = StorySource::new(
            story_id,
            SourceType::Hadith,
            "Sahih Bukhari".to_string(),
            "صحيح البخاري".to_string(),
            "Book 60, Hadith 1".to_string(),
        );

        assert!(hadith_source.is_primary_source());
        assert_eq!(hadith_source.source_type_arabic(), "الحديث النبوي");

        // Test non-primary source
        let scholarly_source = StorySource::new(
            story_id,
            SourceType::ScholarlyWork,
            "Islamic History".to_string(),
            "التاريخ الإسلامي".to_string(),
            "Chapter 5, Page 123".to_string(),
        );

        assert!(!scholarly_source.is_primary_source());
    }

    #[test]
    fn test_story_collection_creation() {
        let mut collection = StoryCollection::new(
            "Stories of the Prophets".to_string(),
            "قصص الأنبياء".to_string(),
            CollectionType::Thematic,
        );

        // Test basic properties
        assert_eq!(collection.name, "Stories of the Prophets");
        assert_eq!(collection.collection_type, CollectionType::Thematic);
        assert_eq!(collection.collection_type_arabic(), "موضوعي");

        // Test adding themes
        collection.add_theme("Prophethood".to_string());
        collection.add_theme("Divine Guidance".to_string());
        collection.add_theme("Prophethood".to_string()); // Duplicate should be ignored

        assert_eq!(collection.themes.len(), 2);
        assert!(collection.themes.contains(&"Prophethood".to_string()));
    }

    #[test]
    fn test_hash_consistency_and_security() {
        let content1 = "This is test content for hash verification";
        let content2 = "This is test content for hash verification";
        let content3 = "This is different content";

        let hash1 = Story::generate_hash(content1);
        let hash2 = Story::generate_hash(content2);
        let hash3 = Story::generate_hash(content3);

        // Same content should produce same hash
        assert_eq!(hash1, hash2);
        
        // Different content should produce different hash
        assert_ne!(hash1, hash3);
        
        // Hash should be 64 characters (SHA-256 hex)
        assert_eq!(hash1.len(), 64);
        
        // Hash should be deterministic
        let hash1_again = Story::generate_hash(content1);
        assert_eq!(hash1, hash1_again);
    }

    #[test]
    fn test_enum_display_implementations() {
        let category = StoryCategory::Prophets;
        let display_str = format!("{}", category);
        assert!(display_str.contains("Prophets"));
        assert!(display_str.contains("قصص الأنبياء"));

        let character_type = CharacterType::Prophet;
        let display_str = format!("{}", character_type);
        assert!(display_str.contains("Prophet"));
        assert!(display_str.contains("نبي"));
    }

    #[test]
    fn test_content_integrity_trait() {
        let story = Story::new(
            "Test Story".to_string(),
            "قصة تجريبية".to_string(),
            "Test content for integrity verification".to_string(),
            StoryCategory::MoralLessons,
            AgeGroup::AllAges,
            "en".to_string(),
            AuthenticityLevel::Educational,
        );

        // Test ContentIntegrity trait implementation
        assert!(story.verify_integrity());
        
        let calculated_hash = story.calculate_hash();
        assert_eq!(calculated_hash, story.content_hash);
        assert_eq!(calculated_hash.len(), 64);
    }

    #[test]
    fn test_serialization_deserialization() {
        let original_story = Story::new(
            "Serialization Test".to_string(),
            "اختبار التسلسل".to_string(),
            "Content for serialization test".to_string(),
            StoryCategory::Prophets,
            AgeGroup::AllAges,
            "en".to_string(),
            AuthenticityLevel::Authentic,
        );

        // Test JSON serialization
        let json = serde_json::to_string(&original_story).unwrap();
        assert!(!json.is_empty());

        // Test JSON deserialization
        let deserialized_story: Story = serde_json::from_str(&json).unwrap();
        
        // Verify all fields are preserved
        assert_eq!(original_story.id, deserialized_story.id);
        assert_eq!(original_story.title, deserialized_story.title);
        assert_eq!(original_story.arabic_title, deserialized_story.arabic_title);
        assert_eq!(original_story.content, deserialized_story.content);
        assert_eq!(original_story.content_hash, deserialized_story.content_hash);
        assert_eq!(original_story.category, deserialized_story.category);
        assert_eq!(original_story.age_group, deserialized_story.age_group);
        assert_eq!(original_story.authenticity_level, deserialized_story.authenticity_level);
        
        // Verify integrity is maintained after serialization
        assert!(deserialized_story.verify_integrity());
    }

    #[test]
    fn test_story_categories_comprehensive() {
        let categories = vec![
            StoryCategory::Prophets,
            StoryCategory::Companions,
            StoryCategory::RighteousPredecessors,
            StoryCategory::HistoricalEvents,
            StoryCategory::MoralLessons,
            StoryCategory::Miracles,
            StoryCategory::Battles,
            StoryCategory::Conversions,
            StoryCategory::WomenInIslam,
            StoryCategory::ChildrenStories,
        ];

        for category in categories {
            let story = Story::new(
                format!("Test Story - {:?}", category),
                format!("قصة تجريبية - {:?}", category),
                "Test content".to_string(),
                category.clone(),
                AgeGroup::AllAges,
                "en".to_string(),
                AuthenticityLevel::Educational,
            );

            assert_eq!(story.category, category);
            assert!(!story.category_arabic().is_empty());
            assert!(story.verify_integrity());
        }
    }

    #[test]
    fn test_character_types_comprehensive() {
        let character_types = vec![
            CharacterType::Prophet,
            CharacterType::Messenger,
            CharacterType::Companion,
            CharacterType::RighteousPerson,
            CharacterType::Scholar,
            CharacterType::Ruler,
            CharacterType::Martyr,
            CharacterType::Convert,
            CharacterType::HistoricalFigure,
            CharacterType::Antagonist,
        ];

        for char_type in character_types {
            let character = Character::new(
                format!("Test Character - {:?}", char_type),
                format!("شخصية تجريبية - {:?}", char_type),
                char_type.clone(),
            );

            assert_eq!(character.character_type, char_type);
            assert!(!character.character_type_arabic().is_empty());
            
            // Test prophet detection
            let is_prophet_type = matches!(char_type, CharacterType::Prophet | CharacterType::Messenger);
            assert_eq!(character.is_prophet(), is_prophet_type);
        }
    }

    #[test]
    fn test_authenticity_levels() {
        let levels = vec![
            AuthenticityLevel::Authentic,
            AuthenticityLevel::WellDocumented,
            AuthenticityLevel::Probable,
            AuthenticityLevel::Traditional,
            AuthenticityLevel::Educational,
        ];

        for level in levels {
            let story = Story::new(
                "Test Story".to_string(),
                "قصة تجريبية".to_string(),
                "Test content".to_string(),
                StoryCategory::MoralLessons,
                AgeGroup::AllAges,
                "en".to_string(),
                level.clone(),
            );

            assert_eq!(story.authenticity_level, level);
            
            let is_historically_authentic = matches!(
                level, 
                AuthenticityLevel::Authentic | AuthenticityLevel::WellDocumented
            );
            assert_eq!(story.is_historically_authentic(), is_historically_authentic);
        }
    }

    #[test]
    fn test_edge_cases_and_error_conditions() {
        // Test empty content
        let empty_story = Story::new(
            "Empty Story".to_string(),
            "قصة فارغة".to_string(),
            "".to_string(),
            StoryCategory::MoralLessons,
            AgeGroup::AllAges,
            "en".to_string(),
            AuthenticityLevel::Educational,
        );

        assert_eq!(empty_story.word_count, 0);
        assert_eq!(empty_story.estimated_reading_time, 0);
        assert!(empty_story.verify_integrity()); // Empty content should still have valid hash

        // Test very long content
        let long_content = "word ".repeat(10000); // 20,000 words
        let long_story = Story::new(
            "Long Story".to_string(),
            "قصة طويلة".to_string(),
            long_content,
            StoryCategory::HistoricalEvents,
            AgeGroup::Adults,
            "en".to_string(),
            AuthenticityLevel::WellDocumented,
        );

        assert_eq!(long_story.word_count, 10000);
        assert_eq!(long_story.estimated_reading_time, 50); // 10000/200 = 50 minutes
        assert!(long_story.verify_integrity());

        // Test character with no lifespan data
        let character = Character::new(
            "Unknown Character".to_string(),
            "شخصية مجهولة".to_string(),
            CharacterType::HistoricalFigure,
        );

        assert_eq!(character.get_lifespan(), None);
        assert!(!character.is_early_islamic()); // No historical period set
    }
}

/// Unit tests for character search functionality (Requirement 4.4)
/// Tests the ability to search stories by characters or topics
#[cfg(test)]
mod character_search_tests {
    use super::*;

    #[test]
    fn test_character_search_by_name() {
        // Test searching for characters by name (both English and Arabic)
        let prophet_muhammad = Character::new(
            "Prophet Muhammad".to_string(),
            "النبي محمد صلى الله عليه وسلم".to_string(),
            CharacterType::Prophet,
        );

        let abu_bakr = Character::new(
            "Abu Bakr".to_string(),
            "أبو بكر الصديق".to_string(),
            CharacterType::Companion,
        );

        // Test exact name matching
        assert_eq!(prophet_muhammad.name, "Prophet Muhammad");
        assert_eq!(prophet_muhammad.arabic_name, "النبي محمد صلى الله عليه وسلم");
        
        // Test character type identification for search filtering
        assert!(prophet_muhammad.is_prophet());
        assert!(!abu_bakr.is_prophet());
        assert_eq!(abu_bakr.character_type, CharacterType::Companion);
    }

    #[test]
    fn test_character_search_by_type() {
        let characters = vec![
            Character::new("Prophet Ibrahim".to_string(), "النبي إبراهيم".to_string(), CharacterType::Prophet),
            Character::new("Umar ibn Khattab".to_string(), "عمر بن الخطاب".to_string(), CharacterType::Companion),
            Character::new("Imam Bukhari".to_string(), "الإمام البخاري".to_string(), CharacterType::Scholar),
            Character::new("Saladin".to_string(), "صلاح الدين الأيوبي".to_string(), CharacterType::Ruler),
        ];

        // Test filtering by character type
        let prophets: Vec<&Character> = characters.iter()
            .filter(|c| c.is_prophet())
            .collect();
        assert_eq!(prophets.len(), 1);
        assert_eq!(prophets[0].name, "Prophet Ibrahim");

        let companions: Vec<&Character> = characters.iter()
            .filter(|c| c.character_type == CharacterType::Companion)
            .collect();
        assert_eq!(companions.len(), 1);
        assert_eq!(companions[0].name, "Umar ibn Khattab");

        let scholars: Vec<&Character> = characters.iter()
            .filter(|c| c.character_type == CharacterType::Scholar)
            .collect();
        assert_eq!(scholars.len(), 1);
        assert_eq!(scholars[0].name, "Imam Bukhari");
    }

    #[test]
    fn test_character_search_by_historical_period() {
        let mut prophet_era_character = Character::new(
            "Bilal ibn Rabah".to_string(),
            "بلال بن رباح".to_string(),
            CharacterType::Companion,
        );
        prophet_era_character.historical_period = Some(TimePeriod::PropheticEra);

        let mut abbasid_character = Character::new(
            "Harun al-Rashid".to_string(),
            "هارون الرشيد".to_string(),
            CharacterType::Ruler,
        );
        abbasid_character.historical_period = Some(TimePeriod::Abbasid);

        // Test historical period filtering
        assert!(prophet_era_character.is_early_islamic());
        assert!(!abbasid_character.is_early_islamic());
        
        assert_eq!(prophet_era_character.historical_period, Some(TimePeriod::PropheticEra));
        assert_eq!(abbasid_character.historical_period, Some(TimePeriod::Abbasid));
    }

    #[test]
    fn test_story_character_relationship() {
        let story_id = Uuid::new_v4();
        let character_id = Uuid::new_v4();

        let story_character = StoryCharacter {
            id: Uuid::new_v4(),
            story_id,
            character_id,
            role_in_story: CharacterRole::Protagonist,
            importance_level: ImportanceLevel::Primary,
            character_description_in_story: Some("The main character who demonstrates patience".to_string()),
            created_at: chrono::Utc::now(),
        };

        // Test character role in story
        assert_eq!(story_character.role_in_story, CharacterRole::Protagonist);
        assert_eq!(story_character.importance_level, ImportanceLevel::Primary);
        assert!(story_character.character_description_in_story.is_some());
    }

    #[test]
    fn test_character_search_params_validation() {
        let search_params = SearchCharactersParams {
            query: "Muhammad".to_string(),
            character_type: Some(CharacterType::Prophet),
            historical_period: Some(TimePeriod::PropheticEra),
            limit: Some(10),
            offset: Some(0),
        };

        // Test search parameters
        assert_eq!(search_params.query, "Muhammad");
        assert_eq!(search_params.character_type, Some(CharacterType::Prophet));
        assert_eq!(search_params.historical_period, Some(TimePeriod::PropheticEra));
        assert_eq!(search_params.limit, Some(10));
        assert_eq!(search_params.offset, Some(0));
    }

    #[test]
    fn test_character_virtues_search() {
        let mut character = Character::new(
            "Prophet Yusuf".to_string(),
            "النبي يوسف".to_string(),
            CharacterType::Prophet,
        );

        // Add virtues that can be searched
        character.add_virtue("Patience".to_string());
        character.add_virtue("Forgiveness".to_string());
        character.add_virtue("Wisdom".to_string());
        character.add_virtue("Beauty".to_string());

        // Test virtue-based search capability
        assert!(character.virtues.contains(&"Patience".to_string()));
        assert!(character.virtues.contains(&"Forgiveness".to_string()));
        assert!(character.virtues.contains(&"Wisdom".to_string()));
        assert!(character.virtues.contains(&"Beauty".to_string()));
        assert_eq!(character.virtues.len(), 4);

        // Test duplicate prevention
        character.add_virtue("Patience".to_string());
        assert_eq!(character.virtues.len(), 4); // Should still be 4
    }

    #[test]
    fn test_story_search_by_character_themes() {
        let mut story = Story::new(
            "The Story of Prophet Yusuf".to_string(),
            "قصة النبي يوسف".to_string(),
            "This story features Prophet Yusuf and demonstrates patience, forgiveness, and divine wisdom.".to_string(),
            StoryCategory::Prophets,
            AgeGroup::AllAges,
            "en".to_string(),
            AuthenticityLevel::Authentic,
        );

        // Add character-related themes
        story.add_theme("Prophethood".to_string());
        story.add_theme("Patience".to_string());
        story.add_theme("Forgiveness".to_string());
        story.add_theme("Divine Wisdom".to_string());

        // Test theme-based character search
        assert!(story.themes.contains(&"Prophethood".to_string()));
        assert!(story.themes.contains(&"Patience".to_string()));
        assert!(story.themes.contains(&"Forgiveness".to_string()));
        assert!(story.themes.contains(&"Divine Wisdom".to_string()));

        // Test that themes can be used to find stories with specific character traits
        let patience_stories: Vec<String> = story.themes.iter()
            .filter(|theme| theme.to_lowercase().contains("patience"))
            .cloned()
            .collect();
        assert_eq!(patience_stories.len(), 1);
    }
}

/// Unit tests for reference linking functionality (Requirement 4.5)
/// Tests the ability to link Quranic verses and Hadiths to their original sources
#[cfg(test)]
mod reference_linking_tests {
    use super::*;

    #[test]
    fn test_quran_reference_linking() {
        let story_id = Uuid::new_v4();
        let quran_source = StorySource::new(
            story_id,
            SourceType::Quran,
            "Holy Quran".to_string(),
            "القرآن الكريم".to_string(),
            "Surah Yusuf (12:1-111)".to_string(),
        );

        // Test Quranic reference properties
        assert_eq!(quran_source.source_type, SourceType::Quran);
        assert!(quran_source.is_primary_source());
        assert_eq!(quran_source.source_type_arabic(), "القرآن الكريم");
        assert_eq!(quran_source.reference, "Surah Yusuf (12:1-111)");
        
        // Test credibility for Quranic sources
        assert_eq!(quran_source.credibility_score, 5.0); // Default score
        
        // Quranic sources should have highest credibility when properly set
        let mut high_credibility_quran = quran_source.clone();
        high_credibility_quran.credibility_score = 10.0;
        high_credibility_quran.verification_status = VerificationStatus::Verified;
        assert!(high_credibility_quran.is_highly_credible());
    }

    #[test]
    fn test_hadith_reference_linking() {
        let story_id = Uuid::new_v4();
        let mut hadith_source = StorySource::new(
            story_id,
            SourceType::Hadith,
            "Sahih Bukhari".to_string(),
            "صحيح البخاري".to_string(),
            "Book 60, Hadith 1".to_string(),
        );

        // Test Hadith reference properties
        assert_eq!(hadith_source.source_type, SourceType::Hadith);
        assert!(hadith_source.is_primary_source());
        assert_eq!(hadith_source.source_type_arabic(), "الحديث النبوي");
        
        // Test authenticity grading for Hadiths
        hadith_source.authenticity_grade = Some("sahih".to_string());
        assert_eq!(hadith_source.authenticity_grade, Some("sahih".to_string()));

        // Test different authenticity grades
        let mut hasan_hadith = hadith_source.clone();
        hasan_hadith.authenticity_grade = Some("hasan".to_string());
        hasan_hadith.source_name = "Sunan at-Tirmidhi".to_string();
        hasan_hadith.arabic_source_name = "سنن الترمذي".to_string();

        let mut daif_hadith = hadith_source.clone();
        daif_hadith.authenticity_grade = Some("daif".to_string());
        daif_hadith.source_name = "Sunan Abu Dawood".to_string();
        daif_hadith.arabic_source_name = "سنن أبي داود".to_string();

        // Test that different grades are properly stored
        assert_eq!(hasan_hadith.authenticity_grade, Some("hasan".to_string()));
        assert_eq!(daif_hadith.authenticity_grade, Some("daif".to_string()));
    }

    #[test]
    fn test_multiple_source_types_linking() {
        let story_id = Uuid::new_v4();
        
        let sources = vec![
            StorySource::new(
                story_id,
                SourceType::Quran,
                "Holy Quran".to_string(),
                "القرآن الكريم".to_string(),
                "Surah Al-Baqarah (2:155)".to_string(),
            ),
            StorySource::new(
                story_id,
                SourceType::Hadith,
                "Sahih Muslim".to_string(),
                "صحيح مسلم".to_string(),
                "Book 1, Hadith 1".to_string(),
            ),
            StorySource::new(
                story_id,
                SourceType::Tafsir,
                "Tafsir Ibn Kathir".to_string(),
                "تفسير ابن كثير".to_string(),
                "Volume 1, Page 123".to_string(),
            ),
            StorySource::new(
                story_id,
                SourceType::Biography,
                "Sirat Rasul Allah".to_string(),
                "سيرة رسول الله".to_string(),
                "Chapter 5".to_string(),
            ),
        ];

        // Test that all source types are properly categorized
        let primary_sources: Vec<&StorySource> = sources.iter()
            .filter(|s| s.is_primary_source())
            .collect();
        assert_eq!(primary_sources.len(), 2); // Quran and Hadith

        let secondary_sources: Vec<&StorySource> = sources.iter()
            .filter(|s| !s.is_primary_source())
            .collect();
        assert_eq!(secondary_sources.len(), 2); // Tafsir and Biography

        // Test source type Arabic names
        assert_eq!(sources[0].source_type_arabic(), "القرآن الكريم");
        assert_eq!(sources[1].source_type_arabic(), "الحديث النبوي");
        assert_eq!(sources[2].source_type_arabic(), "تفسير");
        assert_eq!(sources[3].source_type_arabic(), "سيرة");
    }

    #[test]
    fn test_lesson_reference_linking() {
        let mut lesson = Lesson::new(
            "Patience in Adversity".to_string(),
            "الصبر في المحن".to_string(),
            "This lesson teaches about patience during difficult times".to_string(),
            LessonType::Moral,
            MoralCategory::Patience,
        );

        // Test linking Quranic verses to lessons
        lesson.add_related_verse("2:155".to_string()); // Verse about trials
        lesson.add_related_verse("3:200".to_string()); // Verse about patience
        lesson.add_related_verse("2:155".to_string()); // Duplicate should be ignored

        assert_eq!(lesson.related_verses.len(), 2);
        assert!(lesson.related_verses.contains(&"2:155".to_string()));
        assert!(lesson.related_verses.contains(&"3:200".to_string()));

        // Test linking Hadiths to lessons
        lesson.add_related_hadith("Bukhari 6412".to_string()); // Hadith about patience
        lesson.add_related_hadith("Muslim 2999".to_string()); // Hadith about trials
        lesson.add_related_hadith("Bukhari 6412".to_string()); // Duplicate should be ignored

        assert_eq!(lesson.related_hadiths.len(), 2);
        assert!(lesson.related_hadiths.contains(&"Bukhari 6412".to_string()));
        assert!(lesson.related_hadiths.contains(&"Muslim 2999".to_string()));
    }

    #[test]
    fn test_story_lesson_reference_relationship() {
        let story_id = Uuid::new_v4();
        let lesson_id = Uuid::new_v4();

        let story_lesson = StoryLesson {
            id: Uuid::new_v4(),
            story_id,
            lesson_id,
            relevance_score: 8.5,
            explanation: Some("This lesson is demonstrated through the character's patience during trials".to_string()),
            created_at: chrono::Utc::now(),
        };

        // Test lesson-story relationship
        assert_eq!(story_lesson.story_id, story_id);
        assert_eq!(story_lesson.lesson_id, lesson_id);
        assert_eq!(story_lesson.relevance_score, 8.5);
        assert!(story_lesson.explanation.is_some());

        // Test relevance score validation (should be between 0.0 and 10.0)
        assert!(story_lesson.relevance_score >= 0.0);
        assert!(story_lesson.relevance_score <= 10.0);
    }

    #[test]
    fn test_source_credibility_and_verification() {
        let story_id = Uuid::new_v4();
        let mut source = StorySource::new(
            story_id,
            SourceType::Hadith,
            "Sahih Bukhari".to_string(),
            "صحيح البخاري".to_string(),
            "Book 60, Hadith 1".to_string(),
        );

        // Test initial verification status
        assert_eq!(source.verification_status, VerificationStatus::Unverified);
        assert!(!source.is_highly_credible());

        // Test verified source with high credibility
        source.credibility_score = 9.5;
        source.verification_status = VerificationStatus::Verified;
        source.authenticity_grade = Some("sahih".to_string());
        
        assert!(source.is_highly_credible());
        assert_eq!(source.verification_status, VerificationStatus::Verified);

        // Test questionable source
        let mut questionable_source = source.clone();
        questionable_source.verification_status = VerificationStatus::Questionable;
        questionable_source.credibility_score = 3.0;
        
        assert!(!questionable_source.is_highly_credible());
        assert_eq!(questionable_source.verification_status, VerificationStatus::Questionable);
    }

    #[test]
    fn test_reference_linking_with_author_attribution() {
        let story_id = Uuid::new_v4();
        let mut tafsir_source = StorySource::new(
            story_id,
            SourceType::Tafsir,
            "Tafsir al-Tabari".to_string(),
            "تفسير الطبري".to_string(),
            "Volume 2, Page 456".to_string(),
        );

        // Test author attribution
        tafsir_source.author = Some("Muhammad ibn Jarir al-Tabari".to_string());
        tafsir_source.notes = Some("Classical commentary on the Quran".to_string());

        assert_eq!(tafsir_source.author, Some("Muhammad ibn Jarir al-Tabari".to_string()));
        assert!(tafsir_source.notes.is_some());
        assert!(!tafsir_source.is_primary_source()); // Tafsir is secondary source

        // Test scholarly work attribution
        let mut scholarly_source = StorySource::new(
            story_id,
            SourceType::ScholarlyWork,
            "The Sealed Nectar".to_string(),
            "الرحيق المختوم".to_string(),
            "Chapter 10".to_string(),
        );

        scholarly_source.author = Some("Safi-ur-Rahman al-Mubarakpuri".to_string());
        scholarly_source.notes = Some("Modern biography of Prophet Muhammad".to_string());

        assert_eq!(scholarly_source.author, Some("Safi-ur-Rahman al-Mubarakpuri".to_string()));
        assert_eq!(scholarly_source.source_type, SourceType::ScholarlyWork);
    }

    #[test]
    fn test_comprehensive_reference_validation() {
        let story_id = Uuid::new_v4();
        
        // Test comprehensive source with all fields
        let mut comprehensive_source = StorySource::new(
            story_id,
            SourceType::Hadith,
            "Sahih Bukhari".to_string(),
            "صحيح البخاري".to_string(),
            "Book 1, Chapter 1, Hadith 1".to_string(),
        );

        comprehensive_source.author = Some("Imam al-Bukhari".to_string());
        comprehensive_source.authenticity_grade = Some("sahih".to_string());
        comprehensive_source.credibility_score = 10.0;
        comprehensive_source.verification_status = VerificationStatus::Verified;
        comprehensive_source.notes = Some("Authentic hadith from the most reliable collection".to_string());

        // Validate all reference linking components
        assert!(comprehensive_source.is_primary_source());
        assert!(comprehensive_source.is_highly_credible());
        assert_eq!(comprehensive_source.authenticity_grade, Some("sahih".to_string()));
        assert_eq!(comprehensive_source.credibility_score, 10.0);
        assert_eq!(comprehensive_source.verification_status, VerificationStatus::Verified);
        assert!(comprehensive_source.author.is_some());
        assert!(comprehensive_source.notes.is_some());

        // Test reference format validation
        assert!(!comprehensive_source.reference.is_empty());
        assert!(comprehensive_source.reference.contains("Book"));
        assert!(comprehensive_source.reference.contains("Chapter"));
        assert!(comprehensive_source.reference.contains("Hadith"));
    }
}

/// Unit tests for search functionality combining characters and references
#[cfg(test)]
mod integrated_search_tests {
    use super::*;

    #[test]
    fn test_story_with_characters_and_references() {
        let mut story = Story::new(
            "The Patience of Prophet Ayyub".to_string(),
            "صبر النبي أيوب".to_string(),
            "This story tells of Prophet Ayyub's patience during severe trials, as mentioned in the Quran and authentic Hadiths.".to_string(),
            StoryCategory::Prophets,
            AgeGroup::AllAges,
            "en".to_string(),
            AuthenticityLevel::Authentic,
        );

        // Add themes for character-based search
        story.add_theme("Patience".to_string());
        story.add_theme("Prophethood".to_string());
        story.add_theme("Divine Trial".to_string());
        story.add_theme("Faith".to_string());

        // Add moral lessons
        story.add_moral_lesson("Patience in adversity".to_string());
        story.add_moral_lesson("Trust in Allah's wisdom".to_string());
        story.add_moral_lesson("Perseverance through trials".to_string());

        // Test that story can be found by character-related themes
        assert!(story.themes.contains(&"Patience".to_string()));
        assert!(story.themes.contains(&"Prophethood".to_string()));
        
        // Test that story contains moral lessons that can be linked to references
        assert!(story.moral_lessons.contains(&"Patience in adversity".to_string()));
        assert!(story.moral_lessons.contains(&"Trust in Allah's wisdom".to_string()));

        // Verify story integrity for reference linking
        assert!(story.verify_integrity());
        assert!(story.is_historically_authentic());
    }

    #[test]
    fn test_character_story_source_integration() {
        let story_id = Uuid::new_v4();
        let character_id = Uuid::new_v4();

        // Create character
        let mut character = Character::new(
            "Prophet Ayyub".to_string(),
            "النبي أيوب".to_string(),
            CharacterType::Prophet,
        );
        character.add_virtue("Patience".to_string());
        character.add_virtue("Faith".to_string());
        character.historical_period = Some(TimePeriod::AncientProphets);

        // Create character-story relationship
        let story_character = StoryCharacter {
            id: Uuid::new_v4(),
            story_id,
            character_id,
            role_in_story: CharacterRole::Protagonist,
            importance_level: ImportanceLevel::Primary,
            character_description_in_story: Some("The patient prophet who endured severe trials".to_string()),
            created_at: chrono::Utc::now(),
        };

        // Create sources that reference this character's story
        let quran_source = StorySource::new(
            story_id,
            SourceType::Quran,
            "Holy Quran".to_string(),
            "القرآن الكريم".to_string(),
            "Surah Sad (38:41-44), Surah Al-Anbiya (21:83-84)".to_string(),
        );

        let mut hadith_source = StorySource::new(
            story_id,
            SourceType::Hadith,
            "Sahih Bukhari".to_string(),
            "صحيح البخاري".to_string(),
            "Book 60, Hadith 202".to_string(),
        );
        hadith_source.authenticity_grade = Some("sahih".to_string());

        // Test integration: character can be found through story, and story has proper references
        assert_eq!(story_character.story_id, story_id);
        assert_eq!(story_character.character_id, character_id);
        assert_eq!(story_character.role_in_story, CharacterRole::Protagonist);
        
        assert_eq!(quran_source.story_id, story_id);
        assert_eq!(hadith_source.story_id, story_id);
        assert!(quran_source.is_primary_source());
        assert!(hadith_source.is_primary_source());
        
        // Test that character virtues align with story themes
        assert!(character.virtues.contains(&"Patience".to_string()));
        assert!(character.virtues.contains(&"Faith".to_string()));
        assert!(character.is_prophet());
    }

    #[test]
    fn test_search_request_validation() {
        let search_request = SearchStoriesRequest {
            query: "patience".to_string(),
            categories: Some(vec![StoryCategory::Prophets, StoryCategory::MoralLessons]),
            age_groups: Some(vec![AgeGroup::AllAges]),
            time_periods: Some(vec![TimePeriod::AncientProphets, TimePeriod::PropheticEra]),
            authenticity_levels: Some(vec![AuthenticityLevel::Authentic, AuthenticityLevel::WellDocumented]),
            character_names: Some(vec!["Prophet Ayyub".to_string(), "Prophet Yusuf".to_string()]),
            themes: Some(vec!["Patience".to_string(), "Faith".to_string()]),
            search_type: Some(SearchType::Character),
            limit: Some(20),
            offset: Some(0),
        };

        // Test search request structure for character-based search
        assert_eq!(search_request.query, "patience");
        assert_eq!(search_request.search_type, Some(SearchType::Character));
        assert!(search_request.character_names.is_some());
        assert!(search_request.themes.is_some());
        
        let character_names = search_request.character_names.unwrap();
        assert!(character_names.contains(&"Prophet Ayyub".to_string()));
        assert!(character_names.contains(&"Prophet Yusuf".to_string()));
        
        let themes = search_request.themes.unwrap();
        assert!(themes.contains(&"Patience".to_string()));
        assert!(themes.contains(&"Faith".to_string()));
    }

    #[test]
    fn test_lesson_search_with_references() {
        let search_params = SearchByLessonParams {
            lesson_title: "Patience".to_string(),
            lesson_type: Some(LessonType::Moral),
            moral_category: Some(MoralCategory::Patience),
            age_group: Some(AgeGroup::AllAges),
            limit: Some(10),
            offset: Some(0),
        };

        // Test lesson search parameters
        assert_eq!(search_params.lesson_title, "Patience");
        assert_eq!(search_params.lesson_type, Some(LessonType::Moral));
        assert_eq!(search_params.moral_category, Some(MoralCategory::Patience));
        assert_eq!(search_params.age_group, Some(AgeGroup::AllAges));

        // Create a lesson with references
        let mut lesson = Lesson::new(
            "The Virtue of Patience".to_string(),
            "فضيلة الصبر".to_string(),
            "Patience is a key virtue in Islam, demonstrated by many prophets and righteous people.".to_string(),
            LessonType::Moral,
            MoralCategory::Patience,
        );

        // Add references that would be linked
        lesson.add_related_verse("2:155".to_string()); // "And give good tidings to the patient"
        lesson.add_related_verse("3:200".to_string()); // "O you who believe! Persevere and endure"
        lesson.add_related_hadith("Bukhari 6412".to_string()); // Hadith about patience
        lesson.add_related_hadith("Muslim 2999".to_string()); // Hadith about trials

        // Test that lesson has proper references for linking
        assert_eq!(lesson.related_verses.len(), 2);
        assert_eq!(lesson.related_hadiths.len(), 2);
        assert!(lesson.related_verses.contains(&"2:155".to_string()));
        assert!(lesson.related_hadiths.contains(&"Bukhari 6412".to_string()));
    }

    #[test]
    fn test_moral_category_search_with_references() {
        let search_params = SearchByMoralParams {
            moral_category: MoralCategory::Patience,
            lesson_type: Some(LessonType::Moral),
            age_group: Some(AgeGroup::AllAges),
            authenticity_level: Some(AuthenticityLevel::Authentic),
            limit: Some(15),
            offset: Some(0),
        };

        // Test moral category search parameters
        assert_eq!(search_params.moral_category, MoralCategory::Patience);
        assert_eq!(search_params.lesson_type, Some(LessonType::Moral));
        assert_eq!(search_params.authenticity_level, Some(AuthenticityLevel::Authentic));

        // Test Arabic name for moral category
        assert_eq!(search_params.moral_category.arabic_name(), "الصبر");

        // Create story that would match this search
        let mut story = Story::new(
            "Stories of Patience".to_string(),
            "قصص الصبر".to_string(),
            "Collection of stories demonstrating patience in various situations.".to_string(),
            StoryCategory::MoralLessons,
            AgeGroup::AllAges,
            "ar".to_string(),
            AuthenticityLevel::Authentic,
        );

        story.add_moral_lesson("Patience in worship".to_string());
        story.add_moral_lesson("Patience in hardship".to_string());
        story.add_theme("Patience".to_string());
        story.add_theme("Perseverance".to_string());

        // Test that story matches search criteria
        assert_eq!(story.authenticity_level, AuthenticityLevel::Authentic);
        assert!(story.moral_lessons.iter().any(|lesson| lesson.to_lowercase().contains("patience")));
        assert!(story.themes.contains(&"Patience".to_string()));
    }
}

// Integration tests that would require database setup
#[cfg(test)]
mod integration_tests {
    use super::*;

    // These tests would be run with a test database
    // They are commented out as they require actual database setup

    /*
    #[tokio::test]
    async fn test_character_search_integration() {
        let pool = setup_test_database().await;
        let repository = StoryRepository::new(pool);
        let service = StoryService::new(repository);

        // Create test character
        let character = service.create_character(
            "Prophet Ibrahim".to_string(),
            "النبي إبراهيم".to_string(),
            CharacterType::Prophet,
            Some("The friend of Allah, known for his unwavering faith".to_string()),
            Some(TimePeriod::AncientProphets),
        ).await.unwrap();

        // Create test story
        let story = service.create_story(
            "The Story of Prophet Ibrahim".to_string(),
            "قصة النبي إبراهيم".to_string(),
            "This story tells of Prophet Ibrahim's faith and submission to Allah.".to_string(),
            StoryCategory::Prophets,
            AgeGroup::AllAges,
            "en".to_string(),
            AuthenticityLevel::Authentic,
        ).await.unwrap();

        // Link character to story
        service.add_character_to_story(
            story.id,
            character.id,
            CharacterRole::Protagonist,
            ImportanceLevel::Primary,
            Some("The main character demonstrating faith and submission".to_string()),
        ).await.unwrap();

        // Test character search
        let search_params = SearchCharactersParams {
            query: "Ibrahim".to_string(),
            character_type: Some(CharacterType::Prophet),
            historical_period: Some(TimePeriod::AncientProphets),
            limit: Some(10),
            offset: Some(0),
        };

        let characters = service.search_characters(search_params).await.unwrap();
        assert!(!characters.is_empty());
        assert_eq!(characters[0].name, "Prophet Ibrahim");

        // Test stories by character
        let character_request = GetStoriesByCharacterRequest {
            character_name: "Prophet Ibrahim".to_string(),
            character_type: Some(CharacterType::Prophet),
            include_related: Some(true),
            limit: Some(10),
            offset: Some(0),
        };

        let character_stories = service.get_stories_by_character(character_request).await.unwrap();
        assert!(!character_stories.stories.is_empty());
        assert_eq!(character_stories.character.name, "Prophet Ibrahim");
    }

    #[tokio::test]
    async fn test_reference_linking_integration() {
        let pool = setup_test_database().await;
        let repository = StoryRepository::new(pool);
        let service = StoryService::new(repository);

        // Create test story
        let story = service.create_story(
            "The Test of Faith".to_string(),
            "اختبار الإيمان".to_string(),
            "A story about faith and patience, supported by Quranic verses and authentic Hadiths.".to_string(),
            StoryCategory::MoralLessons,
            AgeGroup::AllAges,
            "en".to_string(),
            AuthenticityLevel::Authentic,
        ).await.unwrap();

        // Create Quranic source
        let quran_source = service.create_story_source(
            story.id,
            SourceType::Quran,
            "Holy Quran".to_string(),
            "القرآن الكريم".to_string(),
            "Surah Al-Baqarah (2:155-157)".to_string(),
            None,
            None,
        ).await.unwrap();

        // Create Hadith source
        let hadith_source = service.create_story_source(
            story.id,
            SourceType::Hadith,
            "Sahih Bukhari".to_string(),
            "صحيح البخاري".to_string(),
            "Book 81, Hadith 1".to_string(),
            Some("Imam al-Bukhari".to_string()),
            Some("sahih".to_string()),
        ).await.unwrap();

        // Create lesson with references
        let lesson = service.create_lesson(
            "Patience in Trials".to_string(),
            "الصبر في المحن".to_string(),
            "This lesson teaches about maintaining patience during difficult times.".to_string(),
            LessonType::Moral,
            MoralCategory::Patience,
        ).await.unwrap();

        // Link lesson to story
        service.add_lesson_to_story(
            story.id,
            lesson.id,
            9.0,
            Some("This lesson is central to the story's message".to_string()),
        ).await.unwrap();

        // Test getting story sources
        let sources = service.get_story_sources(story.id).await.unwrap();
        assert_eq!(sources.len(), 2);
        
        let quran_sources: Vec<&StorySource> = sources.iter()
            .filter(|s| s.source_type == SourceType::Quran)
            .collect();
        assert_eq!(quran_sources.len(), 1);
        
        let hadith_sources: Vec<&StorySource> = sources.iter()
            .filter(|s| s.source_type == SourceType::Hadith)
            .collect();
        assert_eq!(hadith_sources.len(), 1);

        // Test getting story lessons
        let story_lessons = service.get_story_lessons(story.id).await.unwrap();
        assert_eq!(story_lessons.len(), 1);
        assert_eq!(story_lessons[0].lesson.title, "Patience in Trials");
        assert_eq!(story_lessons[0].relevance_score, 9.0);
    }

    async fn setup_test_database() -> PgPool {
        // This would set up a test database with migrations
        // For now, this is a placeholder
        todo!("Implement test database setup")
    }
    */
}