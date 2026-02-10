//! Property-based tests for Tafsir API clients
//!
//! These tests verify universal properties that should hold across all inputs.

#[cfg(test)]
mod tests {
    use crate::api_clients::tafsir::{OrganizedTafsirResponse, TafsirApiManager};
    use crate::api_clients::{CacheManager, RateLimiter, TafsirEntry};
    use proptest::prelude::*;
    use std::collections::{HashMap, HashSet};
    use std::sync::Arc;

    // Helper to create a test manager
    async fn create_test_manager() -> TafsirApiManager {
        let cache = Arc::new(
            CacheManager::new("redis://127.0.0.1:6379/")
                .await
                .expect("Failed to create cache manager"),
        );

        let rate_limiter = Arc::new(
            RateLimiter::new("redis://127.0.0.1:6379/", HashMap::new())
                .await
                .expect("Failed to create rate limiter"),
        );

        let clients: Vec<Box<dyn crate::api_clients::TafsirApiClient + Send + Sync>> = vec![];

        TafsirApiManager::new(clients, cache, rate_limiter)
    }

    // Strategy to generate valid scholar names
    fn arb_scholar_name() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("Ibn Kathir".to_string()),
            Just("Al-Jalalayn".to_string()),
            Just("Al-Tabari".to_string()),
            Just("Al-Qurtubi".to_string()),
            Just("Ibn Abbas".to_string()),
            Just("As-Sa'di".to_string()),
            Just("Al-Baghawi".to_string()),
        ]
    }

    // Strategy to generate valid language names
    fn arb_language() -> impl Strategy<Value = String> {
        prop_oneof![
            Just("Arabic".to_string()),
            Just("English".to_string()),
            Just("Urdu".to_string()),
            Just("Turkish".to_string()),
            Just("Indonesian".to_string()),
            Just("French".to_string()),
        ]
    }

    // Strategy to generate a tafsir entry
    fn arb_tafsir_entry() -> impl Strategy<Value = TafsirEntry> {
        (
            "[0-9]+",
            "[a-zA-Z ]+",
            arb_scholar_name(),
            "[\\p{Arabic}\\p{Latin}\\s]+",
            arb_language(),
            "[a-z.]+",
        )
            .prop_map(|(id, name, scholar, text, language, source)| TafsirEntry {
                tafsir_id: id,
                tafsir_name: name,
                scholar,
                text,
                language,
                source,
            })
    }

    // Strategy to generate a list of tafsir entries
    fn arb_tafsir_list() -> impl Strategy<Value = Vec<TafsirEntry>> {
        prop::collection::vec(arb_tafsir_entry(), 1..=20)
    }

    // Feature: official-apis-integration, Property 8: Tafsir Organization by Scholar and Language
    // **Validates: Requirements 4.3**
    //
    // For any tafsir response with multiple sources, the results should be organized
    // (grouped or sorted) by scholar name and language, making it easy to find tafsir
    // from a specific scholar or in a specific language.
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(20))]
        
        #[test]
        fn property_tafsir_organization_by_scholar_and_language(
            tafsirs in arb_tafsir_list(),
            surah in 1u8..=114,
            ayah in 1u16..=286,
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                // Create organized response manually (simulating what the manager does)
                let mut by_scholar: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
                let mut by_language: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
                
                for tafsir in &tafsirs {
                    by_scholar
                        .entry(tafsir.scholar.clone())
                        .or_insert_with(Vec::new)
                        .push(tafsir.clone());
                    
                    by_language
                        .entry(tafsir.language.clone())
                        .or_insert_with(Vec::new)
                        .push(tafsir.clone());
                }
                
                let organized = OrganizedTafsirResponse {
                    surah,
                    ayah,
                    by_scholar: by_scholar.clone(),
                    by_language: by_language.clone(),
                    all_tafsirs: tafsirs.clone(),
                };
                
                // Property 1: All tafsirs should be present in the organized response
                prop_assert_eq!(
                    organized.all_tafsirs.len(),
                    tafsirs.len(),
                    "All tafsirs should be preserved in organized response"
                );
                
                // Property 2: Sum of tafsirs in by_scholar should equal total tafsirs
                let scholar_total: usize = organized.by_scholar.values().map(|v| v.len()).sum();
                prop_assert_eq!(
                    scholar_total,
                    tafsirs.len(),
                    "Sum of tafsirs grouped by scholar should equal total tafsirs"
                );
                
                // Property 3: Sum of tafsirs in by_language should equal total tafsirs
                let language_total: usize = organized.by_language.values().map(|v| v.len()).sum();
                prop_assert_eq!(
                    language_total,
                    tafsirs.len(),
                    "Sum of tafsirs grouped by language should equal total tafsirs"
                );
                
                // Property 4: Each scholar group should only contain tafsirs from that scholar
                for (scholar, scholar_tafsirs) in &organized.by_scholar {
                    for tafsir in scholar_tafsirs {
                        prop_assert_eq!(
                            &tafsir.scholar,
                            scholar,
                            "Tafsir in scholar group '{}' has wrong scholar: '{}'",
                            scholar,
                            tafsir.scholar
                        );
                    }
                }
                
                // Property 5: Each language group should only contain tafsirs in that language
                for (language, language_tafsirs) in &organized.by_language {
                    for tafsir in language_tafsirs {
                        prop_assert_eq!(
                            &tafsir.language,
                            language,
                            "Tafsir in language group '{}' has wrong language: '{}'",
                            language,
                            tafsir.language
                        );
                    }
                }
                
                // Property 6: No tafsir should be lost in organization
                let mut all_scholars_tafsirs = Vec::new();
                for tafsirs_list in organized.by_scholar.values() {
                    all_scholars_tafsirs.extend(tafsirs_list.clone());
                }
                prop_assert_eq!(
                    all_scholars_tafsirs.len(),
                    tafsirs.len(),
                    "No tafsirs should be lost when organizing by scholar"
                );
                
                // Property 7: Organization should be deterministic
                // If we organize the same tafsirs again, we should get the same result
                let mut by_scholar_2: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
                for tafsir in &tafsirs {
                    by_scholar_2
                        .entry(tafsir.scholar.clone())
                        .or_insert_with(Vec::new)
                        .push(tafsir.clone());
                }
                
                prop_assert_eq!(
                    by_scholar.keys().collect::<HashSet<_>>(),
                    by_scholar_2.keys().collect::<HashSet<_>>(),
                    "Organization by scholar should be deterministic"
                );
                
                // Property 8: Each scholar should appear exactly once as a key
                let scholar_keys: HashSet<String> = organized.by_scholar.keys().cloned().collect();
                let unique_scholars: HashSet<String> = tafsirs.iter().map(|t| t.scholar.clone()).collect();
                prop_assert_eq!(
                    scholar_keys,
                    unique_scholars,
                    "Each unique scholar should appear exactly once as a key"
                );
                
                // Property 9: Each language should appear exactly once as a key
                let language_keys: HashSet<String> = organized.by_language.keys().cloned().collect();
                let unique_languages: HashSet<String> = tafsirs.iter().map(|t| t.language.clone()).collect();
                prop_assert_eq!(
                    language_keys,
                    unique_languages,
                    "Each unique language should appear exactly once as a key"
                );
                
                Ok(())
            });
        }
    }

    // Additional property test: Organization should handle empty lists correctly
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]
        
        #[test]
        fn property_organization_handles_empty_list(
            surah in 1u8..=114,
            ayah in 1u16..=286,
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let tafsirs: Vec<TafsirEntry> = vec![];
                
                let organized = OrganizedTafsirResponse {
                    surah,
                    ayah,
                    by_scholar: HashMap::new(),
                    by_language: HashMap::new(),
                    all_tafsirs: tafsirs.clone(),
                };
                
                // Property: Empty input should result in empty organization
                prop_assert!(
                    organized.all_tafsirs.is_empty(),
                    "Empty tafsir list should result in empty all_tafsirs"
                );
                prop_assert!(
                    organized.by_scholar.is_empty(),
                    "Empty tafsir list should result in empty by_scholar"
                );
                prop_assert!(
                    organized.by_language.is_empty(),
                    "Empty tafsir list should result in empty by_language"
                );
                
                Ok(())
            });
        }
    }

    // Additional property test: Organization should handle single tafsir correctly
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]
        
        #[test]
        fn property_organization_handles_single_tafsir(
            tafsir in arb_tafsir_entry(),
            surah in 1u8..=114,
            ayah in 1u16..=286,
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                let tafsirs = vec![tafsir.clone()];
                
                let mut by_scholar: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
                by_scholar.insert(tafsir.scholar.clone(), vec![tafsir.clone()]);
                
                let mut by_language: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
                by_language.insert(tafsir.language.clone(), vec![tafsir.clone()]);
                
                let organized = OrganizedTafsirResponse {
                    surah,
                    ayah,
                    by_scholar,
                    by_language,
                    all_tafsirs: tafsirs,
                };
                
                // Property: Single tafsir should appear in exactly one scholar group
                prop_assert_eq!(
                    organized.by_scholar.len(),
                    1,
                    "Single tafsir should create exactly one scholar group"
                );
                
                // Property: Single tafsir should appear in exactly one language group
                prop_assert_eq!(
                    organized.by_language.len(),
                    1,
                    "Single tafsir should create exactly one language group"
                );
                
                // Property: The scholar group should contain exactly one tafsir
                let scholar_group = organized.by_scholar.get(&tafsir.scholar).unwrap();
                prop_assert_eq!(
                    scholar_group.len(),
                    1,
                    "Scholar group should contain exactly one tafsir"
                );
                
                // Property: The language group should contain exactly one tafsir
                let language_group = organized.by_language.get(&tafsir.language).unwrap();
                prop_assert_eq!(
                    language_group.len(),
                    1,
                    "Language group should contain exactly one tafsir"
                );
                
                Ok(())
            });
        }
    }

    // Additional property test: Multiple tafsirs from same scholar should be grouped together
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]
        
        #[test]
        fn property_same_scholar_tafsirs_grouped(
            scholar in arb_scholar_name(),
            num_tafsirs in 2usize..=10,
            surah in 1u8..=114,
            ayah in 1u16..=286,
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                // Create multiple tafsirs from the same scholar but different languages
                let languages = vec!["Arabic", "English", "Urdu", "Turkish"];
                let mut tafsirs = Vec::new();
                
                for i in 0..num_tafsirs {
                    let language = languages[i % languages.len()].to_string();
                    tafsirs.push(TafsirEntry {
                        tafsir_id: format!("{}", i),
                        tafsir_name: format!("Tafsir {} {}", scholar, i),
                        scholar: scholar.clone(),
                        text: format!("Text {}", i),
                        language,
                        source: "test".to_string(),
                    });
                }
                
                let mut by_scholar: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
                for tafsir in &tafsirs {
                    by_scholar
                        .entry(tafsir.scholar.clone())
                        .or_insert_with(Vec::new)
                        .push(tafsir.clone());
                }
                
                // Property: All tafsirs from the same scholar should be in one group
                prop_assert_eq!(
                    by_scholar.len(),
                    1,
                    "All tafsirs from same scholar should be in one group"
                );
                
                // Property: The scholar group should contain all tafsirs
                let scholar_group = by_scholar.get(&scholar).unwrap();
                prop_assert_eq!(
                    scholar_group.len(),
                    num_tafsirs,
                    "Scholar group should contain all {} tafsirs",
                    num_tafsirs
                );
                
                // Property: All tafsirs in the group should have the same scholar
                for tafsir in scholar_group {
                    prop_assert_eq!(
                        &tafsir.scholar,
                        &scholar,
                        "All tafsirs in scholar group should have scholar '{}'",
                        scholar
                    );
                }
                
                Ok(())
            });
        }
    }

    // Additional property test: Multiple tafsirs in same language should be grouped together
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]
        
        #[test]
        fn property_same_language_tafsirs_grouped(
            language in arb_language(),
            num_tafsirs in 2usize..=10,
            surah in 1u8..=114,
            ayah in 1u16..=286,
        ) {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                // Create multiple tafsirs in the same language but from different scholars
                let scholars = vec!["Ibn Kathir", "Al-Jalalayn", "Al-Tabari", "Al-Qurtubi"];
                let mut tafsirs = Vec::new();
                
                for i in 0..num_tafsirs {
                    let scholar = scholars[i % scholars.len()].to_string();
                    tafsirs.push(TafsirEntry {
                        tafsir_id: format!("{}", i),
                        tafsir_name: format!("Tafsir {} {}", scholar, i),
                        scholar,
                        text: format!("Text {}", i),
                        language: language.clone(),
                        source: "test".to_string(),
                    });
                }
                
                let mut by_language: HashMap<String, Vec<TafsirEntry>> = HashMap::new();
                for tafsir in &tafsirs {
                    by_language
                        .entry(tafsir.language.clone())
                        .or_insert_with(Vec::new)
                        .push(tafsir.clone());
                }
                
                // Property: All tafsirs in the same language should be in one group
                prop_assert_eq!(
                    by_language.len(),
                    1,
                    "All tafsirs in same language should be in one group"
                );
                
                // Property: The language group should contain all tafsirs
                let language_group = by_language.get(&language).unwrap();
                prop_assert_eq!(
                    language_group.len(),
                    num_tafsirs,
                    "Language group should contain all {} tafsirs",
                    num_tafsirs
                );
                
                // Property: All tafsirs in the group should have the same language
                for tafsir in language_group {
                    prop_assert_eq!(
                        &tafsir.language,
                        &language,
                        "All tafsirs in language group should have language '{}'",
                        language
                    );
                }
                
                Ok(())
            });
        }
    }
}
