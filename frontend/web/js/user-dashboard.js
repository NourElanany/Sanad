/**
 * Personal User Dashboard for Sanad Islamic App
 * Provides personalized dashboard with customizable widgets and user preferences
 */

window.SanadUserDashboard = {
    
    /**
     * Dashboard state
     */
    state: {
        userId: null,
        userProfile: null,
        widgets: [],
        preferences: {},
        isEditing: false,
        draggedWidget: null
    },
    
    /**
     * DOM elements cache
     */
    elements: {},
    
    /**
     * Initialize user dashboard
     */
    init() {
        this.cacheElements();
        this.setupEventListeners();
        this.loadUserProfile();
        this.initializeDashboard();
    },
    
    /**
     * Cache DOM elements
     */
    cacheElements() {
        this.elements = {
            dashboardContainer: document.getElementById('dashboard'),
            widgetsGrid: document.getElementById('widgetsGrid'),
            userProfileSection: null, // Will be created
            dashboardControls: null, // Will be created
            customizationPanel: null // Will be created
        };
    },
    
    /**
     * Setup event listeners
     */
    setupEventListeners() {
        // Listen for dashboard customization events
        document.addEventListener('customizeDashboard', () => {
            this.toggleCustomizationMode();
        });
        
        // Listen for widget updates
        document.addEventListener('widgetUpdated', (e) => {
            this.handleWidgetUpdate(e.detail);
        });
        
        // Listen for user profile updates
        document.addEventListener('userProfileUpdated', (e) => {
            this.handleProfileUpdate(e.detail);
        });
    },
    
    /**
     * Load user profile
     */
    async loadUserProfile() {
        try {
            // For now, use mock data - in real implementation, get from API
            this.state.userProfile = {
                id: 'user_123',
                name: 'أحمد محمد',
                email: 'ahmed@example.com',
                joinDate: '2024-01-15',
                preferences: {
                    language: 'ar',
                    theme: 'light',
                    prayerMethod: 'MWL',
                    location: {
                        city: 'الرياض',
                        country: 'السعودية'
                    }
                },
                stats: {
                    totalReadingTime: 1250, // minutes
                    completedKhatmas: 3,
                    bookmarkedAyahs: 45,
                    bookmarkedHadiths: 23,
                    streakDays: 12,
                    favoriteSupplication: 'أستغفر الله العظيم'
                }
            };
            
            this.state.userId = this.state.userProfile.id;
            this.state.preferences = this.state.userProfile.preferences;
            
        } catch (error) {
            console.error('Failed to load user profile:', error);
        }
    },
    
    /**
     * Initialize dashboard
     */
    async initializeDashboard() {
        this.createUserProfileSection();
        this.createDashboardControls();
        await this.loadDashboardWidgets();
        this.setupDragAndDrop();
    },
    
    /**
     * Create user profile section
     */
    createUserProfileSection() {
        const dashboardHeader = this.elements.dashboardContainer.querySelector('.section-header');
        
        const profileSection = window.SanadUtils.dom.create('div', {
            className: 'user-profile-section'
        });
        
        profileSection.innerHTML = `
            <div class="profile-card">
                <div class="profile-avatar">
                    <div class="avatar-circle">
                        <span class="avatar-initial">${this.state.userProfile.name.charAt(0)}</span>
                    </div>
                    <div class="online-indicator"></div>
                </div>
                
                <div class="profile-info">
                    <h2 class="profile-name">${this.state.userProfile.name}</h2>
                    <p class="profile-greeting">السلام عليكم ورحمة الله وبركاته</p>
                    <div class="profile-stats-quick">
                        <div class="quick-stat">
                            <span class="stat-value">${this.state.userProfile.stats.streakDays}</span>
                            <span class="stat-label">يوم متتالي</span>
                        </div>
                        <div class="quick-stat">
                            <span class="stat-value">${this.state.userProfile.stats.completedKhatmas}</span>
                            <span class="stat-label">ختمة مكتملة</span>
                        </div>
                        <div class="quick-stat">
                            <span class="stat-value">${Math.round(this.state.userProfile.stats.totalReadingTime / 60)}</span>
                            <span class="stat-label">ساعة قراءة</span>
                        </div>
                    </div>
                </div>
                
                <div class="profile-actions">
                    <button class="btn btn-outline" id="editProfile">
                        <span class="btn-icon">✏️</span>
                        تعديل الملف الشخصي
                    </button>
                    <button class="btn btn-primary" id="customizeDashboard">
                        <span class="btn-icon">⚙️</span>
                        تخصيص اللوحة
                    </button>
                </div>
            </div>
            
            <div class="daily-motivation">
                <div class="motivation-content">
                    <div class="motivation-icon">🌟</div>
                    <div class="motivation-text">
                        <h3>دعاء اليوم</h3>
                        <p class="arabic-text">"رَبَّنَا آتِنَا فِي الدُّنْيَا حَسَنَةً وَفِي الْآخِرَةِ حَسَنَةً وَقِنَا عَذَابَ النَّارِ"</p>
                        <p class="translation">ربنا آتنا في الدنيا حسنة وفي الآخرة حسنة وقنا عذاب النار</p>
                    </div>
                </div>
            </div>
        `;
        
        dashboardHeader.appendChild(profileSection);
        this.elements.userProfileSection = profileSection;
        
        // Setup profile event listeners
        profileSection.querySelector('#editProfile').addEventListener('click', () => {
            this.openProfileEditor();
        });
        
        profileSection.querySelector('#customizeDashboard').addEventListener('click', () => {
            this.toggleCustomizationMode();
        });
    },
    
    /**
     * Create dashboard controls
     */
    createDashboardControls() {
        const controlsContainer = window.SanadUtils.dom.create('div', {
            className: 'dashboard-controls'
        });
        
        controlsContainer.innerHTML = `
            <div class="controls-left">
                <div class="view-options">
                    <button class="view-btn active" data-view="grid">
                        <span class="view-icon">⊞</span>
                        شبكة
                    </button>
                    <button class="view-btn" data-view="list">
                        <span class="view-icon">☰</span>
                        قائمة
                    </button>
                </div>
                
                <div class="filter-options">
                    <select id="widgetFilter" class="filter-select">
                        <option value="all">جميع الودجات</option>
                        <option value="prayer">المواقيت والعبادة</option>
                        <option value="reading">القراءة والدراسة</option>
                        <option value="progress">التقدم والإحصائيات</option>
                        <option value="reminders">التذكيرات والأذكار</option>
                    </select>
                </div>
            </div>
            
            <div class="controls-right">
                <button class="btn btn-secondary" id="addWidget">
                    <span class="btn-icon">➕</span>
                    إضافة ودجة
                </button>
                
                <button class="btn btn-outline" id="resetLayout">
                    <span class="btn-icon">🔄</span>
                    إعادة تعيين التخطيط
                </button>
                
                <div class="dashboard-settings">
                    <button class="settings-toggle" id="dashboardSettings">
                        <span class="settings-icon">⚙️</span>
                    </button>
                    <div class="settings-dropdown" id="settingsDropdown">
                        <div class="settings-item">
                            <label>
                                <input type="checkbox" id="autoRefresh" checked>
                                التحديث التلقائي
                            </label>
                        </div>
                        <div class="settings-item">
                            <label>
                                <input type="checkbox" id="compactMode">
                                الوضع المضغوط
                            </label>
                        </div>
                        <div class="settings-item">
                            <label>
                                <input type="checkbox" id="showAnimations" checked>
                                إظهار الحركات
                            </label>
                        </div>
                        <div class="settings-divider"></div>
                        <button class="settings-action" id="exportSettings">
                            تصدير الإعدادات
                        </button>
                        <button class="settings-action" id="importSettings">
                            استيراد الإعدادات
                        </button>
                    </div>
                </div>
            </div>
        `;
        
        this.elements.widgetsGrid.parentNode.insertBefore(controlsContainer, this.elements.widgetsGrid);
        this.elements.dashboardControls = controlsContainer;
        
        // Setup controls event listeners
        this.setupControlsEventListeners();
    },
    
    /**
     * Setup controls event listeners
     */
    setupControlsEventListeners() {
        const controls = this.elements.dashboardControls;
        
        // View toggle
        controls.querySelectorAll('.view-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                this.switchView(btn.dataset.view);
            });
        });
        
        // Widget filter
        controls.querySelector('#widgetFilter').addEventListener('change', (e) => {
            this.filterWidgets(e.target.value);
        });
        
        // Add widget
        controls.querySelector('#addWidget').addEventListener('click', () => {
            this.openWidgetSelector();
        });
        
        // Reset layout
        controls.querySelector('#resetLayout').addEventListener('click', () => {
            this.resetDashboardLayout();
        });
        
        // Settings dropdown
        const settingsToggle = controls.querySelector('#dashboardSettings');
        const settingsDropdown = controls.querySelector('#settingsDropdown');
        
        settingsToggle.addEventListener('click', () => {
            settingsDropdown.classList.toggle('active');
        });
        
        // Settings options
        controls.querySelector('#autoRefresh').addEventListener('change', (e) => {
            this.toggleAutoRefresh(e.target.checked);
        });
        
        controls.querySelector('#compactMode').addEventListener('change', (e) => {
            this.toggleCompactMode(e.target.checked);
        });
        
        controls.querySelector('#showAnimations').addEventListener('change', (e) => {
            this.toggleAnimations(e.target.checked);
        });
        
        // Export/Import settings
        controls.querySelector('#exportSettings').addEventListener('click', () => {
            this.exportDashboardSettings();
        });
        
        controls.querySelector('#importSettings').addEventListener('click', () => {
            this.importDashboardSettings();
        });
        
        // Close dropdown when clicking outside
        document.addEventListener('click', (e) => {
            if (!e.target.closest('.dashboard-settings')) {
                settingsDropdown.classList.remove('active');
            }
        });
    },
    
    /**
     * Load dashboard widgets
     */
    async loadDashboardWidgets() {
        try {
            // Load user's widget configuration
            const savedLayout = window.SanadUtils.storage.get('dashboard_layout');
            
            if (savedLayout && savedLayout.widgets) {
                this.state.widgets = savedLayout.widgets;
            } else {
                // Default widgets for new users
                this.state.widgets = [
                    {
                        id: 'prayer-times',
                        type: 'prayer-times',
                        title: 'مواقيت الصلاة',
                        position: { row: 1, col: 1, width: 2, height: 1 },
                        settings: { showNextPrayer: true, showAllPrayers: true }
                    },
                    {
                        id: 'verse-of-day',
                        type: 'verse-of-day',
                        title: 'آية اليوم',
                        position: { row: 1, col: 3, width: 2, height: 1 },
                        settings: { showTranslation: true, showTafsir: false }
                    },
                    {
                        id: 'khatma-progress',
                        type: 'khatma-progress',
                        title: 'تقدم الختمة',
                        position: { row: 2, col: 1, width: 1, height: 1 },
                        settings: { showStats: true, showChart: true }
                    },
                    {
                        id: 'dhikr-counter',
                        type: 'dhikr-counter',
                        title: 'عداد الأذكار',
                        position: { row: 2, col: 2, width: 1, height: 1 },
                        settings: { defaultDhikr: 'استغفار', target: 100 }
                    },
                    {
                        id: 'hijri-calendar',
                        type: 'hijri-calendar',
                        title: 'التقويم الهجري',
                        position: { row: 2, col: 3, width: 1, height: 1 },
                        settings: { showEvents: true, showGregorian: true }
                    },
                    {
                        id: 'quick-stats',
                        type: 'quick-stats',
                        title: 'الإحصائيات السريعة',
                        position: { row: 2, col: 4, width: 1, height: 1 },
                        settings: { showReadingTime: true, showStreak: true }
                    }
                ];
            }
            
            // Render widgets
            await this.renderWidgets();
            
        } catch (error) {
            console.error('Failed to load dashboard widgets:', error);
        }
    },
    
    /**
     * Render widgets
     */
    async renderWidgets() {
        this.elements.widgetsGrid.innerHTML = '';
        
        for (const widget of this.state.widgets) {
            const widgetElement = await this.createWidgetElement(widget);
            this.elements.widgetsGrid.appendChild(widgetElement);
        }
        
        // Apply grid layout
        this.applyGridLayout();
    },
    
    /**
     * Create widget element
     */
    async createWidgetElement(widget) {
        const widgetDiv = window.SanadUtils.dom.create('div', {
            className: `dashboard-widget widget-${widget.type}`,
            id: `widget-${widget.id}`,
            'data-widget-id': widget.id,
            'data-widget-type': widget.type
        });
        
        // Widget header
        const header = window.SanadUtils.dom.create('div', {
            className: 'widget-header'
        });
        
        header.innerHTML = `
            <h3 class="widget-title">${widget.title}</h3>
            <div class="widget-controls">
                <button class="widget-control refresh-widget" title="تحديث">
                    <span class="control-icon">🔄</span>
                </button>
                <button class="widget-control settings-widget" title="إعدادات">
                    <span class="control-icon">⚙️</span>
                </button>
                <button class="widget-control remove-widget" title="إزالة">
                    <span class="control-icon">✕</span>
                </button>
                <button class="widget-control drag-handle" title="سحب">
                    <span class="control-icon">⋮⋮</span>
                </button>
            </div>
        `;
        
        // Widget content
        const content = window.SanadUtils.dom.create('div', {
            className: 'widget-content'
        });
        
        // Load widget content based on type
        content.innerHTML = await this.getWidgetContent(widget);
        
        widgetDiv.appendChild(header);
        widgetDiv.appendChild(content);
        
        // Setup widget event listeners
        this.setupWidgetEventListeners(widgetDiv, widget);
        
        return widgetDiv;
    },
    
    /**
     * Get widget content based on type
     */
    async getWidgetContent(widget) {
        try {
            switch (widget.type) {
                case 'prayer-times':
                    return await this.getPrayerTimesContent(widget);
                case 'verse-of-day':
                    return await this.getVerseOfDayContent(widget);
                case 'khatma-progress':
                    return await this.getKhatmaProgressContent(widget);
                case 'dhikr-counter':
                    return await this.getDhikrCounterContent(widget);
                case 'hijri-calendar':
                    return await this.getHijriCalendarContent(widget);
                case 'quick-stats':
                    return await this.getQuickStatsContent(widget);
                default:
                    return '<p>نوع الودجة غير مدعوم</p>';
            }
        } catch (error) {
            console.error(`Failed to load content for widget ${widget.id}:`, error);
            return '<p>فشل في تحميل محتوى الودجة</p>';
        }
    },
    
    /**
     * Get prayer times widget content
     */
    async getPrayerTimesContent(widget) {
        // Mock prayer times data
        const prayerTimes = {
            fajr: '05:15',
            sunrise: '06:45',
            dhuhr: '12:30',
            asr: '15:45',
            maghrib: '18:20',
            isha: '19:50',
            nextPrayer: 'المغرب',
            timeToNext: '2:15:30'
        };
        
        return `
            <div class="prayer-times-widget">
                <div class="next-prayer-info">
                    <div class="next-prayer-name">${prayerTimes.nextPrayer}</div>
                    <div class="time-remaining">${prayerTimes.timeToNext}</div>
                    <div class="next-prayer-time">في ${prayerTimes.maghrib}</div>
                </div>
                
                ${widget.settings.showAllPrayers ? `
                <div class="all-prayers">
                    <div class="prayer-item">
                        <span class="prayer-name">الفجر</span>
                        <span class="prayer-time">${prayerTimes.fajr}</span>
                    </div>
                    <div class="prayer-item">
                        <span class="prayer-name">الظهر</span>
                        <span class="prayer-time">${prayerTimes.dhuhr}</span>
                    </div>
                    <div class="prayer-item">
                        <span class="prayer-name">العصر</span>
                        <span class="prayer-time">${prayerTimes.asr}</span>
                    </div>
                    <div class="prayer-item">
                        <span class="prayer-name">المغرب</span>
                        <span class="prayer-time">${prayerTimes.maghrib}</span>
                    </div>
                    <div class="prayer-item">
                        <span class="prayer-name">العشاء</span>
                        <span class="prayer-time">${prayerTimes.isha}</span>
                    </div>
                </div>
                ` : ''}
            </div>
        `;
    },
    
    /**
     * Get verse of day widget content
     */
    async getVerseOfDayContent(widget) {
        // Mock verse data
        const verse = {
            arabic: 'وَمَن يَتَّقِ اللَّهَ يَجْعَل لَّهُ مَخْرَجًا',
            translation: 'ومن يتق الله يجعل له مخرجاً',
            reference: 'سورة الطلاق - آية 2'
        };
        
        return `
            <div class="verse-widget">
                <div class="verse-arabic">${verse.arabic}</div>
                ${widget.settings.showTranslation ? `
                <div class="verse-translation">${verse.translation}</div>
                ` : ''}
                <div class="verse-reference">${verse.reference}</div>
            </div>
        `;
    },
    
    /**
     * Get Khatma progress widget content
     */
    async getKhatmaProgressContent(widget) {
        // Mock progress data
        const progress = {
            currentSurah: 'البقرة',
            currentAyah: 156,
            totalProgress: 35,
            pagesRead: 42,
            totalPages: 604,
            estimatedCompletion: '15 يوم'
        };
        
        return `
            <div class="khatma-progress-widget">
                <div class="progress-info">
                    <div class="current-position">
                        <span class="surah-name">${progress.currentSurah}</span>
                        <span class="ayah-number">آية ${progress.currentAyah}</span>
                    </div>
                    <div class="progress-percentage">${progress.totalProgress}%</div>
                </div>
                
                <div class="progress-bar">
                    <div class="progress-fill" style="width: ${progress.totalProgress}%"></div>
                </div>
                
                ${widget.settings.showStats ? `
                <div class="progress-stats">
                    <div class="stat-item">
                        <span class="stat-value">${progress.pagesRead}</span>
                        <span class="stat-label">صفحة مقروءة</span>
                    </div>
                    <div class="stat-item">
                        <span class="stat-value">${progress.estimatedCompletion}</span>
                        <span class="stat-label">للإنجاز</span>
                    </div>
                </div>
                ` : ''}
            </div>
        `;
    },
    
    /**
     * Get Dhikr counter widget content
     */
    async getDhikrCounterContent(widget) {
        const dhikrData = {
            text: 'أستغفر الله العظيم',
            count: 47,
            target: widget.settings.target || 100
        };
        
        return `
            <div class="dhikr-counter-widget">
                <div class="dhikr-text">${dhikrData.text}</div>
                <div class="counter-display">
                    <div class="count-number">${dhikrData.count}</div>
                    <div class="count-target">من ${dhikrData.target}</div>
                </div>
                <div class="counter-controls">
                    <button class="counter-btn" data-action="increment">+</button>
                    <button class="counter-btn" data-action="reset">إعادة تعيين</button>
                </div>
            </div>
        `;
    },
    
    /**
     * Get Hijri calendar widget content
     */
    async getHijriCalendarContent(widget) {
        const hijriDate = {
            day: 15,
            month: 'رجب',
            year: 1445,
            gregorian: '2024-01-27',
            events: ['ليلة الإسراء والمعراج']
        };
        
        return `
            <div class="hijri-calendar-widget">
                <div class="hijri-date">
                    <div class="hijri-day">${hijriDate.day}</div>
                    <div class="hijri-month">${hijriDate.month}</div>
                    <div class="hijri-year">${hijriDate.year} هـ</div>
                </div>
                
                ${widget.settings.showGregorian ? `
                <div class="gregorian-date">${hijriDate.gregorian}</div>
                ` : ''}
                
                ${widget.settings.showEvents && hijriDate.events.length > 0 ? `
                <div class="islamic-events">
                    ${hijriDate.events.map(event => `<div class="event">${event}</div>`).join('')}
                </div>
                ` : ''}
            </div>
        `;
    },
    
    /**
     * Get quick stats widget content
     */
    async getQuickStatsContent(widget) {
        const stats = this.state.userProfile.stats;
        
        return `
            <div class="quick-stats-widget">
                <div class="stats-grid">
                    ${widget.settings.showReadingTime ? `
                    <div class="stat-item">
                        <div class="stat-icon">📖</div>
                        <div class="stat-info">
                            <div class="stat-value">${Math.round(stats.totalReadingTime / 60)}</div>
                            <div class="stat-label">ساعة قراءة</div>
                        </div>
                    </div>
                    ` : ''}
                    
                    ${widget.settings.showStreak ? `
                    <div class="stat-item">
                        <div class="stat-icon">🔥</div>
                        <div class="stat-info">
                            <div class="stat-value">${stats.streakDays}</div>
                            <div class="stat-label">يوم متتالي</div>
                        </div>
                    </div>
                    ` : ''}
                    
                    <div class="stat-item">
                        <div class="stat-icon">🔖</div>
                        <div class="stat-info">
                            <div class="stat-value">${stats.bookmarkedAyahs + stats.bookmarkedHadiths}</div>
                            <div class="stat-label">مرجعية محفوظة</div>
                        </div>
                    </div>
                    
                    <div class="stat-item">
                        <div class="stat-icon">✅</div>
                        <div class="stat-info">
                            <div class="stat-value">${stats.completedKhatmas}</div>
                            <div class="stat-label">ختمة مكتملة</div>
                        </div>
                    </div>
                </div>
            </div>
        `;
    },
    
    /**
     * Setup widget event listeners
     */
    setupWidgetEventListeners(widgetElement, widget) {
        // Refresh widget
        const refreshBtn = widgetElement.querySelector('.refresh-widget');
        if (refreshBtn) {
            refreshBtn.addEventListener('click', () => {
                this.refreshWidget(widget.id);
            });
        }
        
        // Widget settings
        const settingsBtn = widgetElement.querySelector('.settings-widget');
        if (settingsBtn) {
            settingsBtn.addEventListener('click', () => {
                this.openWidgetSettings(widget.id);
            });
        }
        
        // Remove widget
        const removeBtn = widgetElement.querySelector('.remove-widget');
        if (removeBtn) {
            removeBtn.addEventListener('click', () => {
                this.removeWidget(widget.id);
            });
        }
        
        // Widget-specific event listeners
        this.setupWidgetSpecificListeners(widgetElement, widget);
    },
    
    /**
     * Setup widget-specific event listeners
     */
    setupWidgetSpecificListeners(widgetElement, widget) {
        switch (widget.type) {
            case 'dhikr-counter':
                this.setupDhikrCounterListeners(widgetElement, widget);
                break;
            // Add more widget-specific listeners as needed
        }
    },
    
    /**
     * Setup Dhikr counter listeners
     */
    setupDhikrCounterListeners(widgetElement, widget) {
        const incrementBtn = widgetElement.querySelector('[data-action="increment"]');
        const resetBtn = widgetElement.querySelector('[data-action="reset"]');
        const countDisplay = widgetElement.querySelector('.count-number');
        
        if (incrementBtn && countDisplay) {
            incrementBtn.addEventListener('click', () => {
                let currentCount = parseInt(countDisplay.textContent);
                currentCount++;
                countDisplay.textContent = currentCount;
                
                // Save count
                this.saveDhikrCount(widget.id, currentCount);
                
                // Check if target reached
                if (currentCount >= widget.settings.target) {
                    this.showDhikrCompletionMessage();
                }
            });
        }
        
        if (resetBtn && countDisplay) {
            resetBtn.addEventListener('click', () => {
                countDisplay.textContent = '0';
                this.saveDhikrCount(widget.id, 0);
            });
        }
    },
    
    /**
     * Apply grid layout
     */
    applyGridLayout() {
        const grid = this.elements.widgetsGrid;
        
        // Calculate grid dimensions
        const maxCol = Math.max(...this.state.widgets.map(w => w.position.col + w.position.width - 1));
        const maxRow = Math.max(...this.state.widgets.map(w => w.position.row + w.position.height - 1));
        
        grid.style.gridTemplateColumns = `repeat(${Math.max(maxCol, 4)}, 1fr)`;
        grid.style.gridTemplateRows = `repeat(${maxRow}, minmax(200px, auto))`;
        
        // Apply positions to widgets
        this.state.widgets.forEach(widget => {
            const element = document.getElementById(`widget-${widget.id}`);
            if (element) {
                element.style.gridColumn = `${widget.position.col} / span ${widget.position.width}`;
                element.style.gridRow = `${widget.position.row} / span ${widget.position.height}`;
            }
        });
    },
    
    /**
     * Setup drag and drop
     */
    setupDragAndDrop() {
        this.state.widgets.forEach(widget => {
            const element = document.getElementById(`widget-${widget.id}`);
            const dragHandle = element.querySelector('.drag-handle');
            
            if (dragHandle) {
                dragHandle.addEventListener('mousedown', (e) => {
                    this.startDrag(widget.id, e);
                });
            }
        });
    },
    
    /**
     * Start drag operation
     */
    startDrag(widgetId, event) {
        if (!this.state.isEditing) return;
        
        this.state.draggedWidget = widgetId;
        const element = document.getElementById(`widget-${widgetId}`);
        
        element.classList.add('dragging');
        
        const onMouseMove = (e) => {
            // Handle drag movement
            this.handleDragMove(e);
        };
        
        const onMouseUp = () => {
            this.endDrag();
            document.removeEventListener('mousemove', onMouseMove);
            document.removeEventListener('mouseup', onMouseUp);
        };
        
        document.addEventListener('mousemove', onMouseMove);
        document.addEventListener('mouseup', onMouseUp);
    },
    
    /**
     * Handle drag movement
     */
    handleDragMove(event) {
        // Implementation for drag movement
        // This would involve calculating grid positions and updating widget positions
    },
    
    /**
     * End drag operation
     */
    endDrag() {
        if (this.state.draggedWidget) {
            const element = document.getElementById(`widget-${this.state.draggedWidget}`);
            element.classList.remove('dragging');
            
            this.state.draggedWidget = null;
            this.saveDashboardLayout();
        }
    },
    
    /**
     * Toggle customization mode
     */
    toggleCustomizationMode() {
        this.state.isEditing = !this.state.isEditing;
        
        const dashboard = this.elements.dashboardContainer;
        if (this.state.isEditing) {
            dashboard.classList.add('editing-mode');
            this.showCustomizationPanel();
        } else {
            dashboard.classList.remove('editing-mode');
            this.hideCustomizationPanel();
        }
    },
    
    /**
     * Show customization panel
     */
    showCustomizationPanel() {
        if (!this.elements.customizationPanel) {
            this.createCustomizationPanel();
        }
        
        this.elements.customizationPanel.classList.add('active');
    },
    
    /**
     * Hide customization panel
     */
    hideCustomizationPanel() {
        if (this.elements.customizationPanel) {
            this.elements.customizationPanel.classList.remove('active');
        }
    },
    
    /**
     * Create customization panel
     */
    createCustomizationPanel() {
        const panel = window.SanadUtils.dom.create('div', {
            className: 'customization-panel'
        });
        
        panel.innerHTML = `
            <div class="panel-header">
                <h3>تخصيص اللوحة</h3>
                <button class="panel-close" id="closeCustomization">×</button>
            </div>
            
            <div class="panel-content">
                <div class="customization-section">
                    <h4>الودجات المتاحة</h4>
                    <div class="available-widgets">
                        <div class="widget-option" data-type="prayer-times">
                            <div class="widget-icon">🕐</div>
                            <div class="widget-name">مواقيت الصلاة</div>
                            <button class="add-widget-btn">إضافة</button>
                        </div>
                        <div class="widget-option" data-type="verse-of-day">
                            <div class="widget-icon">📖</div>
                            <div class="widget-name">آية اليوم</div>
                            <button class="add-widget-btn">إضافة</button>
                        </div>
                        <div class="widget-option" data-type="dhikr-counter">
                            <div class="widget-icon">📿</div>
                            <div class="widget-name">عداد الأذكار</div>
                            <button class="add-widget-btn">إضافة</button>
                        </div>
                        <div class="widget-option" data-type="hijri-calendar">
                            <div class="widget-icon">📅</div>
                            <div class="widget-name">التقويم الهجري</div>
                            <button class="add-widget-btn">إضافة</button>
                        </div>
                    </div>
                </div>
                
                <div class="customization-section">
                    <h4>تخطيط اللوحة</h4>
                    <div class="layout-options">
                        <button class="layout-btn" data-layout="grid">شبكة</button>
                        <button class="layout-btn" data-layout="masonry">متدرج</button>
                        <button class="layout-btn" data-layout="list">قائمة</button>
                    </div>
                </div>
                
                <div class="customization-section">
                    <h4>الإعدادات العامة</h4>
                    <div class="general-settings">
                        <label class="setting-item">
                            <input type="checkbox" id="showWidgetTitles" checked>
                            إظهار عناوين الودجات
                        </label>
                        <label class="setting-item">
                            <input type="checkbox" id="enableAnimations" checked>
                            تفعيل الحركات
                        </label>
                        <label class="setting-item">
                            <input type="checkbox" id="autoSaveLayout" checked>
                            حفظ التخطيط تلقائياً
                        </label>
                    </div>
                </div>
            </div>
            
            <div class="panel-footer">
                <button class="btn btn-secondary" id="resetToDefault">إعادة تعيين افتراضي</button>
                <button class="btn btn-primary" id="saveCustomization">حفظ التخصيص</button>
            </div>
        `;
        
        document.body.appendChild(panel);
        this.elements.customizationPanel = panel;
        
        // Setup panel event listeners
        this.setupCustomizationPanelListeners();
    },
    
    /**
     * Setup customization panel listeners
     */
    setupCustomizationPanelListeners() {
        const panel = this.elements.customizationPanel;
        
        // Close panel
        panel.querySelector('#closeCustomization').addEventListener('click', () => {
            this.toggleCustomizationMode();
        });
        
        // Add widget buttons
        panel.querySelectorAll('.add-widget-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const widgetType = e.target.closest('.widget-option').dataset.type;
                this.addWidget(widgetType);
            });
        });
        
        // Layout options
        panel.querySelectorAll('.layout-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                this.changeLayout(btn.dataset.layout);
            });
        });
        
        // Save customization
        panel.querySelector('#saveCustomization').addEventListener('click', () => {
            this.saveCustomization();
        });
        
        // Reset to default
        panel.querySelector('#resetToDefault').addEventListener('click', () => {
            this.resetToDefaultLayout();
        });
    },
    
    /**
     * Switch view
     */
    switchView(view) {
        const controls = this.elements.dashboardControls;
        controls.querySelectorAll('.view-btn').forEach(btn => {
            btn.classList.remove('active');
            if (btn.dataset.view === view) {
                btn.classList.add('active');
            }
        });
        
        this.elements.widgetsGrid.className = `widgets-grid view-${view}`;
    },
    
    /**
     * Filter widgets
     */
    filterWidgets(filter) {
        const widgets = this.elements.widgetsGrid.querySelectorAll('.dashboard-widget');
        
        widgets.forEach(widget => {
            const widgetType = widget.dataset.widgetType;
            let shouldShow = true;
            
            if (filter !== 'all') {
                shouldShow = this.getWidgetCategory(widgetType) === filter;
            }
            
            widget.style.display = shouldShow ? 'block' : 'none';
        });
    },
    
    /**
     * Get widget category
     */
    getWidgetCategory(widgetType) {
        const categories = {
            'prayer-times': 'prayer',
            'hijri-calendar': 'prayer',
            'dhikr-counter': 'reminders',
            'verse-of-day': 'reading',
            'khatma-progress': 'progress',
            'quick-stats': 'progress'
        };
        
        return categories[widgetType] || 'other';
    },
    
    /**
     * Add widget
     */
    addWidget(widgetType) {
        const newWidget = {
            id: `${widgetType}-${Date.now()}`,
            type: widgetType,
            title: this.getWidgetTitle(widgetType),
            position: this.findAvailablePosition(),
            settings: this.getDefaultWidgetSettings(widgetType)
        };
        
        this.state.widgets.push(newWidget);
        this.renderWidgets();
        this.saveDashboardLayout();
    },
    
    /**
     * Get widget title
     */
    getWidgetTitle(widgetType) {
        const titles = {
            'prayer-times': 'مواقيت الصلاة',
            'verse-of-day': 'آية اليوم',
            'khatma-progress': 'تقدم الختمة',
            'dhikr-counter': 'عداد الأذكار',
            'hijri-calendar': 'التقويم الهجري',
            'quick-stats': 'الإحصائيات السريعة'
        };
        
        return titles[widgetType] || widgetType;
    },
    
    /**
     * Find available position for new widget
     */
    findAvailablePosition() {
        // Simple algorithm to find next available position
        const occupiedPositions = new Set();
        
        this.state.widgets.forEach(widget => {
            for (let row = widget.position.row; row < widget.position.row + widget.position.height; row++) {
                for (let col = widget.position.col; col < widget.position.col + widget.position.width; col++) {
                    occupiedPositions.add(`${row}-${col}`);
                }
            }
        });
        
        // Find first available 1x1 position
        for (let row = 1; row <= 10; row++) {
            for (let col = 1; col <= 4; col++) {
                if (!occupiedPositions.has(`${row}-${col}`)) {
                    return { row, col, width: 1, height: 1 };
                }
            }
        }
        
        // If no position found, add to end
        const maxRow = Math.max(...this.state.widgets.map(w => w.position.row + w.position.height - 1));
        return { row: maxRow + 1, col: 1, width: 1, height: 1 };
    },
    
    /**
     * Get default widget settings
     */
    getDefaultWidgetSettings(widgetType) {
        const defaults = {
            'prayer-times': { showNextPrayer: true, showAllPrayers: true },
            'verse-of-day': { showTranslation: true, showTafsir: false },
            'khatma-progress': { showStats: true, showChart: true },
            'dhikr-counter': { defaultDhikr: 'استغفار', target: 100 },
            'hijri-calendar': { showEvents: true, showGregorian: true },
            'quick-stats': { showReadingTime: true, showStreak: true }
        };
        
        return defaults[widgetType] || {};
    },
    
    /**
     * Remove widget
     */
    removeWidget(widgetId) {
        if (confirm('هل أنت متأكد من إزالة هذه الودجة؟')) {
            this.state.widgets = this.state.widgets.filter(w => w.id !== widgetId);
            this.renderWidgets();
            this.saveDashboardLayout();
        }
    },
    
    /**
     * Refresh widget
     */
    async refreshWidget(widgetId) {
        const widget = this.state.widgets.find(w => w.id === widgetId);
        if (!widget) return;
        
        const widgetElement = document.getElementById(`widget-${widgetId}`);
        const contentElement = widgetElement.querySelector('.widget-content');
        
        // Show loading
        contentElement.innerHTML = '<div class="widget-loading">جاري التحديث...</div>';
        
        try {
            // Reload widget content
            const newContent = await this.getWidgetContent(widget);
            contentElement.innerHTML = newContent;
            
            // Re-setup widget-specific listeners
            this.setupWidgetSpecificListeners(widgetElement, widget);
            
        } catch (error) {
            console.error(`Failed to refresh widget ${widgetId}:`, error);
            contentElement.innerHTML = '<div class="widget-error">فشل في التحديث</div>';
        }
    },
    
    /**
     * Save dashboard layout
     */
    saveDashboardLayout() {
        const layout = {
            widgets: this.state.widgets,
            lastUpdated: new Date().toISOString()
        };
        
        window.SanadUtils.storage.set('dashboard_layout', layout);
    },
    
    /**
     * Save dhikr count
     */
    saveDhikrCount(widgetId, count) {
        const dhikrCounts = window.SanadUtils.storage.get('dhikr_counts') || {};
        dhikrCounts[widgetId] = count;
        window.SanadUtils.storage.set('dhikr_counts', dhikrCounts);
    },
    
    /**
     * Show dhikr completion message
     */
    showDhikrCompletionMessage() {
        if (window.SanadApp && window.SanadApp.showNotification) {
            window.SanadApp.showNotification('بارك الله فيك! لقد أكملت العدد المطلوب من الأذكار', 'success');
        }
    },
    
    /**
     * Open profile editor
     */
    openProfileEditor() {
        // Implementation for profile editor modal
        console.log('Opening profile editor...');
    },
    
    /**
     * Open widget selector
     */
    openWidgetSelector() {
        this.toggleCustomizationMode();
    },
    
    /**
     * Open widget settings
     */
    openWidgetSettings(widgetId) {
        // Implementation for widget settings modal
        console.log(`Opening settings for widget ${widgetId}...`);
    },
    
    /**
     * Reset dashboard layout
     */
    resetDashboardLayout() {
        if (confirm('هل أنت متأكد من إعادة تعيين تخطيط اللوحة؟')) {
            window.SanadUtils.storage.remove('dashboard_layout');
            this.loadDashboardWidgets();
        }
    },
    
    /**
     * Toggle auto refresh
     */
    toggleAutoRefresh(enabled) {
        // Implementation for auto refresh toggle
        console.log('Auto refresh:', enabled);
    },
    
    /**
     * Toggle compact mode
     */
    toggleCompactMode(enabled) {
        const dashboard = this.elements.dashboardContainer;
        if (enabled) {
            dashboard.classList.add('compact-mode');
        } else {
            dashboard.classList.remove('compact-mode');
        }
    },
    
    /**
     * Toggle animations
     */
    toggleAnimations(enabled) {
        const dashboard = this.elements.dashboardContainer;
        if (enabled) {
            dashboard.classList.remove('no-animations');
        } else {
            dashboard.classList.add('no-animations');
        }
    },
    
    /**
     * Export dashboard settings
     */
    exportDashboardSettings() {
        const settings = {
            layout: this.state.widgets,
            preferences: this.state.preferences,
            exportDate: new Date().toISOString()
        };
        
        const blob = new Blob([JSON.stringify(settings, null, 2)], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        
        const a = document.createElement('a');
        a.href = url;
        a.download = 'sanad-dashboard-settings.json';
        a.click();
        
        URL.revokeObjectURL(url);
    },
    
    /**
     * Import dashboard settings
     */
    importDashboardSettings() {
        const input = document.createElement('input');
        input.type = 'file';
        input.accept = '.json';
        
        input.onchange = (e) => {
            const file = e.target.files[0];
            if (file) {
                const reader = new FileReader();
                reader.onload = (e) => {
                    try {
                        const settings = JSON.parse(e.target.result);
                        this.applyImportedSettings(settings);
                    } catch (error) {
                        alert('فشل في قراءة ملف الإعدادات');
                    }
                };
                reader.readAsText(file);
            }
        };
        
        input.click();
    },
    
    /**
     * Apply imported settings
     */
    applyImportedSettings(settings) {
        if (settings.layout) {
            this.state.widgets = settings.layout;
            this.renderWidgets();
            this.saveDashboardLayout();
        }
        
        if (settings.preferences) {
            this.state.preferences = { ...this.state.preferences, ...settings.preferences };
        }
        
        if (window.SanadApp && window.SanadApp.showNotification) {
            window.SanadApp.showNotification('تم استيراد الإعدادات بنجاح', 'success');
        }
    },
    
    /**
     * Handle widget update
     */
    handleWidgetUpdate(detail) {
        const { widgetId, data } = detail;
        this.refreshWidget(widgetId);
    },
    
    /**
     * Handle profile update
     */
    handleProfileUpdate(detail) {
        this.state.userProfile = { ...this.state.userProfile, ...detail };
        // Update profile display
        this.updateProfileDisplay();
    },
    
    /**
     * Update profile display
     */
    updateProfileDisplay() {
        const profileSection = this.elements.userProfileSection;
        if (profileSection) {
            const nameElement = profileSection.querySelector('.profile-name');
            if (nameElement) {
                nameElement.textContent = this.state.userProfile.name;
            }
            
            // Update other profile elements as needed
        }
    }
};

// Initialize when DOM is ready
window.SanadUtils.timing.ready(() => {
    window.SanadUserDashboard.init();
});

// Freeze the object to prevent modifications
Object.freeze(window.SanadUserDashboard);