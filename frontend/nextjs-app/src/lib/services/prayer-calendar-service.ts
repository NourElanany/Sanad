import { axiosClient } from '../api/axios-client';
import { API_ENDPOINTS } from '../api/endpoints';
import type {
  MonthlyCalendar,
  CalendarDay,
  IslamicEvent,
  CalendarExportOptions,
} from '@/types/prayer-calendar';

export class PrayerCalendarService {
  /**
   * Get monthly prayer times calendar
   */
  static async getMonthlyCalendar(
    latitude: number,
    longitude: number,
    hijriYear: number,
    hijriMonth: number,
    calculationMethod?: string
  ): Promise<MonthlyCalendar> {
    const response = await axiosClient.get<{ data: MonthlyCalendar }>(
      `${API_ENDPOINTS.PRAYER_CALENDAR}/${hijriYear}/${hijriMonth}`,
      {
        params: {
          latitude,
          longitude,
          ...(calculationMethod && { method: calculationMethod }),
        },
      }
    );
    return response.data.data;
  }

  /**
   * Get prayer times for a date range
   */
  static async getPrayerTimesRange(
    latitude: number,
    longitude: number,
    startDate: string,
    endDate: string,
    calculationMethod?: string
  ): Promise<CalendarDay[]> {
    const response = await axiosClient.get<{ data: CalendarDay[] }>(
      API_ENDPOINTS.PRAYER_TIMES_RANGE,
      {
        params: {
          latitude,
          longitude,
          start_date: startDate,
          end_date: endDate,
          ...(calculationMethod && { method: calculationMethod }),
        },
      }
    );
    return response.data.data;
  }

  /**
   * Get Islamic events for a specific month
   */
  static async getIslamicEvents(
    hijriMonth?: number,
    hijriYear?: number,
    importanceLevel?: number
  ): Promise<IslamicEvent[]> {
    const response = await axiosClient.get<{ data: IslamicEvent[] }>(
      API_ENDPOINTS.ISLAMIC_EVENTS,
      {
        params: {
          ...(hijriMonth && { hijri_month: hijriMonth }),
          ...(hijriYear && { hijri_year: hijriYear }),
          ...(importanceLevel && { importance_level: importanceLevel }),
        },
      }
    );
    return response.data.data;
  }

  /**
   * Export calendar to iCal format
   */
  static async exportCalendarToICal(
    latitude: number,
    longitude: number,
    hijriYear: number,
    hijriMonth: number
  ): Promise<string> {
    const response = await axiosClient.get<string>(
      `${API_ENDPOINTS.PRAYER_CALENDAR}/${hijriYear}/${hijriMonth}/export`,
      {
        params: {
          latitude,
          longitude,
          format: 'ical',
        },
        responseType: 'text',
      }
    );
    return response.data;
  }

  /**
   * Get shareable calendar link
   */
  static async getShareableLink(
    latitude: number,
    longitude: number,
    hijriYear: number,
    hijriMonth: number
  ): Promise<string> {
    const response = await axiosClient.post<{ data: { share_url: string } }>(
      `${API_ENDPOINTS.PRAYER_CALENDAR}/share`,
      {
        latitude,
        longitude,
        hijri_year: hijriYear,
        hijri_month: hijriMonth,
      }
    );
    return response.data.data.share_url;
  }

  /**
   * Download calendar as file
   */
  static async downloadCalendar(
    calendar: MonthlyCalendar,
    options: CalendarExportOptions
  ): Promise<Blob> {
    if (options.format === 'ical') {
      // For iCal, we already have the export endpoint
      const icalData = await this.exportCalendarToICal(
        0, // These would come from the calendar context
        0,
        calendar.hijriYear,
        calendar.hijriMonth.monthNumber
      );
      return new Blob([icalData], { type: 'text/calendar' });
    }

    // For other formats, implement client-side generation
    throw new Error(`Export format ${options.format} not yet implemented`);
  }

  /**
   * Helper: Check if a day is special (Friday or has events)
   */
  static isSpecialDay(day: CalendarDay): boolean {
    return day.isFriday || day.events.length > 0;
  }

  /**
   * Helper: Get day background color based on its properties
   */
  static getDayBackgroundColor(day: CalendarDay): string {
    if (day.isToday) return 'bg-primary-50 border-accent';
    if (day.isFriday) return 'bg-secondary-50';
    if (day.events.some((e) => e.eventType === 'eid')) return 'bg-green-50';
    if (day.events.length > 0) return 'bg-accent-50';
    return 'bg-white';
  }

  /**
   * Helper: Get day text color based on its properties
   */
  static getDayTextColor(day: CalendarDay): string {
    if (day.isToday) return 'text-primary';
    if (day.isFriday) return 'text-secondary';
    if (day.events.some((e) => e.eventType === 'eid')) return 'text-green-600';
    return 'text-gray-900';
  }

  /**
   * Helper: Format Hijri date
   */
  static formatHijriDate(hijriDate: HijriDate): string {
    return `${hijriDate.weekdayArabic}، ${hijriDate.day} ${hijriDate.monthNameArabic} ${hijriDate.year} هـ`;
  }

  /**
   * Helper: Format Gregorian date
   */
  static formatGregorianDate(dateString: string): string {
    const date = new Date(dateString);
    const months = [
      'يناير',
      'فبراير',
      'مارس',
      'أبريل',
      'مايو',
      'يونيو',
      'يوليو',
      'أغسطس',
      'سبتمبر',
      'أكتوبر',
      'نوفمبر',
      'ديسمبر',
    ];
    const weekdays = [
      'الأحد',
      'الاثنين',
      'الثلاثاء',
      'الأربعاء',
      'الخميس',
      'الجمعة',
      'السبت',
    ];

    return `${weekdays[date.getDay()]}، ${date.getDate()} ${
      months[date.getMonth()]
    } ${date.getFullYear()} م`;
  }
}
