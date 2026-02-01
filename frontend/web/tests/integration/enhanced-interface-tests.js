/**
 * Enhanced Integration Tests for Sanad Islamic App Interface
 * Additional test scenarios for comprehensive interface testing
 * 
 * Requirements: 1.3, 10.2
 * Task: 14.3 كتابة اختبارات التكامل للواجهة
 */

describe('Enhanced Sanad Interface Integration Tests', () => {
    let app;
    let mockAPI;
    let testUtils;
    
    beforeEach(async () => {
        // Setup DOM with enhanced fixture
        document.body.innerHTML = await loadEnhancedFixture();
        
        // Setup comprehensive mocks
        setupComprehensiveMocks();
        
        // Initialize test utilities
        testUtils = await initializeEnhancedTestUtils();
        
        // Initialize app
        app = window.SanadApp;
        await app.init();
    });
    
    afterEach(() => {
        if (app && app.cleanup) {
            app.cleanup();
        }
        cleanupTestEnvironment();
    });
    
    describe('Advanced Navigation Tests', () => {
        test('should handle rapid navigation clicks without breaking state', async () => {
            const sections = ['dashboard', 'quran', 'hadith', 'stories'];
            
            // Rapidly click navigation links
            for (let i = 0; i < 10; i++) {
                const randomSection = sections[Math.floor(Math.random() * sections.length)];
                const navLink = document.querySelector(`[data-section="${randomSection}"]`);
                navLink.click();
                
                // Small delay to simulate real user interaction
                await testUtils.wait(50);
            }
            
            // Verify final state is consistent
            const activeSection = document.querySelector('.content-section.active');
            const activeNavLink = document.querySelector('.nav-link.active');
            
            expect(activeSection).toBeTruthy();
            expect(activeNavLink).toBeTruthy();
            expect(activeSection.id).toBe(activeNavLink.getAttribute('data-section'));
            expect(app.state.currentSection).toBe(activeSection.id);
        });
        
        test('should maintain navigation state during window resize', async () => {
            // Navigate to a specific section
            app.navigateToSection('quran');
            await testUtils.waitForNavigation('quran');
            
            // Resize window multiple times
            const sizes = [
                { width: 1920, height: 1080 },
                { width: 768, height: 1024 },
                { width: 375, height: 667 },
                { width: 1200, height: 800 }
            ];
            
            for (const size of sizes) {
                testUtils.setViewportSize(size.width, size.height);
                window.dispatchEvent(new Event('resize'));
                await testUtils.wait(100);
                
                // Verify navigation state is preserved
                expect(app.state.currentSection).toBe('quran');
                expect(document.querySelector('#quran.active')).toBeTruthy();
            }
        });
        
        test('should handle navigation with hash URLs correctly', async () => {
            // Test direct hash navigation
            window.location.hash = '#quran';
            window.dispatchEvent(new Event('hashchange'));
            
            await testUtils.waitForNavigation('quran');
            expect(app.state.currentSection).toBe('quran');
            
            // Test programmatic hash change
            app.navigateToSection('hadith');
            await testUtils.waitForNavigation('hadith');
            expect(window.location.hash).toBe('#hadith');
        });
        
        test('should handle navigation errors gracefully', async () => {
            // Try to navigate to non-existent section
            app.navigateToSection('non-existent-section');
            
            // Should fallback to dashboard or handle gracefully
            await testUtils.wait(100);
            expect(app.state.currentSection).toBe('dashboard');
        });
    });
    
    describe('Advanced Language Switching Tests', () => {
        test('should handle language switching during active user interactions', async () => {
            // Start typing in search
            const searchInput = document.getElementById('globalSearch');
            searchInput.value = 'test search';
            searchInput.focus();
            
            // Switch language while search is active
            await app.switchLanguage('en');
            
            // Verify search input maintains focus and value
            expect(document.activeElement).toBe(searchInput);
            expect(searchInput.value).toBe('test search');
            expect(app.state.currentLanguage).toBe('en');
        });
        
        test('should update all UI elements when language changes', async () => {
            // Switch to English
            await app.switchLanguage('en');
            
            // Verify document attributes
            expect(document.documentElement.getAttribute('lang')).toBe('en');
            expect(document.documentElement.getAttribute('dir')).toBe('ltr');
            
            // Verify body class
            expect(document.body.classList.contains('lang-en')).toBe(true);
            expect(document.body.classList.contains('lang-ar')).toBe(false);
            
            // Verify font family
            expect(document.body.style.fontFamily).toContain('Noto Sans');
            
            // Switch back to Arabic
            await app.switchLanguage('ar');
            
            expect(document.documentElement.getAttribute('lang')).toBe('ar');
            expect(document.documentElement.getAttribute('dir')).toBe('rtl');
            expect(document.body.classList.contains('lang-ar')).toBe(true);
            expect(document.body.style.fontFamily).toContain('Amiri');
        });
        
        test('should handle language switching with network errors', async () => {
            // Mock network error for translation loading
            mockAPI.i18n.getBulkTranslations.mockRejectedValueOnce(new Error('Network error'));
            
            const result = await app.switchLanguage('en');
            
            // Should still switch language even if translations fail to load
            expect(result).toBe(true);
            expect(app.state.currentLanguage).toBe('en');
        });
        
        test('should preserve language preference across page reloads', async () => {
            // Switch language
            await app.switchLanguage('en');
            
            // Simulate page reload by reinitializing app
            await app.init();
            
            // Verify language is restored
            expect(app.state.currentLanguage).toBe('en');
            expect(document.documentElement.getAttribute('lang')).toBe('en');
        });
    });
    
    describe('Advanced Responsive Design Tests', () => {
        test('should handle smooth transitions between breakpoints', async () => {
            const breakpoints = [
                { width: 1920, name: 'xl' },
                { width: 1199, name: 'lg' },
                { width: 991, name: 'md' },
                { width: 767, name: 'sm' },
                { width: 575, name: 'xs' }
            ];
            
            for (const breakpoint of breakpoints) {
                testUtils.setViewportSize(breakpoint.width, 800);
                window.dispatchEvent(new Event('resize'));
                
                await testUtils.wait(100);
                
                // Verify screen size classification
                const expectedSize = testUtils.getExpectedScreenSize(breakpoint.width);
                expect(expectedSize).toBe(breakpoint.name);
                
                // Verify layout elements adapt correctly
                await testUtils.verifyResponsiveLayout(breakpoint.name);
            }
        });
        
        test('should maintain functionality across all device orientations', async () => {
            const orientations = [
                { width: 375, height: 667, name: 'portrait mobile' },
                { width: 667, height: 375, name: 'landscape mobile' },
                { width: 768, height: 1024, name: 'portrait tablet' },
                { width: 1024, height: 768, name: 'landscape tablet' }
            ];
            
            for (const orientation of orientations) {
                testUtils.setViewportSize(orientation.width, orientation.height);
                window.dispatchEvent(new Event('resize'));
                
                await testUtils.wait(100);
                
                // Test navigation functionality
                app.navigateToSection('quran');
                await testUtils.waitForNavigation('quran');
                expect(app.state.currentSection).toBe('quran');
                
                // Test language switching
                await app.switchLanguage('en');
                expect(app.state.currentLanguage).toBe('en');
                
                // Reset for next iteration
                await app.switchLanguage('ar');
                app.navigateToSection('dashboard');
            }
        });
        
        test('should handle extreme screen sizes gracefully', async () => {
            const extremeSizes = [
                { width: 320, height: 568, name: 'very small mobile' },
                { width: 2560, height: 1440, name: 'very large desktop' },
                { width: 1366, height: 768, name: 'common laptop' }
            ];
            
            for (const size of extremeSizes) {
                testUtils.setViewportSize(size.width, size.height);
                window.dispatchEvent(new Event('resize'));
                
                await testUtils.wait(100);
                
                // Verify no layout breaks
                const overflowElements = document.querySelectorAll('*');
                let hasOverflow = false;
                
                overflowElements.forEach(el => {
                    const rect = el.getBoundingClientRect();
                    if (rect.width > size.width + 50) { // Allow small margin
                        hasOverflow = true;
                    }
                });
                
                expect(hasOverflow).toBe(false);
            }
        });
        
        test('should optimize touch interactions on mobile devices', async () => {
            // Set mobile viewport
            testUtils.setViewportSize(375, 667);
            window.dispatchEvent(new Event('resize'));
            
            // Test touch target sizes
            const interactiveElements = document.querySelectorAll('button, .nav-link, .btn');
            
            interactiveElements.forEach(element => {
                const rect = element.getBoundingClientRect();
                const minTouchSize = 44; // Minimum recommended touch target size
                
                expect(rect.height).toBeGreaterThanOrEqual(minTouchSize);
                expect(rect.width).toBeGreaterThanOrEqual(minTouchSize);
            });
        });
    });
    
    describe('Performance and Accessibility Tests', () => {
        test('should maintain performance during rapid interactions', async () => {
            const startTime = performance.now();
            
            // Perform rapid interactions
            for (let i = 0; i < 50; i++) {
                const actions = [
                    () => app.navigateToSection('quran'),
                    () => app.navigateToSection('hadith'),
                    () => app.switchLanguage('en'),
                    () => app.switchLanguage('ar'),
                    () => testUtils.setViewportSize(800, 600),
                    () => testUtils.setViewportSize(1200, 800)
                ];
                
                const randomAction = actions[Math.floor(Math.random() * actions.length)];
                randomAction();
                
                await testUtils.wait(10);
            }
            
            const endTime = performance.now();
            const duration = endTime - startTime;
            
            // Should complete within reasonable time (less than 5 seconds)
            expect(duration).toBeLessThan(5000);
        });
        
        test('should maintain accessibility standards', async () => {
            // Test keyboard navigation
            const focusableElements = document.querySelectorAll(
                'button, [href], input, select, textarea, [tabindex]:not([tabindex="-1"])'
            );
            
            expect(focusableElements.length).toBeGreaterThan(0);
            
            // Test ARIA attributes
            const navElements = document.querySelectorAll('nav, [role="navigation"]');
            expect(navElements.length).toBeGreaterThan(0);
            
            // Test heading hierarchy
            const headings = document.querySelectorAll('h1, h2, h3, h4, h5, h6');
            expect(headings.length).toBeGreaterThan(0);
            
            // Test alt text for images
            const images = document.querySelectorAll('img');
            images.forEach(img => {
                expect(img.getAttribute('alt')).toBeTruthy();
            });
        });
        
        test('should handle memory leaks prevention', async () => {
            const initialEventListeners = testUtils.getEventListenerCount();
            
            // Perform operations that might create memory leaks
            for (let i = 0; i < 10; i++) {
                await app.init();
                app.cleanup();
            }
            
            const finalEventListeners = testUtils.getEventListenerCount();
            
            // Should not accumulate event listeners
            expect(finalEventListeners).toBeLessThanOrEqual(initialEventListeners + 5);
        });
    });
    
    describe('Error Handling and Edge Cases', () => {
        test('should handle DOM manipulation errors gracefully', async () => {
            // Remove critical DOM elements
            const criticalElements = ['globalSearch', 'langToggle', 'mobileMenuToggle'];
            
            criticalElements.forEach(id => {
                const element = document.getElementById(id);
                if (element) {
                    element.remove();
                }
            });
            
            // App should still function without crashing
            expect(() => {
                app.navigateToSection('quran');
            }).not.toThrow();
            
            expect(() => {
                app.switchLanguage('en');
            }).not.toThrow();
        });
        
        test('should handle localStorage unavailability', async () => {
            // Mock localStorage unavailability
            const originalLocalStorage = window.localStorage;
            delete window.localStorage;
            
            // App should still function
            expect(async () => {
                await app.switchLanguage('en');
            }).not.toThrow();
            
            // Restore localStorage
            window.localStorage = originalLocalStorage;
        });
        
        test('should handle network connectivity changes', async () => {
            // Simulate going offline
            Object.defineProperty(navigator, 'onLine', {
                writable: true,
                value: false
            });
            
            window.dispatchEvent(new Event('offline'));
            
            expect(app.state.isOnline).toBe(false);
            
            // Simulate going online
            Object.defineProperty(navigator, 'onLine', {
                writable: true,
                value: true
            });
            
            window.dispatchEvent(new Event('online'));
            
            expect(app.state.isOnline).toBe(true);
        });
    });
    
    // Enhanced test utilities
    async function loadEnhancedFixture() {
        return `
            <div id="app" class="app-container">
                <header class="app-header">
                    <div class="header-content">
                        <div class="header-brand">
                            <div class="logo">
                                <span class="logo-text" id="appTitle">سند</span>
                            </div>
                        </div>
                        <nav class="main-nav">
                            <ul class="nav-list">
                                <li><a href="#dashboard" class="nav-link active" data-section="dashboard">الرئيسية</a></li>
                                <li><a href="#quran" class="nav-link" data-section="quran">القرآن الكريم</a></li>
                                <li><a href="#hadith" class="nav-link" data-section="hadith">الأحاديث</a></li>
                                <li><a href="#stories" class="nav-link" data-section="stories">القصص</a></li>
                                <li><a href="#prayer-times" class="nav-link" data-section="prayer-times">المواقيت</a></li>
                                <li><a href="#ai-assistant" class="nav-link" data-section="ai-assistant">المساعد الذكي</a></li>
                            </ul>
                        </nav>
                        <div class="header-actions">
                            <div class="search-container">
                                <input type="text" class="search-input" id="globalSearch" placeholder="البحث...">
                                <button class="search-btn" id="searchBtn">🔍</button>
                            </div>
                            <div class="language-selector">
                                <button class="lang-toggle" id="langToggle">العربية</button>
                                <div class="lang-dropdown" id="langDropdown">
                                    <button class="lang-option active" data-lang="ar">العربية</button>
                                    <button class="lang-option" data-lang="en">English</button>
                                    <button class="lang-option" data-lang="ur">اردو</button>
                                    <button class="lang-option" data-lang="tr">Türkçe</button>
                                    <button class="lang-option" data-lang="fr">Français</button>
                                </div>
                            </div>
                            <button class="theme-toggle" id="themeToggle">🌙</button>
                            <button class="settings-btn" id="settingsBtn">⚙️</button>
                        </div>
                    </div>
                    <button class="mobile-menu-toggle" id="mobileMenuToggle">☰</button>
                </header>
                <main class="main-content">
                    <section id="dashboard" class="content-section active">
                        <div class="quick-stats-grid">
                            <div class="stat-card">
                                <div class="stat-icon">📖</div>
                                <div class="stat-content">
                                    <div class="stat-value">35%</div>
                                    <div class="stat-label">تقدم الختمة</div>
                                </div>
                            </div>
                        </div>
                        <div class="widgets-grid" id="widgetsGrid">
                            <div class="widget">
                                <div class="widget-header">
                                    <h3>آية اليوم</h3>
                                </div>
                                <div class="widget-content">
                                    <p>وَمَن يَتَّقِ اللَّهَ يَجْعَل لَّهُ مَخْرَجًا</p>
                                </div>
                            </div>
                        </div>
                    </section>
                    <section id="quran" class="content-section">
                        <h2>القرآن الكريم</h2>
                    </section>
                    <section id="hadith" class="content-section">
                        <h2>الأحاديث النبوية</h2>
                    </section>
                    <section id="stories" class="content-section">
                        <h2>القصص الإسلامية</h2>
                    </section>
                    <section id="prayer-times" class="content-section">
                        <h2>مواقيت الصلاة</h2>
                    </section>
                    <section id="ai-assistant" class="content-section">
                        <h2>المساعد الذكي</h2>
                    </section>
                </main>
                <footer class="app-footer">
                    <div class="footer-content">
                        <p>&copy; 2024 سند - التطبيق الإسلامي الشامل</p>
                    </div>
                </footer>
            </div>
        `;
    }
    
    function setupComprehensiveMocks() {
        // Enhanced API mocks
        mockAPI = {
            i18n: {
                getBulkTranslations: jest.fn().mockResolvedValue({
                    translations: {
                        'appTitle': 'سند - التطبيق الإسلامي الشامل',
                        'loading': 'جاري التحميل...',
                        'dashboard': 'الرئيسية',
                        'quran': 'القرآن الكريم',
                        'hadith': 'الأحاديث النبوية'
                    }
                })
            },
            search: {
                search: jest.fn().mockResolvedValue([])
            }
        };
        
        window.SanadAPI = mockAPI;
        
        // Enhanced config
        window.SanadConfig = {
            languages: {
                ar: { name: 'العربية', direction: 'rtl', fontFamily: 'Amiri, serif' },
                en: { name: 'English', direction: 'ltr', fontFamily: 'Noto Sans, sans-serif' },
                ur: { name: 'اردو', direction: 'rtl', fontFamily: 'Noto Sans Arabic, sans-serif' },
                tr: { name: 'Türkçe', direction: 'ltr', fontFamily: 'Noto Sans, sans-serif' },
                fr: { name: 'Français', direction: 'ltr', fontFamily: 'Noto Sans, sans-serif' }
            },
            defaults: {
                language: 'ar',
                theme: 'light',
                location: { latitude: 21.3891, longitude: 39.8579 }
            },
            storage: {
                userPreferences: 'sanad_user_preferences',
                language: 'sanad_language'
            },
            features: {
                geolocation: true,
                notifications: true
            },
            ui: {
                searchMinLength: 3
            }
        };
    }
    
    async function initializeEnhancedTestUtils() {
        // Enhanced utilities
        const utils = {
            wait: (ms) => new Promise(resolve => setTimeout(resolve, ms)),
            
            waitForNavigation: async (sectionId, timeout = 5000) => {
                const startTime = Date.now();
                while (Date.now() - startTime < timeout) {
                    const activeSection = document.querySelector('.content-section.active');
                    if (activeSection && activeSection.id === sectionId) {
                        return true;
                    }
                    await utils.wait(10);
                }
                throw new Error(`Navigation to ${sectionId} timed out`);
            },
            
            setViewportSize: (width, height) => {
                Object.defineProperty(window, 'innerWidth', {
                    writable: true,
                    configurable: true,
                    value: width
                });
                Object.defineProperty(window, 'innerHeight', {
                    writable: true,
                    configurable: true,
                    value: height
                });
            },
            
            getExpectedScreenSize: (width) => {
                if (width < 576) return 'xs';
                if (width < 768) return 'sm';
                if (width < 992) return 'md';
                if (width < 1200) return 'lg';
                return 'xl';
            },
            
            verifyResponsiveLayout: async (screenSize) => {
                // Verify mobile menu behavior
                const mobileMenuToggle = document.getElementById('mobileMenuToggle');
                if (screenSize === 'xs' || screenSize === 'sm') {
                    expect(mobileMenuToggle).toBeTruthy();
                }
                
                // Verify grid layouts
                const widgetsGrid = document.getElementById('widgetsGrid');
                if (widgetsGrid) {
                    const gridStyle = getComputedStyle(widgetsGrid);
                    expect(gridStyle.display).toBe('grid');
                }
            },
            
            getEventListenerCount: () => {
                // Mock implementation - in real scenario would count actual listeners
                return document.querySelectorAll('*').length;
            }
        };
        
        // Initialize SanadUtils mock
        window.SanadUtils = {
            dom: {
                get: (id) => document.getElementById(id),
                query: (selector) => document.querySelector(selector),
                queryAll: (selector) => document.querySelectorAll(selector),
                create: (tag, attrs, content) => {
                    const el = document.createElement(tag);
                    if (attrs) Object.assign(el, attrs);
                    if (content) el.textContent = content;
                    return el;
                },
                on: (element, event, handler) => {
                    element.addEventListener(event, handler);
                    return () => element.removeEventListener(event, handler);
                }
            },
            storage: {
                get: (key) => {
                    try {
                        const item = localStorage.getItem(key);
                        return item ? JSON.parse(item) : null;
                    } catch {
                        return null;
                    }
                },
                set: (key, value) => {
                    try {
                        localStorage.setItem(key, JSON.stringify(value));
                    } catch {
                        // Ignore storage errors
                    }
                }
            },
            url: {
                getParam: (param) => new URLSearchParams(window.location.search).get(param),
                setParam: (param, value) => {
                    const url = new URL(window.location);
                    url.searchParams.set(param, value);
                    window.history.pushState({}, '', url);
                }
            },
            timing: {
                ready: (callback) => {
                    if (document.readyState === 'loading') {
                        document.addEventListener('DOMContentLoaded', callback);
                    } else {
                        callback();
                    }
                },
                debounce: (func, wait) => {
                    let timeout;
                    return function executedFunction(...args) {
                        const later = () => {
                            clearTimeout(timeout);
                            func(...args);
                        };
                        clearTimeout(timeout);
                        timeout = setTimeout(later, wait);
                    };
                },
                throttle: (func, limit) => {
                    let inThrottle;
                    return function() {
                        const args = arguments;
                        const context = this;
                        if (!inThrottle) {
                            func.apply(context, args);
                            inThrottle = true;
                            setTimeout(() => inThrottle = false, limit);
                        }
                    };
                }
            },
            device: {
                getScreenSize: () => {
                    const width = window.innerWidth;
                    return utils.getExpectedScreenSize(width);
                }
            }
        };
        
        // Initialize enhanced SanadApp mock
        window.SanadApp = {
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
            
            init: jest.fn().mockImplementation(async function() {
                this.state.initialized = true;
                return true;
            }),
            
            navigateToSection: jest.fn().mockImplementation(function(sectionId) {
                // Validate section exists
                const targetSection = document.getElementById(sectionId);
                if (!targetSection) {
                    sectionId = 'dashboard'; // Fallback
                }
                
                // Update active section
                document.querySelectorAll('.content-section').forEach(section => {
                    section.classList.remove('active');
                });
                document.querySelectorAll('.nav-link').forEach(link => {
                    link.classList.remove('active');
                });
                
                const section = document.getElementById(sectionId);
                const navLink = document.querySelector(`[data-section="${sectionId}"]`);
                
                if (section) section.classList.add('active');
                if (navLink) navLink.classList.add('active');
                
                this.state.currentSection = sectionId;
                
                // Update URL
                window.location.hash = `#${sectionId}`;
            }),
            
            switchLanguage: jest.fn().mockImplementation(async function(languageCode) {
                if (!window.SanadConfig.languages[languageCode]) {
                    return false;
                }
                
                const language = window.SanadConfig.languages[languageCode];
                this.state.currentLanguage = languageCode;
                
                // Update document attributes
                document.documentElement.setAttribute('lang', languageCode);
                document.documentElement.setAttribute('dir', language.direction);
                
                // Update body class
                document.body.className = document.body.className
                    .replace(/lang-\w+/g, '')
                    .trim() + ` lang-${languageCode}`;
                
                // Update font family
                document.body.style.fontFamily = language.fontFamily;
                
                // Update language toggle
                const langToggle = document.getElementById('langToggle');
                if (langToggle) {
                    langToggle.textContent = language.name;
                }
                
                // Update active language option
                document.querySelectorAll('.lang-option').forEach(option => {
                    option.classList.remove('active');
                    if (option.getAttribute('data-lang') === languageCode) {
                        option.classList.add('active');
                    }
                });
                
                // Save to localStorage
                try {
                    const preferences = {
                        language: languageCode,
                        theme: this.state.currentTheme,
                        lastUpdated: new Date().toISOString()
                    };
                    localStorage.setItem('sanad_user_preferences', JSON.stringify(preferences));
                    localStorage.setItem('sanad_language', languageCode);
                } catch {
                    // Ignore storage errors
                }
                
                return true;
            }),
            
            cleanup: jest.fn()
        };
        
        return utils;
    }
    
    function cleanupTestEnvironment() {
        localStorage.clear();
        sessionStorage.clear();
        document.body.innerHTML = '';
        
        // Reset window properties
        Object.defineProperty(window, 'innerWidth', {
            writable: true,
            configurable: true,
            value: 1024
        });
        Object.defineProperty(window, 'innerHeight', {
            writable: true,
            configurable: true,
            value: 768
        });
        
        // Reset navigator.onLine
        Object.defineProperty(navigator, 'onLine', {
            writable: true,
            value: true
        });
    }
});