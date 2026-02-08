/**
 * Search types for comprehensive Islamic content search
 * Corresponds to backend search service API
 */

export enum ContentType {
  Quran = 'quran',
  SahihHadith = 'sahih_hadith',
  HasanHadith = 'hasan_hadith',
  DaifHadith = 'daif_hadith',
  MawduHadith = 'mawdu_hadith',
  Tafsir = 'tafsir',
  FiqhRuling = 'fiqh_ruling',
  ScholarOpinion = 'scholar_opinion',
  IslamicStory = 'islamic_story',
  Dua = 'dua',
  Dhikr = 'dhikr',
  Biography = 'biography',
  History = 'history',
}

export enum AuthenticityGrade {
  Sahih = 'sahih',
  Hasan = 'hasan',
  Daif = 'daif',
  Mawdu = 'mawdu',
  Unknown = 'unknown',
}

export enum SortBy {
  Similarity = 'similarity',
  Priority = 'priority',
  CreatedAt = 'created_at',
  UpdatedAt = 'updated_at',
  TextLength = 'text_length',
  Relevance = 'relevance',
}

export enum SortDirection {
  Asc = 'asc',
  Desc = 'desc',
}

export interface SearchFilters {
  source?: string[];
  author?: string[];
  language?: string;
  content_types?: ContentType[];
  authenticity_grades?: AuthenticityGrade[];
  min_similarity?: number;
  max_similarity?: number;
}

export interface SearchRequest {
  query: string;
  limit?: number;
  content_types?: string[];
  min_similarity?: number;
  include_metadata?: boolean;
  filters?: SearchFilters;
  offset?: number;
  page?: number;
  page_size?: number;
  include_suggestions?: boolean;
  enable_caching?: boolean;
  sort_by?: SortBy;
  sort_direction?: SortDirection;
}

export interface IslamicDocument {
  id: string;
  text: string;
  content_type: string;
  source: string;
  author?: string;
  language: string;
  metadata: Record<string, any>;
  created_at?: string;
  updated_at?: string;
}

export interface SearchResult {
  document: IslamicDocument;
  similarity_score: number;
  rank: number;
  highlighted_text?: string;
  explanation?: string;
}

export interface PaginationInfo {
  current_page: number;
  total_pages: number;
  page_size: number;
  total_items: number;
  has_next_page: boolean;
  has_previous_page: boolean;
  next_page?: number;
  previous_page?: number;
}

export interface QuerySuggestion {
  suggested_query: string;
  similarity_score: number;
  expected_results_count: number;
  suggestion_type: string;
  explanation?: string;
}

export interface SearchMetadata {
  query_processed: string;
  query_keywords: string[];
  content_types_searched: string[];
  filters_applied: boolean;
  embedding_model: string;
}

export interface SearchResponse {
  results: SearchResult[];
  total_results: number;
  search_time_ms: number;
  query_embedding_time_ms: number;
  search_metadata: SearchMetadata;
  pagination?: PaginationInfo;
  suggestions?: QuerySuggestion[];
  from_cache: boolean;
  cache_key?: string;
}

export interface SavedSearch {
  id: string;
  query: string;
  filters?: SearchFilters;
  created_at: string;
  name?: string;
}

// Helper functions
export const getContentTypeLabel = (type: ContentType): string => {
  const labels: Record<ContentType, string> = {
    [ContentType.Quran]: 'القرآن الكريم',
    [ContentType.SahihHadith]: 'حديث صحيح',
    [ContentType.HasanHadith]: 'حديث حسن',
    [ContentType.DaifHadith]: 'حديث ضعيف',
    [ContentType.MawduHadith]: 'حديث موضوع',
    [ContentType.Tafsir]: 'تفسير',
    [ContentType.FiqhRuling]: 'حكم فقهي',
    [ContentType.ScholarOpinion]: 'رأي عالم',
    [ContentType.IslamicStory]: 'قصة إسلامية',
    [ContentType.Dua]: 'دعاء',
    [ContentType.Dhikr]: 'ذكر',
    [ContentType.Biography]: 'سيرة',
    [ContentType.History]: 'تاريخ',
  };
  return labels[type];
};

export const getContentTypeIcon = (type: ContentType): string => {
  const icons: Record<ContentType, string> = {
    [ContentType.Quran]: '📖',
    [ContentType.SahihHadith]: '📜',
    [ContentType.HasanHadith]: '📜',
    [ContentType.DaifHadith]: '📜',
    [ContentType.MawduHadith]: '📜',
    [ContentType.Tafsir]: '📚',
    [ContentType.FiqhRuling]: '⚖️',
    [ContentType.ScholarOpinion]: '👨‍🏫',
    [ContentType.IslamicStory]: '📖',
    [ContentType.Dua]: '🤲',
    [ContentType.Dhikr]: '📿',
    [ContentType.Biography]: '👤',
    [ContentType.History]: '🏛️',
  };
  return icons[type];
};

export const getAuthenticityLabel = (grade: AuthenticityGrade): string => {
  const labels: Record<AuthenticityGrade, string> = {
    [AuthenticityGrade.Sahih]: 'صحيح',
    [AuthenticityGrade.Hasan]: 'حسن',
    [AuthenticityGrade.Daif]: 'ضعيف',
    [AuthenticityGrade.Mawdu]: 'موضوع',
    [AuthenticityGrade.Unknown]: 'غير معروف',
  };
  return labels[grade];
};

export const getAuthenticityColor = (grade: AuthenticityGrade): string => {
  const colors: Record<AuthenticityGrade, string> = {
    [AuthenticityGrade.Sahih]: '#28A745',
    [AuthenticityGrade.Hasan]: '#FFC107',
    [AuthenticityGrade.Daif]: '#FF9800',
    [AuthenticityGrade.Mawdu]: '#DC3545',
    [AuthenticityGrade.Unknown]: '#6C757D',
  };
  return colors[grade];
};

export const getSimilarityColor = (score: number): string => {
  if (score >= 0.8) return '#28A745'; // Green
  if (score >= 0.6) return '#B8860B'; // Gold
  return '#FFC107'; // Yellow
};
