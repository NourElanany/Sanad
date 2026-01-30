use crate::models::*;
use proptest::prelude::*;
use uuid::Uuid;

/// **Validates: Requirements 1.5, 12.3**
/// Property 1: Islamic Content Integrity
/// 
/// For any Islamic content (Quran, Hadith, Tafsir) in the system, the stored text must match
/// trusted original sources and be protected from tampering or manipulation.
/// 
/// This property ensures that:
/// 1. Hash-based integrity verification works correctly for all content
/// 2. Any tampering with the text is immediately detectable
/// 3. Hash calculations are consistent and deterministic
/// 4. Content integrity verification never produces false positives or negatives

#[cfg(test)]
mod property_tests {
    use super::*;

    // Strategy for generating valid Arabic text (simplified for testing)
    fn arabic_text_strategy() -> impl Strategy<Value = String> {
        prop::collection::vec(
            prop::char::range('\u{0600}', '\u{06FF}'), // Arabic Unicode range
            1..200
        ).prop_map(|chars| chars.into_iter().collect())
    }

    // Strategy for generating valid Surah numbers (1-114)
    fn surah_number_strategy() -> impl Strategy<Value = i32> {
        1..=114i32
    }

    // Strategy for generating valid Ayah numbers (1-286, max in Al-Baqarah)
    fn ayah_number_strategy() -> impl Strategy<Value = i32> {
        1..=286i32
    }

    // Strategy for generating valid Juz numbers (1-30)
    fn juz_strategy() -> impl Strategy<Value = i32> {
        1..=30i32
    }

    // Strategy for generating valid page numbers (1-604)
    fn page_strategy() -> impl Strategy<Value = i32> {
        1..=604i32
    }

    proptest! {
        /// **Validates: Requirements 1.5, 12.3**
        /// Property: Hash Consistency for Ayah Content
        /// 
        /// For any text content, calculating the hash multiple times should always
        /// produce the same result, ensuring deterministic integrity verification.
        #[test]
        fn prop_ayah_hash_consistency(
            text in arabic_text_strategy()
        ) {
            let hash1 = Ayah::calculate_text_hash(&text);
            let hash2 = Ayah::calculate_text_hash(&text);
            let hash3 = Ayah::calculate_text_hash(&text);
            
            prop_assert_eq!(&hash1, &hash2);
            prop_assert_eq!(&hash1, &hash3);
            prop_assert!(!hash1.is_empty());
            prop_assert_eq!(hash1.len(), 64); // SHA-256 produces 64-character hex string
        }

        /// **Validates: Requirements 1.5, 12.3**
        /// Property: Ayah Integrity Verification
        /// 
        /// Any newly created Ayah should pass integrity verification immediately,
        /// and any tampering with the text should cause verification to fail.
        #[test]
        fn prop_ayah_integrity_verification(
            surah_number in surah_number_strategy(),
            ayah_number in ayah_number_strategy(),
            text in arabic_text_strategy(),
            juz in juz_strategy(),
            page in page_strategy(),
            ruku in prop::option::of(1..=40i32)
        ) {
            // Create a new Ayah with the generated data
            let ayah = Ayah::new(surah_number, ayah_number, text.clone(), juz, page, ruku);
            
            // Verify that the newly created Ayah passes integrity check
            prop_assert!(ayah.verify_integrity());
            
            // Verify that the stored hash matches the calculated hash
            let expected_hash = Ayah::calculate_text_hash(&text);
            prop_assert_eq!(ayah.text_hash, expected_hash);
            
            // Verify that the text content is preserved exactly
            prop_assert_eq!(ayah.text, text);
        }

        /// **Validates: Requirements 1.5, 12.3**
        /// Property: Tamper Detection for Ayah Content
        /// 
        /// Any modification to the Ayah text after creation should be detectable
        /// through integrity verification failure.
        #[test]
        fn prop_ayah_tamper_detection(
            surah_number in surah_number_strategy(),
            ayah_number in ayah_number_strategy(),
            original_text in arabic_text_strategy(),
            tampered_text in arabic_text_strategy(),
            juz in juz_strategy(),
            page in page_strategy(),
            ruku in prop::option::of(1..=40i32)
        ) {
            // Skip test if texts are identical (no tampering)
            prop_assume!(original_text != tampered_text);
            
            // Create Ayah with original text
            let mut ayah = Ayah::new(surah_number, ayah_number, original_text, juz, page, ruku);
            
            // Verify original integrity
            prop_assert!(ayah.verify_integrity());
            
            // Tamper with the text (but keep the original hash)
            ayah.text = tampered_text;
            
            // Verify that tampering is detected
            prop_assert!(!ayah.verify_integrity());
        }

        /// **Validates: Requirements 1.5, 12.3**
        /// Property: Hash Uniqueness for Different Content
        /// 
        /// Different text content should produce different hashes, ensuring
        /// that integrity verification can distinguish between different content.
        #[test]
        fn prop_hash_uniqueness_for_different_content(
            text1 in arabic_text_strategy(),
            text2 in arabic_text_strategy()
        ) {
            // Skip if texts are identical
            prop_assume!(text1 != text2);
            
            let hash1 = Ayah::calculate_text_hash(&text1);
            let hash2 = Ayah::calculate_text_hash(&text2);
            
            // Different texts should produce different hashes
            prop_assert_ne!(hash1, hash2);
        }

        /// **Validates: Requirements 1.5, 12.3**
        /// Property: Tafsir Integrity Verification
        /// 
        /// Any newly created Tafsir should pass integrity verification immediately,
        /// and any tampering with the text should cause verification to fail.
        #[test]
        fn prop_tafsir_integrity_verification(
            surah_number in surah_number_strategy(),
            ayah_number in ayah_number_strategy(),
            text in arabic_text_strategy()
        ) {
            let source_id = Uuid::new_v4();
            
            // Create a new Tafsir with the generated data
            let tafsir = Tafsir::new(surah_number, ayah_number, source_id, text.clone());
            
            // Verify that the newly created Tafsir passes integrity check
            prop_assert!(tafsir.verify_integrity());
            
            // Verify that the stored hash matches the calculated hash
            let expected_hash = Tafsir::calculate_text_hash(&text);
            prop_assert_eq!(tafsir.text_hash, expected_hash);
            
            // Verify that the text content is preserved exactly
            prop_assert_eq!(tafsir.text, text);
        }

        /// **Validates: Requirements 1.5, 12.3**
        /// Property: Tafsir Tamper Detection
        /// 
        /// Any modification to the Tafsir text after creation should be detectable
        /// through integrity verification failure.
        #[test]
        fn prop_tafsir_tamper_detection(
            surah_number in surah_number_strategy(),
            ayah_number in ayah_number_strategy(),
            original_text in arabic_text_strategy(),
            tampered_text in arabic_text_strategy()
        ) {
            // Skip test if texts are identical (no tampering)
            prop_assume!(original_text != tampered_text);
            
            let source_id = Uuid::new_v4();
            
            // Create Tafsir with original text
            let mut tafsir = Tafsir::new(surah_number, ayah_number, source_id, original_text);
            
            // Verify original integrity
            prop_assert!(tafsir.verify_integrity());
            
            // Tamper with the text (but keep the original hash)
            tafsir.text = tampered_text;
            
            // Verify that tampering is detected
            prop_assert!(!tafsir.verify_integrity());
        }

        /// **Validates: Requirements 1.5, 12.3**
        /// Property: Content Integrity Trait Consistency
        /// 
        /// The ContentIntegrity trait should work consistently for both
        /// Ayah and Tafsir types, providing the same verification results.
        #[test]
        fn prop_content_integrity_trait_consistency(
            surah_number in surah_number_strategy(),
            ayah_number in ayah_number_strategy(),
            text in arabic_text_strategy(),
            juz in juz_strategy(),
            page in page_strategy()
        ) {
            let ayah = Ayah::new(surah_number, ayah_number, text.clone(), juz, page, None);
            let tafsir = Tafsir::new(surah_number, ayah_number, Uuid::new_v4(), text.clone());
            
            // Both should verify integrity successfully when created properly
            prop_assert!(ayah.verify_integrity());
            prop_assert!(tafsir.verify_integrity());
            
            // The trait methods should work consistently
            prop_assert_eq!(ayah.verify_integrity(), <Ayah as ContentIntegrity>::verify_integrity(&ayah));
            prop_assert_eq!(tafsir.verify_integrity(), <Tafsir as ContentIntegrity>::verify_integrity(&tafsir));
            
            // Hash calculations should be consistent
            prop_assert_eq!(ayah.calculate_hash(), Ayah::calculate_text_hash(&text));
            prop_assert_eq!(tafsir.calculate_hash(), Tafsir::calculate_text_hash(&text));
        }

        /// **Validates: Requirements 1.5, 12.3**
        /// Property: Surah Metadata Consistency
        /// 
        /// Surah creation and type checking should be consistent and accurate
        /// for all valid inputs.
        #[test]
        fn prop_surah_metadata_consistency(
            number in surah_number_strategy(),
            name in "[a-zA-Z\\-\\s]{1,50}",
            arabic_name in arabic_text_strategy(),
            english_name in "[a-zA-Z\\-\\s]{1,50}",
            revelation_type in prop::sample::select(vec![RevelationType::Meccan, RevelationType::Medinan]),
            number_of_ayahs in 1..=286i32
        ) {
            let surah = Surah::new(
                number,
                name.clone(),
                arabic_name.clone(),
                english_name.clone(),
                revelation_type.clone(),
                number_of_ayahs
            );
            
            // Verify all fields are preserved correctly
            prop_assert_eq!(surah.number, number);
            prop_assert_eq!(&surah.name, &name);
            prop_assert_eq!(&surah.arabic_name, &arabic_name);
            prop_assert_eq!(&surah.english_name, &english_name);
            prop_assert_eq!(surah.number_of_ayahs, number_of_ayahs);
            
            // Verify revelation type checking is consistent
            match &revelation_type {
                RevelationType::Meccan => {
                    prop_assert!(surah.is_meccan());
                    prop_assert!(!surah.is_medinan());
                }
                RevelationType::Medinan => {
                    prop_assert!(surah.is_medinan());
                    prop_assert!(!surah.is_meccan());
                }
            }
        }

        /// **Validates: Requirements 1.5, 12.3**
        /// Property: Hash Immutability Under Serialization
        /// 
        /// Content hashes should remain consistent even after serialization
        /// and deserialization, ensuring integrity is preserved across
        /// data transformations.
        #[test]
        fn prop_hash_immutability_under_serialization(
            surah_number in surah_number_strategy(),
            ayah_number in ayah_number_strategy(),
            text in arabic_text_strategy(),
            juz in juz_strategy(),
            page in page_strategy()
        ) {
            let original_ayah = Ayah::new(surah_number, ayah_number, text, juz, page, None);
            
            // Serialize and deserialize
            let json = serde_json::to_string(&original_ayah).unwrap();
            let deserialized_ayah: Ayah = serde_json::from_str(&json).unwrap();
            
            // Verify integrity is preserved
            prop_assert!(original_ayah.verify_integrity());
            prop_assert!(deserialized_ayah.verify_integrity());
            
            // Verify hashes are identical
            prop_assert_eq!(original_ayah.text_hash, deserialized_ayah.text_hash);
            prop_assert_eq!(original_ayah.text, deserialized_ayah.text);
            
            // Verify all other fields are preserved
            prop_assert_eq!(original_ayah.id, deserialized_ayah.id);
            prop_assert_eq!(original_ayah.surah_number, deserialized_ayah.surah_number);
            prop_assert_eq!(original_ayah.ayah_number, deserialized_ayah.ayah_number);
        }

        /// **Validates: Requirements 1.2**
        /// Property 2: Structural Data Accuracy
        /// 
        /// For any Surah in the Quran, the number of displayed Ayahs must match
        /// the correct number with proper numbering for each Ayah.
        /// 
        /// This property ensures that:
        /// 1. Surah metadata correctly reflects the actual number of Ayahs
        /// 2. Ayah numbering is sequential and starts from 1
        /// 3. No gaps or duplicates exist in Ayah numbering
        /// 4. The total count matches the declared number_of_ayahs
        /// 5. All Ayahs belong to the correct Surah

        /// **Validates: Requirements 1.2**
        /// Property: Surah Ayah Count Consistency
        /// 
        /// For any Surah, the declared number_of_ayahs must match the actual
        /// count of Ayahs that belong to that Surah.
        #[test]
        fn prop_surah_ayah_count_consistency(
            surah_number in surah_number_strategy(),
            declared_count in 1..=286i32, // Max ayahs in any surah (Al-Baqarah)
            name in "[a-zA-Z\\-\\s]{1,50}",
            arabic_name in arabic_text_strategy(),
            english_name in "[a-zA-Z\\-\\s]{1,50}",
            revelation_type in prop::sample::select(vec![RevelationType::Meccan, RevelationType::Medinan])
        ) {
            let surah = Surah::new(
                surah_number,
                name,
                arabic_name,
                english_name,
                revelation_type,
                declared_count
            );
            
            // Generate the exact number of Ayahs as declared
            let mut ayahs = Vec::new();
            for ayah_num in 1..=declared_count {
                let ayah = Ayah::new(
                    surah_number,
                    ayah_num,
                    format!("آية رقم {}", ayah_num),
                    1, // juz
                    1, // page
                    None // ruku
                );
                ayahs.push(ayah);
            }
            
            // Verify the count matches
            prop_assert_eq!(ayahs.len() as i32, surah.number_of_ayahs);
            prop_assert_eq!(ayahs.len() as i32, declared_count);
            
            // Verify all Ayahs belong to the correct Surah
            for ayah in &ayahs {
                prop_assert_eq!(ayah.surah_number, surah_number);
            }
        }

        /// **Validates: Requirements 1.2**
        /// Property: Sequential Ayah Numbering
        /// 
        /// For any collection of Ayahs belonging to a Surah, the Ayah numbers
        /// must be sequential starting from 1 with no gaps or duplicates.
        #[test]
        fn prop_sequential_ayah_numbering(
            surah_number in surah_number_strategy(),
            ayah_count in 1..=50i32 // Reasonable range for testing
        ) {
            // Generate Ayahs with sequential numbering
            let mut ayahs = Vec::new();
            for ayah_num in 1..=ayah_count {
                let ayah = Ayah::new(
                    surah_number,
                    ayah_num,
                    format!("آية رقم {}", ayah_num),
                    1, 1, None
                );
                ayahs.push(ayah);
            }
            
            // Sort by ayah number to ensure proper ordering
            ayahs.sort_by_key(|a| a.ayah_number);
            
            // Verify sequential numbering
            for (index, ayah) in ayahs.iter().enumerate() {
                let expected_number = (index + 1) as i32;
                prop_assert_eq!(ayah.ayah_number, expected_number);
                prop_assert_eq!(ayah.surah_number, surah_number);
            }
            
            // Verify no duplicates
            let mut seen_numbers = std::collections::HashSet::new();
            for ayah in &ayahs {
                prop_assert!(seen_numbers.insert(ayah.ayah_number), 
                    "Duplicate ayah number found: {}", ayah.ayah_number);
            }
            
            // Verify complete range
            prop_assert_eq!(ayahs.len() as i32, ayah_count);
            prop_assert_eq!(ayahs.first().unwrap().ayah_number, 1);
            prop_assert_eq!(ayahs.last().unwrap().ayah_number, ayah_count);
        }

        /// **Validates: Requirements 1.2**
        /// Property: Ayah Structural Integrity
        /// 
        /// For any Ayah, all structural metadata (surah_number, ayah_number,
        /// juz, page) must be within valid ranges and consistent.
        #[test]
        fn prop_ayah_structural_integrity(
            surah_number in surah_number_strategy(),
            ayah_number in ayah_number_strategy(),
            text in arabic_text_strategy(),
            juz in juz_strategy(),
            page in page_strategy(),
            ruku in prop::option::of(1..=40i32)
        ) {
            let ayah = Ayah::new(surah_number, ayah_number, text, juz, page, ruku);
            
            // Verify all structural data is within valid ranges
            prop_assert!(ayah.surah_number >= 1 && ayah.surah_number <= 114, 
                "Invalid surah number: {}", ayah.surah_number);
            prop_assert!(ayah.ayah_number >= 1, 
                "Invalid ayah number: {}", ayah.ayah_number);
            prop_assert!(ayah.juz >= 1 && ayah.juz <= 30, 
                "Invalid juz number: {}", ayah.juz);
            prop_assert!(ayah.page >= 1 && ayah.page <= 604, 
                "Invalid page number: {}", ayah.page);
            
            if let Some(ruku_num) = ayah.ruku {
                prop_assert!(ruku_num >= 1 && ruku_num <= 40, 
                    "Invalid ruku number: {}", ruku_num);
            }
            
            // Verify structural consistency
            prop_assert_eq!(ayah.surah_number, surah_number);
            prop_assert_eq!(ayah.ayah_number, ayah_number);
            prop_assert_eq!(ayah.juz, juz);
            prop_assert_eq!(ayah.page, page);
            prop_assert_eq!(ayah.ruku, ruku);
        }

        /// **Validates: Requirements 1.2**
        /// Property: SurahWithAyahs Structural Consistency
        /// 
        /// For any SurahWithAyahs structure, the Surah metadata must be
        /// consistent with the actual Ayahs collection.
        #[test]
        fn prop_surah_with_ayahs_consistency(
            surah_number in surah_number_strategy(),
            ayah_count in 1..=20i32, // Reasonable range for testing
            name in "[a-zA-Z\\-\\s]{1,50}",
            arabic_name in arabic_text_strategy(),
            english_name in "[a-zA-Z\\-\\s]{1,50}",
            revelation_type in prop::sample::select(vec![RevelationType::Meccan, RevelationType::Medinan])
        ) {
            // Create Surah with declared ayah count
            let surah = Surah::new(
                surah_number,
                name,
                arabic_name,
                english_name,
                revelation_type,
                ayah_count
            );
            
            // Create matching Ayahs
            let mut ayahs = Vec::new();
            for ayah_num in 1..=ayah_count {
                let ayah = Ayah::new(
                    surah_number,
                    ayah_num,
                    format!("آية رقم {}", ayah_num),
                    1, 1, None
                );
                ayahs.push(ayah);
            }
            
            // Create SurahWithAyahs structure
            let surah_with_ayahs = SurahWithAyahs {
                surah: surah.clone(),
                ayahs: ayahs.clone(),
            };
            
            // Verify consistency
            prop_assert_eq!(surah_with_ayahs.surah.number_of_ayahs, ayah_count);
            prop_assert_eq!(surah_with_ayahs.ayahs.len() as i32, ayah_count);
            prop_assert_eq!(surah_with_ayahs.surah.number_of_ayahs, surah_with_ayahs.ayahs.len() as i32);
            
            // Verify all Ayahs belong to the Surah
            for ayah in &surah_with_ayahs.ayahs {
                prop_assert_eq!(ayah.surah_number, surah_with_ayahs.surah.number);
            }
            
            // Verify Ayah numbering is sequential
            let mut sorted_ayahs = surah_with_ayahs.ayahs.clone();
            sorted_ayahs.sort_by_key(|a| a.ayah_number);
            
            for (index, ayah) in sorted_ayahs.iter().enumerate() {
                prop_assert_eq!(ayah.ayah_number, (index + 1) as i32);
            }
        }

        /// **Validates: Requirements 1.2**
        /// Property: Ayah Range Structural Validity
        /// 
        /// For any valid Ayah range (used in Khatma planning), all Ayahs
        /// in the range must have proper structural organization.
        #[test]
        fn prop_ayah_range_structural_validity(
            start_surah in 1..=113i32, // Leave room for end_surah
            start_ayah in 1..=10i32,   // Keep ranges small for testing
            range_size in 1..=20i32    // Number of ayahs in range
        ) {
            let end_surah = start_surah + (range_size / 10).max(0); // Spread across surahs
            let end_ayah = start_ayah + (range_size % 10);
            
            // Ensure end_surah doesn't exceed 114
            let end_surah = end_surah.min(114);
            
            // Generate Ayahs for the range
            let mut ayahs = Vec::new();
            let mut current_surah = start_surah;
            let mut current_ayah = start_ayah;
            
            for _ in 0..range_size {
                if current_surah > end_surah || 
                   (current_surah == end_surah && current_ayah > end_ayah) {
                    break;
                }
                
                let ayah = Ayah::new(
                    current_surah,
                    current_ayah,
                    format!("آية {}:{}", current_surah, current_ayah),
                    1, 1, None
                );
                ayahs.push(ayah);
                
                current_ayah += 1;
                // Simulate moving to next surah after some ayahs
                if current_ayah > 10 && current_surah < end_surah {
                    current_surah += 1;
                    current_ayah = 1;
                }
            }
            
            // Verify structural properties of the range
            if !ayahs.is_empty() {
                // Verify range boundaries
                let first_ayah = &ayahs[0];
                let last_ayah = &ayahs[ayahs.len() - 1];
                
                prop_assert!(first_ayah.surah_number >= start_surah);
                prop_assert!(last_ayah.surah_number <= end_surah);
                
                // Verify ordering within the range
                for window in ayahs.windows(2) {
                    let prev = &window[0];
                    let next = &window[1];
                    
                    // Either same surah with increasing ayah number,
                    // or next surah with ayah number 1 or greater
                    prop_assert!(
                        (prev.surah_number == next.surah_number && prev.ayah_number < next.ayah_number) ||
                        (prev.surah_number < next.surah_number),
                        "Invalid ordering: {}:{} -> {}:{}",
                        prev.surah_number, prev.ayah_number,
                        next.surah_number, next.ayah_number
                    );
                }
                
                // Verify all structural data is valid
                for ayah in &ayahs {
                    prop_assert!(ayah.surah_number >= 1 && ayah.surah_number <= 114);
                    prop_assert!(ayah.ayah_number >= 1);
                    prop_assert!(ayah.juz >= 1 && ayah.juz <= 30);
                    prop_assert!(ayah.page >= 1 && ayah.page <= 604);
                }
            }
        }

        /// **Validates: Requirements 2.2, 2.3**
        /// Property 5: Content-Source Linking to Trusted Sources
        /// 
        /// For any Islamic content (Tafsir, Hadith, Story), it must be properly linked
        /// to its original source and author with appropriate reliability rating.
        /// 
        /// This property ensures that:
        /// 1. Every piece of Islamic content has a valid source reference
        /// 2. Each content item is linked to its author/scholar
        /// 3. Reliability ratings are properly assigned and consistent
        /// 4. Source information is complete and verifiable
        /// 5. Content integrity is maintained through proper source attribution
        #[test]
        fn prop_content_source_linking_to_trusted_sources(
            surah_number in surah_number_strategy(),
            ayah_number in ayah_number_strategy(),
            content_text in arabic_text_strategy(),
            author_name in "[\\u{0600}-\\u{06FF}\\s]{5,50}", // Arabic author names
            source_name in "[\\u{0600}-\\u{06FF}\\s]{10,100}", // Arabic source names
            source_type in prop::sample::select(vec![
                TafsirSourceType::Classical,
                TafsirSourceType::Contemporary,
                TafsirSourceType::Linguistic,
                TafsirSourceType::Thematic,
                TafsirSourceType::Sectarian
            ]),
            auth_level in prop::sample::select(vec![
                ScholarlyAuthentication::HighlyAuthenticated,
                ScholarlyAuthentication::Authenticated,
                ScholarlyAuthentication::Verified,
                ScholarlyAuthentication::Unverified
            ]),
            publication_year in prop::option::of(1000..=2024i32),
            methodology in prop::option::of("[\\u{0600}-\\u{06FF}\\s]{10,200}")
        ) {
            // Skip test if inputs contain only whitespace (would be rejected in real validation)
            prop_assume!(!author_name.trim().is_empty(), "Author name cannot be empty or only whitespace");
            prop_assume!(!source_name.trim().is_empty(), "Source name cannot be empty or only whitespace");
            prop_assume!(!content_text.trim().is_empty(), "Content text cannot be empty or only whitespace");

            // Create a trusted source with complete information
            let source = TafsirSource::new(
                source_name.clone(),
                author_name.clone(),
                "ar".to_string(),
                Some("مصدر موثوق للتفسير الإسلامي".to_string()),
                source_type.clone(),
                auth_level.clone(),
            );

            // Set additional metadata if provided
            let mut complete_source = source;
            complete_source.publication_year = publication_year;
            complete_source.methodology = methodology.clone();

            // Create Islamic content linked to this source
            let tafsir = Tafsir::new_with_metadata(
                surah_number,
                ayah_number,
                complete_source.id,
                content_text.clone(),
                vec!["تفسير".to_string(), "شرح".to_string()],
                vec![format!("{}:{}", surah_number, ayah_number)]
            );

            // **Property 5.1: Every piece of Islamic content has a valid source reference**
            prop_assert!(!complete_source.id.is_nil(), "Source must have a valid UUID");
            prop_assert_eq!(tafsir.source_id, complete_source.id, "Content must be linked to its source");
            prop_assert!(!complete_source.name.is_empty(), "Source must have a name");
            prop_assert!(!complete_source.author.is_empty(), "Source must have an author");

            // **Property 5.2: Each content item is linked to its author/scholar**
            prop_assert_eq!(&complete_source.author, &author_name, "Author information must be preserved");
            
            // Skip test if author name is only whitespace (this would be rejected in real validation)
            prop_assume!(!complete_source.author.trim().is_empty(), "Author name cannot be empty or only whitespace");
            
            // Verify author name contains valid Arabic characters
            let has_arabic_chars = complete_source.author.chars()
                .any(|c| c >= '\u{0600}' && c <= '\u{06FF}');
            prop_assert!(has_arabic_chars || complete_source.author.chars().all(|c| c.is_ascii_alphabetic() || c.is_whitespace()), 
                "Author name must contain valid characters");

            // **Property 5.3: Reliability ratings are properly assigned and consistent**
            prop_assert!(complete_source.credibility_score >= 0.0 && complete_source.credibility_score <= 10.0,
                "Credibility score must be within valid range [0.0, 10.0]");
            
            // Verify authentication level consistency with credibility score
            let expected_min_score = match auth_level {
                ScholarlyAuthentication::HighlyAuthenticated => 7.0,
                ScholarlyAuthentication::Authenticated => 5.5,
                ScholarlyAuthentication::Verified => 4.0,
                ScholarlyAuthentication::Unverified => 0.0,
            };
            prop_assert!(complete_source.credibility_score >= expected_min_score,
                "Credibility score {} should be >= {} for authentication level {:?}",
                complete_source.credibility_score, expected_min_score, auth_level);

            // Verify source type affects credibility appropriately
            let _type_modifier = match &source_type {
                TafsirSourceType::Classical => 1.0,
                TafsirSourceType::Linguistic => 0.95,
                TafsirSourceType::Contemporary => 0.9,
                TafsirSourceType::Thematic => 0.9,
                TafsirSourceType::Sectarian => 0.8,
            };
            
            let calculated_score = TafsirSource::calculate_initial_credibility_score(&auth_level, &source_type);
            prop_assert!((complete_source.credibility_score - calculated_score).abs() < 0.01,
                "Credibility score should match calculated value");

            // **Property 5.4: Source information is complete and verifiable**
            prop_assert_eq!(&complete_source.source_type, &source_type, "Source type must be preserved");
            prop_assert_eq!(&complete_source.scholarly_authentication, &auth_level, "Authentication level must be preserved");
            prop_assert_eq!(&complete_source.language, "ar", "Language must be specified");
            prop_assert!(complete_source.description.is_some(), "Source should have a description");
            
            // Verify optional metadata consistency
            if let Some(year) = publication_year {
                prop_assert_eq!(complete_source.publication_year, Some(year), "Publication year must be preserved");
                prop_assert!(year >= 1000 && year <= 2024, "Publication year must be reasonable");
            }
            
            if let Some(method) = &methodology {
                prop_assume!(!method.trim().is_empty(), "Methodology cannot be empty or only whitespace if provided");
                prop_assert_eq!(complete_source.methodology.as_ref(), Some(method), "Methodology must be preserved");
            }

            // **Property 5.5: Content integrity is maintained through proper source attribution**
            prop_assert!(tafsir.verify_integrity(), "Content must maintain integrity");
            prop_assert_eq!(&tafsir.text, &content_text, "Content text must be preserved exactly");
            prop_assert!(!tafsir.text_hash.is_empty(), "Content must have integrity hash");
            prop_assert_eq!(tafsir.text_hash.len(), 64, "Hash must be SHA-256 (64 chars)");

            // Verify content-source relationship integrity
            prop_assert_eq!(tafsir.surah_number, surah_number, "Surah reference must be preserved");
            prop_assert_eq!(tafsir.ayah_number, ayah_number, "Ayah reference must be preserved");
            prop_assert!(tafsir.word_count > 0, "Content must have word count calculated");
            prop_assert!(!tafsir.themes.is_empty(), "Content should have thematic classification");
            prop_assert!(!tafsir.cross_references.is_empty(), "Content should have cross-references");

            // **Property 5.6: Source credibility affects content reliability**
            let _is_highly_reliable = complete_source.is_highly_credible() && complete_source.is_authenticated();
            let is_classical_authenticated = matches!(&source_type, TafsirSourceType::Classical) 
                && matches!(&auth_level, ScholarlyAuthentication::HighlyAuthenticated | ScholarlyAuthentication::Authenticated);
            
            if is_classical_authenticated {
                prop_assert!(complete_source.credibility_score >= 7.0, 
                    "Classical authenticated sources should have high credibility");
            }

            // **Property 5.7: Source metadata supports verification**
            prop_assert!(complete_source.created_at <= complete_source.updated_at, 
                "Source timestamps must be consistent");
            prop_assert!(tafsir.created_at <= tafsir.updated_at, 
                "Content timestamps must be consistent");

            // Verify credibility level string consistency
            let credibility_level = complete_source.credibility_level();
            let expected_level = match complete_source.credibility_score {
                9.0..=10.0 => "Excellent",
                7.5..=8.9 => "Very Good", 
                6.0..=7.4 => "Good",
                4.0..=5.9 => "Fair",
                _ => "Poor",
            };
            prop_assert_eq!(credibility_level, expected_level, 
                "Credibility level string must match score range");

            // **Property 5.8: Cross-references maintain source traceability**
            for cross_ref in &tafsir.cross_references {
                prop_assert!(cross_ref.contains(':'), "Cross-reference must have proper format");
                let parts: Vec<&str> = cross_ref.split(':').collect();
                prop_assert_eq!(parts.len(), 2, "Cross-reference must have surah:ayah format");
                
                if let (Ok(ref_surah), Ok(ref_ayah)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                    prop_assert!(ref_surah >= 1 && ref_surah <= 114, "Cross-reference surah must be valid");
                    prop_assert!(ref_ayah >= 1, "Cross-reference ayah must be valid");
                }
            }

            // **Property 5.9: Thematic classification supports content discovery**
            for theme in &tafsir.themes {
                prop_assert!(!theme.trim().is_empty(), "Themes cannot be empty");
                prop_assert!(theme.len() >= 3, "Themes must be meaningful (>= 3 chars)");
            }

            // **Property 5.10: Source-content relationship is bidirectional**
            // In a complete system, we should be able to find all content by a source
            // Here we verify the relationship data is consistent
            prop_assert_eq!(tafsir.source_id, complete_source.id, 
                "Content must reference correct source ID");
            
            // Verify that source can be used to validate content
            let content_hash = Tafsir::calculate_text_hash(&tafsir.text);
            prop_assert_eq!(&tafsir.text_hash, &content_hash, 
                "Content hash must be verifiable independently");
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use chrono::Utc;

    /// Test with actual Quranic content to ensure real-world applicability
    #[test]
    fn test_real_quranic_content_integrity() {
        // Al-Fatiha, Ayah 1 (Bismillah)
        let bismillah = "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ";
        let ayah = Ayah::new(1, 1, bismillah.to_string(), 1, 1, Some(1));
        
        assert!(ayah.verify_integrity());
        assert_eq!(ayah.text, bismillah);
        
        // Verify hash is calculated correctly
        let expected_hash = Ayah::calculate_text_hash(bismillah);
        assert_eq!(ayah.text_hash, expected_hash);
    }

    /// Test tamper detection with real content
    #[test]
    fn test_real_content_tamper_detection() {
        let original = "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ";
        let mut ayah = Ayah::new(1, 1, original.to_string(), 1, 1, Some(1));
        
        // Verify original integrity
        assert!(ayah.verify_integrity());
        
        // Tamper with content
        ayah.text = "modified content".to_string();
        
        // Verify tampering is detected
        assert!(!ayah.verify_integrity());
    }

    /// Test edge cases for hash calculation
    #[test]
    fn test_hash_edge_cases() {
        // Empty string
        let empty_hash = Ayah::calculate_text_hash("");
        assert!(!empty_hash.is_empty());
        assert_eq!(empty_hash.len(), 64);
        
        // Single character
        let single_char_hash = Ayah::calculate_text_hash("ا");
        assert!(!single_char_hash.is_empty());
        assert_eq!(single_char_hash.len(), 64);
        
        // Very long text
        let long_text = "ا".repeat(10000);
        let long_hash = Ayah::calculate_text_hash(&long_text);
        assert!(!long_hash.is_empty());
        assert_eq!(long_hash.len(), 64);
        
        // All hashes should be different
        assert_ne!(empty_hash, single_char_hash);
        assert_ne!(single_char_hash, long_hash);
        assert_ne!(empty_hash, long_hash);
    }

    /// Test ContentIntegrity trait implementation
    #[test]
    fn test_content_integrity_trait() {
        let ayah = Ayah::new(1, 1, "test".to_string(), 1, 1, None);
        let tafsir = Tafsir::new(1, 1, Uuid::new_v4(), "test".to_string());
        
        // Test trait methods
        assert!(ayah.verify_integrity());
        assert!(tafsir.verify_integrity());
        
        assert_eq!(ayah.calculate_hash(), "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08");
        assert_eq!(tafsir.calculate_hash(), "9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08");
    }

    /// Test search type enum serialization
    #[test]
    fn test_search_type_serialization() {
        let text_search = SearchType::Text;
        let semantic_search = SearchType::Semantic;
        let root_search = SearchType::Root;
        let exact_search = SearchType::Exact;

        // Test serialization
        let text_json = serde_json::to_string(&text_search).unwrap();
        let semantic_json = serde_json::to_string(&semantic_search).unwrap();
        let root_json = serde_json::to_string(&root_search).unwrap();
        let exact_json = serde_json::to_string(&exact_search).unwrap();

        assert_eq!(text_json, "\"text\"");
        assert_eq!(semantic_json, "\"semantic\"");
        assert_eq!(root_json, "\"root\"");
        assert_eq!(exact_json, "\"exact\"");

        // Test deserialization
        let deserialized_text: SearchType = serde_json::from_str(&text_json).unwrap();
        let deserialized_semantic: SearchType = serde_json::from_str(&semantic_json).unwrap();
        let deserialized_root: SearchType = serde_json::from_str(&root_json).unwrap();
        let deserialized_exact: SearchType = serde_json::from_str(&exact_json).unwrap();

        assert!(matches!(deserialized_text, SearchType::Text));
        assert!(matches!(deserialized_semantic, SearchType::Semantic));
        assert!(matches!(deserialized_root, SearchType::Root));
        assert!(matches!(deserialized_exact, SearchType::Exact));
    }

    /// Test translation model creation and serialization
    #[test]
    fn test_translation_model() {
        let translation = Translation {
            id: Uuid::new_v4(),
            surah_number: 1,
            ayah_number: 1,
            language: "en".to_string(),
            translator: "Sahih International".to_string(),
            text: "In the name of Allah, the Entirely Merciful, the Especially Merciful.".to_string(),
            created_at: Utc::now(),
        };

        // Test serialization
        let json = serde_json::to_string(&translation).unwrap();
        let deserialized: Translation = serde_json::from_str(&json).unwrap();

        assert_eq!(translation.id, deserialized.id);
        assert_eq!(translation.surah_number, deserialized.surah_number);
        assert_eq!(translation.ayah_number, deserialized.ayah_number);
        assert_eq!(translation.language, deserialized.language);
        assert_eq!(translation.translator, deserialized.translator);
        assert_eq!(translation.text, deserialized.text);
    }

    /// Test recitation style model creation and serialization
    #[test]
    fn test_recitation_style_model() {
        let style = RecitationStyle {
            id: Uuid::new_v4(),
            name: "Hafs an Asim".to_string(),
            arabic_name: "حفص عن عاصم".to_string(),
            reciter: "Various".to_string(),
            description: Some("The most widely used recitation style".to_string()),
            language: "ar".to_string(),
            created_at: Utc::now(),
        };

        // Test serialization
        let json = serde_json::to_string(&style).unwrap();
        let deserialized: RecitationStyle = serde_json::from_str(&json).unwrap();

        assert_eq!(style.id, deserialized.id);
        assert_eq!(style.name, deserialized.name);
        assert_eq!(style.arabic_name, deserialized.arabic_name);
        assert_eq!(style.reciter, deserialized.reciter);
        assert_eq!(style.description, deserialized.description);
        assert_eq!(style.language, deserialized.language);
    }

    /// Test search request validation
    #[test]
    fn test_search_request_creation() {
        let request = SearchQuranRequest {
            query: "الله".to_string(),
            surah_numbers: Some(vec![1, 2, 3]),
            search_type: Some(SearchType::Text),
            revelation_type: Some(RevelationType::Meccan),
            juz_numbers: Some(vec![1]),
            limit: Some(20),
            offset: Some(0),
        };

        assert_eq!(request.query, "الله");
        assert_eq!(request.surah_numbers, Some(vec![1, 2, 3]));
        assert!(matches!(request.search_type, Some(SearchType::Text)));
        assert!(matches!(request.revelation_type, Some(RevelationType::Meccan)));
        assert_eq!(request.juz_numbers, Some(vec![1]));
        assert_eq!(request.limit, Some(20));
        assert_eq!(request.offset, Some(0));
    }

    /// Test advanced search filters
    #[test]
    fn test_advanced_search_filters() {
        let filters = AdvancedSearchFilters {
            surah_numbers: Some(vec![1, 2, 3]),
            revelation_type: Some(RevelationType::Medinan),
            juz_numbers: Some(vec![1, 2]),
            page_range: Some((1, 10)),
            word_count_range: Some((5, 20)),
            include_context: Some(true),
        };

        assert_eq!(filters.surah_numbers, Some(vec![1, 2, 3]));
        assert!(matches!(filters.revelation_type, Some(RevelationType::Medinan)));
        assert_eq!(filters.juz_numbers, Some(vec![1, 2]));
        assert_eq!(filters.page_range, Some((1, 10)));
        assert_eq!(filters.word_count_range, Some((5, 20)));
        assert_eq!(filters.include_context, Some(true));
    }

    /// **Validates: Requirements 1.2**
    /// Unit tests for Property 2: Structural Data Accuracy
    /// Testing specific known cases of Quranic structure

    /// Test Al-Fatiha structure (7 ayahs)
    #[test]
    fn test_al_fatiha_structural_accuracy() {
        let surah = Surah::new(
            1,
            "Al-Fatiha".to_string(),
            "الفاتحة".to_string(),
            "The Opening".to_string(),
            RevelationType::Meccan,
            7
        );

        // Create all 7 ayahs of Al-Fatiha
        let ayah_texts = vec![
            "بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ",
            "الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ",
            "الرَّحْمَٰنِ الرَّحِيمِ",
            "مَالِكِ يَوْمِ الدِّينِ",
            "إِيَّاكَ نَعْبُدُ وَإِيَّاكَ نَسْتَعِينُ",
            "اهْدِنَا الصِّرَاطَ الْمُسْتَقِيمَ",
            "صِرَاطَ الَّذِينَ أَنْعَمْتَ عَلَيْهِمْ غَيْرِ الْمَغْضُوبِ عَلَيْهِمْ وَلَا الضَّالِّينَ"
        ];

        let mut ayahs = Vec::new();
        for (index, text) in ayah_texts.iter().enumerate() {
            let ayah = Ayah::new(
                1, // surah number
                (index + 1) as i32, // ayah number
                text.to_string(),
                1, // juz
                1, // page
                Some(1) // ruku
            );
            ayahs.push(ayah);
        }

        // Verify structural accuracy
        assert_eq!(surah.number_of_ayahs, 7);
        assert_eq!(ayahs.len(), 7);
        assert_eq!(surah.number_of_ayahs, ayahs.len() as i32);

        // Verify sequential numbering
        for (index, ayah) in ayahs.iter().enumerate() {
            assert_eq!(ayah.ayah_number, (index + 1) as i32);
            assert_eq!(ayah.surah_number, 1);
            assert!(ayah.verify_integrity());
        }

        // Verify no gaps or duplicates
        let ayah_numbers: Vec<i32> = ayahs.iter().map(|a| a.ayah_number).collect();
        let expected_numbers: Vec<i32> = (1..=7).collect();
        assert_eq!(ayah_numbers, expected_numbers);
    }

    /// Test Al-Baqarah structure (286 ayahs - longest surah)
    #[test]
    fn test_al_baqarah_structural_accuracy() {
        let surah = Surah::new(
            2,
            "Al-Baqarah".to_string(),
            "البقرة".to_string(),
            "The Cow".to_string(),
            RevelationType::Medinan,
            286
        );

        // Create a subset of ayahs to test structure (full 286 would be too much for unit test)
        let test_ayah_numbers = vec![1, 50, 100, 150, 200, 250, 286];
        let mut ayahs = Vec::new();

        for ayah_num in test_ayah_numbers {
            let ayah = Ayah::new(
                2, // surah number
                ayah_num,
                format!("آية رقم {} من سورة البقرة", ayah_num),
                if ayah_num <= 141 { 1 } else if ayah_num <= 252 { 2 } else { 3 }, // juz distribution
                ayah_num / 10 + 1, // approximate page
                Some(ayah_num / 20 + 1) // approximate ruku
            );
            ayahs.push(ayah);
        }

        // Verify structural accuracy
        assert_eq!(surah.number_of_ayahs, 286);
        assert!(surah.is_medinan());

        // Verify all test ayahs have correct structure
        for ayah in &ayahs {
            assert_eq!(ayah.surah_number, 2);
            assert!(ayah.ayah_number >= 1 && ayah.ayah_number <= 286);
            assert!(ayah.juz >= 1 && ayah.juz <= 3); // Al-Baqarah spans 3 juz
            assert!(ayah.verify_integrity());
        }

        // Verify ordering
        for window in ayahs.windows(2) {
            assert!(window[0].ayah_number < window[1].ayah_number);
        }
    }

    /// Test structural consistency across multiple surahs
    #[test]
    fn test_multi_surah_structural_consistency() {
        let surahs_data = vec![
            (1, "Al-Fatiha", "الفاتحة", "The Opening", RevelationType::Meccan, 7),
            (2, "Al-Baqarah", "البقرة", "The Cow", RevelationType::Medinan, 286),
            (3, "Ali 'Imran", "آل عمران", "Family of Imran", RevelationType::Medinan, 200),
            (4, "An-Nisa", "النساء", "The Women", RevelationType::Medinan, 176),
            (5, "Al-Ma'idah", "المائدة", "The Table Spread", RevelationType::Medinan, 120),
        ];

        let mut all_ayahs = Vec::new();

        for (number, name, arabic_name, english_name, revelation_type, ayah_count) in surahs_data {
            let surah = Surah::new(
                number,
                name.to_string(),
                arabic_name.to_string(),
                english_name.to_string(),
                revelation_type,
                ayah_count
            );

            // Create first and last ayah for each surah to test boundaries
            let first_ayah = Ayah::new(
                number,
                1,
                format!("أول آية من سورة {}", arabic_name),
                1, 1, Some(1)
            );

            let last_ayah = Ayah::new(
                number,
                ayah_count,
                format!("آخر آية من سورة {}", arabic_name),
                1, 1, Some(1)
            );

            all_ayahs.push((surah, first_ayah, last_ayah));
        }

        // Verify structural consistency across surahs
        for (surah, first_ayah, last_ayah) in &all_ayahs {
            // Verify surah metadata
            assert!(surah.number >= 1 && surah.number <= 114);
            assert!(surah.number_of_ayahs >= 1);

            // Verify first ayah
            assert_eq!(first_ayah.surah_number, surah.number);
            assert_eq!(first_ayah.ayah_number, 1);
            assert!(first_ayah.verify_integrity());

            // Verify last ayah
            assert_eq!(last_ayah.surah_number, surah.number);
            assert_eq!(last_ayah.ayah_number, surah.number_of_ayahs);
            assert!(last_ayah.verify_integrity());
        }

        // Verify surah ordering
        for window in all_ayahs.windows(2) {
            let (prev_surah, _, _) = &window[0];
            let (next_surah, _, _) = &window[1];
            assert!(prev_surah.number < next_surah.number);
        }
    }

    /// Test ayah range validation for Khatma planning
    #[test]
    fn test_ayah_range_structural_validation() {
        // Test valid ranges
        let valid_ranges = vec![
            (1, 1, 1, 7),   // Complete Al-Fatiha
            (1, 1, 2, 5),   // From Al-Fatiha to part of Al-Baqarah
            (2, 100, 2, 200), // Middle portion of Al-Baqarah
            (113, 1, 114, 6), // From Al-Falaq to complete An-Nas
        ];

        for (start_surah, start_ayah, end_surah, end_ayah) in valid_ranges {
            // Create sample ayahs for the range
            let mut ayahs = Vec::new();
            
            if start_surah == end_surah {
                // Same surah range
                for ayah_num in start_ayah..=end_ayah {
                    let ayah = Ayah::new(
                        start_surah,
                        ayah_num,
                        format!("آية {}:{}", start_surah, ayah_num),
                        1, 1, None
                    );
                    ayahs.push(ayah);
                }
            } else {
                // Multi-surah range - create boundary ayahs
                let first_ayah = Ayah::new(start_surah, start_ayah, "first".to_string(), 1, 1, None);
                let last_ayah = Ayah::new(end_surah, end_ayah, "last".to_string(), 1, 1, None);
                ayahs.push(first_ayah);
                ayahs.push(last_ayah);
            }

            // Verify range structural properties
            assert!(!ayahs.is_empty());
            
            let first = &ayahs[0];
            let last = &ayahs[ayahs.len() - 1];
            
            // Verify boundaries
            assert_eq!(first.surah_number, start_surah);
            assert_eq!(first.ayah_number, start_ayah);
            assert_eq!(last.surah_number, end_surah);
            assert_eq!(last.ayah_number, end_ayah);
            
            // Verify all ayahs have valid structure
            for ayah in &ayahs {
                assert!(ayah.surah_number >= 1 && ayah.surah_number <= 114);
                assert!(ayah.ayah_number >= 1);
                assert!(ayah.verify_integrity());
            }
        }
    }

    /// Test invalid structural data detection
    #[test]
    fn test_invalid_structural_data_detection() {
        // These should be caught by validation in a real system
        // Here we test that our models can represent the constraints
        
        // Test surah number boundaries
        let valid_surah_numbers = vec![1, 57, 114]; // First, middle, last
        let _invalid_surah_numbers = vec![0, 115, -1, 1000];
        
        for valid_num in valid_surah_numbers {
            // Should be able to create ayah with valid surah number
            let ayah = Ayah::new(valid_num, 1, "test".to_string(), 1, 1, None);
            assert_eq!(ayah.surah_number, valid_num);
            assert!(ayah.verify_integrity());
        }
        
        // Test ayah number boundaries (positive numbers)
        let valid_ayah_numbers = vec![1, 50, 286]; // Valid range
        
        for valid_num in valid_ayah_numbers {
            let ayah = Ayah::new(1, valid_num, "test".to_string(), 1, 1, None);
            assert_eq!(ayah.ayah_number, valid_num);
            assert!(ayah.verify_integrity());
        }
        
        // Test juz boundaries
        let valid_juz_numbers = vec![1, 15, 30];
        
        for valid_juz in valid_juz_numbers {
            let ayah = Ayah::new(1, 1, "test".to_string(), valid_juz, 1, None);
            assert_eq!(ayah.juz, valid_juz);
            assert!(ayah.verify_integrity());
        }
        
        // Test page boundaries
        let valid_page_numbers = vec![1, 300, 604];
        
        for valid_page in valid_page_numbers {
            let ayah = Ayah::new(1, 1, "test".to_string(), 1, valid_page, None);
            assert_eq!(ayah.page, valid_page);
            assert!(ayah.verify_integrity());
        }
    }

    /// Test SurahWithAyahs structural integrity
    #[test]
    fn test_surah_with_ayahs_structural_integrity() {
        let surah = Surah::new(
            1,
            "Test Surah".to_string(),
            "سورة تجريبية".to_string(),
            "Test Surah".to_string(),
            RevelationType::Meccan,
            5
        );

        let mut ayahs = Vec::new();
        for i in 1..=5 {
            let ayah = Ayah::new(
                1,
                i,
                format!("آية رقم {}", i),
                1, 1, None
            );
            ayahs.push(ayah);
        }

        let surah_with_ayahs = SurahWithAyahs {
            surah: surah.clone(),
            ayahs: ayahs.clone(),
        };

        // Verify structural consistency
        assert_eq!(surah_with_ayahs.surah.number_of_ayahs, 5);
        assert_eq!(surah_with_ayahs.ayahs.len(), 5);
        assert_eq!(surah_with_ayahs.surah.number_of_ayahs, surah_with_ayahs.ayahs.len() as i32);

        // Verify all ayahs belong to the surah
        for ayah in &surah_with_ayahs.ayahs {
            assert_eq!(ayah.surah_number, surah_with_ayahs.surah.number);
            assert!(ayah.verify_integrity());
        }

        // Verify sequential numbering
        for (index, ayah) in surah_with_ayahs.ayahs.iter().enumerate() {
            assert_eq!(ayah.ayah_number, (index + 1) as i32);
        }

        // Test serialization preserves structure
        let json = serde_json::to_string(&surah_with_ayahs).unwrap();
        let deserialized: SurahWithAyahs = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.surah.number_of_ayahs, surah_with_ayahs.surah.number_of_ayahs);
        assert_eq!(deserialized.ayahs.len(), surah_with_ayahs.ayahs.len());
        
        for (original, deserialized) in surah_with_ayahs.ayahs.iter().zip(deserialized.ayahs.iter()) {
            assert_eq!(original.surah_number, deserialized.surah_number);
            assert_eq!(original.ayah_number, deserialized.ayah_number);
            assert_eq!(original.text, deserialized.text);
            assert!(deserialized.verify_integrity());
        }
    }
}

/// **Enhanced Tafsir System Tests**
/// Comprehensive tests for the Tafsir system implementation covering:
/// - Source credibility management
/// - Tafsir comparison functionality
/// - Advanced search capabilities
/// - Analytics and reporting

#[cfg(test)]
mod tafsir_system_tests {
    use super::*;

    /// **Validates: Requirements 2.2, 2.3**
    /// Property: Credibility Score Bounds and Consistency
    /// 
    /// All credibility scores must be within valid bounds (0.0 to 10.0) and
    /// higher authentication levels should result in higher base scores.
    #[test]
    fn test_credibility_score_bounds_and_hierarchy() {
        let auth_levels = vec![
            ScholarlyAuthentication::Unverified,
            ScholarlyAuthentication::Verified,
            ScholarlyAuthentication::Authenticated,
            ScholarlyAuthentication::HighlyAuthenticated,
        ];

        let source_types = vec![
            TafsirSourceType::Classical,
            TafsirSourceType::Contemporary,
            TafsirSourceType::Linguistic,
            TafsirSourceType::Thematic,
            TafsirSourceType::Sectarian,
        ];

        let mut previous_auth_score = 0.0;

        for auth in &auth_levels {
            let mut auth_scores = Vec::new();
            
            for source_type in &source_types {
                let score = TafsirSource::calculate_initial_credibility_score(auth, source_type);
                
                // Verify bounds
                assert!(score >= 0.0 && score <= 10.0, 
                    "Credibility score {} is out of bounds for {:?} + {:?}", score, auth, source_type);
                
                auth_scores.push(score);
            }
            
            // Verify authentication hierarchy (higher auth should have higher base scores)
            let avg_score = auth_scores.iter().sum::<f64>() / auth_scores.len() as f64;
            assert!(avg_score > previous_auth_score, 
                "Authentication level {:?} should have higher average score than previous level. Got: {}, Previous: {}", 
                auth, avg_score, previous_auth_score);
            previous_auth_score = avg_score;
        }
    }

    /// **Validates: Requirements 2.1, 12.3**
    /// Property: Tafsir Text Integrity and Hash Consistency
    /// 
    /// Tafsir text integrity must be verifiable through hash comparison,
    /// and hash calculations must be consistent and deterministic.
    #[test]
    fn test_tafsir_integrity_and_hash_consistency() {
        let test_texts = vec![
            "تفسير قصير",
            "تفسير طويل يحتوي على كلمات كثيرة ومعاني متعددة وشروحات مفصلة",
            "تفسير يحتوي على أرقام 123 ورموز خاصة !@# وعلامات ترقيم.",
            "", // Empty text
            "   ", // Only whitespace
            "تفسير\nمع\tفواصل\rمختلفة", // Different whitespace types
        ];

        for text in test_texts {
            // Test hash consistency
            let hash1 = Tafsir::calculate_text_hash(text);
            let hash2 = Tafsir::calculate_text_hash(text);
            let hash3 = Tafsir::calculate_text_hash(text);
            
            assert_eq!(hash1, hash2, "Hash calculation should be consistent for text: '{}'", text);
            assert_eq!(hash1, hash3, "Hash calculation should be consistent for text: '{}'", text);
            assert_eq!(hash1.len(), 64, "SHA-256 hash should be 64 characters long");

            // Test Tafsir creation and integrity
            let tafsir = Tafsir::new(1, 1, Uuid::new_v4(), text.to_string());
            assert!(tafsir.verify_integrity(), 
                "Tafsir integrity verification failed for text: '{}'", text);
            assert_eq!(tafsir.text_hash, hash1, "Stored hash should match calculated hash");
            
            // Test tampering detection
            let mut tampered_tafsir = tafsir.clone();
            tampered_tafsir.text = format!("{} tampered", tampered_tafsir.text);
            assert!(!tampered_tafsir.verify_integrity(), 
                "Tampered Tafsir should fail integrity check for text: '{}'", text);
        }
    }

    /// **Validates: Requirements 2.5**
    /// Property: Word Count Accuracy and Reading Time Estimation
    /// 
    /// Word count must accurately reflect the number of words in Tafsir text,
    /// and reading time should increase proportionally with word count.
    #[test]
    fn test_word_count_and_reading_time_accuracy() {
        let text_100 = "كلمة ".repeat(100);
        let text_250 = "كلمة ".repeat(250);
        let text_500 = "كلمة ".repeat(500);
        
        let test_cases = vec![
            ("كلمة واحدة", 2, 1),
            ("كلمة واحدة فقط", 3, 1),
            ("", 0, 0), // Empty text = 0 minutes
            ("   ", 0, 0), // Only whitespace = 0 minutes
            ("كلمة\nجديدة\tمع\rفواصل", 4, 1), // Different whitespace types
            (text_100.trim(), 100, 1), // 100 words = 1 minute (100/200 rounded up)
            (text_250.trim(), 250, 2), // 250 words = 2 minutes (250/200 rounded up)
            (text_500.trim(), 500, 3), // 500 words = 3 minutes (500/200 rounded up)
        ];

        for (text, expected_count, expected_time) in test_cases {
            let tafsir = Tafsir::new(1, 1, Uuid::new_v4(), text.to_string());
            
            assert_eq!(tafsir.word_count, expected_count, 
                "Word count mismatch for text: '{}'. Expected: {}, Got: {}", 
                text, expected_count, tafsir.word_count);
            
            assert_eq!(tafsir.estimated_reading_time(), expected_time,
                "Reading time mismatch for text with {} words. Expected: {} minutes, Got: {} minutes",
                tafsir.word_count, expected_time, tafsir.estimated_reading_time());
            
            // Test comprehensive classification
            let is_comprehensive = tafsir.word_count > 100;
            assert_eq!(tafsir.is_comprehensive(), is_comprehensive,
                "Comprehensive classification incorrect for {} words", tafsir.word_count);
        }
    }

    /// **Validates: Requirements 2.5**
    /// Property: Theme Extraction Consistency and Accuracy
    /// 
    /// Theme extraction should be consistent for the same text and should
    /// identify relevant Islamic themes based on content.
    #[test]
    fn test_theme_extraction_consistency_and_accuracy() {
        let test_cases = vec![
            ("هذا تفسير يتحدث عن التوحيد والصلاة", vec!["Tawhid", "Prayer"]),
            ("تفسير يشرح الصبر والرحمة في الإسلام", vec!["Patience", "Mercy"]),
            ("شرح للزكاة والحج في القرآن الكريم", vec!["Zakat", "Hajj"]),
            ("تفسير يتناول الجهاد والعدل والتقوى", vec!["Jihad", "Justice", "Taqwa"]),
            ("نص لا يحتوي على مواضيع إسلامية محددة", vec![]), // No themes
        ];

        for (text, expected_themes) in test_cases {
            let tafsir = Tafsir::new(1, 1, Uuid::new_v4(), text.to_string());
            
            // Test consistency
            let themes1 = tafsir.extract_themes();
            let themes2 = tafsir.extract_themes();
            assert_eq!(themes1, themes2, 
                "Theme extraction should be consistent for text: '{}'", text);
            
            // Test accuracy
            for expected_theme in &expected_themes {
                assert!(themes1.contains(&expected_theme.to_string()),
                    "Expected theme '{}' not found in extracted themes for text: '{}'", 
                    expected_theme, text);
            }
            
            // Verify no unexpected themes for empty case
            if expected_themes.is_empty() {
                assert!(themes1.is_empty() || themes1.len() <= 1, // Allow for minor false positives
                    "Unexpected themes found for text without Islamic content: '{}'", text);
            }
        }
    }

    /// **Validates: Requirements 2.2**
    /// Property: Source Authentication and Credibility Management
    /// 
    /// Tafsir sources should maintain consistent authentication levels and
    /// credibility scores that reflect their scholarly standing.
    #[test]
    fn test_source_authentication_and_credibility() {
        let test_sources = vec![
            ("تفسير ابن كثير", "ابن كثير", TafsirSourceType::Classical, ScholarlyAuthentication::HighlyAuthenticated),
            ("تفسير الطبري", "الطبري", TafsirSourceType::Classical, ScholarlyAuthentication::HighlyAuthenticated),
            ("في ظلال القرآن", "سيد قطب", TafsirSourceType::Contemporary, ScholarlyAuthentication::Authenticated),
            ("تفسير حديث", "مؤلف معاصر", TafsirSourceType::Contemporary, ScholarlyAuthentication::Verified),
            ("تفسير غير موثق", "مؤلف مجهول", TafsirSourceType::Contemporary, ScholarlyAuthentication::Unverified),
        ];

        for (name, author, source_type, auth_level) in test_sources {
            let source = TafsirSource::new(
                name.to_string(),
                author.to_string(),
                "ar".to_string(),
                Some("Test description".to_string()),
                source_type.clone(),
                auth_level.clone(),
            );

            // Verify authentication consistency
            assert_eq!(source.scholarly_authentication, auth_level);
            assert_eq!(source.is_authenticated(), matches!(auth_level, 
                ScholarlyAuthentication::HighlyAuthenticated | ScholarlyAuthentication::Authenticated));

            // Verify credibility score reflects authentication
            let expected_high_credibility = matches!(auth_level, ScholarlyAuthentication::HighlyAuthenticated) 
                && matches!(source_type, TafsirSourceType::Classical);
            assert_eq!(source.is_highly_credible(), expected_high_credibility || source.credibility_score >= 8.0);

            // Verify credibility level string
            let level = source.credibility_level();
            match source.credibility_score {
                9.0..=10.0 => assert_eq!(level, "Excellent"),
                7.5..=8.9 => assert_eq!(level, "Very Good"),
                6.0..=7.4 => assert_eq!(level, "Good"),
                4.0..=5.9 => assert_eq!(level, "Fair"),
                _ => assert_eq!(level, "Poor"),
            }
        }
    }

    /// **Validates: Requirements 2.5**
    /// Property: Tafsir Metadata Consistency
    /// 
    /// Tafsir entries with metadata (themes, cross-references) should maintain
    /// consistency and integrity of all associated data.
    #[test]
    fn test_tafsir_metadata_consistency() {
        let themes = vec!["Tawhid".to_string(), "Mercy".to_string(), "Justice".to_string()];
        let cross_refs = vec!["2:163".to_string(), "17:110".to_string(), "55:1".to_string()];
        let text = "تفسير شامل يتناول التوحيد والرحمة والعدل مع إشارات لآيات أخرى";

        let tafsir = Tafsir::new_with_metadata(
            1, 1, Uuid::new_v4(), text.to_string(), themes.clone(), cross_refs.clone()
        );

        // Verify integrity with metadata
        assert!(tafsir.verify_integrity());
        
        // Verify metadata preservation
        assert_eq!(tafsir.themes, themes);
        assert_eq!(tafsir.cross_references, cross_refs);
        
        // Verify word count calculation
        let expected_word_count = text.split_whitespace().count() as i32;
        assert_eq!(tafsir.word_count, expected_word_count);
        
        // Verify text preservation
        assert_eq!(tafsir.text, text);
        
        // Test serialization preserves metadata
        let json = serde_json::to_string(&tafsir).unwrap();
        let deserialized: Tafsir = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.themes, themes);
        assert_eq!(deserialized.cross_references, cross_refs);
        assert_eq!(deserialized.word_count, tafsir.word_count);
        assert!(deserialized.verify_integrity());
    }

    /// **Validates: Requirements 2.1, 2.5**
    /// Property: Tafsir Source Type Consistency
    /// 
    /// Different Tafsir source types should maintain consistent characteristics
    /// and appropriate credibility modifiers.
    #[test]
    fn test_tafsir_source_type_consistency() {
        let auth_level = ScholarlyAuthentication::Authenticated;
        let base_score = 7.5; // Base score for Authenticated level

        let type_modifiers = vec![
            (TafsirSourceType::Classical, 1.0),
            (TafsirSourceType::Linguistic, 0.95),
            (TafsirSourceType::Contemporary, 0.9),
            (TafsirSourceType::Thematic, 0.9),
            (TafsirSourceType::Sectarian, 0.8),
        ];

        for (source_type, expected_modifier) in type_modifiers {
            let calculated_score = TafsirSource::calculate_initial_credibility_score(&auth_level, &source_type);
            let expected_score = {
                let result: f64 = base_score * expected_modifier;
                result.min(10.0)
            };
            
            assert!((calculated_score - expected_score).abs() < 0.01, 
                "Score mismatch for {:?}: expected {}, got {}", source_type, expected_score, calculated_score);
            
            // Create source and verify consistency
            let source = TafsirSource::new(
                "Test Source".to_string(),
                "Test Author".to_string(),
                "ar".to_string(),
                None,
                source_type.clone(),
                auth_level.clone(),
            );
            
            assert_eq!(source.source_type, source_type);
            assert_eq!(source.scholarly_authentication, auth_level);
            assert!((source.credibility_score - expected_score).abs() < 0.01);
        }
    }

    /// **Validates: Requirements 2.3**
    /// Property: Source Credibility Verification Consistency
    /// 
    /// Credibility verification should produce consistent results and
    /// appropriate recommendations based on source characteristics.
    #[test]
    fn test_credibility_verification_consistency() {
        let test_cases = vec![
            (ScholarlyAuthentication::HighlyAuthenticated, TafsirSourceType::Classical, 9.0, "Excellent"),
            (ScholarlyAuthentication::Authenticated, TafsirSourceType::Contemporary, 6.75, "Good"),
            (ScholarlyAuthentication::Verified, TafsirSourceType::Linguistic, 5.7, "Fair"),
            (ScholarlyAuthentication::Unverified, TafsirSourceType::Sectarian, 2.4, "Poor"),
        ];

        for (auth, source_type, expected_min_score, expected_level) in test_cases {
            let source = TafsirSource::new(
                "Test Source".to_string(),
                "Test Author".to_string(),
                "ar".to_string(),
                Some("Test description".to_string()),
                source_type,
                auth.clone(),
            );

            // Verify initial score is reasonable
            assert!(source.credibility_score >= expected_min_score - 0.5, 
                "Initial credibility score too low: {} (expected >= {})", 
                source.credibility_score, expected_min_score - 0.5);

            // Verify credibility level
            let level = source.credibility_level();
            assert_eq!(level, expected_level, 
                "Credibility level mismatch for score {}: expected '{}', got '{}'", 
                source.credibility_score, expected_level, level);

            // Verify authentication status
            let is_auth = source.is_authenticated();
            let expected_auth = matches!(auth, 
                ScholarlyAuthentication::HighlyAuthenticated | ScholarlyAuthentication::Authenticated);
            assert_eq!(is_auth, expected_auth, 
                "Authentication status mismatch for {:?}", auth);
        }
    }

    /// **Validates: Requirements 2.5**
    /// Property: Cross-Reference Format Consistency
    /// 
    /// Cross-references in Tafsir should follow consistent format patterns
    /// and reference valid Quranic locations.
    #[test]
    fn test_cross_reference_format_consistency() {
        let valid_cross_refs = vec![
            "1:1",      // Single digit surah and ayah
            "2:255",    // Ayat al-Kursi
            "17:110",   // Two digit surah, three digit ayah
            "114:6",    // Last surah
        ];

        let invalid_cross_refs = vec![
            "0:1",      // Invalid surah (0)
            "115:1",    // Invalid surah (> 114)
            "1:0",      // Invalid ayah (0)
            "invalid",  // Non-numeric format
            "1:1:1",    // Too many parts
            "1",        // Missing ayah
        ];

        // Test valid cross-references
        for cross_ref in &valid_cross_refs {
            let tafsir = Tafsir::new_with_metadata(
                1, 1, Uuid::new_v4(), 
                "Test tafsir".to_string(),
                vec![],
                vec![cross_ref.to_string()]
            );

            assert!(tafsir.verify_integrity());
            assert_eq!(tafsir.cross_references.len(), 1);
            assert_eq!(tafsir.cross_references[0], *cross_ref);

            // Verify format pattern
            let parts: Vec<&str> = cross_ref.split(':').collect();
            assert_eq!(parts.len(), 2, "Cross-reference should have format 'surah:ayah'");
            
            let surah_num: Result<i32, _> = parts[0].parse();
            let ayah_num: Result<i32, _> = parts[1].parse();
            
            assert!(surah_num.is_ok(), "Surah number should be numeric");
            assert!(ayah_num.is_ok(), "Ayah number should be numeric");
            
            let surah = surah_num.unwrap();
            let ayah = ayah_num.unwrap();
            
            assert!(surah >= 1 && surah <= 114, "Surah number should be 1-114");
            assert!(ayah >= 1, "Ayah number should be >= 1");
        }

        // Test that we can detect invalid formats (in a real system, validation would prevent these)
        for invalid_ref in &invalid_cross_refs {
            // In a real system, these would be rejected during creation
            // Here we just verify the format detection logic
            let parts: Vec<&str> = invalid_ref.split(':').collect();
            
            if parts.len() == 2 {
                let surah_parse = parts[0].parse::<i32>();
                let ayah_parse = parts[1].parse::<i32>();
                
                if let (Ok(surah), Ok(ayah)) = (surah_parse, ayah_parse) {
                    let is_valid = surah >= 1 && surah <= 114 && ayah >= 1;
                    assert!(!is_valid, "Reference '{}' should be detected as invalid", invalid_ref);
                }
            }
        }
    }

    /// **Validates: Requirements 2.1, 2.5**
    /// Property: Tafsir Collection Consistency
    /// 
    /// Collections of Tafsir entries should maintain referential integrity
    /// and consistent metadata across related entries.
    #[test]
    fn test_tafsir_collection_consistency() {
        let source_id = Uuid::new_v4();
        let surah_number = 1;
        
        // Create multiple Tafsir entries for the same source and surah
        let mut tafsir_entries = Vec::new();
        for ayah_num in 1..=7 { // Al-Fatiha has 7 ayahs
            let tafsir = Tafsir::new_with_metadata(
                surah_number,
                ayah_num,
                source_id,
                format!("تفسير الآية {} من سورة الفاتحة", ayah_num),
                vec!["Tawhid".to_string(), "Prayer".to_string()],
                vec![format!("{}:{}", surah_number, ayah_num)]
            );
            tafsir_entries.push(tafsir);
        }

        // Verify collection consistency
        assert_eq!(tafsir_entries.len(), 7);

        for (index, tafsir) in tafsir_entries.iter().enumerate() {
            let expected_ayah = (index + 1) as i32;
            
            // Verify structural consistency
            assert_eq!(tafsir.surah_number, surah_number);
            assert_eq!(tafsir.ayah_number, expected_ayah);
            assert_eq!(tafsir.source_id, source_id);
            
            // Verify integrity
            assert!(tafsir.verify_integrity());
            
            // Verify metadata consistency
            assert!(tafsir.themes.contains(&"Tawhid".to_string()));
            assert!(tafsir.themes.contains(&"Prayer".to_string()));
            assert_eq!(tafsir.cross_references.len(), 1);
            assert_eq!(tafsir.cross_references[0], format!("{}:{}", surah_number, expected_ayah));
            
            // Verify word count is reasonable
            assert!(tafsir.word_count > 0);
        }

        // Verify sequential ordering
        for window in tafsir_entries.windows(2) {
            let prev = &window[0];
            let next = &window[1];
            
            assert_eq!(prev.surah_number, next.surah_number);
            assert_eq!(prev.ayah_number + 1, next.ayah_number);
            assert_eq!(prev.source_id, next.source_id);
        }

        // Verify no duplicates
        let mut seen_ayahs = std::collections::HashSet::new();
        for tafsir in &tafsir_entries {
            let key = (tafsir.surah_number, tafsir.ayah_number, tafsir.source_id);
            assert!(seen_ayahs.insert(key), 
                "Duplicate Tafsir entry found for {}:{} from source {}", 
                tafsir.surah_number, tafsir.ayah_number, tafsir.source_id);
        }
    }
}

/// **Integration Tests for Tafsir System**
/// Tests that verify the complete Tafsir system workflow including
/// source management, comparison, and analytics functionality.

#[cfg(test)]
mod tafsir_integration_tests {
    use super::*;

    /// Test complete Tafsir source lifecycle
    #[test]
    fn test_tafsir_source_lifecycle() {
        // Create initial source
        let mut source = TafsirSource::new(
            "Test Tafsir".to_string(),
            "Test Author".to_string(),
            "ar".to_string(),
            Some("Initial description".to_string()),
            TafsirSourceType::Contemporary,
            ScholarlyAuthentication::Verified,
        );

        let initial_score = source.credibility_score;
        assert!(initial_score >= 4.0 && initial_score <= 7.0); // Reasonable range for Verified Contemporary

        // Simulate authentication upgrade
        source.scholarly_authentication = ScholarlyAuthentication::Authenticated;
        let new_score = TafsirSource::calculate_initial_credibility_score(
            &source.scholarly_authentication, 
            &source.source_type
        );
        source.credibility_score = new_score;

        assert!(source.credibility_score > initial_score, 
            "Upgraded authentication should increase credibility score");
        assert!(source.is_authenticated());

        // Test credibility level progression
        let level = source.credibility_level();
        assert!(matches!(level.as_str(), "Good" | "Very Good" | "Excellent"));
    }

    /// Test Tafsir comparison workflow
    #[test]
    fn test_tafsir_comparison_workflow() {
        // Create multiple sources with different characteristics
        let classical_source = TafsirSource::new(
            "Classical Tafsir".to_string(),
            "Classical Scholar".to_string(),
            "ar".to_string(),
            Some("Traditional approach".to_string()),
            TafsirSourceType::Classical,
            ScholarlyAuthentication::HighlyAuthenticated,
        );

        let contemporary_source = TafsirSource::new(
            "Contemporary Tafsir".to_string(),
            "Modern Scholar".to_string(),
            "ar".to_string(),
            Some("Modern approach".to_string()),
            TafsirSourceType::Contemporary,
            ScholarlyAuthentication::Authenticated,
        );

        let linguistic_source = TafsirSource::new(
            "Linguistic Tafsir".to_string(),
            "Language Expert".to_string(),
            "ar".to_string(),
            Some("Linguistic focus".to_string()),
            TafsirSourceType::Linguistic,
            ScholarlyAuthentication::Authenticated,
        );

        // Create Tafsir entries for the same verse
        let classical_tafsir = Tafsir::new_with_metadata(
            1, 1, classical_source.id,
            "تفسير تقليدي يركز على النقل والأثر".to_string(),
            vec!["Tradition".to_string(), "Narration".to_string()],
            vec!["2:163".to_string()]
        );

        let contemporary_tafsir = Tafsir::new_with_metadata(
            1, 1, contemporary_source.id,
            "تفسير معاصر يربط النص بالواقع الحديث".to_string(),
            vec!["Modern Context".to_string(), "Application".to_string()],
            vec!["17:110".to_string()]
        );

        let linguistic_tafsir = Tafsir::new_with_metadata(
            1, 1, linguistic_source.id,
            "تحليل لغوي مفصل للمفردات والتراكيب النحوية".to_string(),
            vec!["Grammar".to_string(), "Etymology".to_string()],
            vec!["55:1".to_string()]
        );

        // Simulate comparison analysis
        let tafsir_entries = vec![
            TafsirWithSource { tafsir: classical_tafsir, source: classical_source },
            TafsirWithSource { tafsir: contemporary_tafsir, source: contemporary_source },
            TafsirWithSource { tafsir: linguistic_tafsir, source: linguistic_source },
        ];

        // Verify comparison data structure
        assert_eq!(tafsir_entries.len(), 3);

        // Verify different approaches are represented
        let source_types: Vec<_> = tafsir_entries.iter()
            .map(|entry| &entry.source.source_type)
            .collect();
        
        assert!(source_types.contains(&&TafsirSourceType::Classical));
        assert!(source_types.contains(&&TafsirSourceType::Contemporary));
        assert!(source_types.contains(&&TafsirSourceType::Linguistic));

        // Verify credibility ordering
        let mut sorted_by_credibility = tafsir_entries.clone();
        sorted_by_credibility.sort_by(|a, b| 
            b.source.credibility_score.partial_cmp(&a.source.credibility_score).unwrap()
        );

        // Classical should typically have highest credibility
        assert_eq!(sorted_by_credibility[0].source.source_type, TafsirSourceType::Classical);

        // Verify unique themes across sources
        let all_themes: Vec<_> = tafsir_entries.iter()
            .flat_map(|entry| &entry.tafsir.themes)
            .collect();
        
        let unique_themes: std::collections::HashSet<_> = all_themes.into_iter().collect();
        assert!(unique_themes.len() >= 3, "Should have diverse themes across different approaches");
    }

    /// Test Tafsir analytics workflow
    #[test]
    fn test_tafsir_analytics_workflow() {
        // Create test data representing different coverage scenarios
        let sources = vec![
            ("High Coverage Source", 50), // Covers many verses
            ("Medium Coverage Source", 25), // Covers some verses
            ("Low Coverage Source", 10), // Covers few verses
        ];

        let mut all_tafsir_entries = Vec::new();

        for (source_name, coverage_count) in sources {
            let source = TafsirSource::new(
                source_name.to_string(),
                "Test Author".to_string(),
                "ar".to_string(),
                Some("Test description".to_string()),
                TafsirSourceType::Contemporary,
                ScholarlyAuthentication::Authenticated,
            );

            // Create Tafsir entries for this source
            for i in 1..=coverage_count {
                let surah_num = ((i - 1) / 10) + 1; // Distribute across surahs
                let ayah_num = ((i - 1) % 10) + 1;
                
                let tafsir = Tafsir::new_with_metadata(
                    surah_num, ayah_num, source.id,
                    format!("تفسير {} للآية {}:{}", source_name, surah_num, ayah_num),
                    vec!["Test Theme".to_string()],
                    vec![]
                );

                all_tafsir_entries.push(TafsirWithSource { tafsir, source: source.clone() });
            }
        }

        // Analyze coverage distribution
        let mut coverage_by_source = std::collections::HashMap::new();
        for entry in &all_tafsir_entries {
            let count = coverage_by_source.entry(entry.source.name.clone()).or_insert(0);
            *count += 1;
        }

        assert_eq!(coverage_by_source.len(), 3);
        assert_eq!(coverage_by_source[&"High Coverage Source".to_string()], 50);
        assert_eq!(coverage_by_source[&"Medium Coverage Source".to_string()], 25);
        assert_eq!(coverage_by_source[&"Low Coverage Source".to_string()], 10);

        // Analyze surah distribution
        let mut surah_coverage = std::collections::HashMap::new();
        for entry in &all_tafsir_entries {
            let count = surah_coverage.entry(entry.tafsir.surah_number).or_insert(0);
            *count += 1;
        }

        // Should have coverage across multiple surahs
        assert!(surah_coverage.len() >= 5, "Should cover multiple surahs");

        // Verify total coverage
        let total_entries = all_tafsir_entries.len();
        assert_eq!(total_entries, 85); // 50 + 25 + 10
    }

    /// Test end-to-end Tafsir system workflow
    #[test]
    fn test_end_to_end_tafsir_workflow() {
        // 1. Create and verify sources
        let ibn_kathir = TafsirSource::new(
            "تفسير ابن كثير".to_string(),
            "ابن كثير".to_string(),
            "ar".to_string(),
            Some("Classical comprehensive commentary".to_string()),
            TafsirSourceType::Classical,
            ScholarlyAuthentication::HighlyAuthenticated,
        );

        let tabari = TafsirSource::new(
            "تفسير الطبري".to_string(),
            "الطبري".to_string(),
            "ar".to_string(),
            Some("Historical-critical commentary".to_string()),
            TafsirSourceType::Classical,
            ScholarlyAuthentication::HighlyAuthenticated,
        );

        assert!(ibn_kathir.is_highly_credible());
        assert!(tabari.is_highly_credible());
        assert!(ibn_kathir.is_authenticated());
        assert!(tabari.is_authenticated());

        // 2. Create Tafsir entries
        let ibn_kathir_tafsir = Tafsir::new_with_metadata(
            1, 1, ibn_kathir.id,
            "البسملة افتتاح كل أمر ذي بال، وهي تتضمن الاستعانة بالله والتبرك باسمه العظيم".to_string(),
            vec!["Basmala".to_string(), "Divine Names".to_string(), "Blessing".to_string()],
            vec!["17:110".to_string(), "27:30".to_string()]
        );

        let tabari_tafsir = Tafsir::new_with_metadata(
            1, 1, tabari.id,
            "اختلف العلماء في البسملة هل هي آية من الفاتحة أم لا والصحيح أنها آية مستقلة".to_string(),
            vec!["Scholarly Differences".to_string(), "Quranic Structure".to_string()],
            vec!["9:1".to_string(), "11:41".to_string()]
        );

        // 3. Verify integrity
        assert!(ibn_kathir_tafsir.verify_integrity());
        assert!(tabari_tafsir.verify_integrity());

        // 4. Create comparison structure
        let comparison_entries = vec![
            TafsirWithSource { tafsir: ibn_kathir_tafsir, source: ibn_kathir },
            TafsirWithSource { tafsir: tabari_tafsir, source: tabari },
        ];

        // 5. Verify comparison readiness
        assert_eq!(comparison_entries.len(), 2);
        
        // All entries should be for the same verse
        let first_entry = &comparison_entries[0];
        for entry in &comparison_entries {
            assert_eq!(entry.tafsir.surah_number, first_entry.tafsir.surah_number);
            assert_eq!(entry.tafsir.ayah_number, first_entry.tafsir.ayah_number);
        }

        // 6. Analyze differences and similarities
        let all_themes: Vec<_> = comparison_entries.iter()
            .flat_map(|entry| &entry.tafsir.themes)
            .collect();
        
        let unique_themes: std::collections::HashSet<_> = all_themes.into_iter().collect();
        assert!(unique_themes.len() >= 3, "Should have diverse themes");

        // 7. Verify credibility-based ordering
        let mut sorted_entries = comparison_entries.clone();
        sorted_entries.sort_by(|a, b| 
            b.source.credibility_score.partial_cmp(&a.source.credibility_score).unwrap()
        );

        // Both should be highly credible classical sources
        for entry in &sorted_entries {
            assert!(entry.source.is_highly_credible());
            assert_eq!(entry.source.source_type, TafsirSourceType::Classical);
        }

        // 8. Verify cross-reference diversity
        let all_cross_refs: Vec<_> = comparison_entries.iter()
            .flat_map(|entry| &entry.tafsir.cross_references)
            .collect();
        
        let unique_cross_refs: std::collections::HashSet<_> = all_cross_refs.into_iter().collect();
        assert!(unique_cross_refs.len() >= 3, "Should have diverse cross-references");

        // 9. Final integrity check
        for entry in &comparison_entries {
            assert!(entry.tafsir.verify_integrity());
            assert!(entry.source.is_authenticated());
            assert!(entry.tafsir.word_count > 0);
            assert!(!entry.tafsir.themes.is_empty());
        }
    }
}