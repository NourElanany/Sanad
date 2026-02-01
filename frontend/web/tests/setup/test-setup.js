/**
 * Global Test Setup for Sanad Interface Tests
 * Common setup and utilities for all test suites
 */

// Import testing libraries
import 'jest-dom/extend-expect';

// Global test utilities
global.testUtils = {
  // DOM utilities
  dom: {
    createMockElement: (tag, attributes = {}, content = '') => {
      const element = document.createElement(tag);
      Object.keys(attributes).forEach(key => {
        element.setAttribute(key, attributes[key]);
      });
      if (content) {
        element.textContent = content;
      }
      return element;
    },
    
    simulateEvent: (element, eventType, eventData = {}) => {
      const event = new Event(eventType, { bubbles: true, cancelable: true });
      Object.keys(eventData).forEach(key => {
        event[key] = eventData[key];
      });
      element.dispatchEvent(event);
      return event;
    },
    
    simulateClick: (element) => {
      const clickEvent = new MouseEvent('click', {
        bubbles: true,
        cancelable: true,
        view: window
      });
      element.dispatchEvent(clickEvent);
    },
    
    simulateKeyPress: (element, key, options = {}) => {
      const keyEvent = new KeyboardEvent('keydown', {
        key,
        bubbles: true,
        cancelable: true,
        ...options
      });
      element.dispatchEvent(keyEvent);
    },
    
    waitForElement: async (selector, timeout = 5000) => {
      const startTime = Date.now();
      while (Date.now() - startTime < timeout) {
        const element = document.querySelector(selector);
        if (element) return element;
        await new Promise(resolve => setTimeout(resolve, 10));
      }
      throw new Error(`Element ${selector} not found within ${timeout}ms`);
    }
  },
  
  // Viewport utilities
  viewport: {
    setSize: (width, height) => {
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
      window.dispatchEvent(new Event('resize'));
    },
    
    getBreakpoint: (width) => {
      if (width < 576) return 'xs';
      if (width < 768) return 'sm';
      if (width < 992) return 'md';
      if (width < 1200) return 'lg';
      return 'xl';
    },
    
    simulateDevices: {
      mobile: () => global.testUtils.viewport.setSize(375, 667),
      tablet: () => global.testUtils.viewport.setSize(768, 1024),
      desktop: () => global.testUtils.viewport.setSize(1200, 800),
      largeDesktop: () => global.testUtils.viewport.setSize(1920, 1080)
    }
  },
  
  // Async utilities
  async: {
    wait: (ms) => new Promise(resolve => setTimeout(resolve, ms)),
    
    waitFor: async (condition, timeout = 5000, interval = 10) => {
      const startTime = Date.now();
      while (Date.now() - startTime < timeout) {
        if (await condition()) return true;
        await global.testUtils.async.wait(interval);
      }
      throw new Error(`Condition not met within ${timeout}ms`);
    },
    
    waitForNavigation: async (sectionId, timeout = 5000) => {
      return global.testUtils.async.waitFor(() => {
        const activeSection = document.querySelector('.content-section.active');
        return activeSection && activeSection.id === sectionId;
      }, timeout);
    },
    
    waitForLanguageChange: async (languageCode, timeout = 5000) => {
      return global.testUtils.async.waitFor(() => {
        return document.documentElement.getAttribute('lang') === languageCode;
      }, timeout);
    }
  },
  
  // Mock utilities
  mocks: {
    createMockAPI: () => ({
      i18n: {
        getBulkTranslations: jest.fn().mockResolvedValue({
          translations: {
            'appTitle': 'سند - التطبيق الإسلامي الشامل',
            'loading': 'جاري التحميل...',
            'dashboard': 'الرئيسية',
            'quran': 'القرآن الكريم',
            'hadith': 'الأحاديث النبوية',
            'stories': 'القصص الإسلامية',
            'prayerTimes': 'مواقيت الصلاة',
            'aiAssistant': 'المساعد الذكي'
          }
        })
      },
      search: {
        search: jest.fn().mockResolvedValue([]),
        suggest: jest.fn().mockResolvedValue([])
      }
    }),
    
    createMockConfig: () => ({
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
    }),
    
    createMockUtils: () => ({
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
            // Ignore storage errors in tests
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
          return global.testUtils.viewport.getBreakpoint(width);
        }
      }
    })
  },
  
  // Fixture utilities
  fixtures: {
    createBasicHTML: () => `
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
    `,
    
    loadFixture: async (fixtureName) => {
      // In a real implementation, this would load fixture files
      switch (fixtureName) {
        case 'basic':
          return global.testUtils.fixtures.createBasicHTML();
        default:
          return global.testUtils.fixtures.createBasicHTML();
      }
    }
  }
};

// Global setup
beforeEach(() => {
  // Reset DOM
  document.body.innerHTML = '';
  document.head.innerHTML = '';
  
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
  
  // Reset localStorage and sessionStorage
  localStorage.clear();
  sessionStorage.clear();
  
  // Reset location
  delete window.location;
  window.location = {
    href: 'http://localhost/',
    search: '',
    hash: '',
    pathname: '/'
  };
  
  // Reset navigator
  Object.defineProperty(navigator, 'onLine', {
    writable: true,
    value: true
  });
  
  // Reset console methods to avoid noise in tests
  jest.spyOn(console, 'log').mockImplementation(() => {});
  jest.spyOn(console, 'warn').mockImplementation(() => {});
  jest.spyOn(console, 'error').mockImplementation(() => {});
});

// Global cleanup
afterEach(() => {
  // Restore console methods
  console.log.mockRestore?.();
  console.warn.mockRestore?.();
  console.error.mockRestore?.();
  
  // Clear all timers
  jest.clearAllTimers();
  
  // Clear all mocks
  jest.clearAllMocks();
});

// Global error handling
process.on('unhandledRejection', (reason, promise) => {
  console.error('Unhandled Rejection at:', promise, 'reason:', reason);
});

// Export for use in other test files
export default global.testUtils;