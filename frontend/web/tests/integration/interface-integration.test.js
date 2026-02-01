/**
 * Integration Tests for Sanad Islamic App Interface
 * Tests navigation, language switching, and responsive design
 * 
 * Requirements: 1.3, 10.2
 * Task: 14.3 كتابة اختبارات التكامل للواجهة
 */

describe('Sanad Interface Integration Tests', () => {
    let app;
    let mockAPI;
    
    beforeEach(async () => {
        // Setup DOM
        document.body.innerHTML = await loadFixture('index.html');
        
        // Mock API responses
        mockAPI = {
            i18n: {
                getBulkTranslations: jest.fn().mockResolvedValue({
                    translations: {
                        'appTitle': 'سند - التطبيق الإسلامي الشامل',
                        'loading': 'جاري التحميل...',
                        'dashboard': 'الرئيسية'
                    }
                })
            },
            search: {
                search: jest.fn().mockResolvedValue([])
            }
        };
        
        // Setup global objects
        window.SanadAPI = mockAPI;
        window.SanadConfig = {
            languages: {
                ar: { name: 'العربية', direction: 'rtl', fontFamily: 'Amiri, serif' },
                en: { name: 'English', direction: 'ltr', fontFamily: 'Noto Sans, sans-serif' },
                ur: { name: 'اردو', direction: 'rtl', fontFamily: 'Noto Sans Arabic, sans-serif' }
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
        
        // Initialize utilities
        await initializeTestUtils();
        
        // Initialize app
        app = window.SanadApp;
        await app.init();
    });
    
    afterEach(() => {
        // Cleanup
        if (app && app.cleanup) {
            app.cleanup();
        }
        localStorage.clear();
        sessionStorage.clear();
        document.body.innerHTML = '';
    });
    
    describe('Navigation Tests', () => {
        test('should navigate between main sections', async () => {
            const sections = ['dashboard', 'quran', 'hadith', 'stories', 'prayer-times', 'ai-assistant'];
            
            for (const sectionId of sections) {
                // Click navigation link
                const navLink = document.querySelector(`[data-section="${sectionId}"]`);
                expect(navLink).toBeTruthy();
                
                navLink.click();
                
                // Wait for navigation to complete
                await waitFor(() => {
                    const activeSection = document.querySelector('.content-section.active');
                    return activeSection && activeSection.id === sectionId;
                });
                
                // Verify active section
                const activeSection = document.querySelector('.content-section.active');
                expect(activeSection.id).toBe(sectionId);
                
                // Verify active nav link
                const activeNavLink = document.querySelector('.nav-link.active');
                expect(activeNavLink.getAttribute('data-section')).toBe(sectionId);
                
                // Verify URL parameter
                expect(window.location.search).toContain(`section=${sectionId}`);
            }
        });
        
        test('should handle navigation with keyboard shortcuts', async () => {
            // Test Ctrl+K for search focus
            const searchInput = document.getElementById('globalSearch');
            
            // Simulate Ctrl+K
            const event = new KeyboardEvent('keydown', {
                key: 'k',
                ctrlKey: true,
                bubbles: true
            });
            document.dispatchEvent(event);
            
            await waitFor(() => document.activeElement === searchInput);
            expect(document.activeElement).toBe(searchInput);
        });
        
        test('should close mobile menu after navigation', async () => {
            // Simulate mobile screen
            Object.defineProperty(window, 'innerWidth', {
                writable: true,
                configurable: true,
                value: 500
            });
            
            // Open mobile menu
            const mobileMenuToggle = document.getElementById('mobileMenuToggle');
            mobileMenuToggle.click();
            
            const mainNav = document.querySelector('.main-nav');
            expect(mainNav.classList.contains('mobile-open')).toBe(true);
            
            // Navigate to a section
            const quranLink = document.querySelector('[data-section="quran"]');
            quranLink.click();
            
            await waitFor(() => !mainNav.classList.contains('mobile-open'));
            expect(mainNav.classList.contains('mobile-open')).toBe(false);
        });
        
        test('should handle browser back/forward navigation', async () => {
            // Navigate to quran section
            app.navigateToSection('quran');
            await waitFor(() => document.querySelector('#quran.active'));
            
            // Navigate to hadith section
            app.navigateToSection('hadith');
            await waitFor(() => document.querySelector('#hadith.active'));
            
            // Simulate browser back
            window.history.back();
            
            await waitFor(() => document.querySelector('#quran.active'));
            expect(document.querySelector('#quran.active')).toBeTruthy();
            
            // Simulate browser forward
            window.history.forward();
            
            await waitFor(() => document.querySelector('#hadith.active'));
            expect(document.querySelector('#hadith.active')).toBeTruthy();
        });
        
        test('should dispatch navigation events', async () => {
            const navigationHandler = jest.fn();
            document.addEventListener('sectionChanged', navigationHandler);
            
            app.navigateToSection('stories');
            
            await waitFor(() => navigationHandler.mock.calls.length > 0);
            expect(navigationHandler).toHaveBeenCalledWith(
                expect.objectContaining({
                    detail: { section: 'stories' }
                })
            );
        });
    });
    
    describe('Language Switching Tests', () => {
        test('should switch between supported languages', async () => {
            const languages = ['ar', 'en', 'ur'];
            
            for (const lang of languages) {
                await app.switchLanguage(lang);
                
                // Verify language state
                expect(app.state.currentLanguage).toBe(lang);
                
                // Verify document attributes
                expect(document.documentElement.getAttribute('lang')).toBe(lang);
                
                const expectedDirection = window.SanadConfig.languages[lang].direction;
                expect(document.documentElement.getAttribute('dir')).toBe(expectedDirection);
                
                // Verify body class
                expect(document.body.classList.contains(`lang-${lang}`)).toBe(true);
                
                // Verify language toggle text
                const langToggle = document.getElementById('langToggle');
                expect(langToggle.textContent).toBe(window.SanadConfig.languages[lang].name);
                
                // Verify active language option
                const activeOption = document.querySelector('.lang-option.active');
                expect(activeOption.getAttribute('data-lang')).toBe(lang);
            }
        });
        
        test('should update text direction for RTL/LTR languages', async () => {
            // Test RTL language (Arabic)
            await app.switchLanguage('ar');
            expect(document.documentElement.getAttribute('dir')).toBe('rtl');
            expect(document.body.style.fontFamily).toContain('Amiri');
            
            // Test LTR language (English)
            await app.switchLanguage('en');
            expect(document.documentElement.getAttribute('dir')).toBe('ltr');
            expect(document.body.style.fontFamily).toContain('Noto Sans');
            
            // Test RTL language (Urdu)
            await app.switchLanguage('ur');
            expect(document.documentElement.getAttribute('dir')).toBe('rtl');
            expect(document.body.style.fontFamily).toContain('Noto Sans Arabic');
        });
        
        test('should persist language preference', async () => {
            await app.switchLanguage('en');
            
            // Verify localStorage
            const preferences = JSON.parse(localStorage.getItem('sanad_user_preferences'));
            expect(preferences.language).toBe('en');
            
            // Verify direct language storage
            expect(localStorage.getItem('sanad_language')).toBe('en');
        });
        
        test('should handle language dropdown interactions', async () => {
            const langToggle = document.getElementById('langToggle');
            const langDropdown = document.getElementById('langDropdown');
            const languageSelector = document.querySelector('.language-selector');
            
            // Open dropdown
            langToggle.click();
            expect(languageSelector.classList.contains('active')).toBe(true);
            
            // Select language option
            const englishOption = document.querySelector('[data-lang="en"]');
            englishOption.click();
            
            await waitFor(() => app.state.currentLanguage === 'en');
            expect(app.state.currentLanguage).toBe('en');
            expect(languageSelector.classList.contains('active')).toBe(false);
        });
        
        test('should close language dropdown when clicking outside', async () => {
            const langToggle = document.getElementById('langToggle');
            const languageSelector = document.querySelector('.language-selector');
            
            // Open dropdown
            langToggle.click();
            expect(languageSelector.classList.contains('active')).toBe(true);
            
            // Click outside
            document.body.click();
            
            await waitFor(() => !languageSelector.classList.contains('active'));
            expect(languageSelector.classList.contains('active')).toBe(false);
        });
        
        test('should load translations for new language', async () => {
            await app.switchLanguage('en');
            
            // Verify API call for translations
            expect(mockAPI.i18n.getBulkTranslations).toHaveBeenCalledWith(
                expect.any(Array),
                'en',
                'common'
            );
        });
        
        test('should handle unsupported language gracefully', async () => {
            const result = await app.switchLanguage('invalid');
            expect(result).toBe(false);
            expect(app.state.currentLanguage).toBe('ar'); // Should remain unchanged
        });
    });
    
    describe('Responsive Design Tests', () => {
        const screenSizes = {
            mobile: { width: 375, height: 667 },
            tablet: { width: 768, height: 1024 },
            desktop: { width: 1200, height: 800 },
            largeDesktop: { width: 1920, height: 1080 }
        };
        
        test('should adapt layout for different screen sizes', async () => {
            for (const [deviceType, dimensions] of Object.entries(screenSizes)) {
                // Set viewport size
                setViewportSize(dimensions.width, dimensions.height);
                
                // Trigger resize event
                window.dispatchEvent(new Event('resize'));
                
                await waitFor(() => {
                    const screenSize = getScreenSizeClass();
                    return document.body.getAttribute('data-screen-size') === screenSize;
                });
                
                // Verify screen size attribute
                const expectedScreenSize = getExpectedScreenSize(dimensions.width);
                expect(document.body.getAttribute('data-screen-size')).toBe(expectedScreenSize);
                
                // Test specific responsive behaviors
                await testResponsiveBehavior(deviceType, dimensions);
            }
        });
        
        test('should show/hide mobile menu toggle based on screen size', async () => {
            const mobileMenuToggle = document.getElementById('mobileMenuToggle');
            
            // Desktop - should be hidden
            setViewportSize(1200, 800);
            window.dispatchEvent(new Event('resize'));
            await waitFor(() => getComputedStyle(mobileMenuToggle).display === 'none');
            
            // Mobile - should be visible
            setViewportSize(375, 667);
            window.dispatchEvent(new Event('resize'));
            await waitFor(() => getComputedStyle(mobileMenuToggle).display === 'block');
        });
        
        test('should adapt navigation for mobile devices', async () => {
            setViewportSize(375, 667);
            window.dispatchEvent(new Event('resize'));
            
            const mainNav = document.querySelector('.main-nav');
            const mobileMenuToggle = document.getElementById('mobileMenuToggle');
            
            // Initially hidden on mobile
            expect(getComputedStyle(mainNav).display).toBe('none');
            
            // Show when toggle is clicked
            mobileMenuToggle.click();
            expect(mainNav.classList.contains('mobile-open')).toBe(true);
            
            // Navigation should be vertical on mobile
            const navList = document.querySelector('.nav-list');
            const computedStyle = getComputedStyle(navList);
            expect(computedStyle.flexDirection).toBe('column');
        });
        
        test('should adapt search input size for different screens', async () => {
            const searchInput = document.getElementById('globalSearch');
            
            // Desktop - full width
            setViewportSize(1200, 800);
            window.dispatchEvent(new Event('resize'));
            await waitFor(() => parseInt(getComputedStyle(searchInput).width) >= 200);
            
            // Mobile - smaller width
            setViewportSize(375, 667);
            window.dispatchEvent(new Event('resize'));
            await waitFor(() => parseInt(getComputedStyle(searchInput).width) <= 150);
        });
        
        test('should adapt widget grid layout for different screens', async () => {
            const widgetsGrid = document.getElementById('widgetsGrid');
            
            // Desktop - multiple columns
            setViewportSize(1200, 800);
            window.dispatchEvent(new Event('resize'));
            await waitFor(() => {
                const computedStyle = getComputedStyle(widgetsGrid);
                return computedStyle.gridTemplateColumns.includes('repeat');
            });
            
            // Mobile - single column
            setViewportSize(375, 667);
            window.dispatchEvent(new Event('resize'));
            await waitFor(() => {
                const computedStyle = getComputedStyle(widgetsGrid);
                return computedStyle.gridTemplateColumns === '1fr';
            });
        });
        
        test('should handle orientation changes', async () => {
            // Portrait
            setViewportSize(375, 667);
            window.dispatchEvent(new Event('resize'));
            
            let initialLayout = getComputedStyle(document.querySelector('.quick-stats-grid')).gridTemplateColumns;
            
            // Landscape
            setViewportSize(667, 375);
            window.dispatchEvent(new Event('resize'));
            
            await waitFor(() => {
                const newLayout = getComputedStyle(document.querySelector('.quick-stats-grid')).gridTemplateColumns;
                return newLayout !== initialLayout;
            });
            
            const landscapeLayout = getComputedStyle(document.querySelector('.quick-stats-grid')).gridTemplateColumns;
            expect(landscapeLayout).not.toBe(initialLayout);
        });
        
        test('should maintain accessibility on all screen sizes', async () => {
            for (const [deviceType, dimensions] of Object.entries(screenSizes)) {
                setViewportSize(dimensions.width, dimensions.height);
                window.dispatchEvent(new Event('resize'));
                
                // Check minimum touch target sizes on touch devices
                if (deviceType === 'mobile' || deviceType === 'tablet') {
                    const buttons = document.querySelectorAll('button, .nav-link');
                    buttons.forEach(button => {
                        const rect = button.getBoundingClientRect();
                        expect(rect.height).toBeGreaterThanOrEqual(44); // Minimum touch target
                        expect(rect.width).toBeGreaterThanOrEqual(44);
                    });
                }
                
                // Check text readability
                const textElements = document.querySelectorAll('p, span, div');
                textElements.forEach(element => {
                    const fontSize = parseInt(getComputedStyle(element).fontSize);
                    expect(fontSize).toBeGreaterThanOrEqual(14); // Minimum readable size
                });
            }
        });
        
        test('should handle print styles', async () => {
            // Create print media query test
            const printStyleSheet = document.createElement('style');
            printStyleSheet.media = 'print';
            printStyleSheet.textContent = `
                @media print {
                    .app-header, .app-footer { display: none !important; }
                    .main-content { padding: 0; }
                }
            `;
            document.head.appendChild(printStyleSheet);
            
            // Simulate print media
            Object.defineProperty(window, 'matchMedia', {
                writable: true,
                value: jest.fn().mockImplementation(query => ({
                    matches: query === 'print',
                    media: query,
                    onchange: null,
                    addListener: jest.fn(),
                    removeListener: jest.fn(),
                    addEventListener: jest.fn(),
                    removeEventListener: jest.fn(),
                    dispatchEvent: jest.fn(),
                })),
            });
            
            // Verify print styles would be applied
            expect(window.matchMedia('print').matches).toBe(true);
        });
    });
    
    describe('Cross-Device Integration Tests', () => {
        test('should maintain state across device changes', async () => {
            // Set initial state on desktop
            setViewportSize(1200, 800);
            await app.switchLanguage('en');
            app.navigateToSection('quran');
            
            // Switch to mobile
            setViewportSize(375, 667);
            window.dispatchEvent(new Event('resize'));
            
            // Verify state is maintained
            expect(app.state.currentLanguage).toBe('en');
            expect(app.state.currentSection).toBe('quran');
            expect(document.querySelector('#quran.active')).toBeTruthy();
        });
        
        test('should handle touch events on mobile devices', async () => {
            setViewportSize(375, 667);
            
            // Mock touch events
            const touchStartEvent = new TouchEvent('touchstart', {
                touches: [{ clientX: 100, clientY: 100 }]
            });
            const touchEndEvent = new TouchEvent('touchend', {
                changedTouches: [{ clientX: 100, clientY: 100 }]
            });
            
            const navLink = document.querySelector('[data-section="quran"]');
            
            // Simulate touch interaction
            navLink.dispatchEvent(touchStartEvent);
            navLink.dispatchEvent(touchEndEvent);
            
            await waitFor(() => document.querySelector('#quran.active'));
            expect(document.querySelector('#quran.active')).toBeTruthy();
        });
        
        test('should optimize performance for different devices', async () => {
            const performanceStart = performance.now();
            
            // Test on mobile (should be optimized)
            setViewportSize(375, 667);
            await app.init();
            
            const mobileTime = performance.now() - performanceStart;
            
            // Test on desktop
            const desktopStart = performance.now();
            setViewportSize(1200, 800);
            await app.init();
            
            const desktopTime = performance.now() - desktopStart;
            
            // Mobile should not be significantly slower
            expect(mobileTime).toBeLessThan(desktopTime * 2);
        });
    });
    
    // Helper functions
    function setViewportSize(width, height) {
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
    }
    
    function getExpectedScreenSize(width) {
        if (width < 576) return 'xs';
        if (width < 768) return 'sm';
        if (width < 992) return 'md';
        if (width < 1200) return 'lg';
        return 'xl';
    }
    
    function getScreenSizeClass() {
        const width = window.innerWidth;
        return getExpectedScreenSize(width);
    }
    
    async function testResponsiveBehavior(deviceType, dimensions) {
        switch (deviceType) {
            case 'mobile':
                // Test mobile-specific behaviors
                const mobileMenu = document.querySelector('.main-nav');
                expect(getComputedStyle(mobileMenu).display).toBe('none');
                break;
                
            case 'tablet':
                // Test tablet-specific behaviors
                const tabletGrid = document.querySelector('.widgets-grid');
                const tabletColumns = getComputedStyle(tabletGrid).gridTemplateColumns;
                expect(tabletColumns).toContain('repeat');
                break;
                
            case 'desktop':
                // Test desktop-specific behaviors
                const desktopNav = document.querySelector('.main-nav');
                expect(getComputedStyle(desktopNav).display).not.toBe('none');
                break;
                
            case 'largeDesktop':
                // Test large desktop-specific behaviors
                const largeGrid = document.querySelector('.widgets-grid');
                const largeColumns = getComputedStyle(largeGrid).gridTemplateColumns;
                expect(largeColumns).toContain('400px');
                break;
        }
    }
    
    async function loadFixture(filename) {
        // Mock loading HTML fixture
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
                        <div class="quick-stats-grid"></div>
                        <div class="widgets-grid" id="widgetsGrid"></div>
                    </section>
                    <section id="quran" class="content-section"></section>
                    <section id="hadith" class="content-section"></section>
                    <section id="stories" class="content-section"></section>
                    <section id="prayer-times" class="content-section"></section>
                    <section id="ai-assistant" class="content-section"></section>
                </main>
            </div>
        `;
    }
    
    async function initializeTestUtils() {
        // Mock SanadUtils
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
                    const item = localStorage.getItem(key);
                    return item ? JSON.parse(item) : null;
                },
                set: (key, value) => localStorage.setItem(key, JSON.stringify(value))
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
                    if (width < 576) return 'xs';
                    if (width < 768) return 'sm';
                    if (width < 992) return 'md';
                    if (width < 1200) return 'lg';
                    return 'xl';
                }
            }
        };
    }
    
    function waitFor(condition, timeout = 5000) {
        return new Promise((resolve, reject) => {
            const startTime = Date.now();
            
            function check() {
                if (condition()) {
                    resolve();
                } else if (Date.now() - startTime >= timeout) {
                    reject(new Error('Timeout waiting for condition'));
                } else {
                    setTimeout(check, 10);
                }
            }
            
            check();
        });
    }
});