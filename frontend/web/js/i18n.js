/**
 * Internationalization (i18n) Manager for Sanad Islamic App
 * Handles language switching, text direction, and translations
 */

window.SanadI18n = {
    
    /**
     * Current language and settings
     */
    currentLanguage: 'ar',
    currentDirection: 'rtl',
    translations: {},
    loadedNamespaces: new Set(),
    
    /**
     * Initialize i18n system
     */
    async init() {
        // Load saved language preference
        const savedLanguage = window.SanadUtils.storage.get(window.SanadConfig.storage.language);
        if (savedLanguage && window.SanadConfig.languages[savedLanguage]) {
            this.currentLanguage = savedLanguage;
        }
        
        // Set initial language
        await this.setLanguage(this.currentLanguage);
        
        // Load common translations
        await this.loadNamespace('common');
        
        console.log('i18n system initialized with language:', this.currentLanguage);
    },
    
    /**
     * Set current language
     */
    async setLanguage(languageCode) {
        if (!window.SanadConfig.languages[languageCode]) {
            console.warn('Unsupported language:', languageCode);
            return false;
        }
        
        const language = window.SanadConfig.languages[languageCode];
        const previousLanguage = this.currentLanguage;
        
        this.currentLanguage = languageCode;
        this.currentDirection = language.direction;
        
        // Update document attributes
        document.documentElement.setAttribute('lang', languageCode);
        document.documentElement.setAttribute('dir', language.direction);
        
        // Update body class
        document.body.className = document.body.className
            .replace(/lang-\w+/g, '')
            .trim() + ` lang-${languageCode}`;
        
        // Update font family
        document.body.style.fontFamily = language.fontFamily;
        
        // Save language preference
        window.SanadUtils.storage.set(window.SanadConfig.storage.language, languageCode);
        
        // Clear translation cache if language changed
        if (previousLanguage !== languageCode) {
            this.translations = {};
            this.loadedNamespaces.clear();
        }
        
        // Notify language change
        this.dispatchLanguageChange(languageCode, previousLanguage);
        
        return true;
    },
    
    /**
     * Load translations for a namespace
     */
    async loadNamespace(namespace = 'common') {
        if (this.loadedNamespaces.has(`${this.currentLanguage}:${namespace}`)) {
            return true;
        }
        
        try {
            const response = await window.SanadAPI.i18n.getBulkTranslations(
                this.getNamespaceKeys(namespace),
                this.currentLanguage,
                namespace
            );
            
            if (response && response.translations) {
                if (!this.translations[namespace]) {
                    this.translations[namespace] = {};
                }
                
                Object.assign(this.translations[namespace], response.translations);
                this.loadedNamespaces.add(`${this.currentLanguage}:${namespace}`);
                
                console.log(`Loaded ${namespace} translations for ${this.currentLanguage}`);
                return true;
            }
        } catch (error) {
            console.error('Failed to load translations:', error);
            // Use fallback translations
            this.loadFallbackTranslations(namespace);
        }
        
        return false;
    },
    
    /**
     * Get translation for a key
     */
    t(key, options = {}) {
        const {
            namespace = 'common',
            interpolation = {},
            plural = null,
            fallback = key
        } = options;
        
        // Get translation from cache
        let translation = this.getTranslationFromCache(key, namespace);
        
        // Use fallback if not found
        if (!translation) {
            translation = this.getFallbackTranslation(key, namespace) || fallback;
        }
        
        // Handle pluralization
        if (plural !== null && typeof plural === 'number') {
            translation = this.handlePluralization(translation, plural);
        }
        
        // Handle interpolation
        if (Object.keys(interpolation).length > 0) {
            translation = this.interpolate(translation, interpolation);
        }
        
        return translation;
    },
    
    /**
     * Get translation from cache
     */
    getTranslationFromCache(key, namespace) {
        if (this.translations[namespace] && this.translations[namespace][key]) {
            const translation = this.translations[namespace][key];
            return typeof translation === 'object' ? translation.value : translation;
        }
        return null;
    },
    
    /**
     * Get fallback translation
     */
    getFallbackTranslation(key, namespace) {
        const fallbackTranslations = this.getFallbackTranslations();
        
        if (fallbackTranslations[namespace] && fallbackTranslations[namespace][key]) {
            return fallbackTranslations[namespace][key];
        }
        
        return null;
    },
    
    /**
     * Handle pluralization
     */
    handlePluralization(translation, count) {
        if (typeof translation !== 'object' || !translation.plural) {
            return translation;
        }
        
        const pluralRules = this.getPluralRules(this.currentLanguage);
        const form = pluralRules(count);
        
        return translation.plural[form] || translation.value || translation;
    },
    
    /**
     * Get plural rules for language
     */
    getPluralRules(language) {
        const rules = {
            ar: (n) => {
                if (n === 0) return 'zero';
                if (n === 1) return 'one';
                if (n === 2) return 'two';
                if (n >= 3 && n <= 10) return 'few';
                return 'many';
            },
            en: (n) => n === 1 ? 'one' : 'other',
            ur: (n) => n === 1 ? 'one' : 'other',
            tr: (n) => n === 1 ? 'one' : 'other',
            fr: (n) => n <= 1 ? 'one' : 'other'
        };
        
        return rules[language] || rules.en;
    },
    
    /**
     * Interpolate variables in translation
     */
    interpolate(translation, variables) {
        let result = translation;
        
        Object.keys(variables).forEach(key => {
            const regex = new RegExp(`{{\\s*${key}\\s*}}`, 'g');
            result = result.replace(regex, variables[key]);
        });
        
        return result;
    },
    
    /**
     * Update all translatable elements on the page
     */
    updatePageTranslations() {
        // Update elements with data-i18n attribute
        const elements = document.querySelectorAll('[data-i18n]');
        elements.forEach(element => {
            const key = element.getAttribute('data-i18n');
            const namespace = element.getAttribute('data-i18n-ns') || 'common';
            const interpolation = this.parseInterpolationData(element);
            
            const translation = this.t(key, { namespace, interpolation });
            
            if (element.tagName === 'INPUT' || element.tagName === 'TEXTAREA') {
                element.placeholder = translation;
            } else {
                element.textContent = translation;
            }
        });
        
        // Update elements with data-i18n-html attribute (for HTML content)
        const htmlElements = document.querySelectorAll('[data-i18n-html]');
        htmlElements.forEach(element => {
            const key = element.getAttribute('data-i18n-html');
            const namespace = element.getAttribute('data-i18n-ns') || 'common';
            const interpolation = this.parseInterpolationData(element);
            
            const translation = this.t(key, { namespace, interpolation });
            element.innerHTML = translation;
        });
        
        // Update title and meta tags
        this.updateDocumentMeta();
    },
    
    /**
     * Parse interpolation data from element attributes
     */
    parseInterpolationData(element) {
        const interpolationAttr = element.getAttribute('data-i18n-interpolation');
        if (!interpolationAttr) return {};
        
        try {
            return JSON.parse(interpolationAttr);
        } catch (error) {
            console.warn('Invalid interpolation data:', interpolationAttr);
            return {};
        }
    },
    
    /**
     * Update document meta information
     */
    updateDocumentMeta() {
        const titleKey = document.documentElement.getAttribute('data-i18n-title');
        if (titleKey) {
            document.title = this.t(titleKey);
        }
        
        const descriptionKey = document.documentElement.getAttribute('data-i18n-description');
        if (descriptionKey) {
            const metaDescription = document.querySelector('meta[name="description"]');
            if (metaDescription) {
                metaDescription.setAttribute('content', this.t(descriptionKey));
            }
        }
    },
    
    /**
     * Format number according to current locale
     */
    formatNumber(number, options = {}) {
        const locale = this.getLocaleCode();
        return new Intl.NumberFormat(locale, options).format(number);
    },
    
    /**
     * Format date according to current locale
     */
    formatDate(date, options = {}) {
        const locale = this.getLocaleCode();
        const defaultOptions = {
            year: 'numeric',
            month: 'long',
            day: 'numeric'
        };
        
        return new Intl.DateTimeFormat(locale, { ...defaultOptions, ...options }).format(date);
    },
    
    /**
     * Format time according to current locale
     */
    formatTime(date, options = {}) {
        const locale = this.getLocaleCode();
        const defaultOptions = {
            hour: '2-digit',
            minute: '2-digit',
            hour12: false
        };
        
        return new Intl.DateTimeFormat(locale, { ...defaultOptions, ...options }).format(date);
    },
    
    /**
     * Get locale code for Intl APIs
     */
    getLocaleCode() {
        const localeMap = {
            ar: 'ar-SA',
            en: 'en-US',
            ur: 'ur-PK',
            tr: 'tr-TR',
            fr: 'fr-FR'
        };
        
        return localeMap[this.currentLanguage] || 'en-US';
    },
    
    /**
     * Get text direction for current language
     */
    getDirection() {
        return this.currentDirection;
    },
    
    /**
     * Check if current language is RTL
     */
    isRTL() {
        return this.currentDirection === 'rtl';
    },
    
    /**
     * Get available languages
     */
    getAvailableLanguages() {
        return Object.keys(window.SanadConfig.languages).map(code => ({
            code,
            ...window.SanadConfig.languages[code]
        }));
    },
    
    /**
     * Dispatch language change event
     */
    dispatchLanguageChange(newLanguage, previousLanguage) {
        const event = new CustomEvent('languageChanged', {
            detail: {
                newLanguage,
                previousLanguage,
                direction: this.currentDirection
            }
        });
        
        document.dispatchEvent(event);
    },
    
    /**
     * Get namespace keys (for loading translations)
     */
    getNamespaceKeys(namespace) {
        const commonKeys = [
            'appTitle', 'appSubtitle', 'loading', 'error', 'success',
            'save', 'cancel', 'delete', 'edit', 'search', 'close',
            'yes', 'no', 'ok', 'back', 'next', 'previous', 'home'
        ];
        
        const namespaceKeys = {
            common: commonKeys,
            navigation: [
                'dashboard', 'quran', 'hadith', 'stories', 'prayerTimes', 'aiAssistant'
            ],
            prayers: [
                'fajr', 'dhuhr', 'asr', 'maghrib', 'isha', 'sunrise',
                'prayerTimes', 'nextPrayer', 'timeRemaining'
            ],
            quran: [
                'quranTitle', 'surah', 'ayah', 'verse', 'tafsir', 'translation',
                'continueReading', 'searchQuran'
            ],
            hadith: [
                'hadithTitle', 'narrator', 'chain', 'grade', 'sahih', 'hasan',
                'daif', 'mawdu', 'randomHadith', 'searchHadith'
            ],
            stories: [
                'storiesTitle', 'prophets', 'companions', 'lessons', 'morals',
                'randomStory', 'searchStories'
            ],
            ai: [
                'aiAssistantTitle', 'askQuestion', 'clearChat', 'thinking',
                'sources', 'confidence', 'aiWelcome'
            ]
        };
        
        return namespaceKeys[namespace] || commonKeys;
    },
    
    /**
     * Get fallback translations (hardcoded for offline use)
     */
    getFallbackTranslations() {
        return {
            common: {
                appTitle: {
                    ar: 'سند - التطبيق الإسلامي الشامل',
                    en: 'Sanad - Comprehensive Islamic App',
                    ur: 'سند - جامع اسلامی ایپ',
                    tr: 'Sanad - Kapsamlı İslami Uygulama',
                    fr: 'Sanad - Application Islamique Complète'
                },
                loading: {
                    ar: 'جاري التحميل...',
                    en: 'Loading...',
                    ur: 'لوڈ ہو رہا ہے...',
                    tr: 'Yükleniyor...',
                    fr: 'Chargement...'
                },
                error: {
                    ar: 'حدث خطأ',
                    en: 'An error occurred',
                    ur: 'خرابی ہوئی',
                    tr: 'Bir hata oluştu',
                    fr: 'Une erreur s\'est produite'
                },
                search: {
                    ar: 'البحث',
                    en: 'Search',
                    ur: 'تلاش',
                    tr: 'Ara',
                    fr: 'Rechercher'
                }
            },
            navigation: {
                dashboard: {
                    ar: 'الرئيسية',
                    en: 'Dashboard',
                    ur: 'ڈیش بورڈ',
                    tr: 'Ana Sayfa',
                    fr: 'Tableau de bord'
                },
                quran: {
                    ar: 'القرآن الكريم',
                    en: 'Holy Quran',
                    ur: 'قرآن مجید',
                    tr: 'Kur\'an-ı Kerim',
                    fr: 'Saint Coran'
                },
                hadith: {
                    ar: 'الأحاديث',
                    en: 'Hadiths',
                    ur: 'احادیث',
                    tr: 'Hadisler',
                    fr: 'Hadiths'
                }
            }
        };
    },
    
    /**
     * Load fallback translations
     */
    loadFallbackTranslations(namespace) {
        const fallbackTranslations = this.getFallbackTranslations();
        
        if (fallbackTranslations[namespace]) {
            if (!this.translations[namespace]) {
                this.translations[namespace] = {};
            }
            
            Object.keys(fallbackTranslations[namespace]).forEach(key => {
                const translations = fallbackTranslations[namespace][key];
                this.translations[namespace][key] = translations[this.currentLanguage] || translations.en || key;
            });
            
            console.log(`Loaded fallback translations for ${namespace}`);
        }
    }
};

// Initialize i18n when DOM is ready
window.SanadUtils.timing.ready(() => {
    window.SanadI18n.init();
});

// Listen for language change events
document.addEventListener('languageChanged', (event) => {
    console.log('Language changed:', event.detail);
    
    // Update page translations
    setTimeout(() => {
        window.SanadI18n.updatePageTranslations();
    }, 100);
});

// Freeze the i18n object to prevent modifications
Object.freeze(window.SanadI18n);