/**
 * Widgets for Sanad Islamic App
 * Dashboard widgets for prayer times, verse of day, khatma progress, etc.
 */

window.SanadWidgets = {

    /**
     * State management
     */
    state: {
        widgets: [],
        refreshIntervals: {}
    },

    /**
     * Initialize widgets
     */
    init() {
        console.log('Initializing Sanad Widgets...');
        this.loadDefaultWidgets();
    },

    /**
     * Load default dashboard widgets
     */
    loadDefaultWidgets() {
        const widgetsGrid = document.getElementById('widgetsGrid');
        if (!widgetsGrid) return;

        // Clear existing widgets
        widgetsGrid.innerHTML = '';

        // Create default widgets
        const widgets = [
            this.createPrayerTimesWidget(),
            this.createVerseOfDayWidget(),
            this.createKhatmaProgressWidget(),
            this.createDhikrCounterWidget(),
            this.createHijriCalendarWidget(),
            this.createQuickLinksWidget()
        ];

        widgets.forEach(widget => {
            widgetsGrid.appendChild(widget);
        });

        this.state.widgets = widgets;
    },

    /**
     * Create Prayer Times Widget
     */
    createPrayerTimesWidget() {
        const widget = this.createWidgetContainer('prayer-times-widget', 'مواقيت الصلاة', '🕌');

        const content = document.createElement('div');
        content.className = 'prayer-times-list';

        const prayers = [
            { name: 'الفجر', nameEn: 'Fajr', time: '05:15' },
            { name: 'الشروق', nameEn: 'Sunrise', time: '06:45' },
            { name: 'الظهر', nameEn: 'Dhuhr', time: '12:30' },
            { name: 'العصر', nameEn: 'Asr', time: '15:45' },
            { name: 'المغرب', nameEn: 'Maghrib', time: '18:20' },
            { name: 'العشاء', nameEn: 'Isha', time: '19:50' }
        ];

        const now = new Date();
        const currentHour = now.getHours();
        let nextPrayer = prayers[0];

        prayers.forEach((prayer, index) => {
            const [hours] = prayer.time.split(':').map(Number);
            if (hours > currentHour) {
                nextPrayer = prayer;
            }

            const prayerRow = document.createElement('div');
            prayerRow.className = `prayer-row ${prayer === nextPrayer ? 'next-prayer' : ''}`;
            prayerRow.innerHTML = `
                <span class="prayer-name">${prayer.name}</span>
                <span class="prayer-time">${prayer.time}</span>
            `;
            content.appendChild(prayerRow);
        });

        widget.querySelector('.widget-content').appendChild(content);
        return widget;
    },

    /**
     * Create Verse of the Day Widget
     */
    createVerseOfDayWidget() {
        const widget = this.createWidgetContainer('verse-of-day-widget', 'آية اليوم', '📖');

        const content = document.createElement('div');
        content.className = 'verse-of-day-content';
        content.innerHTML = `
            <div class="verse-arabic">
                بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ ۝ الْحَمْدُ لِلَّهِ رَبِّ الْعَالَمِينَ
            </div>
            <div class="verse-translation">
                In the name of Allah, the Most Gracious, the Most Merciful. Praise be to Allah, Lord of all the worlds.
            </div>
            <div class="verse-reference">
                سورة الفاتحة - الآية ١-٢
            </div>
        `;

        widget.querySelector('.widget-content').appendChild(content);
        return widget;
    },

    /**
     * Create Khatma Progress Widget
     */
    createKhatmaProgressWidget() {
        const widget = this.createWidgetContainer('khatma-progress-widget', 'تقدم الختمة', '📚');

        const progress = 35; // Example progress
        const content = document.createElement('div');
        content.className = 'khatma-progress-content';
        content.innerHTML = `
            <div class="progress-circle">
                <svg viewBox="0 0 100 100">
                    <circle class="progress-bg" cx="50" cy="50" r="45"></circle>
                    <circle class="progress-fill" cx="50" cy="50" r="45" 
                        style="stroke-dasharray: ${progress * 2.83}, 283"></circle>
                    <text x="50" y="55" text-anchor="middle" class="progress-text">${progress}%</text>
                </svg>
            </div>
            <div class="khatma-stats">
                <div class="stat">
                    <span class="stat-value">12</span>
                    <span class="stat-label">جزء</span>
                </div>
                <div class="stat">
                    <span class="stat-value">204</span>
                    <span class="stat-label">صفحة</span>
                </div>
            </div>
        `;

        widget.querySelector('.widget-content').appendChild(content);
        return widget;
    },

    /**
     * Create Dhikr Counter Widget
     */
    createDhikrCounterWidget() {
        const widget = this.createWidgetContainer('dhikr-counter-widget', 'عداد الأذكار', '📿');

        const count = window.SanadUtils.storage.get('dhikr_count') || 0;
        const content = document.createElement('div');
        content.className = 'dhikr-counter-content';
        content.innerHTML = `
            <div class="dhikr-display">
                <span class="dhikr-count">${count}</span>
            </div>
            <div class="dhikr-controls">
                <button class="dhikr-btn dhikr-increment">+</button>
                <button class="dhikr-btn dhikr-reset">إعادة</button>
            </div>
            <div class="dhikr-label">سبحان الله</div>
        `;

        // Add event listeners
        setTimeout(() => {
            const incrementBtn = content.querySelector('.dhikr-increment');
            const resetBtn = content.querySelector('.dhikr-reset');
            const countDisplay = content.querySelector('.dhikr-count');

            if (incrementBtn) {
                incrementBtn.addEventListener('click', () => {
                    let currentCount = parseInt(countDisplay.textContent) || 0;
                    currentCount++;
                    countDisplay.textContent = currentCount;
                    window.SanadUtils.storage.set('dhikr_count', currentCount);
                });
            }

            if (resetBtn) {
                resetBtn.addEventListener('click', () => {
                    countDisplay.textContent = '0';
                    window.SanadUtils.storage.set('dhikr_count', 0);
                });
            }
        }, 100);

        widget.querySelector('.widget-content').appendChild(content);
        return widget;
    },

    /**
     * Create Hijri Calendar Widget
     */
    createHijriCalendarWidget() {
        const widget = this.createWidgetContainer('hijri-calendar-widget', 'التقويم الهجري', '📅');

        const content = document.createElement('div');
        content.className = 'hijri-calendar-content';

        // Simple Hijri date calculation (approximate)
        const today = new Date();
        const hijriDate = this.getApproximateHijriDate(today);

        content.innerHTML = `
            <div class="hijri-date">
                <div class="hijri-day">${hijriDate.day}</div>
                <div class="hijri-month">${hijriDate.monthName}</div>
                <div class="hijri-year">${hijriDate.year} هـ</div>
            </div>
            <div class="gregorian-date">
                ${today.toLocaleDateString('ar-SA', { weekday: 'long', year: 'numeric', month: 'long', day: 'numeric' })}
            </div>
        `;

        widget.querySelector('.widget-content').appendChild(content);
        return widget;
    },

    /**
     * Create Quick Links Widget
     */
    createQuickLinksWidget() {
        const widget = this.createWidgetContainer('quick-links-widget', 'روابط سريعة', '⚡');

        const content = document.createElement('div');
        content.className = 'quick-links-content';

        const links = [
            { icon: '📖', label: 'القرآن الكريم', section: 'quran' },
            { icon: '📚', label: 'الأحاديث', section: 'hadith' },
            { icon: '📝', label: 'القصص', section: 'stories' },
            { icon: '🤖', label: 'المساعد الذكي', section: 'ai-assistant' }
        ];

        links.forEach(link => {
            const linkEl = document.createElement('button');
            linkEl.className = 'quick-link-btn';
            linkEl.innerHTML = `
                <span class="quick-link-icon">${link.icon}</span>
                <span class="quick-link-label">${link.label}</span>
            `;
            linkEl.addEventListener('click', () => {
                if (window.SanadApp) {
                    window.SanadApp.navigateToSection(link.section);
                }
            });
            content.appendChild(linkEl);
        });

        widget.querySelector('.widget-content').appendChild(content);
        return widget;
    },

    /**
     * Create widget container
     */
    createWidgetContainer(id, title, icon) {
        const widget = document.createElement('div');
        widget.className = 'widget';
        widget.id = id;
        widget.innerHTML = `
            <div class="widget-header">
                <span class="widget-icon">${icon}</span>
                <h3 class="widget-title">${title}</h3>
            </div>
            <div class="widget-content"></div>
        `;
        return widget;
    },

    /**
     * Get approximate Hijri date (simplified calculation)
     */
    getApproximateHijriDate(gregorianDate) {
        const hijriMonths = [
            'محرم', 'صفر', 'ربيع الأول', 'ربيع الثاني',
            'جمادى الأولى', 'جمادى الآخرة', 'رجب', 'شعبان',
            'رمضان', 'شوال', 'ذو القعدة', 'ذو الحجة'
        ];

        // Simplified calculation (not accurate, just for demo)
        const jd = Math.floor((gregorianDate.getTime() / 86400000) + 2440587.5);
        const l = jd - 1948440 + 10632;
        const n = Math.floor((l - 1) / 10631);
        const l2 = l - 10631 * n + 354;
        const j = Math.floor((10985 - l2) / 5316) * Math.floor((50 * l2) / 17719) + Math.floor(l2 / 5670) * Math.floor((43 * l2) / 15238);
        const l3 = l2 - Math.floor((30 - j) / 15) * Math.floor((17719 * j) / 50) - Math.floor(j / 16) * Math.floor((15238 * j) / 43) + 29;
        const month = Math.floor((24 * l3) / 709);
        const day = l3 - Math.floor((709 * month) / 24);
        const year = 30 * n + j - 30;

        return {
            day: day,
            month: month,
            monthName: hijriMonths[month - 1] || 'محرم',
            year: year
        };
    },

    /**
     * Refresh all widgets
     */
    refreshAll() {
        this.loadDefaultWidgets();
    }
};

// Initialize widgets when DOM is ready
if (window.SanadUtils && window.SanadUtils.timing) {
    window.SanadUtils.timing.ready(() => {
        window.SanadWidgets.init();
    });
}

// Freeze the widgets object
Object.freeze(window.SanadWidgets);
