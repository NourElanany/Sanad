/**
 * Main Application Controller for Sanad Islamic App
 * Handles app initialization, navigation, and core functionality
 */

window.SanadApp = {

    /**
     * Application state
     */
    state: {
        initialized: false,
        currentSection: 'dashboard',
        currentLanguage: 'ar',
        currentTheme: 'light',
        user: null,
        location: null,
        isOnline: navigator.onLine,
        notifications: []
    },

    /**
     * DOM elements cache
     */
    elements: {},

    /**
     * Event listeners cleanup functions
     */
    cleanupFunctions: [],

    /**
     * Initialize the application
     */
    async init() {
        try {
            console.log('Initializing Sanad Islamic App...');

            // Cache DOM elements first
            this.cacheElements();

            // Show loading screen (elements are now cached)
            this.showLoadingScreen();

            // Load user preferences
            await this.loadUserPreferences();

            // Initialize services (with timeout to prevent hanging)
            try {
                await Promise.race([
                    this.initializeServices(),
                    new Promise((_, reject) => setTimeout(() => reject(new Error('Service init timeout')), 2000))
                ]);
            } catch (serviceError) {
                console.warn('Services initialization skipped:', serviceError.message);
            }

            // Setup event listeners
            this.setupEventListeners();

            // Initialize UI components
            this.initializeUI();

            // Load initial data (with timeout)
            try {
                await Promise.race([
                    this.loadInitialData(),
                    new Promise((_, reject) => setTimeout(() => reject(new Error('Data load timeout')), 2000))
                ]);
            } catch (dataError) {
                console.warn('Initial data load skipped:', dataError.message);
            }

            // Mark as initialized
            this.state.initialized = true;

            console.log('Sanad Islamic App initialized successfully');

        } catch (error) {
            console.error('Failed to initialize app:', error);
        } finally {
            // Always hide loading screen
            this.hideLoadingScreen();
        }
    },

    /**
     * Cache frequently used DOM elements
     */
    cacheElements() {
        this.elements = {
            // Loading screen
            loadingScreen: window.SanadUtils.dom.get('loadingScreen'),
            loadingText: window.SanadUtils.dom.get('loadingText'),

            // App container
            app: window.SanadUtils.dom.get('app'),

            // Header elements
            appTitle: window.SanadUtils.dom.get('appTitle'),
            globalSearch: window.SanadUtils.dom.get('globalSearch'),
            searchBtn: window.SanadUtils.dom.get('searchBtn'),
            langToggle: window.SanadUtils.dom.get('langToggle'),
            langDropdown: window.SanadUtils.dom.get('langDropdown'),
            themeToggle: window.SanadUtils.dom.get('themeToggle'),
            settingsBtn: window.SanadUtils.dom.get('settingsBtn'),
            mobileMenuToggle: window.SanadUtils.dom.get('mobileMenuToggle'),

            // Navigation
            navLinks: window.SanadUtils.dom.queryAll('.nav-link'),
            mainNav: window.SanadUtils.dom.query('.main-nav'),

            // Content sections
            sections: window.SanadUtils.dom.queryAll('.content-section'),

            // Widgets
            widgetsGrid: window.SanadUtils.dom.get('widgetsGrid'),

            // Notification container
            notificationContainer: window.SanadUtils.dom.get('notificationContainer'),

            // Modal overlay
            modalOverlay: window.SanadUtils.dom.get('modalOverlay')
        };
    },

    /**
     * Load user preferences from storage
     */
    async loadUserPreferences() {
        const preferences = window.SanadUtils.storage.get(window.SanadConfig.storage.userPreferences);

        if (preferences) {
            this.state.currentLanguage = preferences.language || 'ar';
            this.state.currentTheme = preferences.theme || 'light';
            this.state.location = preferences.location || window.SanadConfig.defaults.location;
        } else {
            // Use defaults
            this.state.currentLanguage = window.SanadConfig.defaults.language;
            this.state.currentTheme = window.SanadConfig.defaults.theme;
            this.state.location = window.SanadConfig.defaults.location;
        }

        // Apply theme
        this.setTheme(this.state.currentTheme);
    },

    /**
     * Initialize services
     */
    async initializeServices() {
        // Initialize i18n (already initialized in i18n.js)
        if (window.SanadI18n && !window.SanadI18n.currentLanguage) {
            await window.SanadI18n.init();
        }

        // Set language if different from i18n
        if (this.state.currentLanguage !== window.SanadI18n.currentLanguage) {
            await window.SanadI18n.setLanguage(this.state.currentLanguage);
        }

        // Initialize geolocation if enabled
        if (window.SanadConfig.features.geolocation) {
            this.initializeGeolocation();
        }

        // Initialize notifications if enabled
        if (window.SanadConfig.features.notifications) {
            this.initializeNotifications();
        }
    },

    /**
     * Setup event listeners
     */
    setupEventListeners() {
        // Navigation
        this.elements.navLinks.forEach(link => {
            const cleanup = window.SanadUtils.dom.on(link, 'click', (e) => {
                e.preventDefault();
                const section = link.getAttribute('data-section');
                this.navigateToSection(section);
            });
            this.cleanupFunctions.push(cleanup);
        });

        // Mobile menu toggle
        const mobileMenuCleanup = window.SanadUtils.dom.on(this.elements.mobileMenuToggle, 'click', () => {
            this.toggleMobileMenu();
        });
        this.cleanupFunctions.push(mobileMenuCleanup);

        // Language selector
        const langToggleCleanup = window.SanadUtils.dom.on(this.elements.langToggle, 'click', () => {
            this.toggleLanguageDropdown();
        });
        this.cleanupFunctions.push(langToggleCleanup);

        // Language options
        const langOptions = window.SanadUtils.dom.queryAll('.lang-option');
        langOptions.forEach(option => {
            const cleanup = window.SanadUtils.dom.on(option, 'click', () => {
                const lang = option.getAttribute('data-lang');
                this.switchLanguage(lang);
            });
            this.cleanupFunctions.push(cleanup);
        });

        // Theme toggle
        const themeToggleCleanup = window.SanadUtils.dom.on(this.elements.themeToggle, 'click', () => {
            this.toggleTheme();
        });
        this.cleanupFunctions.push(themeToggleCleanup);

        // Global search
        const searchCleanup = window.SanadUtils.dom.on(this.elements.globalSearch, 'input',
            window.SanadUtils.timing.debounce((e) => {
                this.handleGlobalSearch(e.target.value);
            }, 500)
        );
        this.cleanupFunctions.push(searchCleanup);

        // Search button
        const searchBtnCleanup = window.SanadUtils.dom.on(this.elements.searchBtn, 'click', () => {
            this.performGlobalSearch();
        });
        this.cleanupFunctions.push(searchBtnCleanup);

        // Online/offline status
        const onlineCleanup = window.SanadUtils.dom.on(window, 'online', () => {
            this.state.isOnline = true;
            this.showNotification('تم استعادة الاتصال بالإنترنت', 'success');
        });
        this.cleanupFunctions.push(onlineCleanup);

        const offlineCleanup = window.SanadUtils.dom.on(window, 'offline', () => {
            this.state.isOnline = false;
            this.showNotification('انقطع الاتصال بالإنترنت. سيتم العمل في الوضع المحدود.', 'warning');
        });
        this.cleanupFunctions.push(offlineCleanup);

        // Close dropdowns when clicking outside
        const documentClickCleanup = window.SanadUtils.dom.on(document, 'click', (e) => {
            if (!e.target.closest('.language-selector')) {
                this.closeLanguageDropdown();
            }
        });
        this.cleanupFunctions.push(documentClickCleanup);

        // Keyboard shortcuts
        const keyboardCleanup = window.SanadUtils.dom.on(document, 'keydown', (e) => {
            this.handleKeyboardShortcuts(e);
        });
        this.cleanupFunctions.push(keyboardCleanup);
    },

    /**
     * Initialize UI components
     */
    initializeUI() {
        // Update language toggle text
        this.updateLanguageToggle();

        // Update theme toggle icon
        this.updateThemeToggle();

        // Initialize tooltips
        this.initializeTooltips();

        // Initialize responsive behavior
        this.initializeResponsive();
    },

    /**
     * Load initial data
     */
    async loadInitialData() {
        try {
            // Load dashboard widgets
            await this.loadDashboardWidgets();

            // Load prayer times if location is available
            if (this.state.location) {
                await this.loadPrayerTimes();
            }

            // Load user bookmarks
            await this.loadUserBookmarks();

        } catch (error) {
            console.error('Failed to load initial data:', error);
            // Continue with app initialization even if some data fails to load
        }
    },

    /**
     * Show loading screen
     */
    showLoadingScreen() {
        if (this.elements.loadingScreen) {
            this.elements.loadingScreen.style.display = 'flex';
            this.elements.loadingScreen.classList.remove('hidden');
        }
    },

    /**
     * Hide loading screen and show app
     */
    hideLoadingScreen() {
        if (this.elements.loadingScreen && this.elements.app) {
            this.elements.loadingScreen.classList.add('hidden');
            this.elements.app.style.display = 'flex';

            // Remove loading screen after animation
            setTimeout(() => {
                this.elements.loadingScreen.style.display = 'none';
            }, 500);
        }
    },

    /**
     * Navigate to a section
     */
    navigateToSection(sectionId) {
        // Update active nav link
        this.elements.navLinks.forEach(link => {
            link.classList.remove('active');
            if (link.getAttribute('data-section') === sectionId) {
                link.classList.add('active');
            }
        });

        // Update active section
        this.elements.sections.forEach(section => {
            section.classList.remove('active');
            if (section.id === sectionId) {
                section.classList.add('active');
            }
        });

        // Update state
        this.state.currentSection = sectionId;

        // Update URL
        window.SanadUtils.url.setParam('section', sectionId);

        // Close mobile menu if open
        this.closeMobileMenu();

        // Load section-specific data
        this.loadSectionData(sectionId);

        // Dispatch navigation event
        this.dispatchNavigationEvent(sectionId);
    },

    /**
     * Load section-specific data
     */
    async loadSectionData(sectionId) {
        try {
            switch (sectionId) {
                case 'quran':
                    await this.loadQuranData();
                    break;
                case 'hadith':
                    await this.loadHadithData();
                    break;
                case 'stories':
                    await this.loadStoriesData();
                    break;
                case 'prayer-times':
                    await this.loadPrayerTimesData();
                    break;
                case 'ai-assistant':
                    await this.loadAIAssistantData();
                    break;
            }
        } catch (error) {
            console.error(`Failed to load data for section ${sectionId}:`, error);
        }
    },

    /**
     * Switch language
     */
    async switchLanguage(languageCode) {
        if (await window.SanadI18n.setLanguage(languageCode)) {
            this.state.currentLanguage = languageCode;
            this.updateLanguageToggle();
            this.closeLanguageDropdown();
            this.saveUserPreferences();

            // Show success notification
            this.showNotification('تم تغيير اللغة بنجاح', 'success');
        }
    },

    /**
     * Toggle theme
     */
    toggleTheme() {
        const newTheme = this.state.currentTheme === 'light' ? 'dark' : 'light';
        this.setTheme(newTheme);
    },

    /**
     * Set theme
     */
    setTheme(theme) {
        this.state.currentTheme = theme;
        document.body.setAttribute('data-theme', theme);
        this.updateThemeToggle();
        this.saveUserPreferences();
    },

    /**
     * Update language toggle text
     */
    updateLanguageToggle() {
        if (this.elements.langToggle) {
            const language = window.SanadConfig.languages[this.state.currentLanguage];
            this.elements.langToggle.textContent = language.name;
        }

        // Update active language option
        const langOptions = window.SanadUtils.dom.queryAll('.lang-option');
        langOptions.forEach(option => {
            option.classList.remove('active');
            if (option.getAttribute('data-lang') === this.state.currentLanguage) {
                option.classList.add('active');
            }
        });
    },

    /**
     * Update theme toggle icon
     */
    updateThemeToggle() {
        if (this.elements.themeToggle) {
            this.elements.themeToggle.textContent = this.state.currentTheme === 'light' ? '🌙' : '☀️';
            this.elements.themeToggle.title = this.state.currentTheme === 'light' ? 'الوضع المظلم' : 'الوضع المضيء';
        }
    },

    /**
     * Toggle language dropdown
     */
    toggleLanguageDropdown() {
        const languageSelector = window.SanadUtils.dom.query('.language-selector');
        if (languageSelector) {
            languageSelector.classList.toggle('active');
        }
    },

    /**
     * Close language dropdown
     */
    closeLanguageDropdown() {
        const languageSelector = window.SanadUtils.dom.query('.language-selector');
        if (languageSelector) {
            languageSelector.classList.remove('active');
        }
    },

    /**
     * Toggle mobile menu
     */
    toggleMobileMenu() {
        if (this.elements.mainNav) {
            this.elements.mainNav.classList.toggle('mobile-open');
        }
    },

    /**
     * Close mobile menu
     */
    closeMobileMenu() {
        if (this.elements.mainNav) {
            this.elements.mainNav.classList.remove('mobile-open');
        }
    },

    /**
     * Handle global search
     */
    handleGlobalSearch(query) {
        if (query.length >= window.SanadConfig.ui.searchMinLength) {
            // Show search suggestions
            this.showSearchSuggestions(query);
        } else {
            this.hideSearchSuggestions();
        }
    },

    /**
     * Perform global search
     */
    async performGlobalSearch() {
        const query = this.elements.globalSearch.value.trim();
        if (query.length < window.SanadConfig.ui.searchMinLength) {
            return;
        }

        // Open advanced search with the query
        if (window.SanadAdvancedSearch) {
            const event = new CustomEvent('openAdvancedSearch', {
                detail: { query }
            });
            document.dispatchEvent(event);
        } else {
            // Fallback to basic search
            try {
                this.showLoadingInSearch();
                const results = await window.SanadAPI.search.search(query);
                this.displaySearchResults(results);
            } catch (error) {
                console.error('Search failed:', error);
                this.showNotification('فشل في البحث. يرجى المحاولة مرة أخرى.', 'error');
            } finally {
                this.hideLoadingInSearch();
            }
        }
    },

    /**
     * Show notification
     */
    showNotification(message, type = 'info', duration = 5000) {
        const notification = this.createNotificationElement(message, type);
        this.elements.notificationContainer.appendChild(notification);

        // Auto remove after duration
        setTimeout(() => {
            this.removeNotification(notification);
        }, duration);

        // Add to state
        this.state.notifications.push({
            id: Date.now(),
            message,
            type,
            timestamp: new Date()
        });
    },

    /**
     * Create notification element
     */
    createNotificationElement(message, type) {
        const notification = window.SanadUtils.dom.create('div', {
            className: `notification notification-${type}`
        });

        const content = window.SanadUtils.dom.create('div', {
            className: 'notification-content'
        }, message);

        const closeBtn = window.SanadUtils.dom.create('button', {
            className: 'notification-close'
        }, '×');

        window.SanadUtils.dom.on(closeBtn, 'click', () => {
            this.removeNotification(notification);
        });

        notification.appendChild(content);
        notification.appendChild(closeBtn);

        return notification;
    },

    /**
     * Remove notification
     */
    removeNotification(notification) {
        if (notification && notification.parentNode) {
            notification.style.animation = 'slideOutRight 0.3s ease';
            setTimeout(() => {
                notification.parentNode.removeChild(notification);
            }, 300);
        }
    },

    /**
     * Save user preferences
     */
    saveUserPreferences() {
        const preferences = {
            language: this.state.currentLanguage,
            theme: this.state.currentTheme,
            location: this.state.location,
            lastUpdated: new Date().toISOString()
        };

        window.SanadUtils.storage.set(window.SanadConfig.storage.userPreferences, preferences);
    },

    /**
     * Handle keyboard shortcuts
     */
    handleKeyboardShortcuts(e) {
        // Ctrl/Cmd + K for search
        if ((e.ctrlKey || e.metaKey) && e.key === 'k') {
            e.preventDefault();
            this.elements.globalSearch.focus();
        }

        // Escape to close modals/dropdowns
        if (e.key === 'Escape') {
            this.closeLanguageDropdown();
            this.closeMobileMenu();
        }
    },

    /**
     * Initialize geolocation
     */
    initializeGeolocation() {
        if (navigator.geolocation) {
            navigator.geolocation.getCurrentPosition(
                (position) => {
                    this.state.location = {
                        latitude: position.coords.latitude,
                        longitude: position.coords.longitude
                    };
                    this.saveUserPreferences();
                    this.loadPrayerTimes();
                },
                (error) => {
                    console.warn('Geolocation failed:', error);
                }
            );
        }
    },

    /**
     * Initialize notifications
     */
    initializeNotifications() {
        if ('Notification' in window && Notification.permission === 'default') {
            Notification.requestPermission();
        }
    },

    /**
     * Initialize tooltips
     */
    initializeTooltips() {
        // Tooltips are handled via CSS
    },

    /**
     * Initialize responsive behavior
     */
    initializeResponsive() {
        const handleResize = window.SanadUtils.timing.throttle(() => {
            const screenSize = window.SanadUtils.device.getScreenSize();
            document.body.setAttribute('data-screen-size', screenSize);
        }, 250);

        window.addEventListener('resize', handleResize);
        handleResize(); // Initial call
    },

    /**
     * Dispatch navigation event
     */
    dispatchNavigationEvent(sectionId) {
        const event = new CustomEvent('sectionChanged', {
            detail: { section: sectionId }
        });
        document.dispatchEvent(event);
    },

    /**
     * Show error message
     */
    showError(message) {
        this.showNotification(message, 'error', 10000);
    },

    /**
     * Load dashboard widgets
     */
    async loadDashboardWidgets() {
        // Initialize user dashboard if available
        if (window.SanadUserDashboard && !window.SanadUserDashboard.state.initialized) {
            await window.SanadUserDashboard.init();
        }
        console.log('Dashboard widgets loaded');
    },

    async loadPrayerTimes() {
        // Will be implemented with prayer times service
        console.log('Loading prayer times...');
    },

    async loadUserBookmarks() {
        // Will be implemented with user service
        console.log('Loading user bookmarks...');
    },

    async loadQuranData() {
        console.log('Loading Quran data...');

        // Initialize tafsir comparison if available
        if (window.SanadTafsirComparison && !window.SanadTafsirComparison.state.initialized) {
            window.SanadTafsirComparison.init();
        }
    },

    async loadHadithData() {
        console.log('Loading Hadith data...');
    },

    async loadStoriesData() {
        console.log('Loading Stories data...');
    },

    async loadPrayerTimesData() {
        console.log('Loading Prayer Times data...');
    },

    async loadAIAssistantData() {
        console.log('Loading AI Assistant data...');
    },

    showSearchSuggestions(query) {
        console.log('Showing search suggestions for:', query);
    },

    hideSearchSuggestions() {
        console.log('Hiding search suggestions');
    },

    showLoadingInSearch() {
        console.log('Showing search loading');
    },

    hideLoadingInSearch() {
        console.log('Hiding search loading');
    },

    displaySearchResults(results) {
        console.log('Displaying search results:', results);
    },

    /**
     * Show loading screen
     */
    showLoadingScreen() {
        const loadingScreen = this.elements?.loadingScreen || document.getElementById('loadingScreen');
        const app = this.elements?.app || document.getElementById('app');

        if (loadingScreen) {
            loadingScreen.classList.remove('hidden');
            loadingScreen.style.display = 'flex';
        }
        if (app) {
            app.classList.add('hidden');
        }
    },

    /**
     * Hide loading screen and show app
     */
    hideLoadingScreen() {
        const loadingScreen = this.elements?.loadingScreen || document.getElementById('loadingScreen');
        const app = this.elements?.app || document.getElementById('app');

        if (loadingScreen) {
            loadingScreen.classList.add('hidden');
            loadingScreen.style.display = 'none';
        }
        if (app) {
            app.classList.remove('hidden');
            app.style.display = 'block';
        }
        console.log('Loading screen hidden, app visible');
    },

    /**
     * Cleanup function
     */
    cleanup() {
        this.cleanupFunctions.forEach(cleanup => cleanup());
        this.cleanupFunctions = [];
    }
};

// Initialize app when DOM is ready
window.SanadUtils.timing.ready(() => {
    window.SanadApp.init();
});

// Cleanup on page unload
window.addEventListener('beforeunload', () => {
    window.SanadApp.cleanup();
});

// Handle initial navigation from URL
window.addEventListener('load', () => {
    const section = window.SanadUtils.url.getParam('section');
    if (section && window.SanadApp.state.initialized) {
        window.SanadApp.navigateToSection(section);
    }
});

// Freeze the app object to prevent modifications
Object.freeze(window.SanadApp);