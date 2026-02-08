// Hadith authenticity grades according to Islamic scholarship
export enum HadithGrade {
  SAHIH = 'sahih',
  HASAN = 'hasan',
  DAIF = 'daif',
  MAWDU = 'mawdu',
}

// Chain of narration grades for Sanad authenticity
export enum ChainGrade {
  SAHIH = 'sahih',
  HASAN = 'hasan',
  DAIF = 'daif',
  MUNQATI = 'munqati',
  MURSAL = 'mursal',
}

// Types of Hadith books
export enum HadithBookType {
  SAHIH = 'sahih',
  SUNAN = 'sunan',
  MUSNAD = 'musnad',
  MUJAM = 'mujam',
  MUSTADRAK = 'mustadrak',
  JAMI = 'jami',
}

// Book authenticity levels
export enum BookAuthenticityLevel {
  HIGHEST = 'highest',
  HIGH = 'high',
  MODERATE = 'moderate',
  VARIABLE = 'variable',
}

// Hadith model
export interface Hadith {
  id: string;
  hadith_number: string;
  text: string;
  text_hash: string;
  narrator: string;
  book: string;
  chapter: string;
  chapter_number?: number;
  hadith_number_in_chapter?: number;
  grade: HadithGrade;
  source: string;
  language: string;
  word_count: number;
  themes: string[];
  keywords: string[];
  created_at: string;
  updated_at: string;
}

// Sanad (Chain of Narration) model
export interface Sanad {
  id: string;
  hadith_id: string;
  chain_text: string;
  chain_hash: string;
  narrators: string[];
  chain_grade: ChainGrade;
  chain_analysis?: string;
  created_at: string;
  updated_at: string;
}

// Hadith Book model
export interface HadithBook {
  id: string;
  name: string;
  arabic_name: string;
  author: string;
  author_arabic_name: string;
  description?: string;
  compilation_year?: number;
  total_hadiths: number;
  book_type: HadithBookType;
  authenticity_level: BookAuthenticityLevel;
  language: string;
  created_at: string;
  updated_at: string;
}

// Hadith Chapter model
export interface HadithChapter {
  id: string;
  book_id: string;
  chapter_number: number;
  title: string;
  arabic_title: string;
  description?: string;
  hadith_count: number;
  themes: string[];
  created_at: string;
}

// Complete Hadith with all related information
export interface HadithWithDetails {
  hadith: Hadith;
  book: HadithBook;
  chapter?: HadithChapter;
  sanad?: Sanad;
}

// Search result for Hadith content
export interface HadithSearchResult {
  hadith: Hadith;
  book: HadithBook;
  chapter?: HadithChapter;
  relevance_score: number;
  highlighted_text: string;
  matching_criteria: string[];
}

// Search response
export interface HadithSearchResponse {
  results: HadithSearchResult[];
  total_count: number;
  query: string;
  search_type: string;
  search_time_ms: number;
}

// Topic response
export interface HadithTopicResponse {
  topic: string;
  hadiths: HadithWithDetails[];
  related_topics: string[];
  total_count: number;
}

// Search filters
export interface HadithSearchFilters {
  books?: string[];
  grades?: HadithGrade[];
  themes?: string[];
  searchType?: 'text' | 'semantic' | 'narrator' | 'theme' | 'exact';
}

// Helper functions
export const getGradeArabicName = (grade: HadithGrade): string => {
  const names: Record<HadithGrade, string> = {
    [HadithGrade.SAHIH]: 'صحيح',
    [HadithGrade.HASAN]: 'حسن',
    [HadithGrade.DAIF]: 'ضعيف',
    [HadithGrade.MAWDU]: 'موضوع',
  };
  return names[grade];
};

export const getChainGradeArabicName = (grade: ChainGrade): string => {
  const names: Record<ChainGrade, string> = {
    [ChainGrade.SAHIH]: 'صحيح',
    [ChainGrade.HASAN]: 'حسن',
    [ChainGrade.DAIF]: 'ضعيف',
    [ChainGrade.MUNQATI]: 'منقطع',
    [ChainGrade.MURSAL]: 'مرسل',
  };
  return names[grade];
};

export const getBookTypeArabicName = (type: HadithBookType): string => {
  const names: Record<HadithBookType, string> = {
    [HadithBookType.SAHIH]: 'صحيح',
    [HadithBookType.SUNAN]: 'سنن',
    [HadithBookType.MUSNAD]: 'مسند',
    [HadithBookType.MUJAM]: 'معجم',
    [HadithBookType.MUSTADRAK]: 'مستدرك',
    [HadithBookType.JAMI]: 'جامع',
  };
  return names[type];
};

export const getAuthenticityLevelArabicName = (level: BookAuthenticityLevel): string => {
  const names: Record<BookAuthenticityLevel, string> = {
    [BookAuthenticityLevel.HIGHEST]: 'أعلى درجة',
    [BookAuthenticityLevel.HIGH]: 'عالية',
    [BookAuthenticityLevel.MODERATE]: 'متوسطة',
    [BookAuthenticityLevel.VARIABLE]: 'متغيرة',
  };
  return names[level];
};

export const getGradeColor = (grade: HadithGrade): string => {
  const colors: Record<HadithGrade, string> = {
    [HadithGrade.SAHIH]: '#28A745', // Green
    [HadithGrade.HASAN]: '#FFC107', // Amber
    [HadithGrade.DAIF]: '#FF9800', // Orange
    [HadithGrade.MAWDU]: '#DC3545', // Red
  };
  return colors[grade];
};

export const getChainGradeColor = (grade: ChainGrade): string => {
  const colors: Record<ChainGrade, string> = {
    [ChainGrade.SAHIH]: '#28A745',
    [ChainGrade.HASAN]: '#FFC107',
    [ChainGrade.DAIF]: '#FF9800',
    [ChainGrade.MUNQATI]: '#DC3545',
    [ChainGrade.MURSAL]: '#DC3545',
  };
  return colors[grade];
};

export const isAuthentic = (hadith: Hadith): boolean => {
  return hadith.grade === HadithGrade.SAHIH || hadith.grade === HadithGrade.HASAN;
};

export const isContinuousChain = (sanad: Sanad): boolean => {
  return sanad.chain_grade !== ChainGrade.MUNQATI && sanad.chain_grade !== ChainGrade.MURSAL;
};
