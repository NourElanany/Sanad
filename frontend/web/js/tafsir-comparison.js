/**
 * Tafsir Comparison Interface for Sanad Islamic App
 * Provides side-by-side comparison of different Tafsir interpretations
 */

window.SanadTafsirComparison = {
    
    /**
     * Comparison state
     */
    state: {
        currentSurah: null,
        currentAyah: null,
        selectedTafasir: [],
        availableTafasir: [],
        comparisonData: {},
        isLoading: false,
        viewMode: 'side-by-side' // 'side-by-side', 'tabbed', 'unified'
    },
    
    /**
     * DOM elements cache
     */
    elements: {},
    
    /**
     * Initialize Tafsir comparison
     */
    init() {
        this.cacheElements();
        this.setupEventListeners();
        this.loadAvailableTafasir();
        this.initializeInterface();
    },
    
    /**
     * Cache DOM elements
     */
    cacheElements() {
        this.elements = {
            comparisonModal: null, // Will be created dynamically
            tafsirSelector: null,
            comparisonContainer: null,
            ayahSelector: null,
            viewModeToggle: null
        };
    },
    
    /**
     * Setup event listeners
     */
    setupEventListeners() {
        // Listen for comparison requests
        document.addEventListener('openTafsirComparison', (e) => {
            const { surahNumber, ayahNumber } = e.detail;
            this.openComparison(surahNumber, ayahNumber);
        });
        
        // Listen for ayah selection changes
        document.addEventListener('ayahSelected', (e) => {
            const { surahNumber, ayahNumber } = e.detail;
            this.updateComparison(surahNumber, ayahNumber);
        });
    },
    
    /**
     * Load available Tafsir sources
     */
    async loadAvailableTafasir() {
        try {
            // Mock data - in real implementation, fetch from API
            this.state.availableTafasir = [
                {
                    id: 'ibn-kathir',
                    name: 'تفسير ابن كثير',
                    author: 'ابن كثير',
                    description: 'تفسير القرآن العظيم',
                    language: 'ar',
                    methodology: 'تفسير بالمأثور',
                    color: '#27ae60'
                },
                {
                    id: 'tabari',
                    name: 'تفسير الطبري',
                    author: 'الطبري',
                    description: 'جامع البيان عن تأويل آي القرآن',
                    language: 'ar',
                    methodology: 'تفسير بالمأثور',
                    color: '#3498db'
                },
                {
                    id: 'qurtubi',
                    name: 'تفسير القرطبي',
                    author: 'القرطبي',
                    description: 'الجامع لأحكام القرآن',
                    language: 'ar',
                    methodology: 'تفسير فقهي',
                    color: '#9b59b6'
                },
                {
                    id: 'baghawi',
                    name: 'تفسير البغوي',
                    author: 'البغوي',
                    description: 'معالم التنزيل',
                    language: 'ar',
                    methodology: 'تفسير بالمأثور',
                    color: '#e67e22'
                },
                {
                    id: 'jalalayn',
                    name: 'تفسير الجلالين',
                    author: 'الجلالان',
                    description: 'تفسير الجلالين',
                    language: 'ar',
                    methodology: 'تفسير مختصر',
                    color: '#e74c3c'
                },
                {
                    id: 'saadi',
                    name: 'تفسير السعدي',
                    author: 'السعدي',
                    description: 'تيسير الكريم الرحمن',
                    language: 'ar',
                    methodology: 'تفسير معاصر',
                    color: '#f39c12'
                }
            ];
            
            // Set default selected Tafasir
            this.state.selectedTafasir = ['ibn-kathir', 'tabari', 'qurtubi'];
            
        } catch (error) {
            console.error('Failed to load available Tafasir:', error);
        }
    },
    
    /**
     * Initialize interface
     */
    initializeInterface() {
        // Add comparison button to Quran section if it exists
        this.addComparisonButtonToQuran();
    },
    
    /**
     * Add comparison button to Quran section
     */
    addComparisonButtonToQuran() {
        const quranSection = document.getElementById('quran');
        if (quranSection) {
            const sectionActions = quranSection.querySelector('.section-actions');
            if (sectionActions && !sectionActions.querySelector('#tafsirComparison')) {
                const comparisonBtn = window.SanadUtils.dom.create('button', {
                    className: 'btn btn-secondary',
                    id: 'tafsirComparison'
                });
                
                comparisonBtn.innerHTML = `
                    <span class="btn-icon">⚖️</span>
                    مقارنة التفاسير
                `;
                
                comparisonBtn.addEventListener('click', () => {
                    this.openComparison(1, 1); // Default to Al-Fatiha, Ayah 1
                });
                
                sectionActions.appendChild(comparisonBtn);
            }
        }
    },
    
    /**
     * Open comparison modal
     */
    openComparison(surahNumber = 1, ayahNumber = 1) {
        if (!this.elements.comparisonModal) {
            this.createComparisonModal();
        }
        
        this.state.currentSurah = surahNumber;
        this.state.currentAyah = ayahNumber;
        
        this.elements.comparisonModal.classList.add('active');
        this.loadComparisonData();
    },
    
    /**
     * Create comparison modal
     */
    createComparisonModal() {
        const modalOverlay = document.getElementById('modalOverlay');
        
        const comparisonModal = window.SanadUtils.dom.create('div', {
            className: 'modal tafsir-comparison-modal',
            id: 'tafsirComparisonModal'
        });
        
        comparisonModal.innerHTML = `
            <div class="modal-content">
                <div class="modal-header">
                    <div class="header-left">
                        <h2 class="modal-title">مقارنة التفاسير</h2>
                        <div class="ayah-selector">
                            <select id="surahSelector" class="selector-input">
                                <option value="1">الفاتحة</option>
                                <option value="2">البقرة</option>
                                <option value="3">آل عمران</option>
                                <!-- More surahs would be loaded dynamically -->
                            </select>
                            <span class="selector-separator">:</span>
                            <input type="number" id="ayahSelector" class="selector-input" min="1" value="1" placeholder="رقم الآية">
                            <button class="btn btn-primary" id="loadComparison">تحميل</button>
                        </div>
                    </div>
                    
                    <div class="header-right">
                        <div class="view-mode-toggle">
                            <button class="view-mode-btn active" data-mode="side-by-side">
                                <span class="mode-icon">⚏</span>
                                جنباً إلى جنب
                            </button>
                            <button class="view-mode-btn" data-mode="tabbed">
                                <span class="mode-icon">📑</span>
                                تبويبات
                            </button>
                            <button class="view-mode-btn" data-mode="unified">
                                <span class="mode-icon">📄</span>
                                موحد
                            </button>
                        </div>
                        <button class="modal-close" id="closeComparison">×</button>
                    </div>
                </div>
                
                <div class="modal-body">
                    <!-- Ayah Display -->
                    <div class="ayah-display-section">
                        <div class="ayah-card">
                            <div class="ayah-header">
                                <div class="ayah-reference" id="ayahReference">سورة الفاتحة - آية 1</div>
                                <div class="ayah-actions">
                                    <button class="action-btn" id="playAyah" title="تشغيل">
                                        <span class="action-icon">▶️</span>
                                    </button>
                                    <button class="action-btn" id="bookmarkAyah" title="حفظ">
                                        <span class="action-icon">🔖</span>
                                    </button>
                                    <button class="action-btn" id="shareAyah" title="مشاركة">
                                        <span class="action-icon">📤</span>
                                    </button>
                                </div>
                            </div>
                            <div class="ayah-content">
                                <div class="ayah-arabic" id="ayahArabic">بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ</div>
                                <div class="ayah-translation" id="ayahTranslation">بسم الله الرحمن الرحيم</div>
                            </div>
                        </div>
                    </div>
                    
                    <!-- Tafsir Selector -->
                    <div class="tafsir-selector-section">
                        <h3 class="section-title">اختيار التفاسير للمقارنة</h3>
                        <div class="tafsir-options" id="tafsirOptions">
                            <!-- Tafsir options will be populated here -->
                        </div>
                        <div class="selector-actions">
                            <button class="btn btn-outline" id="selectAllTafasir">تحديد الكل</button>
                            <button class="btn btn-outline" id="clearAllTafasir">إلغاء الكل</button>
                            <button class="btn btn-secondary" id="resetToDefault">الافتراضي</button>
                        </div>
                    </div>
                    
                    <!-- Comparison Container -->
                    <div class="comparison-container" id="comparisonContainer">
                        <div class="comparison-loading" id="comparisonLoading" style="display: none;">
                            <div class="loading-spinner"></div>
                            <p>جاري تحميل التفاسير...</p>
                        </div>
                        
                        <div class="comparison-content" id="comparisonContent">
                            <!-- Comparison content will be loaded here -->
                        </div>
                        
                        <div class="comparison-empty" id="comparisonEmpty" style="display: none;">
                            <div class="empty-icon">📚</div>
                            <h3>لا توجد تفاسير محددة</h3>
                            <p>يرجى اختيار تفسير واحد على الأقل للمقارنة</p>
                        </div>
                    </div>
                    
                    <!-- Comparison Tools -->
                    <div class="comparison-tools">
                        <div class="tools-left">
                            <button class="tool-btn" id="highlightDifferences">
                                <span class="tool-icon">🔍</span>
                                إبراز الاختلافات
                            </button>
                            <button class="tool-btn" id="showSimilarities">
                                <span class="tool-icon">🔗</span>
                                إظهار التشابهات
                            </button>
                            <button class="tool-btn" id="analyzeThemes">
                                <span class="tool-icon">🎯</span>
                                تحليل المواضيع
                            </button>
                        </div>
                        
                        <div class="tools-right">
                            <button class="tool-btn" id="exportComparison">
                                <span class="tool-icon">📄</span>
                                تصدير المقارنة
                            </button>
                            <button class="tool-btn" id="printComparison">
                                <span class="tool-icon">🖨️</span>
                                طباعة
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        `;
        
        modalOverlay.appendChild(comparisonModal);
        this.elements.comparisonModal = comparisonModal;
        
        // Cache new elements
        this.elements.tafsirSelector = comparisonModal.querySelector('#tafsirOptions');
        this.elements.comparisonContainer = comparisonModal.querySelector('#comparisonContent');
        this.elements.ayahSelector = comparisonModal.querySelector('#ayahSelector');
        this.elements.viewModeToggle = comparisonModal.querySelectorAll('.view-mode-btn');
        
        // Setup modal event listeners
        this.setupModalEventListeners();
        
        // Populate Tafsir options
        this.populateTafsirOptions();
        
        // Populate Surah options
        this.populateSurahOptions();
    },
    
    /**
     * Setup modal event listeners
     */
    setupModalEventListeners() {
        const modal = this.elements.comparisonModal;
        
        // Close modal
        modal.querySelector('#closeComparison').addEventListener('click', () => {
            this.closeComparison();
        });
        
        // Load comparison
        modal.querySelector('#loadComparison').addEventListener('click', () => {
            const surahNumber = parseInt(modal.querySelector('#surahSelector').value);
            const ayahNumber = parseInt(modal.querySelector('#ayahSelector').value);
            this.updateComparison(surahNumber, ayahNumber);
        });
        
        // View mode toggle
        this.elements.viewModeToggle.forEach(btn => {
            btn.addEventListener('click', () => {
                this.switchViewMode(btn.dataset.mode);
            });
        });
        
        // Tafsir selector actions
        modal.querySelector('#selectAllTafasir').addEventListener('click', () => {
            this.selectAllTafasir();
        });
        
        modal.querySelector('#clearAllTafasir').addEventListener('click', () => {
            this.clearAllTafasir();
        });
        
        modal.querySelector('#resetToDefault').addEventListener('click', () => {
            this.resetToDefaultTafasir();
        });
        
        // Ayah actions
        modal.querySelector('#playAyah').addEventListener('click', () => {
            this.playAyah();
        });
        
        modal.querySelector('#bookmarkAyah').addEventListener('click', () => {
            this.bookmarkAyah();
        });
        
        modal.querySelector('#shareAyah').addEventListener('click', () => {
            this.shareAyah();
        });
        
        // Comparison tools
        modal.querySelector('#highlightDifferences').addEventListener('click', () => {
            this.highlightDifferences();
        });
        
        modal.querySelector('#showSimilarities').addEventListener('click', () => {
            this.showSimilarities();
        });
        
        modal.querySelector('#analyzeThemes').addEventListener('click', () => {
            this.analyzeThemes();
        });
        
        modal.querySelector('#exportComparison').addEventListener('click', () => {
            this.exportComparison();
        });
        
        modal.querySelector('#printComparison').addEventListener('click', () => {
            this.printComparison();
        });
        
        // Close modal when clicking outside
        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                this.closeComparison();
            }
        });
        
        // Keyboard shortcuts
        document.addEventListener('keydown', (e) => {
            if (modal.classList.contains('active')) {
                this.handleKeyboardShortcuts(e);
            }
        });
    },
    
    /**
     * Populate Tafsir options
     */
    populateTafsirOptions() {
        const container = this.elements.tafsirSelector;
        container.innerHTML = '';
        
        this.state.availableTafasir.forEach(tafsir => {
            const option = window.SanadUtils.dom.create('div', {
                className: 'tafsir-option'
            });
            
            const isSelected = this.state.selectedTafasir.includes(tafsir.id);
            
            option.innerHTML = `
                <label class="tafsir-checkbox">
                    <input type="checkbox" 
                           value="${tafsir.id}" 
                           ${isSelected ? 'checked' : ''}
                           data-color="${tafsir.color}">
                    <span class="checkmark" style="border-color: ${tafsir.color}"></span>
                    <div class="tafsir-info">
                        <div class="tafsir-name">${tafsir.name}</div>
                        <div class="tafsir-author">المؤلف: ${tafsir.author}</div>
                        <div class="tafsir-methodology">${tafsir.methodology}</div>
                    </div>
                </label>
            `;
            
            // Add event listener
            const checkbox = option.querySelector('input[type="checkbox"]');
            checkbox.addEventListener('change', (e) => {
                this.toggleTafsirSelection(tafsir.id, e.target.checked);
            });
            
            container.appendChild(option);
        });
    },
    
    /**
     * Populate Surah options
     */
    populateSurahOptions() {
        const surahSelector = this.elements.comparisonModal.querySelector('#surahSelector');
        
        // Mock Surah data - in real implementation, fetch from API
        const surahs = [
            { number: 1, name: 'الفاتحة', ayahCount: 7 },
            { number: 2, name: 'البقرة', ayahCount: 286 },
            { number: 3, name: 'آل عمران', ayahCount: 200 },
            { number: 4, name: 'النساء', ayahCount: 176 },
            { number: 5, name: 'المائدة', ayahCount: 120 }
            // Add more surahs as needed
        ];
        
        surahSelector.innerHTML = '';
        surahs.forEach(surah => {
            const option = window.SanadUtils.dom.create('option', {
                value: surah.number
            }, `${surah.number}. ${surah.name}`);
            
            surahSelector.appendChild(option);
        });
        
        // Update ayah selector when surah changes
        surahSelector.addEventListener('change', (e) => {
            const selectedSurah = surahs.find(s => s.number === parseInt(e.target.value));
            if (selectedSurah) {
                const ayahSelector = this.elements.comparisonModal.querySelector('#ayahSelector');
                ayahSelector.max = selectedSurah.ayahCount;
                ayahSelector.value = Math.min(parseInt(ayahSelector.value), selectedSurah.ayahCount);
            }
        });
    },
    
    /**
     * Toggle Tafsir selection
     */
    toggleTafsirSelection(tafsirId, isSelected) {
        if (isSelected) {
            if (!this.state.selectedTafasir.includes(tafsirId)) {
                this.state.selectedTafasir.push(tafsirId);
            }
        } else {
            this.state.selectedTafasir = this.state.selectedTafasir.filter(id => id !== tafsirId);
        }
        
        // Update comparison if data is already loaded
        if (this.state.currentSurah && this.state.currentAyah) {
            this.renderComparison();
        }
    },
    
    /**
     * Select all Tafasir
     */
    selectAllTafasir() {
        this.state.selectedTafasir = this.state.availableTafasir.map(t => t.id);
        this.updateTafsirCheckboxes();
        this.renderComparison();
    },
    
    /**
     * Clear all Tafasir
     */
    clearAllTafasir() {
        this.state.selectedTafasir = [];
        this.updateTafsirCheckboxes();
        this.renderComparison();
    },
    
    /**
     * Reset to default Tafasir
     */
    resetToDefaultTafasir() {
        this.state.selectedTafasir = ['ibn-kathir', 'tabari', 'qurtubi'];
        this.updateTafsirCheckboxes();
        this.renderComparison();
    },
    
    /**
     * Update Tafsir checkboxes
     */
    updateTafsirCheckboxes() {
        const checkboxes = this.elements.tafsirSelector.querySelectorAll('input[type="checkbox"]');
        checkboxes.forEach(checkbox => {
            checkbox.checked = this.state.selectedTafasir.includes(checkbox.value);
        });
    },
    
    /**
     * Switch view mode
     */
    switchViewMode(mode) {
        this.state.viewMode = mode;
        
        // Update active button
        this.elements.viewModeToggle.forEach(btn => {
            btn.classList.remove('active');
            if (btn.dataset.mode === mode) {
                btn.classList.add('active');
            }
        });
        
        // Update comparison display
        this.renderComparison();
    },
    
    /**
     * Load comparison data
     */
    async loadComparisonData() {
        if (!this.state.currentSurah || !this.state.currentAyah) return;
        
        this.showLoading();
        
        try {
            // Update ayah display
            await this.updateAyahDisplay();
            
            // Load Tafsir data for selected Tafasir
            const comparisonData = {};
            
            for (const tafsirId of this.state.selectedTafasir) {
                const tafsirData = await this.loadTafsirData(tafsirId, this.state.currentSurah, this.state.currentAyah);
                comparisonData[tafsirId] = tafsirData;
            }
            
            this.state.comparisonData = comparisonData;
            this.renderComparison();
            
        } catch (error) {
            console.error('Failed to load comparison data:', error);
            this.showError('فشل في تحميل بيانات المقارنة');
        } finally {
            this.hideLoading();
        }
    },
    
    /**
     * Load Tafsir data
     */
    async loadTafsirData(tafsirId, surahNumber, ayahNumber) {
        try {
            // Mock data - in real implementation, fetch from API
            const mockTafsirData = {
                'ibn-kathir': {
                    text: 'يقول تعالى ذكره: بسم الله الرحمن الرحيم، وهذا تعليم من الله عز وجل لعباده ليفتتحوا به كتابه، وقد استحب العلماء أن يبدأ بالبسملة في كل أمر ذي بال.',
                    themes: ['التوحيد', 'الرحمة', 'البركة'],
                    keyPoints: ['البسملة تعليم من الله', 'استحباب البدء بها', 'أهمية التوحيد']
                },
                'tabari': {
                    text: 'القول في تأويل قوله تعالى: بسم الله الرحمن الرحيم. قال أبو جعفر: إن الله تعالى ذكره وتقدست أسماؤه، علم عباده تقديم اسمه على جميع أعمالهم وأقوالهم.',
                    themes: ['التأويل', 'التقديم', 'الأسماء الحسنى'],
                    keyPoints: ['تقديم اسم الله', 'تعليم العباد', 'الأسماء الحسنى']
                },
                'qurtubi': {
                    text: 'بسم الله الرحمن الرحيم. هذه الآية كتبت في أول كل سورة سوى براءة، وهي آية من الفاتحة بإجماع، واختلف في غيرها من السور.',
                    themes: ['الفقه', 'الإجماع', 'الخلاف'],
                    keyPoints: ['آية من الفاتحة', 'الإجماع والخلاف', 'الأحكام الفقهية']
                }
            };
            
            return mockTafsirData[tafsirId] || {
                text: 'لم يتم العثور على تفسير لهذه الآية في هذا المصدر.',
                themes: [],
                keyPoints: []
            };
            
        } catch (error) {
            console.error(`Failed to load Tafsir data for ${tafsirId}:`, error);
            return {
                text: 'خطأ في تحميل التفسير',
                themes: [],
                keyPoints: []
            };
        }
    },
    
    /**
     * Update ayah display
     */
    async updateAyahDisplay() {
        const modal = this.elements.comparisonModal;
        
        // Mock ayah data - in real implementation, fetch from API
        const ayahData = {
            arabic: 'بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ',
            translation: 'بسم الله الرحمن الرحيم',
            surahName: 'الفاتحة'
        };
        
        modal.querySelector('#ayahReference').textContent = `سورة ${ayahData.surahName} - آية ${this.state.currentAyah}`;
        modal.querySelector('#ayahArabic').textContent = ayahData.arabic;
        modal.querySelector('#ayahTranslation').textContent = ayahData.translation;
        
        // Update selectors
        modal.querySelector('#surahSelector').value = this.state.currentSurah;
        modal.querySelector('#ayahSelector').value = this.state.currentAyah;
    },
    
    /**
     * Render comparison
     */
    renderComparison() {
        const container = this.elements.comparisonContainer;
        const emptyState = this.elements.comparisonModal.querySelector('#comparisonEmpty');
        
        if (this.state.selectedTafasir.length === 0) {
            container.style.display = 'none';
            emptyState.style.display = 'block';
            return;
        }
        
        container.style.display = 'block';
        emptyState.style.display = 'none';
        
        switch (this.state.viewMode) {
            case 'side-by-side':
                this.renderSideBySideView();
                break;
            case 'tabbed':
                this.renderTabbedView();
                break;
            case 'unified':
                this.renderUnifiedView();
                break;
        }
    },
    
    /**
     * Render side-by-side view
     */
    renderSideBySideView() {
        const container = this.elements.comparisonContainer;
        container.className = 'comparison-content side-by-side-view';
        container.innerHTML = '';
        
        const grid = window.SanadUtils.dom.create('div', {
            className: 'tafsir-grid'
        });
        
        this.state.selectedTafasir.forEach(tafsirId => {
            const tafsir = this.state.availableTafasir.find(t => t.id === tafsirId);
            const data = this.state.comparisonData[tafsirId];
            
            if (tafsir && data) {
                const tafsirCard = this.createTafsirCard(tafsir, data);
                grid.appendChild(tafsirCard);
            }
        });
        
        container.appendChild(grid);
    },
    
    /**
     * Render tabbed view
     */
    renderTabbedView() {
        const container = this.elements.comparisonContainer;
        container.className = 'comparison-content tabbed-view';
        container.innerHTML = '';
        
        // Create tabs
        const tabsContainer = window.SanadUtils.dom.create('div', {
            className: 'tafsir-tabs'
        });
        
        const tabsList = window.SanadUtils.dom.create('div', {
            className: 'tabs-list'
        });
        
        const tabsContent = window.SanadUtils.dom.create('div', {
            className: 'tabs-content'
        });
        
        this.state.selectedTafasir.forEach((tafsirId, index) => {
            const tafsir = this.state.availableTafasir.find(t => t.id === tafsirId);
            const data = this.state.comparisonData[tafsirId];
            
            if (tafsir && data) {
                // Create tab button
                const tabBtn = window.SanadUtils.dom.create('button', {
                    className: `tab-btn ${index === 0 ? 'active' : ''}`,
                    'data-tab': tafsirId
                });
                
                tabBtn.innerHTML = `
                    <span class="tab-color" style="background-color: ${tafsir.color}"></span>
                    ${tafsir.name}
                `;
                
                tabBtn.addEventListener('click', () => {
                    this.switchTab(tafsirId);
                });
                
                tabsList.appendChild(tabBtn);
                
                // Create tab content
                const tabContent = window.SanadUtils.dom.create('div', {
                    className: `tab-content ${index === 0 ? 'active' : ''}`,
                    'data-tab': tafsirId
                });
                
                const tafsirCard = this.createTafsirCard(tafsir, data, true);
                tabContent.appendChild(tafsirCard);
                tabsContent.appendChild(tabContent);
            }
        });
        
        tabsContainer.appendChild(tabsList);
        tabsContainer.appendChild(tabsContent);
        container.appendChild(tabsContainer);
    },
    
    /**
     * Render unified view
     */
    renderUnifiedView() {
        const container = this.elements.comparisonContainer;
        container.className = 'comparison-content unified-view';
        container.innerHTML = '';
        
        const unifiedContainer = window.SanadUtils.dom.create('div', {
            className: 'unified-container'
        });
        
        this.state.selectedTafasir.forEach(tafsirId => {
            const tafsir = this.state.availableTafasir.find(t => t.id === tafsirId);
            const data = this.state.comparisonData[tafsirId];
            
            if (tafsir && data) {
                const section = window.SanadUtils.dom.create('div', {
                    className: 'unified-section'
                });
                
                section.innerHTML = `
                    <div class="section-header">
                        <div class="section-indicator" style="background-color: ${tafsir.color}"></div>
                        <h3 class="section-title">${tafsir.name}</h3>
                        <div class="section-author">المؤلف: ${tafsir.author}</div>
                    </div>
                    <div class="section-content">
                        <div class="tafsir-text">${data.text}</div>
                        ${data.keyPoints.length > 0 ? `
                        <div class="key-points">
                            <h4>النقاط الرئيسية:</h4>
                            <ul>
                                ${data.keyPoints.map(point => `<li>${point}</li>`).join('')}
                            </ul>
                        </div>
                        ` : ''}
                    </div>
                `;
                
                unifiedContainer.appendChild(section);
            }
        });
        
        container.appendChild(unifiedContainer);
    },
    
    /**
     * Create Tafsir card
     */
    createTafsirCard(tafsir, data, isFullWidth = false) {
        const card = window.SanadUtils.dom.create('div', {
            className: `tafsir-card ${isFullWidth ? 'full-width' : ''}`
        });
        
        card.innerHTML = `
            <div class="card-header">
                <div class="header-left">
                    <div class="tafsir-indicator" style="background-color: ${tafsir.color}"></div>
                    <div class="tafsir-info">
                        <h3 class="tafsir-title">${tafsir.name}</h3>
                        <div class="tafsir-author">المؤلف: ${tafsir.author}</div>
                        <div class="tafsir-methodology">${tafsir.methodology}</div>
                    </div>
                </div>
                <div class="header-right">
                    <button class="card-action" title="نسخ النص">
                        <span class="action-icon">📋</span>
                    </button>
                    <button class="card-action" title="تكبير">
                        <span class="action-icon">🔍</span>
                    </button>
                </div>
            </div>
            
            <div class="card-content">
                <div class="tafsir-text">${data.text}</div>
                
                ${data.themes.length > 0 ? `
                <div class="tafsir-themes">
                    <h4>المواضيع الرئيسية:</h4>
                    <div class="themes-list">
                        ${data.themes.map(theme => `
                            <span class="theme-tag" style="border-color: ${tafsir.color}">${theme}</span>
                        `).join('')}
                    </div>
                </div>
                ` : ''}
                
                ${data.keyPoints.length > 0 ? `
                <div class="key-points">
                    <h4>النقاط الرئيسية:</h4>
                    <ul class="points-list">
                        ${data.keyPoints.map(point => `<li>${point}</li>`).join('')}
                    </ul>
                </div>
                ` : ''}
            </div>
        `;
        
        // Add event listeners for card actions
        const copyBtn = card.querySelector('.card-action[title="نسخ النص"]');
        const expandBtn = card.querySelector('.card-action[title="تكبير"]');
        
        copyBtn.addEventListener('click', () => {
            this.copyTafsirText(data.text);
        });
        
        expandBtn.addEventListener('click', () => {
            this.expandTafsirCard(card);
        });
        
        return card;
    },
    
    /**
     * Switch tab in tabbed view
     */
    switchTab(tafsirId) {
        const container = this.elements.comparisonContainer;
        
        // Update tab buttons
        container.querySelectorAll('.tab-btn').forEach(btn => {
            btn.classList.remove('active');
            if (btn.dataset.tab === tafsirId) {
                btn.classList.add('active');
            }
        });
        
        // Update tab content
        container.querySelectorAll('.tab-content').forEach(content => {
            content.classList.remove('active');
            if (content.dataset.tab === tafsirId) {
                content.classList.add('active');
            }
        });
    },
    
    /**
     * Update comparison
     */
    async updateComparison(surahNumber, ayahNumber) {
        this.state.currentSurah = surahNumber;
        this.state.currentAyah = ayahNumber;
        await this.loadComparisonData();
    },
    
    /**
     * Show loading
     */
    showLoading() {
        this.state.isLoading = true;
        const loading = this.elements.comparisonModal.querySelector('#comparisonLoading');
        const content = this.elements.comparisonContainer;
        
        loading.style.display = 'flex';
        content.style.opacity = '0.5';
    },
    
    /**
     * Hide loading
     */
    hideLoading() {
        this.state.isLoading = false;
        const loading = this.elements.comparisonModal.querySelector('#comparisonLoading');
        const content = this.elements.comparisonContainer;
        
        loading.style.display = 'none';
        content.style.opacity = '1';
    },
    
    /**
     * Show error
     */
    showError(message) {
        this.elements.comparisonContainer.innerHTML = `
            <div class="comparison-error">
                <div class="error-icon">⚠️</div>
                <h3>خطأ في تحميل المقارنة</h3>
                <p>${message}</p>
                <button class="btn btn-primary" onclick="window.SanadTafsirComparison.loadComparisonData()">
                    إعادة المحاولة
                </button>
            </div>
        `;
    },
    
    /**
     * Close comparison modal
     */
    closeComparison() {
        if (this.elements.comparisonModal) {
            this.elements.comparisonModal.classList.remove('active');
        }
    },
    
    /**
     * Handle keyboard shortcuts
     */
    handleKeyboardShortcuts(e) {
        switch (e.key) {
            case 'Escape':
                this.closeComparison();
                break;
            case '1':
                if (e.ctrlKey) {
                    e.preventDefault();
                    this.switchViewMode('side-by-side');
                }
                break;
            case '2':
                if (e.ctrlKey) {
                    e.preventDefault();
                    this.switchViewMode('tabbed');
                }
                break;
            case '3':
                if (e.ctrlKey) {
                    e.preventDefault();
                    this.switchViewMode('unified');
                }
                break;
        }
    },
    
    /**
     * Play ayah audio
     */
    playAyah() {
        // Implementation for audio playback
        console.log('Playing ayah audio...');
        if (window.SanadApp && window.SanadApp.showNotification) {
            window.SanadApp.showNotification('تشغيل الآية...', 'info');
        }
    },
    
    /**
     * Bookmark ayah
     */
    bookmarkAyah() {
        // Implementation for bookmarking
        console.log('Bookmarking ayah...');
        if (window.SanadApp && window.SanadApp.showNotification) {
            window.SanadApp.showNotification('تم حفظ الآية في المفضلة', 'success');
        }
    },
    
    /**
     * Share ayah
     */
    shareAyah() {
        // Implementation for sharing
        const shareText = `سورة ${this.getSurahName(this.state.currentSurah)} - آية ${this.state.currentAyah}`;
        
        if (navigator.share) {
            navigator.share({
                title: 'آية من القرآن الكريم',
                text: shareText,
                url: window.location.href
            });
        } else {
            // Fallback to clipboard
            navigator.clipboard.writeText(shareText).then(() => {
                if (window.SanadApp && window.SanadApp.showNotification) {
                    window.SanadApp.showNotification('تم نسخ الآية للحافظة', 'success');
                }
            });
        }
    },
    
    /**
     * Highlight differences
     */
    highlightDifferences() {
        // Implementation for highlighting differences
        console.log('Highlighting differences...');
        const cards = this.elements.comparisonContainer.querySelectorAll('.tafsir-card');
        cards.forEach(card => {
            card.classList.toggle('highlight-differences');
        });
    },
    
    /**
     * Show similarities
     */
    showSimilarities() {
        // Implementation for showing similarities
        console.log('Showing similarities...');
        const cards = this.elements.comparisonContainer.querySelectorAll('.tafsir-card');
        cards.forEach(card => {
            card.classList.toggle('show-similarities');
        });
    },
    
    /**
     * Analyze themes
     */
    analyzeThemes() {
        // Implementation for theme analysis
        console.log('Analyzing themes...');
        this.openThemeAnalysisModal();
    },
    
    /**
     * Open theme analysis modal
     */
    openThemeAnalysisModal() {
        // Create and show theme analysis modal
        const analysisModal = window.SanadUtils.dom.create('div', {
            className: 'modal theme-analysis-modal'
        });
        
        analysisModal.innerHTML = `
            <div class="modal-content">
                <div class="modal-header">
                    <h3>تحليل المواضيع</h3>
                    <button class="modal-close">×</button>
                </div>
                <div class="modal-body">
                    <div class="theme-analysis">
                        <h4>المواضيع المشتركة:</h4>
                        <div class="common-themes">
                            <span class="theme-tag">التوحيد</span>
                            <span class="theme-tag">الرحمة</span>
                            <span class="theme-tag">البركة</span>
                        </div>
                        
                        <h4>المواضيع المختلفة:</h4>
                        <div class="different-themes">
                            <div class="theme-group">
                                <strong>ابن كثير:</strong>
                                <span class="theme-tag">التعليم الإلهي</span>
                            </div>
                            <div class="theme-group">
                                <strong>الطبري:</strong>
                                <span class="theme-tag">التأويل</span>
                            </div>
                            <div class="theme-group">
                                <strong>القرطبي:</strong>
                                <span class="theme-tag">الأحكام الفقهية</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        `;
        
        document.getElementById('modalOverlay').appendChild(analysisModal);
        analysisModal.classList.add('active');
        
        // Close modal
        analysisModal.querySelector('.modal-close').addEventListener('click', () => {
            analysisModal.remove();
        });
    },
    
    /**
     * Export comparison
     */
    exportComparison() {
        const comparisonData = {
            ayah: {
                surah: this.state.currentSurah,
                ayah: this.state.currentAyah,
                arabic: this.elements.comparisonModal.querySelector('#ayahArabic').textContent,
                translation: this.elements.comparisonModal.querySelector('#ayahTranslation').textContent
            },
            tafasir: this.state.selectedTafasir.map(tafsirId => {
                const tafsir = this.state.availableTafasir.find(t => t.id === tafsirId);
                const data = this.state.comparisonData[tafsirId];
                return {
                    name: tafsir.name,
                    author: tafsir.author,
                    text: data.text,
                    themes: data.themes,
                    keyPoints: data.keyPoints
                };
            }),
            exportDate: new Date().toISOString()
        };
        
        const blob = new Blob([JSON.stringify(comparisonData, null, 2)], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        
        const a = document.createElement('a');
        a.href = url;
        a.download = `tafsir-comparison-${this.state.currentSurah}-${this.state.currentAyah}.json`;
        a.click();
        
        URL.revokeObjectURL(url);
    },
    
    /**
     * Print comparison
     */
    printComparison() {
        const printWindow = window.open('', '_blank');
        const comparisonHtml = this.generatePrintableHtml();
        
        printWindow.document.write(comparisonHtml);
        printWindow.document.close();
        printWindow.print();
    },
    
    /**
     * Generate printable HTML
     */
    generatePrintableHtml() {
        const ayahArabic = this.elements.comparisonModal.querySelector('#ayahArabic').textContent;
        const ayahTranslation = this.elements.comparisonModal.querySelector('#ayahTranslation').textContent;
        const surahName = this.getSurahName(this.state.currentSurah);
        
        let tafasirHtml = '';
        this.state.selectedTafasir.forEach(tafsirId => {
            const tafsir = this.state.availableTafasir.find(t => t.id === tafsirId);
            const data = this.state.comparisonData[tafsirId];
            
            tafasirHtml += `
                <div class="tafsir-section">
                    <h3>${tafsir.name}</h3>
                    <p><strong>المؤلف:</strong> ${tafsir.author}</p>
                    <div class="tafsir-text">${data.text}</div>
                    ${data.keyPoints.length > 0 ? `
                    <div class="key-points">
                        <h4>النقاط الرئيسية:</h4>
                        <ul>
                            ${data.keyPoints.map(point => `<li>${point}</li>`).join('')}
                        </ul>
                    </div>
                    ` : ''}
                </div>
            `;
        });
        
        return `
            <!DOCTYPE html>
            <html dir="rtl" lang="ar">
            <head>
                <meta charset="UTF-8">
                <title>مقارنة التفاسير - سورة ${surahName} آية ${this.state.currentAyah}</title>
                <style>
                    body { font-family: 'Amiri', serif; line-height: 1.6; margin: 20px; }
                    .ayah-section { text-align: center; margin-bottom: 30px; border-bottom: 2px solid #ccc; padding-bottom: 20px; }
                    .ayah-arabic { font-size: 24px; color: #27ae60; margin-bottom: 10px; }
                    .ayah-translation { font-size: 18px; color: #666; }
                    .tafsir-section { margin-bottom: 30px; page-break-inside: avoid; }
                    .tafsir-section h3 { color: #2c3e50; border-bottom: 1px solid #eee; padding-bottom: 5px; }
                    .tafsir-text { text-align: justify; margin: 15px 0; }
                    .key-points { background: #f8f9fa; padding: 15px; border-right: 4px solid #3498db; }
                    @media print { body { margin: 0; } }
                </style>
            </head>
            <body>
                <h1>مقارنة التفاسير</h1>
                <div class="ayah-section">
                    <h2>سورة ${surahName} - آية ${this.state.currentAyah}</h2>
                    <div class="ayah-arabic">${ayahArabic}</div>
                    <div class="ayah-translation">${ayahTranslation}</div>
                </div>
                ${tafasirHtml}
                <div class="footer">
                    <p><small>تم إنشاء هذه المقارنة في ${new Date().toLocaleDateString('ar-SA')} من تطبيق سند الإسلامي</small></p>
                </div>
            </body>
            </html>
        `;
    },
    
    /**
     * Copy Tafsir text
     */
    copyTafsirText(text) {
        navigator.clipboard.writeText(text).then(() => {
            if (window.SanadApp && window.SanadApp.showNotification) {
                window.SanadApp.showNotification('تم نسخ النص للحافظة', 'success');
            }
        });
    },
    
    /**
     * Expand Tafsir card
     */
    expandTafsirCard(card) {
        card.classList.toggle('expanded');
    },
    
    /**
     * Get Surah name by number
     */
    getSurahName(surahNumber) {
        const surahNames = {
            1: 'الفاتحة',
            2: 'البقرة',
            3: 'آل عمران',
            4: 'النساء',
            5: 'المائدة'
            // Add more as needed
        };
        
        return surahNames[surahNumber] || `السورة ${surahNumber}`;
    }
};

// Initialize when DOM is ready
window.SanadUtils.timing.ready(() => {
    window.SanadTafsirComparison.init();
});

// Freeze the object to prevent modifications
Object.freeze(window.SanadTafsirComparison);