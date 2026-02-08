/**
 * Tafsir-related type definitions
 */

export enum ScholarlyAuthentication {
  HighlyAuthenticated = 'highly_authenticated',
  Authenticated = 'authenticated',
  Verified = 'verified',
  Unverified = 'unverified',
}

export enum TafsirSourceType {
  Classical = 'classical',
  Contemporary = 'contemporary',
  Linguistic = 'linguistic',
  Thematic = 'thematic',
  Sectarian = 'sectarian',
}

export interface TafsirSource {
  id: string;
  name: string;
  author: string;
  language: string;
  description?: string;
  credibility_score: number;
  scholarly_authentication: ScholarlyAuthentication;
  source_type: TafsirSourceType;
  publication_year?: number;
  methodology?: string;
  created_at: string;
  updated_at: string;
}

export interface Tafsir {
  id: string;
  surah_number: number;
  ayah_number: number;
  source_id: string;
  text: string;
  text_hash: string;
  word_count: number;
  themes: string[];
  cross_references: string[];
  created_at: string;
  updated_at: string;
}

export interface TafsirWithSource {
  tafsir: Tafsir;
  source: TafsirSource;
}

export enum ComparisonCriteria {
  Linguistic = 'linguistic',
  Thematic = 'thematic',
  Historical = 'historical',
  Jurisprudential = 'jurisprudential',
  Spiritual = 'spiritual',
}

export interface TafsirComparisonRequest {
  surah_number: number;
  ayah_number: number;
  source_ids: string[];
  comparison_criteria?: ComparisonCriteria[];
}

export enum ViewSignificance {
  Major = 'major',
  Moderate = 'moderate',
  Minor = 'minor',
}

export interface SourcePosition {
  source_id: string;
  source_name: string;
  position: string;
  evidence: string[];
}

export interface DivergentView {
  topic: string;
  source_positions: SourcePosition[];
  significance: ViewSignificance;
}

export interface ComparisonSummary {
  common_themes: string[];
  divergent_views: DivergentView[];
  scholarly_consensus?: string;
  recommended_reading_order: string[];
}

export interface TafsirComparison {
  source: TafsirSource;
  tafsir: Tafsir;
  key_points: string[];
  unique_insights: string[];
  methodology_notes?: string;
}

export interface TafsirComparisonResponse {
  ayah: any; // Reference to Ayah type
  surah: any; // Reference to Surah type
  comparisons: TafsirComparison[];
  summary: ComparisonSummary;
  recommendations: string[];
}

export enum TafsirSearchCriteria {
  TextContent = 'text_content',
  Themes = 'themes',
  CrossReferences = 'cross_references',
  AuthorName = 'author_name',
  Methodology = 'methodology',
}

export interface TafsirSourceFilters {
  source_types?: TafsirSourceType[];
  authentication_levels?: ScholarlyAuthentication[];
  languages?: string[];
  credibility_range?: [number, number];
  publication_year_range?: [number, number];
}

export interface TafsirSearchRequest {
  query: string;
  search_criteria: TafsirSearchCriteria[];
  source_filters?: TafsirSourceFilters;
  limit?: number;
  offset?: number;
}

export interface FacetCount {
  value: string;
  count: number;
}

export interface SearchFacets {
  source_types: FacetCount[];
  authentication_levels: FacetCount[];
  languages: FacetCount[];
  authors: FacetCount[];
}

export interface TafsirSearchResult {
  tafsir: Tafsir;
  source: TafsirSource;
  ayah: any;
  surah: any;
  relevance_score: number;
  highlighted_text: string;
  matching_criteria: string[];
}

export interface TafsirSearchResponse {
  results: TafsirSearchResult[];
  total_count: number;
  search_time_ms: number;
  facets: SearchFacets;
}

export interface TafsirDisplayPreferences {
  selected_sources: string[];
  layout: 'stacked' | 'side-by-side' | 'tabbed';
  show_cross_references: boolean;
  show_themes: boolean;
  font_size: 'small' | 'medium' | 'large';
}
