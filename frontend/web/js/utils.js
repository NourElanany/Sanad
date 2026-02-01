/**
 * Utility Functions for Sanad Islamic App
 * Contains common helper functions used throughout the application
 */

window.SanadUtils = {
    
    /**
     * DOM Utilities
     */
    dom: {
        /**
         * Get element by ID
         */
        get: (id) => document.getElementById(id),
        
        /**
         * Get elements by class name
         */
        getByClass: (className) => document.getElementsByClassName(className),
        
        /**
         * Get elements by selector
         */
        query: (selector) => document.querySelector(selector),
        
        /**
         * Get all elements by selector
         */
        queryAll: (selector) => document.querySelectorAll(selector),
        
        /**
         * Create element with attributes
         */
        create: (tag, attributes = {}, content = '') => {
            const element = document.createElement(tag);
            Object.keys(attributes).forEach(key => {
                if (key === 'className') {
                    element.className = attributes[key];
                } else if (key === 'innerHTML') {
                    element.innerHTML = attributes[key];
                } else {
                    element.setAttribute(key, attributes[key]);
                }
            });
            if (content) {
                element.textContent = content;
            }
            return element;
        },
        
        /**
         * Add event listener with automatic cleanup
         */
        on: (element, event, handler, options = {}) => {
            if (typeof element === 'string') {
                element = document.querySelector(element);
            }
            if (element) {
                element.addEventListener(event, handler, options);
                return () => element.removeEventListener(event, handler, options);
            }
            return () => {};
        },
        
        /**
         * Remove element from DOM
         */
        remove: (element) => {
            if (typeof element === 'string') {
                element = document.querySelector(element);
            }
            if (element && element.parentNode) {
                element.parentNode.removeChild(element);
            }
        },
        
        /**
         * Toggle class on element
         */
        toggleClass: (element, className) => {
            if (typeof element === 'string') {
                element = document.querySelector(element);
            }
            if (element) {
                element.classList.toggle(className);
            }
        },
        
        /**
         * Add class to element
         */
        addClass: (element, className) => {
            if (typeof element === 'string') {
                element = document.querySelector(element);
            }
            if (element) {
                element.classList.add(className);
            }
        },
        
        /**
         * Remove class from element
         */
        removeClass: (element, className) => {
            if (typeof element === 'string') {
                element = document.querySelector(element);
            }
            if (element) {
                element.classList.remove(className);
            }
        },
        
        /**
         * Check if element has class
         */
        hasClass: (element, className) => {
            if (typeof element === 'string') {
                element = document.querySelector(element);
            }
            return element ? element.classList.contains(className) : false;
        }
    },

    /**
     * Storage Utilities
     */
    storage: {
        /**
         * Set item in localStorage with JSON serialization
         */
        set: (key, value) => {
            try {
                localStorage.setItem(key, JSON.stringify(value));
                return true;
            } catch (error) {
                console.error('Storage set error:', error);
                return false;
            }
        },
        
        /**
         * Get item from localStorage with JSON parsing
         */
        get: (key, defaultValue = null) => {
            try {
                const item = localStorage.getItem(key);
                return item ? JSON.parse(item) : defaultValue;
            } catch (error) {
                console.error('Storage get error:', error);
                return defaultValue;
            }
        },
        
        /**
         * Remove item from localStorage
         */
        remove: (key) => {
            try {
                localStorage.removeItem(key);
                return true;
            } catch (error) {
                console.error('Storage remove error:', error);
                return false;
            }
        },
        
        /**
         * Clear all localStorage
         */
        clear: () => {
            try {
                localStorage.clear();
                return true;
            } catch (error) {
                console.error('Storage clear error:', error);
                return false;
            }
        },
        
        /**
         * Check if localStorage is available
         */
        isAvailable: () => {
            try {
                const test = '__storage_test__';
                localStorage.setItem(test, test);
                localStorage.removeItem(test);
                return true;
            } catch (error) {
                return false;
            }
        }
    },

    /**
     * String Utilities
     */
    string: {
        /**
         * Capitalize first letter
         */
        capitalize: (str) => {
            return str.charAt(0).toUpperCase() + str.slice(1);
        },
        
        /**
         * Truncate string with ellipsis
         */
        truncate: (str, length = 100, suffix = '...') => {
            if (str.length <= length) return str;
            return str.substring(0, length) + suffix;
        },
        
        /**
         * Remove HTML tags
         */
        stripHtml: (str) => {
            const div = document.createElement('div');
            div.innerHTML = str;
            return div.textContent || div.innerText || '';
        },
        
        /**
         * Escape HTML characters
         */
        escapeHtml: (str) => {
            const div = document.createElement('div');
            div.textContent = str;
            return div.innerHTML;
        },
        
        /**
         * Generate random string
         */
        random: (length = 10) => {
            const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
            let result = '';
            for (let i = 0; i < length; i++) {
                result += chars.charAt(Math.floor(Math.random() * chars.length));
            }
            return result;
        },
        
        /**
         * Convert to slug
         */
        toSlug: (str) => {
            return str
                .toLowerCase()
                .replace(/[^\w\s-]/g, '')
                .replace(/[\s_-]+/g, '-')
                .replace(/^-+|-+$/g, '');
        }
    },

    /**
     * Date and Time Utilities
     */
    date: {
        /**
         * Format date for display
         */
        format: (date, locale = 'ar-SA', options = {}) => {
            if (!(date instanceof Date)) {
                date = new Date(date);
            }
            const defaultOptions = {
                year: 'numeric',
                month: 'long',
                day: 'numeric'
            };
            return date.toLocaleDateString(locale, { ...defaultOptions, ...options });
        },
        
        /**
         * Format time for display
         */
        formatTime: (date, locale = 'ar-SA', options = {}) => {
            if (!(date instanceof Date)) {
                date = new Date(date);
            }
            const defaultOptions = {
                hour: '2-digit',
                minute: '2-digit',
                hour12: false
            };
            return date.toLocaleTimeString(locale, { ...defaultOptions, ...options });
        },
        
        /**
         * Get relative time (e.g., "2 hours ago")
         */
        relative: (date, locale = 'ar') => {
            if (!(date instanceof Date)) {
                date = new Date(date);
            }
            const now = new Date();
            const diff = now - date;
            const seconds = Math.floor(diff / 1000);
            const minutes = Math.floor(seconds / 60);
            const hours = Math.floor(minutes / 60);
            const days = Math.floor(hours / 24);
            
            const translations = {
                ar: {
                    now: 'الآن',
                    secondsAgo: 'منذ ثوانٍ',
                    minuteAgo: 'منذ دقيقة',
                    minutesAgo: 'منذ {n} دقائق',
                    hourAgo: 'منذ ساعة',
                    hoursAgo: 'منذ {n} ساعات',
                    dayAgo: 'منذ يوم',
                    daysAgo: 'منذ {n} أيام'
                },
                en: {
                    now: 'now',
                    secondsAgo: 'seconds ago',
                    minuteAgo: '1 minute ago',
                    minutesAgo: '{n} minutes ago',
                    hourAgo: '1 hour ago',
                    hoursAgo: '{n} hours ago',
                    dayAgo: '1 day ago',
                    daysAgo: '{n} days ago'
                }
            };
            
            const t = translations[locale] || translations.en;
            
            if (seconds < 30) return t.now;
            if (seconds < 60) return t.secondsAgo;
            if (minutes === 1) return t.minuteAgo;
            if (minutes < 60) return t.minutesAgo.replace('{n}', minutes);
            if (hours === 1) return t.hourAgo;
            if (hours < 24) return t.hoursAgo.replace('{n}', hours);
            if (days === 1) return t.dayAgo;
            return t.daysAgo.replace('{n}', days);
        },
        
        /**
         * Check if date is today
         */
        isToday: (date) => {
            if (!(date instanceof Date)) {
                date = new Date(date);
            }
            const today = new Date();
            return date.toDateString() === today.toDateString();
        },
        
        /**
         * Add days to date
         */
        addDays: (date, days) => {
            if (!(date instanceof Date)) {
                date = new Date(date);
            }
            const result = new Date(date);
            result.setDate(result.getDate() + days);
            return result;
        }
    },

    /**
     * Number Utilities
     */
    number: {
        /**
         * Format number with locale
         */
        format: (num, locale = 'ar-SA') => {
            return new Intl.NumberFormat(locale).format(num);
        },
        
        /**
         * Convert to Arabic numerals
         */
        toArabic: (num) => {
            const arabicNumerals = '٠١٢٣٤٥٦٧٨٩';
            return num.toString().replace(/[0-9]/g, (digit) => arabicNumerals[digit]);
        },
        
        /**
         * Convert from Arabic numerals
         */
        fromArabic: (str) => {
            const arabicNumerals = '٠١٢٣٤٥٦٧٨٩';
            return str.replace(/[٠-٩]/g, (digit) => arabicNumerals.indexOf(digit));
        },
        
        /**
         * Clamp number between min and max
         */
        clamp: (num, min, max) => {
            return Math.min(Math.max(num, min), max);
        },
        
        /**
         * Generate random number between min and max
         */
        random: (min = 0, max = 100) => {
            return Math.floor(Math.random() * (max - min + 1)) + min;
        }
    },

    /**
     * Validation Utilities
     */
    validate: {
        /**
         * Check if email is valid
         */
        email: (email) => {
            const regex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
            return regex.test(email);
        },
        
        /**
         * Check if string is not empty
         */
        required: (value) => {
            return value !== null && value !== undefined && value.toString().trim() !== '';
        },
        
        /**
         * Check minimum length
         */
        minLength: (value, min) => {
            return value && value.toString().length >= min;
        },
        
        /**
         * Check maximum length
         */
        maxLength: (value, max) => {
            return !value || value.toString().length <= max;
        },
        
        /**
         * Check if value is numeric
         */
        numeric: (value) => {
            return !isNaN(value) && !isNaN(parseFloat(value));
        },
        
        /**
         * Check if value is integer
         */
        integer: (value) => {
            return Number.isInteger(Number(value));
        }
    },

    /**
     * URL Utilities
     */
    url: {
        /**
         * Get query parameter value
         */
        getParam: (name) => {
            const urlParams = new URLSearchParams(window.location.search);
            return urlParams.get(name);
        },
        
        /**
         * Set query parameter
         */
        setParam: (name, value) => {
            const url = new URL(window.location);
            url.searchParams.set(name, value);
            window.history.pushState({}, '', url);
        },
        
        /**
         * Remove query parameter
         */
        removeParam: (name) => {
            const url = new URL(window.location);
            url.searchParams.delete(name);
            window.history.pushState({}, '', url);
        },
        
        /**
         * Build URL with parameters
         */
        build: (base, params = {}) => {
            const url = new URL(base, window.location.origin);
            Object.keys(params).forEach(key => {
                if (params[key] !== null && params[key] !== undefined) {
                    url.searchParams.set(key, params[key]);
                }
            });
            return url.toString();
        }
    },

    /**
     * Debounce and Throttle Utilities
     */
    timing: {
        /**
         * Debounce function execution
         */
        debounce: (func, delay = 300) => {
            let timeoutId;
            return function (...args) {
                clearTimeout(timeoutId);
                timeoutId = setTimeout(() => func.apply(this, args), delay);
            };
        },
        
        /**
         * Throttle function execution
         */
        throttle: (func, limit = 100) => {
            let inThrottle;
            return function (...args) {
                if (!inThrottle) {
                    func.apply(this, args);
                    inThrottle = true;
                    setTimeout(() => inThrottle = false, limit);
                }
            };
        },
        
        /**
         * Delay execution
         */
        delay: (ms) => new Promise(resolve => setTimeout(resolve, ms)),
        
        /**
         * Execute function after DOM is ready
         */
        ready: (callback) => {
            if (document.readyState === 'loading') {
                document.addEventListener('DOMContentLoaded', callback);
            } else {
                callback();
            }
        }
    },

    /**
     * Array Utilities
     */
    array: {
        /**
         * Remove duplicates from array
         */
        unique: (arr) => [...new Set(arr)],
        
        /**
         * Shuffle array
         */
        shuffle: (arr) => {
            const result = [...arr];
            for (let i = result.length - 1; i > 0; i--) {
                const j = Math.floor(Math.random() * (i + 1));
                [result[i], result[j]] = [result[j], result[i]];
            }
            return result;
        },
        
        /**
         * Chunk array into smaller arrays
         */
        chunk: (arr, size) => {
            const chunks = [];
            for (let i = 0; i < arr.length; i += size) {
                chunks.push(arr.slice(i, i + size));
            }
            return chunks;
        },
        
        /**
         * Get random item from array
         */
        random: (arr) => arr[Math.floor(Math.random() * arr.length)],
        
        /**
         * Sort array by property
         */
        sortBy: (arr, prop, desc = false) => {
            return arr.sort((a, b) => {
                const aVal = a[prop];
                const bVal = b[prop];
                if (desc) {
                    return bVal > aVal ? 1 : bVal < aVal ? -1 : 0;
                }
                return aVal > bVal ? 1 : aVal < bVal ? -1 : 0;
            });
        }
    },

    /**
     * Object Utilities
     */
    object: {
        /**
         * Deep clone object
         */
        clone: (obj) => {
            if (obj === null || typeof obj !== 'object') return obj;
            if (obj instanceof Date) return new Date(obj.getTime());
            if (obj instanceof Array) return obj.map(item => SanadUtils.object.clone(item));
            if (typeof obj === 'object') {
                const cloned = {};
                Object.keys(obj).forEach(key => {
                    cloned[key] = SanadUtils.object.clone(obj[key]);
                });
                return cloned;
            }
        },
        
        /**
         * Merge objects deeply
         */
        merge: (target, ...sources) => {
            if (!sources.length) return target;
            const source = sources.shift();
            
            if (SanadUtils.object.isObject(target) && SanadUtils.object.isObject(source)) {
                for (const key in source) {
                    if (SanadUtils.object.isObject(source[key])) {
                        if (!target[key]) Object.assign(target, { [key]: {} });
                        SanadUtils.object.merge(target[key], source[key]);
                    } else {
                        Object.assign(target, { [key]: source[key] });
                    }
                }
            }
            
            return SanadUtils.object.merge(target, ...sources);
        },
        
        /**
         * Check if value is object
         */
        isObject: (item) => {
            return item && typeof item === 'object' && !Array.isArray(item);
        },
        
        /**
         * Get nested property value
         */
        get: (obj, path, defaultValue = undefined) => {
            const keys = path.split('.');
            let result = obj;
            
            for (const key of keys) {
                if (result === null || result === undefined || !(key in result)) {
                    return defaultValue;
                }
                result = result[key];
            }
            
            return result;
        },
        
        /**
         * Set nested property value
         */
        set: (obj, path, value) => {
            const keys = path.split('.');
            const lastKey = keys.pop();
            let current = obj;
            
            for (const key of keys) {
                if (!(key in current) || !SanadUtils.object.isObject(current[key])) {
                    current[key] = {};
                }
                current = current[key];
            }
            
            current[lastKey] = value;
            return obj;
        }
    },

    /**
     * Device and Browser Detection
     */
    device: {
        /**
         * Check if mobile device
         */
        isMobile: () => {
            return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(navigator.userAgent);
        },
        
        /**
         * Check if tablet
         */
        isTablet: () => {
            return /iPad|Android(?!.*Mobile)/i.test(navigator.userAgent);
        },
        
        /**
         * Check if desktop
         */
        isDesktop: () => {
            return !SanadUtils.device.isMobile() && !SanadUtils.device.isTablet();
        },
        
        /**
         * Check if touch device
         */
        isTouch: () => {
            return 'ontouchstart' in window || navigator.maxTouchPoints > 0;
        },
        
        /**
         * Get screen size category
         */
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

// Freeze the utilities to prevent modifications
Object.freeze(window.SanadUtils);