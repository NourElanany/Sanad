/**
 * Type definitions for Quran-related data structures
 */

export interface Surah {
  number: number;
  name_arabic: string;
  name_english: string;
  name_transliteration: string;
  ayah_count: number;
  revelation_type: 'Meccan' | 'Medinan';
  revelation_order: number;
  juz_start: number;
  juz_end: number;
  page_start: number;
  page_end: number;
}

export interface Juz {
  number: number;
  start_surah: number;
  start_ayah: number;
  end_surah: number;
  end_ayah: number;
  page_start: number;
  page_end: number;
}

export interface QuranBookmark {
  id: string;
  surah_number: number;
  ayah_number: number;
  page_number: number;
  note?: string;
  created_at: string;
}

export interface ReadingProgress {
  surah_number: number;
  ayah_number: number;
  page_number: number;
  last_read_at: string;
}

export interface Ayah {
  number: number;
  number_in_surah: number;
  surah_number: number;
  text_arabic: string;
  text_uthmani: string;
  juz_number: number;
  hizb_number: number;
  page_number: number;
  manzil_number: number;
  ruku_number: number;
}

export interface QuranPage {
  page_number: number;
  ayahs: Ayah[];
  juz_number: number;
  surah_number: number;
  surah_name: string;
}
