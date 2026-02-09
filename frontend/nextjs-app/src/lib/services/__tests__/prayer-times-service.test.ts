import { describe, it, expect, jest, beforeEach } from '@jest/globals';
import axios from 'axios';
import { PrayerTimesService } from '../prayer-times-service';

jest.mock('axios');
const mockedAxios = axios as jest.Mocked<typeof axios>;

/**
 * Unit tests for Prayer Times Service
 * **Validates: Requirements 20.1**
 */
describe('PrayerTimesService', () => {
  let service: PrayerTimesService;

  beforeEach(() => {
    service = new PrayerTimesService();
    jest.clearAllMocks();
  });

  describe('getPrayerTimes', () => {
    it('should fetch prayer times for valid location', async () => {
      // Arrange
      const mockResponse = {
        data: {
          data: {
            fajr: '05:30',
            sunrise: '06:45',
            dhuhr: '12:30',
            asr: '15:45',
            maghrib: '18:15',
            isha: '19:30',
          },
        },
      };

      mockedAxios.get.mockResolvedValue(mockResponse);

      // Act
      const result = await service.getPrayerTimes({
        latitude: 24.7136,
        longitude: 46.6753,
        date: new Date('2024-01-15'),
      });

      // Assert
      expect(result.fajr).toBe('05:30');
      expect(result.dhuhr).toBe('12:30');
      expect(result.maghrib).toBe('18:15');
      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
    });

    it('should throw error for invalid coordinates', async () => {
      // Arrange
      mockedAxios.get.mockRejectedValue(new Error('Invalid coordinates'));

      // Act & Assert
      await expect(
        service.getPrayerTimes({
          latitude: 200, // Invalid
          longitude: 46.6753,
          date: new Date(),
        })
      ).rejects.toThrow('Invalid coordinates');
    });

    it('should cache prayer times for same location and date', async () => {
      // Arrange
      const mockResponse = {
        data: {
          data: {
            fajr: '05:30',
            dhuhr: '12:30',
            asr: '15:45',
            maghrib: '18:15',
            isha: '19:30',
          },
        },
      };

      mockedAxios.get.mockResolvedValue(mockResponse);

      const params = {
        latitude: 24.7136,
        longitude: 46.6753,
        date: new Date('2024-01-15'),
      };

      // Act
      await service.getPrayerTimes(params);
      await service.getPrayerTimes(params);

      // Assert - Should only call API once due to caching
      expect(mockedAxios.get).toHaveBeenCalledTimes(1);
    });

    it('should handle network errors gracefully', async () => {
      // Arrange
      mockedAxios.get.mockRejectedValue(new Error('Network error'));

      // Act & Assert
      await expect(
        service.getPrayerTimes({
          latitude: 24.7136,
          longitude: 46.6753,
          date: new Date(),
        })
      ).rejects.toThrow('Network error');
    });
  });

  describe('getNextPrayer', () => {
    it('should return next prayer correctly', () => {
      // Arrange
      const prayerTimes = {
        fajr: '05:30',
        sunrise: '06:45',
        dhuhr: '12:30',
        asr: '15:45',
        maghrib: '18:15',
        isha: '19:30',
        date: new Date('2024-01-15'),
      };

      const currentTime = new Date('2024-01-15T14:00:00');

      // Act
      const nextPrayer = service.getNextPrayer(prayerTimes, currentTime);

      // Assert
      expect(nextPrayer.name).toBe('asr');
      expect(nextPrayer.time).toBe('15:45');
    });

    it('should return fajr as next prayer after isha', () => {
      // Arrange
      const prayerTimes = {
        fajr: '05:30',
        sunrise: '06:45',
        dhuhr: '12:30',
        asr: '15:45',
        maghrib: '18:15',
        isha: '19:30',
        date: new Date('2024-01-15'),
      };

      const currentTime = new Date('2024-01-15T20:00:00');

      // Act
      const nextPrayer = service.getNextPrayer(prayerTimes, currentTime);

      // Assert
      expect(nextPrayer.name).toBe('fajr');
      expect(nextPrayer.isNextDay).toBe(true);
    });

    it('should handle edge case at exact prayer time', () => {
      // Arrange
      const prayerTimes = {
        fajr: '05:30',
        sunrise: '06:45',
        dhuhr: '12:30',
        asr: '15:45',
        maghrib: '18:15',
        isha: '19:30',
        date: new Date('2024-01-15'),
      };

      const currentTime = new Date('2024-01-15T12:30:00');

      // Act
      const nextPrayer = service.getNextPrayer(prayerTimes, currentTime);

      // Assert
      expect(nextPrayer.name).toBe('asr');
    });
  });

  describe('calculateTimeRemaining', () => {
    it('should calculate time remaining correctly', () => {
      // Arrange
      const prayerTime = new Date('2024-01-15T15:45:00');
      const currentTime = new Date('2024-01-15T14:30:00');

      // Act
      const remaining = service.calculateTimeRemaining(prayerTime, currentTime);

      // Assert
      expect(remaining.hours).toBe(1);
      expect(remaining.minutes).toBe(15);
      expect(remaining.seconds).toBe(0);
    });

    it('should return zero for past prayer times', () => {
      // Arrange
      const prayerTime = new Date('2024-01-15T12:00:00');
      const currentTime = new Date('2024-01-15T14:00:00');

      // Act
      const remaining = service.calculateTimeRemaining(prayerTime, currentTime);

      // Assert
      expect(remaining.hours).toBe(0);
      expect(remaining.minutes).toBe(0);
      expect(remaining.seconds).toBe(0);
    });

    it('should handle seconds correctly', () => {
      // Arrange
      const prayerTime = new Date('2024-01-15T15:45:30');
      const currentTime = new Date('2024-01-15T15:44:00');

      // Act
      const remaining = service.calculateTimeRemaining(prayerTime, currentTime);

      // Assert
      expect(remaining.minutes).toBe(1);
      expect(remaining.seconds).toBe(30);
    });
  });

  describe('getMonthlyPrayerTimes', () => {
    it('should fetch prayer times for entire month', async () => {
      // Arrange
      const mockResponse = {
        data: {
          data: Array.from({ length: 30 }, (_, i) => ({
            date: new Date(2024, 0, i + 1).toISOString(),
            fajr: '05:30',
            dhuhr: '12:30',
            asr: '15:45',
            maghrib: '18:15',
            isha: '19:30',
          })),
        },
      };

      mockedAxios.get.mockResolvedValue(mockResponse);

      // Act
      const result = await service.getMonthlyPrayerTimes({
        latitude: 24.7136,
        longitude: 46.6753,
        year: 2024,
        month: 1,
      });

      // Assert
      expect(result).toHaveLength(30);
      expect(result[0].date.getDate()).toBe(1);
      expect(result[29].date.getDate()).toBe(30);
    });

    it('should handle February correctly', async () => {
      // Arrange
      const mockResponse = {
        data: {
          data: Array.from({ length: 29 }, (_, i) => ({
            date: new Date(2024, 1, i + 1).toISOString(),
            fajr: '05:30',
            dhuhr: '12:30',
            asr: '15:45',
            maghrib: '18:15',
            isha: '19:30',
          })),
        },
      };

      mockedAxios.get.mockResolvedValue(mockResponse);

      // Act
      const result = await service.getMonthlyPrayerTimes({
        latitude: 24.7136,
        longitude: 46.6753,
        year: 2024,
        month: 2,
      });

      // Assert
      expect(result).toHaveLength(29); // 2024 is a leap year
    });
  });

  describe('formatPrayerTime', () => {
    it('should format time in 12-hour format', () => {
      // Act
      const formatted = service.formatPrayerTime('15:45');

      // Assert
      expect(formatted).toBe('3:45 PM');
    });

    it('should format morning times correctly', () => {
      // Act
      const formatted = service.formatPrayerTime('05:30');

      // Assert
      expect(formatted).toBe('5:30 AM');
    });

    it('should handle midnight correctly', () => {
      // Act
      const formatted = service.formatPrayerTime('00:00');

      // Assert
      expect(formatted).toBe('12:00 AM');
    });

    it('should handle noon correctly', () => {
      // Act
      const formatted = service.formatPrayerTime('12:00');

      // Assert
      expect(formatted).toBe('12:00 PM');
    });
  });
});
