import { axiosClient } from '../api/axios-client';
import { API_ENDPOINTS } from '../api/endpoints';

export interface PrayerTimes {
  fajr: string;
  sunrise: string;
  dhuhr: string;
  asr: string;
  maghrib: string;
  isha: string;
  date: string;
  location: string;
}

export interface HijriDate {
  day: number;
  month: number;
  year: number;
  monthName: string;
  weekday: string;
}

export interface NextPrayer {
  name: string;
  time: string;
  timeRemaining: {
    hours: number;
    minutes: number;
    seconds: number;
  };
}

export class PrayerTimesService {
  /**
   * Get prayer times for a specific location
   */
  static async getPrayerTimes(
    latitude: number,
    longitude: number,
    madhab?: string
  ): Promise<PrayerTimes> {
    const response = await axiosClient.get<PrayerTimes>(
      API_ENDPOINTS.PRAYER_TIMES,
      {
        params: {
          latitude,
          longitude,
          ...(madhab && { madhab }),
        },
      }
    );
    return response.data;
  }

  /**
   * Get Hijri date for today
   */
  static async getHijriDate(): Promise<HijriDate> {
    const response = await axiosClient.get<HijriDate>(
      API_ENDPOINTS.HIJRI_DATE
    );
    return response.data;
  }

  /**
   * Get monthly prayer times
   */
  static async getMonthlyPrayerTimes(
    latitude: number,
    longitude: number,
    month: number,
    year: number,
    madhab?: string
  ): Promise<PrayerTimes[]> {
    const response = await axiosClient.get<PrayerTimes[]>(
      API_ENDPOINTS.MONTHLY_PRAYER_TIMES,
      {
        params: {
          latitude,
          longitude,
          month,
          year,
          ...(madhab && { madhab }),
        },
      }
    );
    return response.data;
  }

  /**
   * Calculate next prayer and time remaining
   */
  static getNextPrayer(prayerTimes: PrayerTimes): NextPrayer {
    const now = new Date();
    const prayers = [
      { name: 'الفجر', time: prayerTimes.fajr },
      { name: 'الشروق', time: prayerTimes.sunrise },
      { name: 'الظهر', time: prayerTimes.dhuhr },
      { name: 'العصر', time: prayerTimes.asr },
      { name: 'المغرب', time: prayerTimes.maghrib },
      { name: 'العشاء', time: prayerTimes.isha },
    ];

    for (const prayer of prayers) {
      const prayerTime = this.parseTime(prayer.time);
      if (prayerTime > now) {
        const diff = prayerTime.getTime() - now.getTime();
        return {
          name: prayer.name,
          time: prayer.time,
          timeRemaining: {
            hours: Math.floor(diff / (1000 * 60 * 60)),
            minutes: Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60)),
            seconds: Math.floor((diff % (1000 * 60)) / 1000),
          },
        };
      }
    }

    // If all prayers passed, return Fajr of next day
    const fajrTime = this.parseTime(prayerTimes.fajr);
    fajrTime.setDate(fajrTime.getDate() + 1);
    const diff = fajrTime.getTime() - now.getTime();

    return {
      name: 'الفجر',
      time: prayerTimes.fajr,
      timeRemaining: {
        hours: Math.floor(diff / (1000 * 60 * 60)),
        minutes: Math.floor((diff % (1000 * 60 * 60)) / (1000 * 60)),
        seconds: Math.floor((diff % (1000 * 60)) / 1000),
      },
    };
  }

  /**
   * Parse time string to Date object
   */
  private static parseTime(time: string): Date {
    const [hours, minutes] = time.split(':').map(Number);
    const date = new Date();
    date.setHours(hours, minutes, 0, 0);
    return date;
  }

  /**
   * Format Hijri date
   */
  static formatHijriDate(hijriDate: HijriDate): string {
    return `${hijriDate.weekday}، ${hijriDate.day} ${hijriDate.monthName} ${hijriDate.year} هـ`;
  }
}
