// Common Types for Sanad Application

export interface User {
  id: string
  name: string
  email: string
  avatar?: string
  preferences: UserPreferences
}

export interface UserPreferences {
  language: 'ar' | 'en'
  theme: 'light' | 'dark'
  madhab: 'hanafi' | 'maliki' | 'shafii' | 'hanbali'
  fontSize: 'small' | 'medium' | 'large'
  notifications: boolean
}

export interface Surah {
  number: number
  name: string
  englishName: string
  numberOfAyahs: number
  revelationType: 'Meccan' | 'Medinan'
}

export interface Ayah {
  number: number
  text: string
  surah: number
  numberInSurah: number
  juz: number
  page: number
}

export interface Hadith {
  id: string
  text: string
  narrator: string
  collection: string
  book: string
  chapter: string
  grade: 'Sahih' | 'Hasan' | 'Daif'
}

export interface PrayerTimes {
  fajr: string
  sunrise: string
  dhuhr: string
  asr: string
  maghrib: string
  isha: string
  date: string
  hijriDate: string
}

export interface Location {
  latitude: number
  longitude: number
  city: string
  country: string
}

export interface AIMessage {
  id: string
  role: 'user' | 'assistant'
  content: string
  sources?: Source[]
  timestamp: Date
}

export interface Source {
  type: 'quran' | 'hadith' | 'fatwa'
  reference: string
  text: string
  url?: string
}

export interface RecitationAnalysis {
  id: string
  audioUrl: string
  surah: number
  ayahStart: number
  ayahEnd: number
  errors: TajweedError[]
  score: number
  timestamp: Date
}

export interface TajweedError {
  type: 'ikhfa' | 'iqlab' | 'madd' | 'ghunna' | 'other'
  position: number
  description: string
  severity: 'low' | 'medium' | 'high'
}

export interface Khatma {
  id: string
  userId: string
  startDate: Date
  endDate?: Date
  targetDate: Date
  progress: number
  completedPages: number[]
  totalPages: number
}

export interface Story {
  id: string
  title: string
  category: 'prophets' | 'companions' | 'scholars'
  content: string
  lessons: string[]
  references: string[]
}

export interface ApiResponse<T> {
  success: boolean
  data?: T
  error?: string
  message?: string
}

export interface PaginatedResponse<T> {
  data: T[]
  total: number
  page: number
  pageSize: number
  hasMore: boolean
}

// Re-export Quran types
export * from './quran';
