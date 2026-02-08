// Islamic Stories Library Types

export enum StoryCategory {
  PROPHETS = 'prophets',
  COMPANIONS = 'companions',
  RIGHTEOUS_PREDECESSORS = 'righteous_predecessors',
  HISTORICAL_EVENTS = 'historical_events',
  MORAL_LESSONS = 'moral_lessons',
  MIRACLES = 'miracles',
  BATTLES = 'battles',
  CONVERSIONS = 'conversions',
  WOMEN_IN_ISLAM = 'women_in_islam',
  CHILDREN_STORIES = 'children_stories',
}

export enum TimePeriod {
  PRE_ISLAMIC = 'pre_islamic',
  PROPHETIC_ERA = 'prophetic_era',
  RIGHTLY_GUIDED_CALIPHS = 'rightly_guided_caliphs',
  UMAYYAD = 'umayyad',
  ABBASID = 'abbasid',
  OTTOMAN = 'ottoman',
  MODERN = 'modern',
  ANCIENT_PROPHETS = 'ancient_prophets',
}

export enum AgeGroup {
  CHILDREN = 'children',
  TEENAGERS = 'teenagers',
  YOUNG_ADULTS = 'young_adults',
  ADULTS = 'adults',
  ALL_AGES = 'all_ages',
}

export enum AuthenticityLevel {
  AUTHENTIC = 'authentic',
  WELL_DOCUMENTED = 'well_documented',
  PROBABLE = 'probable',
  TRADITIONAL = 'traditional',
  EDUCATIONAL = 'educational',
}

export enum ScholarlyVerification {
  VERIFIED = 'verified',
  UNDER_REVIEW = 'under_review',
  PENDING = 'pending',
  DISPUTED = 'disputed',
}

export enum CharacterType {
  PROPHET = 'prophet',
  MESSENGER = 'messenger',
  COMPANION = 'companion',
  RIGHTEOUS_PERSON = 'righteous_person',
  SCHOLAR = 'scholar',
  RULER = 'ruler',
  MARTYR = 'martyr',
  CONVERT = 'convert',
  HISTORICAL_FIGURE = 'historical_figure',
  ANTAGONIST = 'antagonist',
}

export enum LessonType {
  MORAL = 'moral',
  SPIRITUAL = 'spiritual',
  PRACTICAL = 'practical',
  HISTORICAL = 'historical',
  THEOLOGICAL = 'theological',
  SOCIAL = 'social',
}

export enum MoralCategory {
  PATIENCE = 'patience',
  GRATITUDE = 'gratitude',
  JUSTICE = 'justice',
  MERCY = 'mercy',
  HONESTY = 'honesty',
  COURAGE = 'courage',
  HUMILITY = 'humility',
  FORGIVENESS = 'forgiveness',
  PERSEVERANCE = 'perseverance',
  FAITH = 'faith',
}

export enum SourceType {
  QURAN = 'quran',
  HADITH = 'hadith',
  HISTORICAL_BOOK = 'historical_book',
  BIOGRAPHY = 'biography',
  TAFSIR = 'tafsir',
  SCHOLARLY_WORK = 'scholarly_work',
}

export enum VerificationStatus {
  VERIFIED = 'verified',
  UNVERIFIED = 'unverified',
  QUESTIONABLE = 'questionable',
}

export interface Story {
  id: string;
  title: string;
  arabicTitle: string;
  content: string;
  contentHash: string;
  summary?: string;
  category: StoryCategory;
  subcategory?: string;
  timePeriod?: TimePeriod;
  location?: string;
  wordCount: number;
  estimatedReadingTime: number;
  ageGroup: AgeGroup;
  moralLessons: string[];
  themes: string[];
  keywords: string[];
  language: string;
  authenticityLevel: AuthenticityLevel;
  scholarlyVerification: ScholarlyVerification;
  createdAt: string;
  updatedAt: string;
}

export interface Character {
  id: string;
  name: string;
  arabicName: string;
  characterType: CharacterType;
  description?: string;
  historicalPeriod?: TimePeriod;
  birthYear?: number;
  deathYear?: number;
  biography?: string;
  virtues: string[];
  roleSignificance?: string;
  relatedStoriesCount: number;
}

export interface Lesson {
  id: string;
  title: string;
  arabicTitle: string;
  description: string;
  lessonType: LessonType;
  moralCategory: MoralCategory;
  practicalApplication?: string;
  targetAudience: AgeGroup[];
  relatedVerses: string[];
  relatedHadiths: string[];
}

export interface StorySource {
  id: string;
  storyId: string;
  sourceType: SourceType;
  sourceName: string;
  arabicSourceName: string;
  author?: string;
  reference: string;
  authenticityGrade?: string;
  credibilityScore: number;
  verificationStatus: VerificationStatus;
  notes?: string;
}

export interface CharacterInStory {
  character: Character;
  roleInStory: string;
  importanceLevel: string;
  characterDescriptionInStory?: string;
}

export interface LessonInStory {
  lesson: Lesson;
  relevanceScore: number;
  explanation?: string;
}

export interface StoryWithDetails {
  story: Story;
  characters: CharacterInStory[];
  lessons: LessonInStory[];
  sources: StorySource[];
}

export interface StorySearchResponse {
  results: StorySearchResult[];
  totalCount: number;
  query: string;
  searchType: string;
  searchTimeMs: number;
}

export interface StorySearchResult {
  story: Story;
  characters: Character[];
  mainLessons: string[];
  relevanceScore: number;
  highlightedText: string;
  matchingCriteria: string[];
}

export interface PaginationOptions {
  limit?: number;
  offset?: number;
}

export interface StoryFilters {
  categories?: StoryCategory[];
  ageGroups?: AgeGroup[];
  authenticityLevels?: AuthenticityLevel[];
  timePeriods?: TimePeriod[];
  themes?: string[];
}

export interface CharacterFilterOptions extends PaginationOptions {
  characterType?: CharacterType;
  includeRelated?: boolean;
}

export interface ThemeFilterOptions extends PaginationOptions {
  lessonType?: LessonType;
  moralCategory?: MoralCategory;
  ageGroup?: AgeGroup;
}

export interface CharacterFilters extends PaginationOptions {
  characterType?: CharacterType;
  historicalPeriod?: TimePeriod;
}

// Helper functions for Arabic names and display

export const getCategoryArabicName = (category: StoryCategory): string => {
  const names: Record<StoryCategory, string> = {
    [StoryCategory.PROPHETS]: 'قصص الأنبياء',
    [StoryCategory.COMPANIONS]: 'قصص الصحابة',
    [StoryCategory.RIGHTEOUS_PREDECESSORS]: 'قصص السلف الصالح',
    [StoryCategory.HISTORICAL_EVENTS]: 'الأحداث التاريخية',
    [StoryCategory.MORAL_LESSONS]: 'العبر والمواعظ',
    [StoryCategory.MIRACLES]: 'المعجزات',
    [StoryCategory.BATTLES]: 'الغزوات والمعارك',
    [StoryCategory.CONVERSIONS]: 'قصص الإسلام',
    [StoryCategory.WOMEN_IN_ISLAM]: 'نساء في الإسلام',
    [StoryCategory.CHILDREN_STORIES]: 'قصص الأطفال',
  };
  return names[category];
};

export const getCategoryIcon = (category: StoryCategory): string => {
  const icons: Record<StoryCategory, string> = {
    [StoryCategory.PROPHETS]: '📖',
    [StoryCategory.COMPANIONS]: '👥',
    [StoryCategory.RIGHTEOUS_PREDECESSORS]: '⭐',
    [StoryCategory.HISTORICAL_EVENTS]: '🏛️',
    [StoryCategory.MORAL_LESSONS]: '💡',
    [StoryCategory.MIRACLES]: '✨',
    [StoryCategory.BATTLES]: '⚔️',
    [StoryCategory.CONVERSIONS]: '🌟',
    [StoryCategory.WOMEN_IN_ISLAM]: '👩',
    [StoryCategory.CHILDREN_STORIES]: '🧒',
  };
  return icons[category];
};

export const getAuthenticityColor = (level: AuthenticityLevel): string => {
  const colors: Record<AuthenticityLevel, string> = {
    [AuthenticityLevel.AUTHENTIC]: '#28A745',
    [AuthenticityLevel.WELL_DOCUMENTED]: '#17A2B8',
    [AuthenticityLevel.PROBABLE]: '#FFC107',
    [AuthenticityLevel.TRADITIONAL]: '#FD7E14',
    [AuthenticityLevel.EDUCATIONAL]: '#6C757D',
  };
  return colors[level];
};

export const getAuthenticityArabicName = (level: AuthenticityLevel): string => {
  const names: Record<AuthenticityLevel, string> = {
    [AuthenticityLevel.AUTHENTIC]: 'صحيح',
    [AuthenticityLevel.WELL_DOCUMENTED]: 'موثق جيداً',
    [AuthenticityLevel.PROBABLE]: 'محتمل',
    [AuthenticityLevel.TRADITIONAL]: 'تراثي',
    [AuthenticityLevel.EDUCATIONAL]: 'تعليمي',
  };
  return names[level];
};

export const getAgeGroupArabicName = (ageGroup: AgeGroup): string => {
  const names: Record<AgeGroup, string> = {
    [AgeGroup.CHILDREN]: 'الأطفال',
    [AgeGroup.TEENAGERS]: 'المراهقون',
    [AgeGroup.YOUNG_ADULTS]: 'الشباب',
    [AgeGroup.ADULTS]: 'البالغون',
    [AgeGroup.ALL_AGES]: 'جميع الأعمار',
  };
  return names[ageGroup];
};

export const getMoralCategoryArabicName = (category: MoralCategory): string => {
  const names: Record<MoralCategory, string> = {
    [MoralCategory.PATIENCE]: 'الصبر',
    [MoralCategory.GRATITUDE]: 'الشكر',
    [MoralCategory.JUSTICE]: 'العدل',
    [MoralCategory.MERCY]: 'الرحمة',
    [MoralCategory.HONESTY]: 'الصدق',
    [MoralCategory.COURAGE]: 'الشجاعة',
    [MoralCategory.HUMILITY]: 'التواضع',
    [MoralCategory.FORGIVENESS]: 'المغفرة',
    [MoralCategory.PERSEVERANCE]: 'المثابرة',
    [MoralCategory.FAITH]: 'الإيمان',
  };
  return names[category];
};

export const getSourceTypeArabicName = (sourceType: SourceType): string => {
  const names: Record<SourceType, string> = {
    [SourceType.QURAN]: 'القرآن الكريم',
    [SourceType.HADITH]: 'الحديث النبوي',
    [SourceType.HISTORICAL_BOOK]: 'كتاب تاريخي',
    [SourceType.BIOGRAPHY]: 'سيرة',
    [SourceType.TAFSIR]: 'تفسير',
    [SourceType.SCHOLARLY_WORK]: 'عمل علمي',
  };
  return names[sourceType];
};

export const getCharacterTypeArabicName = (characterType: CharacterType): string => {
  const names: Record<CharacterType, string> = {
    [CharacterType.PROPHET]: 'نبي',
    [CharacterType.MESSENGER]: 'رسول',
    [CharacterType.COMPANION]: 'صحابي',
    [CharacterType.RIGHTEOUS_PERSON]: 'صالح',
    [CharacterType.SCHOLAR]: 'عالم',
    [CharacterType.RULER]: 'حاكم',
    [CharacterType.MARTYR]: 'شهيد',
    [CharacterType.CONVERT]: 'مسلم جديد',
    [CharacterType.HISTORICAL_FIGURE]: 'شخصية تاريخية',
    [CharacterType.ANTAGONIST]: 'معارض',
  };
  return names[characterType];
};
