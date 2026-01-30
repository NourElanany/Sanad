/// Simple test runner for unit tests that don't require database
/// This allows us to test the core business logic without database dependencies

use crate::models::*;
use uuid::Uuid;

pub fn run_character_search_tests() {
    println!("Running Character Search Tests (Requirement 4.4)...");
    
    test_character_search_by_name();
    test_character_search_by_type();
    test_character_search_by_historical_period();
    test_story_character_relationship();
    test_character_virtues_search();
    test_story_search_by_character_themes();
    
    println!("✅ All Character Search Tests Passed!");
}

pub fn run_reference_linking_tests() {
    println!("Running Reference Linking Tests (Requirement 4.5)...");
    
    test_quran_reference_linking();
    test_hadith_reference_linking();
    test_multiple_source_types_linking();
    test_lesson_reference_linking();
    test_story_lesson_reference_relationship();
    test_source_credibility_and_verification();
    test_reference_linking_with_author_attribution();
    test_comprehensive_reference_validation();
    
    println!("✅ All Reference Linking Tests Passed!");
}

pub fn run_integrated_search_tests() {
    println!("Running Integrated Search Tests...");
    
    test_story_with_characters_and_references();
    test_character_story_source_integration();
    test_search_request_validation();
    test_lesson_search_with_references();
    test_moral_category_search_with_references();
    
    println!("✅ All Integrated Search Tests Passed!");
}

// Character Search Tests (Requirement 4.4)
fn test_character_search_by_name() {
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
    
    println!("  ✓ Character search by name test passed");
}

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
    
    println!("  ✓ Character search by type test passed");
}

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
    
    println!("  ✓ Character search by historical period test passed");
}

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
    
    println!("  ✓ Story-character relationship test passed");
}

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
    
    println!("  ✓ Character virtues search test passed");
}

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
    
    println!("  ✓ Story search by character themes test passed");
}

// Reference Linking Tests (Requirement 4.5)
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
    
    println!("  ✓ Quran reference linking test passed");
}

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
    
    println!("  ✓ Hadith reference linking test passed");
}

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
    
    println!("  ✓ Multiple source types linking test passed");
}

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
    
    println!("  ✓ Lesson reference linking test passed");
}

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
    
    println!("  ✓ Story-lesson reference relationship test passed");
}

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
    
    println!("  ✓ Source credibility and verification test passed");
}

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
    
    println!("  ✓ Reference linking with author attribution test passed");
}

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
    
    println!("  ✓ Comprehensive reference validation test passed");
}

// Integrated Search Tests
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
    
    println!("  ✓ Story with characters and references test passed");
}

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
    
    println!("  ✓ Character-story-source integration test passed");
}

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
    
    println!("  ✓ Search request validation test passed");
}

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
    
    println!("  ✓ Lesson search with references test passed");
}

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
    
    println!("  ✓ Moral category search with references test passed");
}

pub fn run_all_tests() {
    println!("🧪 Running Islamic Stories Service Unit Tests");
    println!("Testing Requirements 4.4 (Character Search) and 4.5 (Reference Linking)");
    println!("=" .repeat(70));
    
    run_character_search_tests();
    println!();
    
    run_reference_linking_tests();
    println!();
    
    run_integrated_search_tests();
    println!();
    
    println!("🎉 All Tests Completed Successfully!");
    println!("✅ Requirement 4.4 (Character Search): PASSED");
    println!("✅ Requirement 4.5 (Reference Linking): PASSED");
    println!("=" .repeat(70));
}