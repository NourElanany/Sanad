export interface HijriDate {
  day: number;
  month: number;
  year: number;
  monthNameArabic: string;
  monthNameEnglish: string;
  weekdayArabic: string;
}

export interface PrayerTimes {
  fajr: string;
  sunrise: string;
  dhuhr: string;
  asr: string;
  maghrib: string;
  isha: string;
}

export interface IslamicEvent {
  id: string;
  nameArabic: string;
  nameEnglish: string;
  descriptionArabic?: string;
  descriptionEnglish?: string;
  eventType: string;
  importanceLevel: number;
}

export interface CalendarDay {
  gregorianDate: string;
  hijriDate: HijriDate;
  prayerTimes: PrayerTimes;
  events: IslamicEvent[];
  isFriday: boolean;
  isWeekend: boolean;
  isToday: boolean;
}

export interface HijriMonth {
  monthNumber: number;
  nameArabic: string;
  nameEnglish: string;
  nameTransliteration: string;
}

export interface MonthlyCalendar {
  hijriMonth: HijriMonth;
  hijriYear: number;
  days: CalendarDay[];
  events: IslamicEvent[];
}

export interface CalendarExportOptions {
  format: 'ical' | 'pdf' | 'image';
  includeEvents: boolean;
  includePrayerTimes: boolean;
}

export interface NotificationSettings {
  prayerName: string;
  enabled: boolean;
  minutesBefore: number;
  graduatedEnabled: boolean;
  graduatedIntervals: number[];
}
