/**
 * Advanced Interfaces Integration for Sanad Islamic App
 * Integrates advanced search, user dashboard, and tafsir comparison interfaces
 */

window.SanadAdvancedIntegration = {
    
    /**
     * Integration state
     */
    state: {
        initialized: false,
        activeInterface: null,
        interfaceStack: []
    },
    
    /**
     * Initialize advanced interfaces integration
     */
    init() {
        if (this.state.initialized) return;
        
        this.setupGlobalEventListeners();
        this.setupKeyboardShortcuts();
        this.setupInterfaceConnections();
        this.enhanceMainNavigation();
        
        this.state.initialized = true;
        console.log('Advanced interfaces integration initialized');
    },
    
    /**
     * Setup global event listeners
     */
    setupGlobalEventListeners() {
        // Listen for interface navigation events
        document.addEventListener('navigateToInterface', (e) => {
            this.navigateToInterface(e.detail.interface, e.detail.params);
        });
        
        // Listen for cross-interface data sharing
        document.addEventListener('shareData', (e) => {
            this.shareDataBetweenInterfaces(e.detail);
        });
        
        // Listen for interface state changes
        document.addEventListener('interfaceStateChanged', (e) => {
            this.handleInterfaceStateChange(e.detail);
        });
        
        // Listen for search result actions
        document.addEventListener('searchResultAction', (e) => {
            this.handleSearchResultAction(e.detail);
        });
    },
    
    /**
     * Setup keyboard shortcuts for advanced interfaces
     */
    setupKeyboardShortcuts() {
        document.addEventListener('keydown', (e) => {
            // Ctrl/Cmd + Shift + S: Open Advanced Search
            if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'S') {
                e.preventDefault();
                this.openAdvancedSearch();
            }
            
            // Ctrl/Cmd + Shift + D: Open Dashboard Customization
            if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'D') {
                e.preventDefault();
                this.openDashboardCustomization();
            }
            
            // Ctrl/Cmd + Shift + T: Open Tafsir Comparison
            if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'T') {
                e.preventDefault();
                this.openTafsirComparison();
            }
            
            // Escape: Close active advanced interface
            if (e.key === 'Escape' && this.state.activeInterface) {
                this.closeActiveInterface();
            }
        });
    },
    
    /**
     * Setup connections between interfaces
     */
    setupInterfaceConnections() {
        // Connect search results to tafsir comparison
        this.setupSearchToTafsirConnection();
        
        // Connect dashboard widgets to other interfaces
        this.setupDashboardConnections();
        
        // Connect tafsir comparison to bookmarks
        this.setupTafsirToBookmarksConnection();
    },
    
    /**
     * Setup search to tafsir connection
     */
    setupSearchToTafsirConnection() {
        document.addEventListener('searchResultAction', (e) => {
            const { action, result } = e.detail;
            
            if (action === 'compare' && result.type === 'tafsir') {
                this.openTafsirComparison(result.surahNumber, result.ayahNumber);
            }
            
            if (action === 'tafsir' && result.type === 'quran') {
                this.openTafsirComparison(result.surahNumber, result.ayahNumber);
            }
        });
    },
    
    /**
     * Setup dashboard connections
     */
    setupDashboardConnections() {
        document.addEventListener('widgetAction', (e) => {
            const { action, widgetType, data } = e.detail;
            
            switch (action) {
                case 'openSearch':
                    this.openAdvancedSearch(data.query);
                    break;
                case 'openTafsir':
                    this.openTafsirComparison(data.surah, data.ayah);
                    break;
                case 'navigateToSection':
                    if (window.SanadApp) {
                        window.SanadApp.navigateToSection(data.section);
                    }
                    break;
            }
        });
    },
    
    /**
     * Setup tafsir to bookmarks connection
     */
    setupTafsirToBookmarksConnection() {
        document.addEventListener('tafsirAction', (e) => {
            const { action, data } = e.detail;
            
            if (action === 'bookmark') {
                this.addToBookmarks(data);
            }
            
            if (action === 'share') {
                this.shareContent(data);
            }
        });
    },
    
    /**
     * Enhance main navigation with advanced interface access
     */
    enhanceMainNavigation() {
        const mainNav = document.querySelector('.main-nav');
        if (!mainNav) return;
        
        // Add advanced search button to header
        this.addAdvancedSearchButton();
        
        // Add dashboard customization button
        this.addDashboardCustomizationButton();
        
        // Add quick access menu
        this.addQuickAccessMenu();
    },
    
    /**
     * Add advanced search button to header
     */
    addAdvancedSearchButton() {
        const searchContainer = document.querySelector('.search-container');
        if (!searchContainer || searchContainer.querySelector('.advanced-search-trigger')) return;
        
        const advancedBtn = window.SanadUtils.dom.create('button', {
            className: 'advanced-search-trigger',
            title: 'البحث المتقدم (Ctrl+Shift+S)'
        });
        
        advancedBtn.innerHTML = `
            <span class="trigger-icon">🔍</span>
            <span class="trigger-text">متقدم</span>
        `;
        
        advancedBtn.addEventListener('click', () => {
            this.openAdvancedSearch();
        });
        
        searchContainer.appendChild(advancedBtn);
    },
    
    /**
     * Add dashboard customization button
     */
    addDashboardCustomizationButton() {
        const dashboardSection = document.getElementById('dashboard');
        if (!dashboardSection) return;
        
        const sectionActions = dashboardSection.querySelector('.section-actions');
        if (!sectionActions) {
            // Create section actions if it doesn't exist
            const sectionHeader = dashboardSection.querySelector('.section-header');
            if (sectionHeader) {
                const actionsDiv = window.SanadUtils.dom.create('div', {
                    className: 'section-actions'
                });
                sectionHeader.appendChild(actionsDiv);
            }
        }
        
        const customizeBtn = window.SanadUtils.dom.create('button', {
            className: 'btn btn-secondary',
            id: 'customizeDashboardBtn'
        });
        
        customizeBtn.innerHTML = `
            <span class="btn-icon">⚙️</span>
            تخصيص اللوحة
        `;
        
        customizeBtn.addEventListener('click', () => {
            this.openDashboardCustomization();
        });
        
        const sectionActions = dashboardSection.querySelector('.section-actions');
        if (sectionActions && !sectionActions.querySelector('#customizeDashboardBtn')) {
            sectionActions.appendChild(customizeBtn);
        }
    },
    
    /**
     * Add quick access menu
     */
    addQuickAccessMenu() {
        const headerActions = document.querySelector('.header-actions');
        if (!headerActions || headerActions.querySelector('.quick-access-menu')) return;
        
        const quickAccessContainer = window.SanadUtils.dom.create('div', {
            className: 'quick-access-menu'
        });
        
        quickAccessContainer.innerHTML = `
            <button class="quick-access-toggle" title="الوصول السريع">
                <span class="toggle-icon">⚡</span>
            </button>
            <div class="quick-access-dropdown">
                <div class="quick-access-item" data-action="advancedSearch">
                    <span class="item-icon">🔍</span>
                    <span class="item-text">البحث المتقدم</span>
                    <span class="item-shortcut">Ctrl+Shift+S</span>
                </div>
                <div class="quick-access-item" data-action="tafsirComparison">
                    <span class="item-icon">⚖️</span>
                    <span class="item-text">مقارنة التفاسير</span>
                    <span class="item-shortcut">Ctrl+Shift+T</span>
                </div>
                <div class="quick-access-item" data-action="dashboardCustomization">
                    <span class="item-icon">⚙️</span>
                    <span class="item-text">تخصيص اللوحة</span>
                    <span class="item-shortcut">Ctrl+Shift+D</span>
                </div>
                <div class="quick-access-divider"></div>
                <div class="quick-access-item" data-action="exportData">
                    <span class="item-icon">📤</span>
                    <span class="item-text">تصدير البيانات</span>
                </div>
                <div class="quick-access-item" data-action="importData">
                    <span class="item-icon">📥</span>
                    <span class="item-text">استيراد البيانات</span>
                </div>
            </div>
        `;
        
        headerActions.insertBefore(quickAccessContainer, headerActions.firstChild);
        
        // Setup quick access menu functionality
        this.setupQuickAccessMenu(quickAccessContainer);
    },
    
    /**
     * Setup quick access menu functionality
     */
    setupQuickAccessMenu(container) {
        const toggle = container.querySelector('.quick-access-toggle');
        const dropdown = container.querySelector('.quick-access-dropdown');
        
        toggle.addEventListener('click', () => {
            dropdown.classList.toggle('active');
        });
        
        // Handle quick access items
        container.querySelectorAll('.quick-access-item').forEach(item => {
            item.addEventListener('click', () => {
                const action = item.dataset.action;
                this.handleQuickAccessAction(action);
                dropdown.classList.remove('active');
            });
        });
        
        // Close dropdown when clicking outside
        document.addEventListener('click', (e) => {
            if (!container.contains(e.target)) {
                dropdown.classList.remove('active');
            }
        });
    },
    
    /**
     * Handle quick access actions
     */
    handleQuickAccessAction(action) {
        switch (action) {
            case 'advancedSearch':
                this.openAdvancedSearch();
                break;
            case 'tafsirComparison':
                this.openTafsirComparison();
                break;
            case 'dashboardCustomization':
                this.openDashboardCustomization();
                break;
            case 'exportData':
                this.exportUserData();
                break;
            case 'importData':
                this.importUserData();
                break;
        }
    },
    
    /**
     * Open advanced search interface
     */
    openAdvancedSearch(query = '') {
        if (window.SanadAdvancedSearch) {
            const event = new CustomEvent('openAdvancedSearch', {
                detail: { query }
            });
            document.dispatchEvent(event);
            this.state.activeInterface = 'advancedSearch';
        } else {
            console.warn('Advanced Search interface not available');
        }
    },
    
    /**
     * Open dashboard customization
     */
    openDashboardCustomization() {
        if (window.SanadUserDashboard) {
            const event = new CustomEvent('customizeDashboard');
            document.dispatchEvent(event);
            this.state.activeInterface = 'dashboardCustomization';
        } else {
            console.warn('User Dashboard interface not available');
        }
    },
    
    /**
     * Open tafsir comparison
     */
    openTafsirComparison(surahNumber = 1, ayahNumber = 1) {
        if (window.SanadTafsirComparison) {
            const event = new CustomEvent('openTafsirComparison', {
                detail: { surahNumber, ayahNumber }
            });
            document.dispatchEvent(event);
            this.state.activeInterface = 'tafsirComparison';
        } else {
            console.warn('Tafsir Comparison interface not available');
        }
    },
    
    /**
     * Close active interface
     */
    closeActiveInterface() {
        switch (this.state.activeInterface) {
            case 'advancedSearch':
                if (window.SanadAdvancedSearch) {
                    window.SanadAdvancedSearch.closeSearchModal();
                }
                break;
            case 'dashboardCustomization':
                if (window.SanadUserDashboard) {
                    window.SanadUserDashboard.toggleCustomizationMode();
                }
                break;
            case 'tafsirComparison':
                if (window.SanadTafsirComparison) {
                    window.SanadTafsirComparison.closeComparison();
                }
                break;
        }
        
        this.state.activeInterface = null;
    },
    
    /**
     * Navigate to interface
     */
    navigateToInterface(interfaceName, params = {}) {
        switch (interfaceName) {
            case 'advancedSearch':
                this.openAdvancedSearch(params.query);
                break;
            case 'tafsirComparison':
                this.openTafsirComparison(params.surahNumber, params.ayahNumber);
                break;
            case 'dashboardCustomization':
                this.openDashboardCustomization();
                break;
        }
    },
    
    /**
     * Share data between interfaces
     */
    shareDataBetweenInterfaces(data) {
        const { from, to, payload } = data;
        
        // Handle data sharing between different interfaces
        switch (`${from}-${to}`) {
            case 'search-tafsir':
                if (payload.type === 'ayah') {
                    this.openTafsirComparison(payload.surahNumber, payload.ayahNumber);
                }
                break;
            case 'dashboard-search':
                this.openAdvancedSearch(payload.query);
                break;
            case 'tafsir-dashboard':
                // Add tafsir to dashboard widget
                this.addTafsirToDashboard(payload);
                break;
        }
    },
    
    /**
     * Handle interface state changes
     */
    handleInterfaceStateChange(detail) {
        const { interface: interfaceName, state, data } = detail;
        
        if (state === 'opened') {
            this.state.activeInterface = interfaceName;
            this.state.interfaceStack.push(interfaceName);
        } else if (state === 'closed') {
            this.state.activeInterface = null;
            this.state.interfaceStack = this.state.interfaceStack.filter(i => i !== interfaceName);
        }
    },
    
    /**
     * Handle search result actions
     */
    handleSearchResultAction(detail) {
        const { action, result } = detail;
        
        switch (action) {
            case 'bookmark':
                this.addToBookmarks(result);
                break;
            case 'share':
                this.shareContent(result);
                break;
            case 'compare':
                if (result.type === 'tafsir' || result.type === 'quran') {
                    this.openTafsirComparison(result.surahNumber, result.ayahNumber);
                }
                break;
            case 'read':
                this.navigateToContent(result);
                break;
        }
    },
    
    /**
     * Add to bookmarks
     */
    addToBookmarks(content) {
        const bookmarks = window.SanadUtils.storage.get('bookmarks') || [];
        
        const bookmark = {
            id: Date.now().toString(),
            type: content.type,
            title: content.title || content.text?.substring(0, 50) + '...',
            content: content,
            timestamp: new Date().toISOString()
        };
        
        bookmarks.unshift(bookmark);
        window.SanadUtils.storage.set('bookmarks', bookmarks.slice(0, 100)); // Keep last 100
        
        if (window.SanadApp && window.SanadApp.showNotification) {
            window.SanadApp.showNotification('تم حفظ المحتوى في المفضلة', 'success');
        }
    },
    
    /**
     * Share content
     */
    shareContent(content) {
        const shareData = {
            title: content.title || 'محتوى من تطبيق سند',
            text: content.text || content.content,
            url: window.location.href
        };
        
        if (navigator.share) {
            navigator.share(shareData);
        } else {
            // Fallback to clipboard
            const shareText = `${shareData.title}\n\n${shareData.text}\n\n${shareData.url}`;
            navigator.clipboard.writeText(shareText).then(() => {
                if (window.SanadApp && window.SanadApp.showNotification) {
                    window.SanadApp.showNotification('تم نسخ المحتوى للحافظة', 'success');
                }
            });
        }
    },
    
    /**
     * Navigate to content
     */
    navigateToContent(content) {
        switch (content.type) {
            case 'quran':
                window.SanadApp.navigateToSection('quran');
                // Additional logic to navigate to specific ayah
                break;
            case 'hadith':
                window.SanadApp.navigateToSection('hadith');
                // Additional logic to show specific hadith
                break;
            case 'story':
                window.SanadApp.navigateToSection('stories');
                // Additional logic to show specific story
                break;
        }
    },
    
    /**
     * Add tafsir to dashboard
     */
    addTafsirToDashboard(tafsirData) {
        if (window.SanadUserDashboard) {
            // Create a tafsir widget for the dashboard
            const widget = {
                id: `tafsir-${Date.now()}`,
                type: 'tafsir-snippet',
                title: `تفسير ${tafsirData.surahName} آية ${tafsirData.ayahNumber}`,
                position: window.SanadUserDashboard.findAvailablePosition(),
                settings: {
                    surahNumber: tafsirData.surahNumber,
                    ayahNumber: tafsirData.ayahNumber,
                    tafsirName: tafsirData.tafsirName,
                    snippet: tafsirData.text.substring(0, 200) + '...'
                }
            };
            
            window.SanadUserDashboard.state.widgets.push(widget);
            window.SanadUserDashboard.renderWidgets();
            window.SanadUserDashboard.saveDashboardLayout();
        }
    },
    
    /**
     * Export user data
     */
    exportUserData() {
        const userData = {
            bookmarks: window.SanadUtils.storage.get('bookmarks') || [],
            searchHistory: window.SanadUtils.storage.get('search_history') || [],
            dashboardLayout: window.SanadUtils.storage.get('dashboard_layout') || {},
            userPreferences: window.SanadUtils.storage.get('user_preferences') || {},
            dhikrCounts: window.SanadUtils.storage.get('dhikr_counts') || {},
            exportDate: new Date().toISOString(),
            version: '1.0'
        };
        
        const blob = new Blob([JSON.stringify(userData, null, 2)], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        
        const a = document.createElement('a');
        a.href = url;
        a.download = `sanad-user-data-${new Date().toISOString().split('T')[0]}.json`;
        a.click();
        
        URL.revokeObjectURL(url);
        
        if (window.SanadApp && window.SanadApp.showNotification) {
            window.SanadApp.showNotification('تم تصدير البيانات بنجاح', 'success');
        }
    },
    
    /**
     * Import user data
     */
    importUserData() {
        const input = document.createElement('input');
        input.type = 'file';
        input.accept = '.json';
        
        input.onchange = (e) => {
            const file = e.target.files[0];
            if (file) {
                const reader = new FileReader();
                reader.onload = (e) => {
                    try {
                        const userData = JSON.parse(e.target.result);
                        this.applyImportedUserData(userData);
                    } catch (error) {
                        if (window.SanadApp && window.SanadApp.showNotification) {
                            window.SanadApp.showNotification('فشل في قراءة ملف البيانات', 'error');
                        }
                    }
                };
                reader.readAsText(file);
            }
        };
        
        input.click();
    },
    
    /**
     * Apply imported user data
     */
    applyImportedUserData(userData) {
        if (confirm('هل أنت متأكد من استيراد البيانات؟ سيتم استبدال البيانات الحالية.')) {
            // Import data selectively
            if (userData.bookmarks) {
                window.SanadUtils.storage.set('bookmarks', userData.bookmarks);
            }
            
            if (userData.searchHistory) {
                window.SanadUtils.storage.set('search_history', userData.searchHistory);
            }
            
            if (userData.dashboardLayout) {
                window.SanadUtils.storage.set('dashboard_layout', userData.dashboardLayout);
            }
            
            if (userData.userPreferences) {
                window.SanadUtils.storage.set('user_preferences', userData.userPreferences);
            }
            
            if (userData.dhikrCounts) {
                window.SanadUtils.storage.set('dhikr_counts', userData.dhikrCounts);
            }
            
            // Reload interfaces to reflect imported data
            if (window.SanadUserDashboard) {
                window.SanadUserDashboard.loadDashboardWidgets();
            }
            
            if (window.SanadAdvancedSearch) {
                window.SanadAdvancedSearch.loadSearchHistory();
            }
            
            if (window.SanadApp && window.SanadApp.showNotification) {
                window.SanadApp.showNotification('تم استيراد البيانات بنجاح', 'success');
            }
        }
    }
};

// Initialize when DOM is ready
window.SanadUtils.timing.ready(() => {
    window.SanadAdvancedIntegration.init();
});

// Freeze the object to prevent modifications
Object.freeze(window.SanadAdvancedIntegration);