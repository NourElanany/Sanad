/**
 * Sections Manager for Sanad Islamic App
 * Handles content loading and management for each section
 */

window.SanadSections = {

    /**
     * State management
     */
    state: {
        loadedSections: new Set(),
        sectionData: {}
    },

    /**
     * Initialize sections
     */
    init() {
        console.log('Initializing Sanad Sections...');
        this.setupSectionListeners();
    },

    /**
     * Setup section change listeners
     */
    setupSectionListeners() {
        document.addEventListener('sectionChanged', (event) => {
            const { section } = event.detail;
            this.loadSection(section);
        });
    },

    /**
     * Load section content
     */
    async loadSection(sectionId) {
        if (this.state.loadedSections.has(sectionId)) {
            console.log(`Section ${sectionId} already loaded`);
            return;
        }

        console.log(`Loading section: ${sectionId}`);

        switch (sectionId) {
            case 'quran':
                await this.loadQuranSection();
                break;
            case 'hadith':
                await this.loadHadithSection();
                break;
            case 'stories':
                await this.loadStoriesSection();
                break;
            case 'prayer-times':
                await this.loadPrayerTimesSection();
                break;
            case 'ai-assistant':
                await this.loadAIAssistantSection();
                break;
            default:
                console.log(`Unknown section: ${sectionId}`);
        }

        this.state.loadedSections.add(sectionId);
    },

    /**
     * Load Quran Section
     */
    async loadQuranSection() {
        const surahList = document.getElementById('surahList');
        if (!surahList) return;

        // Sample surah data
        const surahs = [
            { number: 1, name: 'الفاتحة', englishName: 'Al-Fatiha', ayahs: 7, type: 'مكية' },
            { number: 2, name: 'البقرة', englishName: 'Al-Baqarah', ayahs: 286, type: 'مدنية' },
            { number: 3, name: 'آل عمران', englishName: 'Aal-Imran', ayahs: 200, type: 'مدنية' },
            { number: 4, name: 'النساء', englishName: 'An-Nisa', ayahs: 176, type: 'مدنية' },
            { number: 5, name: 'المائدة', englishName: 'Al-Maidah', ayahs: 120, type: 'مدنية' },
            { number: 6, name: 'الأنعام', englishName: 'Al-Anam', ayahs: 165, type: 'مكية' },
            { number: 7, name: 'الأعراف', englishName: 'Al-Araf', ayahs: 206, type: 'مكية' },
            { number: 8, name: 'الأنفال', englishName: 'Al-Anfal', ayahs: 75, type: 'مدنية' },
            { number: 9, name: 'التوبة', englishName: 'At-Tawbah', ayahs: 129, type: 'مدنية' },
            { number: 10, name: 'يونس', englishName: 'Yunus', ayahs: 109, type: 'مكية' }
        ];

        surahList.innerHTML = '';

        surahs.forEach(surah => {
            const surahCard = document.createElement('div');
            surahCard.className = 'surah-card';
            surahCard.innerHTML = `
                <div class="surah-number">${surah.number}</div>
                <div class="surah-info">
                    <h3 class="surah-name">${surah.name}</h3>
                    <p class="surah-english">${surah.englishName}</p>
                </div>
                <div class="surah-meta">
                    <span class="surah-ayahs">${surah.ayahs} آية</span>
                    <span class="surah-type">${surah.type}</span>
                </div>
            `;
            surahCard.addEventListener('click', () => {
                this.openSurah(surah.number);
            });
            surahList.appendChild(surahCard);
        });
    },

    /**
     * Load Hadith Section
     */
    async loadHadithSection() {
        const hadithCollections = document.getElementById('hadithCollections');
        if (!hadithCollections) return;

        const collections = [
            { name: 'صحيح البخاري', englishName: 'Sahih al-Bukhari', count: 7563 },
            { name: 'صحيح مسلم', englishName: 'Sahih Muslim', count: 7500 },
            { name: 'سنن أبي داود', englishName: 'Sunan Abu Dawud', count: 5274 },
            { name: 'سنن الترمذي', englishName: 'Jami at-Tirmidhi', count: 3956 },
            { name: 'سنن النسائي', englishName: 'Sunan an-Nasai', count: 5758 },
            { name: 'سنن ابن ماجه', englishName: 'Sunan Ibn Majah', count: 4341 }
        ];

        hadithCollections.innerHTML = '';

        collections.forEach(collection => {
            const collectionCard = document.createElement('div');
            collectionCard.className = 'collection-card';
            collectionCard.innerHTML = `
                <div class="collection-info">
                    <h3 class="collection-name">${collection.name}</h3>
                    <p class="collection-english">${collection.englishName}</p>
                </div>
                <div class="collection-meta">
                    <span class="hadith-count">${collection.count.toLocaleString('ar-SA')} حديث</span>
                </div>
            `;
            collectionCard.addEventListener('click', () => {
                this.openCollection(collection.name);
            });
            hadithCollections.appendChild(collectionCard);
        });

        // Load a sample hadith
        this.loadSampleHadith();
    },

    /**
     * Load Stories Section
     */
    async loadStoriesSection() {
        const storyCategories = document.getElementById('storyCategories');
        if (!storyCategories) return;

        const categories = [
            { name: 'قصص الأنبياء', englishName: 'Stories of the Prophets', icon: '🌟', count: 25 },
            { name: 'قصص الصحابة', englishName: 'Stories of the Companions', icon: '⚔️', count: 50 },
            { name: 'قصص القرآن', englishName: 'Stories from Quran', icon: '📖', count: 30 },
            { name: 'قصص السلف', englishName: 'Stories of the Righteous', icon: '🏛️', count: 40 }
        ];

        storyCategories.innerHTML = '';

        categories.forEach(category => {
            const categoryCard = document.createElement('div');
            categoryCard.className = 'category-card';
            categoryCard.innerHTML = `
                <span class="category-icon">${category.icon}</span>
                <div class="category-info">
                    <h3 class="category-name">${category.name}</h3>
                    <p class="category-english">${category.englishName}</p>
                </div>
                <span class="story-count">${category.count} قصة</span>
            `;
            categoryCard.addEventListener('click', () => {
                this.openCategory(category.name);
            });
            storyCategories.appendChild(categoryCard);
        });
    },

    /**
     * Load Prayer Times Section
     */
    async loadPrayerTimesSection() {
        const prayerTimesGrid = document.getElementById('prayerTimesGrid');
        const currentPrayerInfo = document.getElementById('currentPrayerInfo');

        if (!prayerTimesGrid || !currentPrayerInfo) return;

        const prayers = [
            { name: 'الفجر', englishName: 'Fajr', time: '05:15', icon: '🌅' },
            { name: 'الشروق', englishName: 'Sunrise', time: '06:45', icon: '☀️' },
            { name: 'الظهر', englishName: 'Dhuhr', time: '12:30', icon: '🌞' },
            { name: 'العصر', englishName: 'Asr', time: '15:45', icon: '🌤️' },
            { name: 'المغرب', englishName: 'Maghrib', time: '18:20', icon: '🌅' },
            { name: 'العشاء', englishName: 'Isha', time: '19:50', icon: '🌙' }
        ];

        // Current prayer info
        const now = new Date();
        currentPrayerInfo.innerHTML = `
            <div class="current-prayer-card">
                <div class="current-prayer-label">الصلاة القادمة</div>
                <div class="current-prayer-name">المغرب</div>
                <div class="current-prayer-time">18:20</div>
                <div class="time-remaining">باقي ساعتين و 15 دقيقة</div>
            </div>
        `;

        // Prayer times grid
        prayerTimesGrid.innerHTML = '';
        prayers.forEach(prayer => {
            const prayerCard = document.createElement('div');
            prayerCard.className = 'prayer-card';
            prayerCard.innerHTML = `
                <span class="prayer-icon">${prayer.icon}</span>
                <h4 class="prayer-name">${prayer.name}</h4>
                <p class="prayer-english">${prayer.englishName}</p>
                <span class="prayer-time">${prayer.time}</span>
            `;
            prayerTimesGrid.appendChild(prayerCard);
        });
    },

    /**
     * Load AI Assistant Section
     */
    async loadAIAssistantSection() {
        const chatInput = document.getElementById('chatInput');
        const sendBtn = document.getElementById('sendBtn');
        const chatMessages = document.getElementById('chatMessages');

        if (!chatInput || !sendBtn || !chatMessages) return;

        // Enable send button when there's input
        chatInput.addEventListener('input', () => {
            sendBtn.disabled = chatInput.value.trim().length === 0;
        });

        // Handle send button click
        sendBtn.addEventListener('click', () => {
            this.sendMessage();
        });

        // Handle Enter key
        chatInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter' && !e.shiftKey) {
                e.preventDefault();
                if (!sendBtn.disabled) {
                    this.sendMessage();
                }
            }
        });

        // Add suggestions
        this.loadAISuggestions();
    },

    /**
     * Send message to AI assistant
     */
    sendMessage() {
        const chatInput = document.getElementById('chatInput');
        const chatMessages = document.getElementById('chatMessages');
        const sendBtn = document.getElementById('sendBtn');

        if (!chatInput || !chatMessages) return;

        const message = chatInput.value.trim();
        if (!message) return;

        // Add user message
        const userMessage = document.createElement('div');
        userMessage.className = 'message user-message';
        userMessage.innerHTML = `
            <div class="message-content">${message}</div>
        `;
        chatMessages.appendChild(userMessage);

        // Clear input
        chatInput.value = '';
        sendBtn.disabled = true;

        // Scroll to bottom
        chatMessages.scrollTop = chatMessages.scrollHeight;

        // Simulate AI response
        setTimeout(() => {
            const aiMessage = document.createElement('div');
            aiMessage.className = 'message ai-message';
            aiMessage.innerHTML = `
                <div class="message-content">
                    <p>جزاكم الله خيراً على سؤالكم. هذه ميزة تجريبية وسيتم ربطها بالخادم قريباً إن شاء الله.</p>
                    <p>يمكنك استكشاف باقي أقسام التطبيق في الوقت الحالي.</p>
                </div>
            `;
            chatMessages.appendChild(aiMessage);
            chatMessages.scrollTop = chatMessages.scrollHeight;
        }, 1000);
    },

    /**
     * Load AI suggestions
     */
    loadAISuggestions() {
        const suggestions = document.getElementById('inputSuggestions');
        if (!suggestions) return;

        const suggestedQuestions = [
            'ما هو حكم الصلاة؟',
            'كيف أتوضأ؟',
            'ما هي أركان الإسلام؟',
            'ما هي أركان الإيمان؟'
        ];

        suggestions.innerHTML = '';
        suggestedQuestions.forEach(question => {
            const chip = document.createElement('button');
            chip.className = 'suggestion-chip';
            chip.textContent = question;
            chip.addEventListener('click', () => {
                document.getElementById('chatInput').value = question;
                document.getElementById('sendBtn').disabled = false;
            });
            suggestions.appendChild(chip);
        });
    },

    /**
     * Open surah reader
     */
    openSurah(surahNumber) {
        console.log(`Opening surah ${surahNumber}`);
        if (window.SanadApp) {
            window.SanadApp.showNotification(`جاري فتح سورة رقم ${surahNumber}...`, 'info');
        }
    },

    /**
     * Open hadith collection
     */
    openCollection(collectionName) {
        console.log(`Opening collection: ${collectionName}`);
        if (window.SanadApp) {
            window.SanadApp.showNotification(`جاري فتح ${collectionName}...`, 'info');
        }
    },

    /**
     * Open story category
     */
    openCategory(categoryName) {
        console.log(`Opening category: ${categoryName}`);
        if (window.SanadApp) {
            window.SanadApp.showNotification(`جاري فتح ${categoryName}...`, 'info');
        }
    },

    /**
     * Load sample hadith
     */
    loadSampleHadith() {
        const hadithDisplay = document.getElementById('hadithDisplay');
        if (!hadithDisplay) return;

        hadithDisplay.innerHTML = `
            <div class="hadith-card featured">
                <div class="hadith-header">
                    <span class="hadith-source">صحيح البخاري</span>
                    <span class="hadith-grade sahih">صحيح</span>
                </div>
                <div class="hadith-text arabic">
                    عَنْ أَبِي هُرَيْرَةَ رَضِيَ اللَّهُ عَنْهُ قَالَ: قَالَ رَسُولُ اللَّهِ صَلَّى اللَّهُ عَلَيْهِ وَسَلَّمَ:
                    "مَنْ كَانَ يُؤْمِنُ بِاللَّهِ وَالْيَوْمِ الْآخِرِ فَلْيَقُلْ خَيْرًا أَوْ لِيَصْمُتْ"
                </div>
                <div class="hadith-translation">
                    Whoever believes in Allah and the Last Day should speak good or remain silent.
                </div>
                <div class="hadith-narrator">
                    الراوي: أبو هريرة رضي الله عنه
                </div>
            </div>
        `;
    }
};

// Initialize sections when DOM is ready
if (window.SanadUtils && window.SanadUtils.timing) {
    window.SanadUtils.timing.ready(() => {
        window.SanadSections.init();
    });
}

// Freeze the sections object
Object.freeze(window.SanadSections);
