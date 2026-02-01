/**
 * Application Configuration
 * Contains all configuration settings for the Sanad Islamic App
 */

window.SanadConfig = {
    // API Configuration
    api: {
        baseUrl: window.location.origin,
        endpoints: {
            // I18n Service
            i18n: '/api/i18n',
            translations: '/api/i18n/translations',
            languages: '/api/i18n/languages',
            
            // Quran Service
            quran: '/api/quran',
            surahs: '/api/quran/surahs',
            ayahs: '/api/quran/ayahs',
            tafsir: '/api/quran/tafsir',
            
            // Hadith Service
            hadith: '/api/hadith',
            hadithBooks: '/api/hadith/books',
            hadithSearch: '/api/hadith/search',
            
            // Stories Service
            stories: '/api/stories',
            storyCategories: '/api/stories/categories',
            storySearch: '/api/stories/search',
            
            // Prayer Times Service
            prayerTimes: '/api/prayer-times',
            hijriCalendar: '/api/prayer-times/hijri',
            qibla: '/api/prayer-times/qibla',
            
            // AI Assistant Service
            aiAssistant: '/api/ai-assistant',
            aiChat: '/api/ai-assistant/chat',
            aiSources: '/api/ai-assistant/sources',
            
            // Search Service
            search: '/api/search',
            semanticSearch: '/api/search/semantic',
            
            // Widgets Service
            widgets: '/api/widgets',
            dashboard: '/api/widgets/dashboard',
            
            // User Service
            user: '/api/user',
            preferences: '/api/user/preferences',
            bookmarks: '/api/user/bookmarks'
        },
        timeout: 30000, // 30 seconds
        retryAttempts: 3,
        retryDelay: 1000 // 1 second
    },

    // Supported Languages
    languages: {
        ar: {
            code: 'ar',
            name: 'العربية',
            nativeName: 'العربية',
            direction: 'rtl',
            fontFamily: "'Noto Sans Arabic', 'Amiri', Arial, sans-serif"
        },
        en: {
            code: 'en',
            name: 'English',
            nativeName: 'English',
            direction: 'ltr',
            fontFamily: "'Noto Sans', 'Open Sans', Arial, sans-serif"
        },
        ur: {
            code: 'ur',
            name: 'اردو',
            nativeName: 'اردو',
            direction: 'rtl',
            fontFamily: "'Noto Nastaliq Urdu', 'Jameel Noori Nastaleeq', Arial, sans-serif"
        },
        tr: {
            code: 'tr',
            name: 'Türkçe',
            nativeName: 'Türkçe',
            direction: 'ltr',
            fontFamily: "'Noto Sans', Arial, sans-serif"
        },
        fr: {
            code: 'fr',
            name: 'Français',
            nativeName: 'Français',
            direction: 'ltr',
            fontFamily: "'Noto Sans', Arial, sans-serif"
        }
    },

    // Default Settings
    defaults: {
        language: 'ar',
        theme: 'light',
        prayerCalculationMethod: 'MWL', // Muslim World League
        location: {
            latitude: 21.3891, // Mecca coordinates as default
            longitude: 39.8579,
            city: 'مكة المكرمة',
            country: 'السعودية'
        },
        notifications: {
            prayerTimes: true,
            dhikrReminders: true,
            khatmaProgress: true,
            islamicEvents: true
        },
        display: {
            fontSize: 'medium',
            lineHeight: 'normal',
            arabicFont: 'noto-sans-arabic'
        }
    },

    // Prayer Calculation Methods
    prayerMethods: {
        MWL: {
            name: 'Muslim World League',
            nameAr: 'رابطة العالم الإسلامي',
            fajrAngle: 18,
            ishaAngle: 17
        },
        ISNA: {
            name: 'Islamic Society of North America',
            nameAr: 'الجمعية الإسلامية لأمريكا الشمالية',
            fajrAngle: 15,
            ishaAngle: 15
        },
        Egypt: {
            name: 'Egyptian General Authority of Survey',
            nameAr: 'الهيئة المصرية العامة للمساحة',
            fajrAngle: 19.5,
            ishaAngle: 17.5
        },
        Makkah: {
            name: 'Umm Al-Qura University, Makkah',
            nameAr: 'جامعة أم القرى، مكة',
            fajrAngle: 18.5,
            ishaAngle: 90 // 90 minutes after Maghrib
        },
        Karachi: {
            name: 'University of Islamic Sciences, Karachi',
            nameAr: 'جامعة العلوم الإسلامية، كراتشي',
            fajrAngle: 18,
            ishaAngle: 18
        }
    },

    // Widget Types
    widgetTypes: {
        PRAYER_TIMES: {
            id: 'prayer-times',
            name: 'Prayer Times',
            nameAr: 'مواقيت الصلاة',
            refreshInterval: 60000 // 1 minute
        },
        VERSE_OF_DAY: {
            id: 'verse-of-day',
            name: 'Verse of the Day',
            nameAr: 'آية اليوم',
            refreshInterval: 86400000 // 24 hours
        },
        KHATMA_PROGRESS: {
            id: 'khatma-progress',
            name: 'Khatma Progress',
            nameAr: 'تقدم الختمة',
            refreshInterval: 300000 // 5 minutes
        },
        DHIKR_COUNTER: {
            id: 'dhikr-counter',
            name: 'Dhikr Counter',
            nameAr: 'عداد الأذكار',
            refreshInterval: 0 // No auto refresh
        },
        HIJRI_CALENDAR: {
            id: 'hijri-calendar',
            name: 'Hijri Calendar',
            nameAr: 'التقويم الهجري',
            refreshInterval: 3600000 // 1 hour
        },
        QUICK_STATS: {
            id: 'quick-stats',
            name: 'Quick Stats',
            nameAr: 'الإحصائيات السريعة',
            refreshInterval: 300000 // 5 minutes
        }
    },

    // Storage Keys
    storage: {
        language: 'sanad_language',
        theme: 'sanad_theme',
        userPreferences: 'sanad_user_preferences',
        location: 'sanad_location',
        bookmarks: 'sanad_bookmarks',
        readingProgress: 'sanad_reading_progress',
        dhikrCounts: 'sanad_dhikr_counts',
        lastSync: 'sanad_last_sync'
    },

    // UI Configuration
    ui: {
        animationDuration: 300,
        debounceDelay: 500,
        searchMinLength: 2,
        maxSearchResults: 50,
        paginationSize: 20,
        notificationDuration: 5000,
        loadingTimeout: 10000
    },

    // Feature Flags
    features: {
        offlineMode: true,
        voiceSearch: false,
        audioRecitation: true,
        socialSharing: true,
        darkMode: true,
        notifications: true,
        geolocation: true,
        analytics: false // Disabled for privacy
    },

    // Error Messages
    errors: {
        network: {
            ar: 'خطأ في الاتصال بالشبكة',
            en: 'Network connection error'
        },
        timeout: {
            ar: 'انتهت مهلة الاتصال',
            en: 'Request timeout'
        },
        notFound: {
            ar: 'المحتوى غير موجود',
            en: 'Content not found'
        },
        serverError: {
            ar: 'خطأ في الخادم',
            en: 'Server error'
        },
        unauthorized: {
            ar: 'غير مصرح بالوصول',
            en: 'Unauthorized access'
        },
        validation: {
            ar: 'بيانات غير صحيحة',
            en: 'Invalid data'
        }
    },

    // Success Messages
    success: {
        saved: {
            ar: 'تم الحفظ بنجاح',
            en: 'Saved successfully'
        },
        updated: {
            ar: 'تم التحديث بنجاح',
            en: 'Updated successfully'
        },
        deleted: {
            ar: 'تم الحذف بنجاح',
            en: 'Deleted successfully'
        },
        synced: {
            ar: 'تم التزامن بنجاح',
            en: 'Synced successfully'
        }
    },

    // Validation Rules
    validation: {
        minSearchLength: 2,
        maxSearchLength: 100,
        maxBookmarkTitle: 200,
        maxNoteLength: 1000,
        allowedFileTypes: ['pdf', 'txt', 'docx'],
        maxFileSize: 5 * 1024 * 1024 // 5MB
    },

    // Performance Settings
    performance: {
        lazyLoadThreshold: 100, // pixels
        imageQuality: 0.8,
        cacheSize: 50 * 1024 * 1024, // 50MB
        maxConcurrentRequests: 6,
        prefetchDelay: 2000 // 2 seconds
    },

    // Security Settings
    security: {
        csrfTokenName: 'X-CSRF-Token',
        sessionTimeout: 30 * 60 * 1000, // 30 minutes
        maxLoginAttempts: 5,
        lockoutDuration: 15 * 60 * 1000 // 15 minutes
    },

    // Development Settings
    development: {
        debug: false,
        mockApi: false,
        logLevel: 'warn', // 'debug', 'info', 'warn', 'error'
        enablePerformanceMetrics: false
    }
};

// Freeze the configuration to prevent modifications
Object.freeze(window.SanadConfig);

// Export for module systems
if (typeof module !== 'undefined' && module.exports) {
    module.exports = window.SanadConfig;
}