/**
 * Advanced Search Interface for Sanad Islamic App
 * Provides comprehensive search functionality with filters and semantic search
 */

window.SanadAdvancedSearch = {
    
    /**
     * Search state
     */
    state: {
        currentQuery: '',
        activeFilters: {},
        searchResults: [],
        isSearching: false,
        searchHistory: [],
        suggestions: []
    },
    
    /**
     * DOM elements cache
     */
    elements: {},
    
    /**
     * Initialize advanced search
     */
    init() {
        this.cacheElements();
        this.setupEventListeners();
        this.loadSearchHistory();
        this.initializeFilters();
    },
    
    /**
     * Cache DOM elements
     */
    cacheElements() {
        this.elements = {
            searchModal: null, // Will be created dynamically
            searchInput: null,
            filtersContainer: null,
            resultsContainer: null,
            suggestionsContainer: null,
            searchTypeToggle: null,
            sortOptions: null,
            loadingIndicator: null
        };
    },
    
    /**
     * Setup event listeners
     */
    setupEventListeners() {
        // Listen for search modal trigger
        document.addEventListener('openAdvancedSearch', (e) => {
            const query = e.detail?.query || '';
            this.openSearchModal(query);
        });
        
        // Listen for global search enhancement
        const globalSearch = document.getElementById('globalSearch');
        if (globalSearch) {
            globalSearch.addEventListener('focus', () => {
                this.enhanceGlobalSearch();
            });
        }
    },
    
    /**
     * Open advanced search modal
     */
    openSearchModal(initialQuery = '') {
        if (!this.elements.searchModal) {
            this.createSearchModal();
        }
        
        this.elements.searchModal.classList.add('active');
        
        // Set initial query if provided
        if (initialQuery) {
            this.elements.searchInput.value = initialQuery;
            this.state.currentQuery = initialQuery;
            this.performSearch();
        } else {
            this.elements.searchInput.focus();
            // Load recent searches
            this.displaySearchHistory();
        }
    },
    
    /**
     * Create search modal
     */
    createSearchModal() {
        const modalOverlay = document.getElementById('modalOverlay');
        
        const searchModal = window.SanadUtils.dom.create('div', {
            className: 'modal advanced-search-modal',
            id: 'advancedSearchModal'
        });
        
        searchModal.innerHTML = `
            <div class="modal-content">
                <div class="modal-header">
                    <h2 class="modal-title">البحث المتقدم</h2>
                    <button class="modal-close" id="closeAdvancedSearch">×</button>
                </div>
                
                <div class="modal-body">
                    <!-- Search Input Section -->
                    <div class="search-section">
                        <div class="search-input-container">
                            <input type="text" 
                                   class="advanced-search-input" 
                                   id="advancedSearchInput"
                                   placeholder="ابحث في القرآن والأحاديث والقصص الإسلامية..."
                                   autocomplete="off">
                            <div class="search-type-toggle">
                                <button class="search-type-btn active" data-type="text">نصي</button>
                                <button class="search-type-btn" data-type="semantic">دلالي</button>
                                <button class="search-type-btn" data-type="root">جذر</button>
                            </div>
                        </div>
                        
                        <div class="search-suggestions" id="searchSuggestions">
                            <!-- Suggestions will appear here -->
                        </div>
                    </div>
                    
                    <!-- Filters Section -->
                    <div class="filters-section">
                        <h3 class="filters-title">الفلاتر</h3>
                        <div class="filters-grid">
                            <!-- Content Type Filter -->
                            <div class="filter-group">
                                <label class="filter-label">نوع المحتوى</label>
                                <div class="filter-options">
                                    <label class="filter-checkbox">
                                        <input type="checkbox" name="contentType" value="quran" checked>
                                        <span class="checkmark"></span>
                                        القرآن الكريم
                                    </label>
                                    <label class="filter-checkbox">
                                        <input type="checkbox" name="contentType" value="hadith" checked>
                                        <span class="checkmark"></span>
                                        الأحاديث النبوية
                                    </label>
                                    <label class="filter-checkbox">
                                        <input type="checkbox" name="contentType" value="tafsir" checked>
                                        <span class="checkmark"></span>
                                        التفاسير
                                    </label>
                                    <label class="filter-checkbox">
                                        <input type="checkbox" name="contentType" value="stories" checked>
                                        <span class="checkmark"></span>
                                        القصص الإسلامية
                                    </label>
                                </div>
                            </div>
                            
                            <!-- Hadith Grade Filter -->
                            <div class="filter-group">
                                <label class="filter-label">درجة الحديث</label>
                                <div class="filter-options">
                                    <label class="filter-checkbox">
                                        <input type="checkbox" name="hadithGrade" value="sahih" checked>
                                        <span class="checkmark"></span>
                                        صحيح
                                    </label>
                                    <label class="filter-checkbox">
                                        <input type="checkbox" name="hadithGrade" value="hasan">
                                        <span class="checkmark"></span>
                                        حسن
                                    </label>
                                    <label class="filter-checkbox">
                                        <input type="checkbox" name="hadithGrade" value="daif">
                                        <span class="checkmark"></span>
                                        ضعيف
                                    </label>
                                </div>
                            </div>
                            
                            <!-- Source Filter -->
                            <div class="filter-group">
                                <label class="filter-label">المصدر</label>
                                <select class="filter-select" name="source" id="sourceFilter">
                                    <option value="">جميع المصادر</option>
                                    <option value="bukhari">صحيح البخاري</option>
                                    <option value="muslim">صحيح مسلم</option>
                                    <option value="abu-dawud">سنن أبي داود</option>
                                    <option value="tirmidhi">سنن الترمذي</option>
                                    <option value="ibn-majah">سنن ابن ماجه</option>
                                    <option value="nasai">سنن النسائي</option>
                                </select>
                            </div>
                            
                            <!-- Tafsir Filter -->
                            <div class="filter-group">
                                <label class="filter-label">التفسير</label>
                                <select class="filter-select" name="tafsir" id="tafsirFilter">
                                    <option value="">جميع التفاسير</option>
                                    <option value="ibn-kathir">تفسير ابن كثير</option>
                                    <option value="tabari">تفسير الطبري</option>
                                    <option value="qurtubi">تفسير القرطبي</option>
                                    <option value="baghawi">تفسير البغوي</option>
                                </select>
                            </div>
                        </div>
                        
                        <div class="filter-actions">
                            <button class="btn btn-secondary" id="clearFilters">مسح الفلاتر</button>
                            <button class="btn btn-primary" id="applyFilters">تطبيق الفلاتر</button>
                        </div>
                    </div>
                    
                    <!-- Results Section -->
                    <div class="results-section">
                        <div class="results-header">
                            <div class="results-info">
                                <span class="results-count" id="resultsCount">0 نتيجة</span>
                                <span class="search-time" id="searchTime"></span>
                            </div>
                            <div class="sort-options">
                                <label>ترتيب حسب:</label>
                                <select id="sortResults">
                                    <option value="relevance">الصلة</option>
                                    <option value="date">التاريخ</option>
                                    <option value="source">المصدر</option>
                                    <option value="length">الطول</option>
                                </select>
                            </div>
                        </div>
                        
                        <div class="search-results" id="searchResults">
                            <!-- Results will appear here -->
                        </div>
                        
                        <div class="search-loading" id="searchLoading" style="display: none;">
                            <div class="loading-spinner"></div>
                            <p>جاري البحث...</p>
                        </div>
                        
                        <div class="load-more-container" style="display: none;">
                            <button class="btn btn-outline" id="loadMoreResults">تحميل المزيد</button>
                        </div>
                    </div>
                    
                    <!-- Search History -->
                    <div class="search-history-section" id="searchHistorySection">
                        <h3>عمليات البحث الأخيرة</h3>
                        <div class="search-history-list" id="searchHistoryList">
                            <!-- History items will appear here -->
                        </div>
                    </div>
                </div>
            </div>
        `;
        
        modalOverlay.appendChild(searchModal);
        this.elements.searchModal = searchModal;
        
        // Cache new elements
        this.elements.searchInput = searchModal.querySelector('#advancedSearchInput');
        this.elements.filtersContainer = searchModal.querySelector('.filters-section');
        this.elements.resultsContainer = searchModal.querySelector('#searchResults');
        this.elements.suggestionsContainer = searchModal.querySelector('#searchSuggestions');
        this.elements.loadingIndicator = searchModal.querySelector('#searchLoading');
        
        // Setup modal event listeners
        this.setupModalEventListeners();
    },
    
    /**
     * Setup modal event listeners
     */
    setupModalEventListeners() {
        const modal = this.elements.searchModal;
        
        // Close modal
        modal.querySelector('#closeAdvancedSearch').addEventListener('click', () => {
            this.closeSearchModal();
        });
        
        // Search input
        this.elements.searchInput.addEventListener('input', 
            window.SanadUtils.timing.debounce((e) => {
                this.handleSearchInput(e.target.value);
            }, 300)
        );
        
        this.elements.searchInput.addEventListener('keydown', (e) => {
            if (e.key === 'Enter') {
                e.preventDefault();
                this.performSearch();
            }
        });
        
        // Search type toggle
        modal.querySelectorAll('.search-type-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                this.switchSearchType(btn.dataset.type);
            });
        });
        
        // Filter actions
        modal.querySelector('#clearFilters').addEventListener('click', () => {
            this.clearFilters();
        });
        
        modal.querySelector('#applyFilters').addEventListener('click', () => {
            this.applyFilters();
        });
        
        // Sort options
        modal.querySelector('#sortResults').addEventListener('change', (e) => {
            this.sortResults(e.target.value);
        });
        
        // Load more results
        modal.querySelector('#loadMoreResults').addEventListener('click', () => {
            this.loadMoreResults();
        });
        
        // Filter checkboxes and selects
        modal.querySelectorAll('input[type="checkbox"], select').forEach(element => {
            element.addEventListener('change', () => {
                this.updateActiveFilters();
            });
        });
        
        // Close modal when clicking outside
        modal.addEventListener('click', (e) => {
            if (e.target === modal) {
                this.closeSearchModal();
            }
        });
    },
    
    /**
     * Handle search input
     */
    async handleSearchInput(query) {
        this.state.currentQuery = query;
        
        if (query.length >= 2) {
            // Show suggestions
            await this.showSuggestions(query);
            
            // Auto-search if enabled
            if (query.length >= 3) {
                await this.performSearch();
            }
        } else {
            this.hideSuggestions();
            this.showSearchHistory();
        }
    },
    
    /**
     * Show search suggestions
     */
    async showSuggestions(query) {
        try {
            const suggestions = await window.SanadAPI.search.getSuggestions(query);
            this.displaySuggestions(suggestions);
        } catch (error) {
            console.error('Failed to get suggestions:', error);
        }
    },
    
    /**
     * Display suggestions
     */
    displaySuggestions(suggestions) {
        const container = this.elements.suggestionsContainer;
        container.innerHTML = '';
        
        if (suggestions && suggestions.length > 0) {
            suggestions.forEach(suggestion => {
                const suggestionElement = window.SanadUtils.dom.create('div', {
                    className: 'search-suggestion'
                });
                
                suggestionElement.innerHTML = `
                    <span class="suggestion-text">${suggestion.text}</span>
                    <span class="suggestion-type">${suggestion.type}</span>
                `;
                
                suggestionElement.addEventListener('click', () => {
                    this.elements.searchInput.value = suggestion.text;
                    this.state.currentQuery = suggestion.text;
                    this.performSearch();
                    this.hideSuggestions();
                });
                
                container.appendChild(suggestionElement);
            });
            
            container.style.display = 'block';
        } else {
            this.hideSuggestions();
        }
    },
    
    /**
     * Hide suggestions
     */
    hideSuggestions() {
        this.elements.suggestionsContainer.style.display = 'none';
    },
    
    /**
     * Switch search type
     */
    switchSearchType(type) {
        // Update active button
        this.elements.searchModal.querySelectorAll('.search-type-btn').forEach(btn => {
            btn.classList.remove('active');
            if (btn.dataset.type === type) {
                btn.classList.add('active');
            }
        });
        
        // Update search placeholder
        const placeholders = {
            text: 'ابحث في النصوص...',
            semantic: 'ابحث بالمعنى والمفهوم...',
            root: 'ابحث بالجذر اللغوي...'
        };
        
        this.elements.searchInput.placeholder = placeholders[type];
        
        // Re-search if there's a query
        if (this.state.currentQuery) {
            this.performSearch();
        }
    },
    
    /**
     * Perform search
     */
    async performSearch() {
        if (!this.state.currentQuery.trim()) {
            return;
        }
        
        this.state.isSearching = true;
        this.showLoading();
        this.hideSearchHistory();
        
        try {
            const searchType = this.getActiveSearchType();
            const filters = this.getActiveFilters();
            
            const startTime = Date.now();
            let results;
            
            if (searchType === 'semantic') {
                results = await window.SanadAPI.search.semanticSearch(
                    this.state.currentQuery,
                    filters.contentTypes,
                    0.7
                );
            } else {
                results = await window.SanadAPI.search.search(
                    this.state.currentQuery,
                    {
                        ...filters,
                        searchType
                    }
                );
            }
            
            const searchTime = Date.now() - startTime;
            
            this.state.searchResults = results.results || [];
            this.displayResults(results, searchTime);
            this.addToSearchHistory(this.state.currentQuery);
            
        } catch (error) {
            console.error('Search failed:', error);
            this.showSearchError('فشل في البحث. يرجى المحاولة مرة أخرى.');
        } finally {
            this.state.isSearching = false;
            this.hideLoading();
        }
    },
    
    /**
     * Display search results
     */
    displayResults(results, searchTime) {
        const container = this.elements.resultsContainer;
        const countElement = this.elements.searchModal.querySelector('#resultsCount');
        const timeElement = this.elements.searchModal.querySelector('#searchTime');
        
        // Update results info
        const count = results.totalResults || results.results?.length || 0;
        countElement.textContent = `${count} نتيجة`;
        timeElement.textContent = `(${searchTime} مللي ثانية)`;
        
        // Clear previous results
        container.innerHTML = '';
        
        if (!results.results || results.results.length === 0) {
            container.innerHTML = `
                <div class="no-results">
                    <div class="no-results-icon">🔍</div>
                    <h3>لا توجد نتائج</h3>
                    <p>لم نجد أي نتائج تطابق بحثك. جرب:</p>
                    <ul>
                        <li>استخدام كلمات مختلفة</li>
                        <li>تقليل الفلاتر</li>
                        <li>البحث الدلالي بدلاً من النصي</li>
                    </ul>
                </div>
            `;
            return;
        }
        
        // Display results
        results.results.forEach(result => {
            const resultElement = this.createResultElement(result);
            container.appendChild(resultElement);
        });
        
        // Show load more button if there are more results
        const loadMoreContainer = this.elements.searchModal.querySelector('.load-more-container');
        if (results.hasMore) {
            loadMoreContainer.style.display = 'block';
        } else {
            loadMoreContainer.style.display = 'none';
        }
    },
    
    /**
     * Create result element
     */
    createResultElement(result) {
        const resultDiv = window.SanadUtils.dom.create('div', {
            className: `search-result search-result-${result.type}`
        });
        
        let contentHtml = '';
        
        switch (result.type) {
            case 'quran':
                contentHtml = this.createQuranResultHtml(result);
                break;
            case 'hadith':
                contentHtml = this.createHadithResultHtml(result);
                break;
            case 'tafsir':
                contentHtml = this.createTafsirResultHtml(result);
                break;
            case 'story':
                contentHtml = this.createStoryResultHtml(result);
                break;
            default:
                contentHtml = this.createGenericResultHtml(result);
        }
        
        resultDiv.innerHTML = contentHtml;
        
        // Add click handler
        resultDiv.addEventListener('click', () => {
            this.openResult(result);
        });
        
        return resultDiv;
    },
    
    /**
     * Create Quran result HTML
     */
    createQuranResultHtml(result) {
        return `
            <div class="result-header">
                <div class="result-type">
                    <span class="type-icon">📖</span>
                    <span class="type-text">القرآن الكريم</span>
                </div>
                <div class="result-reference">
                    سورة ${result.surahName} - آية ${result.ayahNumber}
                </div>
            </div>
            <div class="result-content">
                <div class="arabic-text">${result.arabicText}</div>
                ${result.translation ? `<div class="translation-text">${result.translation}</div>` : ''}
                ${result.highlightedText ? `<div class="highlighted-text">${result.highlightedText}</div>` : ''}
            </div>
            <div class="result-footer">
                <div class="result-actions">
                    <button class="action-btn" data-action="bookmark">
                        <span class="action-icon">🔖</span>
                        حفظ
                    </button>
                    <button class="action-btn" data-action="share">
                        <span class="action-icon">📤</span>
                        مشاركة
                    </button>
                    <button class="action-btn" data-action="tafsir">
                        <span class="action-icon">📚</span>
                        التفسير
                    </button>
                </div>
                <div class="result-score">
                    الصلة: ${Math.round((result.score || 0) * 100)}%
                </div>
            </div>
        `;
    },
    
    /**
     * Create Hadith result HTML
     */
    createHadithResultHtml(result) {
        const gradeColors = {
            sahih: '#27ae60',
            hasan: '#f39c12',
            daif: '#e74c3c',
            mawdu: '#8e44ad'
        };
        
        return `
            <div class="result-header">
                <div class="result-type">
                    <span class="type-icon">📜</span>
                    <span class="type-text">الأحاديث النبوية</span>
                </div>
                <div class="result-reference">
                    ${result.book} - ${result.chapter}
                </div>
            </div>
            <div class="result-content">
                <div class="hadith-text">${result.text}</div>
                ${result.highlightedText ? `<div class="highlighted-text">${result.highlightedText}</div>` : ''}
                <div class="hadith-narrator">الراوي: ${result.narrator}</div>
            </div>
            <div class="result-footer">
                <div class="result-actions">
                    <button class="action-btn" data-action="bookmark">
                        <span class="action-icon">🔖</span>
                        حفظ
                    </button>
                    <button class="action-btn" data-action="share">
                        <span class="action-icon">📤</span>
                        مشاركة
                    </button>
                    <button class="action-btn" data-action="explanation">
                        <span class="action-icon">💡</span>
                        الشرح
                    </button>
                </div>
                <div class="result-metadata">
                    <span class="hadith-grade" style="color: ${gradeColors[result.grade] || '#6c757d'}">
                        ${result.gradeArabic || result.grade}
                    </span>
                    <span class="result-score">
                        الصلة: ${Math.round((result.score || 0) * 100)}%
                    </span>
                </div>
            </div>
        `;
    },
    
    /**
     * Create Tafsir result HTML
     */
    createTafsirResultHtml(result) {
        return `
            <div class="result-header">
                <div class="result-type">
                    <span class="type-icon">📚</span>
                    <span class="type-text">التفاسير</span>
                </div>
                <div class="result-reference">
                    ${result.tafsirName} - سورة ${result.surahName} آية ${result.ayahNumber}
                </div>
            </div>
            <div class="result-content">
                <div class="tafsir-text">${result.text}</div>
                ${result.highlightedText ? `<div class="highlighted-text">${result.highlightedText}</div>` : ''}
            </div>
            <div class="result-footer">
                <div class="result-actions">
                    <button class="action-btn" data-action="bookmark">
                        <span class="action-icon">🔖</span>
                        حفظ
                    </button>
                    <button class="action-btn" data-action="share">
                        <span class="action-icon">📤</span>
                        مشاركة
                    </button>
                    <button class="action-btn" data-action="compare">
                        <span class="action-icon">⚖️</span>
                        مقارنة
                    </button>
                </div>
                <div class="result-score">
                    الصلة: ${Math.round((result.score || 0) * 100)}%
                </div>
            </div>
        `;
    },
    
    /**
     * Create Story result HTML
     */
    createStoryResultHtml(result) {
        return `
            <div class="result-header">
                <div class="result-type">
                    <span class="type-icon">📖</span>
                    <span class="type-text">القصص الإسلامية</span>
                </div>
                <div class="result-reference">
                    ${result.category} - ${result.title}
                </div>
            </div>
            <div class="result-content">
                <div class="story-excerpt">${result.excerpt}</div>
                ${result.highlightedText ? `<div class="highlighted-text">${result.highlightedText}</div>` : ''}
                ${result.characters ? `<div class="story-characters">الشخصيات: ${result.characters.join(', ')}</div>` : ''}
            </div>
            <div class="result-footer">
                <div class="result-actions">
                    <button class="action-btn" data-action="bookmark">
                        <span class="action-icon">🔖</span>
                        حفظ
                    </button>
                    <button class="action-btn" data-action="share">
                        <span class="action-icon">📤</span>
                        مشاركة
                    </button>
                    <button class="action-btn" data-action="read">
                        <span class="action-icon">👁️</span>
                        قراءة
                    </button>
                </div>
                <div class="result-score">
                    الصلة: ${Math.round((result.score || 0) * 100)}%
                </div>
            </div>
        `;
    },
    
    /**
     * Create generic result HTML
     */
    createGenericResultHtml(result) {
        return `
            <div class="result-header">
                <div class="result-type">
                    <span class="type-icon">📄</span>
                    <span class="type-text">${result.type}</span>
                </div>
                <div class="result-reference">${result.reference || ''}</div>
            </div>
            <div class="result-content">
                <div class="result-text">${result.text || result.content}</div>
                ${result.highlightedText ? `<div class="highlighted-text">${result.highlightedText}</div>` : ''}
            </div>
            <div class="result-footer">
                <div class="result-actions">
                    <button class="action-btn" data-action="bookmark">
                        <span class="action-icon">🔖</span>
                        حفظ
                    </button>
                    <button class="action-btn" data-action="share">
                        <span class="action-icon">📤</span>
                        مشاركة
                    </button>
                </div>
                <div class="result-score">
                    الصلة: ${Math.round((result.score || 0) * 100)}%
                </div>
            </div>
        `;
    },
    
    /**
     * Get active search type
     */
    getActiveSearchType() {
        const activeBtn = this.elements.searchModal.querySelector('.search-type-btn.active');
        return activeBtn ? activeBtn.dataset.type : 'text';
    },
    
    /**
     * Get active filters
     */
    getActiveFilters() {
        const filters = {};
        
        // Content types
        const contentTypes = [];
        this.elements.searchModal.querySelectorAll('input[name="contentType"]:checked').forEach(cb => {
            contentTypes.push(cb.value);
        });
        filters.contentTypes = contentTypes;
        
        // Hadith grades
        const hadithGrades = [];
        this.elements.searchModal.querySelectorAll('input[name="hadithGrade"]:checked').forEach(cb => {
            hadithGrades.push(cb.value);
        });
        filters.hadithGrades = hadithGrades;
        
        // Source
        const sourceSelect = this.elements.searchModal.querySelector('#sourceFilter');
        if (sourceSelect.value) {
            filters.source = sourceSelect.value;
        }
        
        // Tafsir
        const tafsirSelect = this.elements.searchModal.querySelector('#tafsirFilter');
        if (tafsirSelect.value) {
            filters.tafsir = tafsirSelect.value;
        }
        
        return filters;
    },
    
    /**
     * Update active filters
     */
    updateActiveFilters() {
        this.state.activeFilters = this.getActiveFilters();
        
        // Re-search if there's a query
        if (this.state.currentQuery && !this.state.isSearching) {
            this.performSearch();
        }
    },
    
    /**
     * Clear filters
     */
    clearFilters() {
        // Uncheck all checkboxes
        this.elements.searchModal.querySelectorAll('input[type="checkbox"]').forEach(cb => {
            cb.checked = false;
        });
        
        // Reset selects
        this.elements.searchModal.querySelectorAll('select').forEach(select => {
            select.selectedIndex = 0;
        });
        
        // Check default content types
        this.elements.searchModal.querySelectorAll('input[name="contentType"]').forEach(cb => {
            cb.checked = true;
        });
        
        this.updateActiveFilters();
    },
    
    /**
     * Apply filters
     */
    applyFilters() {
        this.updateActiveFilters();
        if (this.state.currentQuery) {
            this.performSearch();
        }
    },
    
    /**
     * Sort results
     */
    sortResults(sortBy) {
        if (!this.state.searchResults.length) return;
        
        const sortedResults = [...this.state.searchResults];
        
        switch (sortBy) {
            case 'relevance':
                sortedResults.sort((a, b) => (b.score || 0) - (a.score || 0));
                break;
            case 'date':
                sortedResults.sort((a, b) => new Date(b.date || 0) - new Date(a.date || 0));
                break;
            case 'source':
                sortedResults.sort((a, b) => (a.source || '').localeCompare(b.source || ''));
                break;
            case 'length':
                sortedResults.sort((a, b) => (a.text?.length || 0) - (b.text?.length || 0));
                break;
        }
        
        this.state.searchResults = sortedResults;
        this.displayResults({ results: sortedResults }, 0);
    },
    
    /**
     * Load more results
     */
    async loadMoreResults() {
        // Implementation for pagination
        console.log('Loading more results...');
    },
    
    /**
     * Show loading
     */
    showLoading() {
        this.elements.loadingIndicator.style.display = 'flex';
        this.elements.resultsContainer.style.opacity = '0.5';
    },
    
    /**
     * Hide loading
     */
    hideLoading() {
        this.elements.loadingIndicator.style.display = 'none';
        this.elements.resultsContainer.style.opacity = '1';
    },
    
    /**
     * Show search error
     */
    showSearchError(message) {
        this.elements.resultsContainer.innerHTML = `
            <div class="search-error">
                <div class="error-icon">⚠️</div>
                <h3>خطأ في البحث</h3>
                <p>${message}</p>
                <button class="btn btn-primary" onclick="window.SanadAdvancedSearch.performSearch()">
                    إعادة المحاولة
                </button>
            </div>
        `;
    },
    
    /**
     * Load search history
     */
    loadSearchHistory() {
        const history = window.SanadUtils.storage.get('search_history') || [];
        this.state.searchHistory = history;
    },
    
    /**
     * Add to search history
     */
    addToSearchHistory(query) {
        if (!query.trim()) return;
        
        // Remove if already exists
        this.state.searchHistory = this.state.searchHistory.filter(item => item.query !== query);
        
        // Add to beginning
        this.state.searchHistory.unshift({
            query,
            timestamp: new Date().toISOString(),
            filters: { ...this.state.activeFilters }
        });
        
        // Keep only last 20 searches
        this.state.searchHistory = this.state.searchHistory.slice(0, 20);
        
        // Save to storage
        window.SanadUtils.storage.set('search_history', this.state.searchHistory);
    },
    
    /**
     * Display search history
     */
    displaySearchHistory() {
        const historySection = this.elements.searchModal.querySelector('#searchHistorySection');
        const historyList = this.elements.searchModal.querySelector('#searchHistoryList');
        
        if (!this.state.searchHistory.length) {
            historySection.style.display = 'none';
            return;
        }
        
        historyList.innerHTML = '';
        
        this.state.searchHistory.slice(0, 10).forEach(item => {
            const historyItem = window.SanadUtils.dom.create('div', {
                className: 'search-history-item'
            });
            
            historyItem.innerHTML = `
                <div class="history-query">${item.query}</div>
                <div class="history-time">${this.formatRelativeTime(item.timestamp)}</div>
                <button class="history-remove" data-query="${item.query}">×</button>
            `;
            
            historyItem.addEventListener('click', (e) => {
                if (!e.target.classList.contains('history-remove')) {
                    this.elements.searchInput.value = item.query;
                    this.state.currentQuery = item.query;
                    this.performSearch();
                }
            });
            
            historyItem.querySelector('.history-remove').addEventListener('click', (e) => {
                e.stopPropagation();
                this.removeFromHistory(item.query);
            });
            
            historyList.appendChild(historyItem);
        });
        
        historySection.style.display = 'block';
    },
    
    /**
     * Show search history
     */
    showSearchHistory() {
        this.displaySearchHistory();
    },
    
    /**
     * Hide search history
     */
    hideSearchHistory() {
        const historySection = this.elements.searchModal.querySelector('#searchHistorySection');
        historySection.style.display = 'none';
    },
    
    /**
     * Remove from history
     */
    removeFromHistory(query) {
        this.state.searchHistory = this.state.searchHistory.filter(item => item.query !== query);
        window.SanadUtils.storage.set('search_history', this.state.searchHistory);
        this.displaySearchHistory();
    },
    
    /**
     * Format relative time
     */
    formatRelativeTime(timestamp) {
        const now = new Date();
        const time = new Date(timestamp);
        const diffMs = now - time;
        const diffMins = Math.floor(diffMs / 60000);
        const diffHours = Math.floor(diffMs / 3600000);
        const diffDays = Math.floor(diffMs / 86400000);
        
        if (diffMins < 1) return 'الآن';
        if (diffMins < 60) return `منذ ${diffMins} دقيقة`;
        if (diffHours < 24) return `منذ ${diffHours} ساعة`;
        if (diffDays < 7) return `منذ ${diffDays} يوم`;
        
        return time.toLocaleDateString('ar-SA');
    },
    
    /**
     * Open result
     */
    openResult(result) {
        // Navigate to the appropriate section with the result
        switch (result.type) {
            case 'quran':
                this.openQuranResult(result);
                break;
            case 'hadith':
                this.openHadithResult(result);
                break;
            case 'tafsir':
                this.openTafsirResult(result);
                break;
            case 'story':
                this.openStoryResult(result);
                break;
        }
        
        this.closeSearchModal();
    },
    
    /**
     * Open Quran result
     */
    openQuranResult(result) {
        // Navigate to Quran section with specific ayah
        window.SanadApp.navigateToSection('quran');
        // Additional logic to highlight the specific ayah
    },
    
    /**
     * Open Hadith result
     */
    openHadithResult(result) {
        // Navigate to Hadith section with specific hadith
        window.SanadApp.navigateToSection('hadith');
        // Additional logic to show the specific hadith
    },
    
    /**
     * Open Tafsir result
     */
    openTafsirResult(result) {
        // Open tafsir comparison interface
        if (window.SanadTafsirComparison) {
            window.SanadTafsirComparison.openComparison(result.surahNumber, result.ayahNumber);
        }
    },
    
    /**
     * Open Story result
     */
    openStoryResult(result) {
        // Navigate to Stories section with specific story
        window.SanadApp.navigateToSection('stories');
        // Additional logic to show the specific story
    },
    
    /**
     * Close search modal
     */
    closeSearchModal() {
        if (this.elements.searchModal) {
            this.elements.searchModal.classList.remove('active');
        }
    },
    
    /**
     * Enhance global search
     */
    enhanceGlobalSearch() {
        const globalSearch = document.getElementById('globalSearch');
        if (!globalSearch) return;
        
        // Add advanced search button
        const searchContainer = globalSearch.parentElement;
        if (!searchContainer.querySelector('.advanced-search-trigger')) {
            const advancedBtn = window.SanadUtils.dom.create('button', {
                className: 'advanced-search-trigger',
                title: 'البحث المتقدم'
            }, '⚙️');
            
            advancedBtn.addEventListener('click', () => {
                this.openSearchModal();
            });
            
            searchContainer.appendChild(advancedBtn);
        }
    },
    
    /**
     * Initialize filters
     */
    initializeFilters() {
        // Load saved filter preferences
        const savedFilters = window.SanadUtils.storage.get('search_filters');
        if (savedFilters) {
            this.state.activeFilters = savedFilters;
        }
    }
};

// Initialize when DOM is ready
window.SanadUtils.timing.ready(() => {
    window.SanadAdvancedSearch.init();
});

// Freeze the object to prevent modifications
Object.freeze(window.SanadAdvancedSearch);