/**
 * API Client for Sanad Islamic App
 * Handles all communication with backend services
 */

window.SanadAPI = {
    
    /**
     * Base API configuration
     */
    config: {
        baseUrl: window.SanadConfig.api.baseUrl,
        timeout: window.SanadConfig.api.timeout,
        retryAttempts: window.SanadConfig.api.retryAttempts,
        retryDelay: window.SanadConfig.api.retryDelay
    },

    /**
     * HTTP Client with retry logic and error handling
     */
    http: {
        /**
         * Make HTTP request with retry logic
         */
        async request(url, options = {}) {
            const config = {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                    'Accept': 'application/json',
                    'Accept-Language': window.SanadApp?.currentLanguage || 'ar'
                },
                ...options
            };

            // Add CSRF token if available
            const csrfToken = document.querySelector('meta[name="csrf-token"]')?.getAttribute('content');
            if (csrfToken) {
                config.headers[window.SanadConfig.security.csrfTokenName] = csrfToken;
            }

            let lastError;
            
            for (let attempt = 0; attempt <= window.SanadAPI.config.retryAttempts; attempt++) {
                try {
                    const controller = new AbortController();
                    const timeoutId = setTimeout(() => controller.abort(), window.SanadAPI.config.timeout);
                    
                    config.signal = controller.signal;
                    
                    const response = await fetch(url, config);
                    clearTimeout(timeoutId);
                    
                    if (!response.ok) {
                        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
                    }
                    
                    const contentType = response.headers.get('content-type');
                    if (contentType && contentType.includes('application/json')) {
                        return await response.json();
                    } else {
                        return await response.text();
                    }
                    
                } catch (error) {
                    lastError = error;
                    
                    // Don't retry on certain errors
                    if (error.name === 'AbortError' || 
                        (error.message && error.message.includes('40'))) {
                        break;
                    }
                    
                    // Wait before retry
                    if (attempt < window.SanadAPI.config.retryAttempts) {
                        await new Promise(resolve => 
                            setTimeout(resolve, window.SanadAPI.config.retryDelay * (attempt + 1))
                        );
                    }
                }
            }
            
            throw lastError;
        },

        /**
         * GET request
         */
        async get(url, params = {}) {
            const queryString = new URLSearchParams(params).toString();
            const fullUrl = queryString ? `${url}?${queryString}` : url;
            return this.request(fullUrl);
        },

        /**
         * POST request
         */
        async post(url, data = {}) {
            return this.request(url, {
                method: 'POST',
                body: JSON.stringify(data)
            });
        },

        /**
         * PUT request
         */
        async put(url, data = {}) {
            return this.request(url, {
                method: 'PUT',
                body: JSON.stringify(data)
            });
        },

        /**
         * DELETE request
         */
        async delete(url) {
            return this.request(url, {
                method: 'DELETE'
            });
        }
    },

    /**
     * I18n Service API
     */
    i18n: {
        /**
         * Get translation for a key
         */
        async getTranslation(key, language, namespace = 'common', interpolation = {}) {
            const endpoint = window.SanadConfig.api.endpoints.translations;
            return window.SanadAPI.http.post(endpoint, {
                key,
                language,
                namespace,
                interpolation_values: interpolation
            });
        },

        /**
         * Get bulk translations
         */
        async getBulkTranslations(keys, language, namespace = 'common') {
            const endpoint = `${window.SanadConfig.api.endpoints.translations}/bulk`;
            return window.SanadAPI.http.post(endpoint, {
                keys,
                language,
                namespace
            });
        },

        /**
         * Switch language
         */
        async switchLanguage(newLanguage, userId = null) {
            const endpoint = `${window.SanadConfig.api.endpoints.i18n}/switch-language`;
            return window.SanadAPI.http.post(endpoint, {
                new_language: newLanguage,
                user_id: userId,
                apply_to_interface: true,
                apply_to_content: true
            });
        },

        /**
         * Get supported languages
         */
        async getSupportedLanguages() {
            const endpoint = `${window.SanadConfig.api.endpoints.languages}`;
            return window.SanadAPI.http.get(endpoint);
        }
    },

    /**
     * Quran Service API
     */
    quran: {
        /**
         * Get all surahs
         */
        async getSurahs() {
            const endpoint = window.SanadConfig.api.endpoints.surahs;
            return window.SanadAPI.http.get(endpoint);
        },

        /**
         * Get specific surah
         */
        async getSurah(surahNumber) {
            const endpoint = `${window.SanadConfig.api.endpoints.surahs}/${surahNumber}`;
            return window.SanadAPI.http.get(endpoint);
        },

        /**
         * Get specific ayah
         */
        async getAyah(surahNumber, ayahNumber) {
            const endpoint = `${window.SanadConfig.api.endpoints.ayahs}/${surahNumber}/${ayahNumber}`;
            return window.SanadAPI.http.get(endpoint);
        },

        /**
         * Search in Quran
         */
        async search(query, options = {}) {
            const endpoint = `${window.SanadConfig.api.endpoints.quran}/search`;
            return window.SanadAPI.http.post(endpoint, {
                query,
                ...options
            });
        },

        /**
         * Get tafsir for ayah
         */
        async getTafsir(surahNumber, ayahNumber, tafsirId = 'ibn-kathir') {
            const endpoint = `${window.SanadConfig.api.endpoints.tafsir}/${surahNumber}/${ayahNumber}`;
            return window.SanadAPI.http.get(endpoint, { tafsir_id: tafsirId });
        },

        /**
         * Get translation for ayah
         */
        async getTranslation(surahNumber, ayahNumber, language = 'en') {
            const endpoint = `${window.SanadConfig.api.endpoints.quran}/translation/${surahNumber}/${ayahNumber}`;
            return window.SanadAPI.http.get(endpoint, { language });
        }
    },

    /**
     * Hadith Service API
     */
    hadith: {
        /**
         * Get hadith collections
         */
        async getCollections() {
            const endpoint = window.SanadConfig.api.endpoints.hadithBooks;
            return window.SanadAPI.http.get(endpoint);
        },

        /**
         * Get specific hadith
         */
        async getHadith(hadithId) {
            const endpoint = `${window.SanadConfig.api.endpoints.hadith}/${hadithId}`;
            return window.SanadAPI.http.get(endpoint);
        },

        /**
         * Search hadiths
         */
        async search(query, filters = {}) {
            const endpoint = window.SanadConfig.api.endpoints.hadithSearch;
            return window.SanadAPI.http.post(endpoint, {
                query,
                filters
            });
        },

        /**
         * Get random hadith
         */
        async getRandom(collection = null) {
            const endpoint = `${window.SanadConfig.api.endpoints.hadith}/random`;
            return window.SanadAPI.http.get(endpoint, collection ? { collection } : {});
        },

        /**
         * Get hadiths by topic
         */
        async getByTopic(topic, page = 1, limit = 20) {
            const endpoint = `${window.SanadConfig.api.endpoints.hadith}/topic/${topic}`;
            return window.SanadAPI.http.get(endpoint, { page, limit });
        }
    },

    /**
     * Stories Service API
     */
    stories: {
        /**
         * Get story categories
         */
        async getCategories() {
            const endpoint = window.SanadConfig.api.endpoints.storyCategories;
            return window.SanadAPI.http.get(endpoint);
        },

        /**
         * Get stories by category
         */
        async getByCategory(category, page = 1, limit = 20) {
            const endpoint = `${window.SanadConfig.api.endpoints.stories}/category/${category}`;
            return window.SanadAPI.http.get(endpoint, { page, limit });
        },

        /**
         * Get specific story
         */
        async getStory(storyId) {
            const endpoint = `${window.SanadConfig.api.endpoints.stories}/${storyId}`;
            return window.SanadAPI.http.get(endpoint);
        },

        /**
         * Search stories
         */
        async search(query, filters = {}) {
            const endpoint = window.SanadConfig.api.endpoints.storySearch;
            return window.SanadAPI.http.post(endpoint, {
                query,
                filters
            });
        },

        /**
         * Get random story
         */
        async getRandom(category = null) {
            const endpoint = `${window.SanadConfig.api.endpoints.stories}/random`;
            return window.SanadAPI.http.get(endpoint, category ? { category } : {});
        }
    },

    /**
     * Prayer Times Service API
     */
    prayerTimes: {
        /**
         * Get prayer times for location and date
         */
        async getPrayerTimes(latitude, longitude, date = null, method = 'MWL') {
            const endpoint = window.SanadConfig.api.endpoints.prayerTimes;
            const params = {
                latitude,
                longitude,
                method
            };
            if (date) {
                params.date = date;
            }
            return window.SanadAPI.http.get(endpoint, params);
        },

        /**
         * Get Qibla direction
         */
        async getQiblaDirection(latitude, longitude) {
            const endpoint = window.SanadConfig.api.endpoints.qibla;
            return window.SanadAPI.http.get(endpoint, { latitude, longitude });
        },

        /**
         * Get Hijri calendar
         */
        async getHijriCalendar(date = null) {
            const endpoint = window.SanadConfig.api.endpoints.hijriCalendar;
            return window.SanadAPI.http.get(endpoint, date ? { date } : {});
        },

        /**
         * Convert Gregorian to Hijri
         */
        async convertToHijri(gregorianDate) {
            const endpoint = `${window.SanadConfig.api.endpoints.hijriCalendar}/convert-to-hijri`;
            return window.SanadAPI.http.post(endpoint, { gregorian_date: gregorianDate });
        },

        /**
         * Convert Hijri to Gregorian
         */
        async convertToGregorian(hijriDate) {
            const endpoint = `${window.SanadConfig.api.endpoints.hijriCalendar}/convert-to-gregorian`;
            return window.SanadAPI.http.post(endpoint, { hijri_date: hijriDate });
        }
    },

    /**
     * AI Assistant Service API
     */
    aiAssistant: {
        /**
         * Send message to AI assistant
         */
        async sendMessage(message, context = {}) {
            const endpoint = window.SanadConfig.api.endpoints.aiChat;
            return window.SanadAPI.http.post(endpoint, {
                message,
                context
            });
        },

        /**
         * Get conversation history
         */
        async getHistory(sessionId) {
            const endpoint = `${window.SanadConfig.api.endpoints.aiChat}/history/${sessionId}`;
            return window.SanadAPI.http.get(endpoint);
        },

        /**
         * Clear conversation
         */
        async clearConversation(sessionId) {
            const endpoint = `${window.SanadConfig.api.endpoints.aiChat}/clear/${sessionId}`;
            return window.SanadAPI.http.delete(endpoint);
        },

        /**
         * Get AI sources for verification
         */
        async getSources(query) {
            const endpoint = window.SanadConfig.api.endpoints.aiSources;
            return window.SanadAPI.http.post(endpoint, { query });
        }
    },

    /**
     * Search Service API
     */
    search: {
        /**
         * Universal search across all content
         */
        async search(query, filters = {}) {
            const endpoint = window.SanadConfig.api.endpoints.search;
            return window.SanadAPI.http.post(endpoint, {
                query,
                filters
            });
        },

        /**
         * Semantic search
         */
        async semanticSearch(query, contentTypes = [], threshold = 0.7) {
            const endpoint = window.SanadConfig.api.endpoints.semanticSearch;
            return window.SanadAPI.http.post(endpoint, {
                query,
                content_types: contentTypes,
                threshold
            });
        },

        /**
         * Get search suggestions
         */
        async getSuggestions(partialQuery) {
            const endpoint = `${window.SanadConfig.api.endpoints.search}/suggestions`;
            return window.SanadAPI.http.get(endpoint, { q: partialQuery });
        }
    },

    /**
     * Widgets Service API
     */
    widgets: {
        /**
         * Get user dashboard
         */
        async getDashboard(userId) {
            const endpoint = `${window.SanadConfig.api.endpoints.dashboard}/${userId}`;
            return window.SanadAPI.http.get(endpoint);
        },

        /**
         * Get widget data
         */
        async getWidgetData(widgetId, userId) {
            const endpoint = `${window.SanadConfig.api.endpoints.widgets}/${widgetId}/data`;
            return window.SanadAPI.http.get(endpoint, { user_id: userId });
        },

        /**
         * Refresh widget
         */
        async refreshWidget(widgetId, userId) {
            const endpoint = `${window.SanadConfig.api.endpoints.widgets}/${widgetId}/refresh`;
            return window.SanadAPI.http.post(endpoint, { user_id: userId });
        },

        /**
         * Update widget settings
         */
        async updateWidget(widgetId, settings) {
            const endpoint = `${window.SanadConfig.api.endpoints.widgets}/${widgetId}`;
            return window.SanadAPI.http.put(endpoint, settings);
        }
    },

    /**
     * User Service API
     */
    user: {
        /**
         * Get user preferences
         */
        async getPreferences(userId) {
            const endpoint = `${window.SanadConfig.api.endpoints.preferences}/${userId}`;
            return window.SanadAPI.http.get(endpoint);
        },

        /**
         * Update user preferences
         */
        async updatePreferences(userId, preferences) {
            const endpoint = `${window.SanadConfig.api.endpoints.preferences}/${userId}`;
            return window.SanadAPI.http.put(endpoint, preferences);
        },

        /**
         * Get bookmarks
         */
        async getBookmarks(userId) {
            const endpoint = `${window.SanadConfig.api.endpoints.bookmarks}/${userId}`;
            return window.SanadAPI.http.get(endpoint);
        },

        /**
         * Add bookmark
         */
        async addBookmark(userId, bookmark) {
            const endpoint = window.SanadConfig.api.endpoints.bookmarks;
            return window.SanadAPI.http.post(endpoint, {
                user_id: userId,
                ...bookmark
            });
        },

        /**
         * Remove bookmark
         */
        async removeBookmark(bookmarkId) {
            const endpoint = `${window.SanadConfig.api.endpoints.bookmarks}/${bookmarkId}`;
            return window.SanadAPI.http.delete(endpoint);
        }
    },

    /**
     * Error handling utilities
     */
    error: {
        /**
         * Handle API errors
         */
        handle(error, context = '') {
            console.error(`API Error ${context}:`, error);
            
            let message = 'حدث خطأ غير متوقع';
            
            if (error.name === 'AbortError') {
                message = window.SanadConfig.errors.timeout.ar;
            } else if (error.message.includes('Failed to fetch')) {
                message = window.SanadConfig.errors.network.ar;
            } else if (error.message.includes('404')) {
                message = window.SanadConfig.errors.notFound.ar;
            } else if (error.message.includes('401')) {
                message = window.SanadConfig.errors.unauthorized.ar;
            } else if (error.message.includes('500')) {
                message = window.SanadConfig.errors.serverError.ar;
            }
            
            // Show notification if notification system is available
            if (window.SanadApp && window.SanadApp.showNotification) {
                window.SanadApp.showNotification(message, 'error');
            }
            
            return { error: true, message };
        }
    }
};

// Freeze the API object to prevent modifications
Object.freeze(window.SanadAPI);