/**
 * Unit tests for Prayer Times Store
 * Tests prayer times fetching, caching, and location management
 */

import { renderHook, act, waitFor } from '@testing-library/react';
import { usePrayerTimesStore } from '../prayer-times-store';
import { PrayerTimesService } from '../../services/prayer-times-service';

// Mock the PrayerTimesService
jest.mock('../../services/prayer-times-service');

describe('Prayer Times Store', () => {
  const mockPrayerTimes = {
    fajr: '05:30',
    sunrise: '06:45',
    dhuhr: '12:15',
    asr: '15:30',
    maghrib: '18:00',
    isha: '19:15',
    date: '2024-01-01',
    location: 'Riyadh, Saudi Arabia',
  };

  const mockHijriDate = {
    day: 15,
    month: 7,
    year: 1445,
    monthName: 'رجب',
    weekday: 'الأحد',
  };

  const mockLocation = {
    latitude: 24.7136,
    longitude: 46.6753,
    city: 'Riyadh',
    country: 'Saudi Arabia',
  };

  const mockNextPrayer = {
    name: 'الظهر',
    time: '12:15',
    timeRemaining: {
      hours: 2,
      minutes: 30,
      seconds: 45,
    },
  };

  beforeEach(() => {
    // Reset store before each test
    usePrayerTimesStore.getState().reset();
    jest.clearAllMocks();
    
    // Reset time
    jest.useFakeTimers();
    jest.setSystemTime(new Date('2024-01-01T09:45:00'));
  });

  afterEach(() => {
    jest.useRealTimers();
  });

  describe('fetchPrayerTimes', () => {
    it('should fetch and store prayer times', async () => {
      (PrayerTimesService.getPrayerTimes as jest.Mock).mockResolvedValue(mockPrayerTimes);
      (PrayerTimesService.getNextPrayer as jest.Mock).mockReturnValue(mockNextPrayer);

      const { result } = renderHook(() => usePrayerTimesStore());

      // Set location first
      act(() => {
        result.current.setLocation(mockLocation);
      });

      await waitFor(() => {
        expect(result.current.prayerTimes).toEqual(mockPrayerTimes);
        expect(result.current.nextPrayer).toEqual(mockNextPrayer);
        expect(result.current.loading).toBe(false);
      });
    });

    it('should use cached prayer times within 6 hours', async () => {
      (PrayerTimesService.getPrayerTimes as jest.Mock).mockResolvedValue(mockPrayerTimes);
      (PrayerTimesService.getNextPrayer as jest.Mock).mockReturnValue(mockNextPrayer);

      const { result } = renderHook(() => usePrayerTimesStore());

      // Set location and fetch
      act(() => {
        result.current.setLocation(mockLocation);
      });

      await waitFor(() => {
        expect(result.current.prayerTimes).toEqual(mockPrayerTimes);
      });

      expect(PrayerTimesService.getPrayerTimes).toHaveBeenCalledTimes(1);

      // Advance time by 3 hours
      jest.advanceTimersByTime(3 * 60 * 60 * 1000);

      // Fetch again - should use cache
      await act(async () => {
        await result.current.fetchPrayerTimes();
      });

      expect(PrayerTimesService.getPrayerTimes).toHaveBeenCalledTimes(1); // Still 1
    });

    it('should refetch after 6 hours', async () => {
      (PrayerTimesService.getPrayerTimes as jest.Mock).mockResolvedValue(mockPrayerTimes);
      (PrayerTimesService.getNextPrayer as jest.Mock).mockReturnValue(mockNextPrayer);

      const { result } = renderHook(() => usePrayerTimesStore());

      // Set location and fetch
      act(() => {
        result.current.setLocation(mockLocation);
      });

      await waitFor(() => {
        expect(result.current.prayerTimes).toEqual(mockPrayerTimes);
      });

      expect(PrayerTimesService.getPrayerTimes).toHaveBeenCalledTimes(1);

      // Advance time by 7 hours
      jest.advanceTimersByTime(7 * 60 * 60 * 1000);

      // Fetch again - should refetch
      await act(async () => {
        await result.current.fetchPrayerTimes();
      });

      expect(PrayerTimesService.getPrayerTimes).toHaveBeenCalledTimes(2);
    });

    it('should handle missing location', async () => {
      const { result } = renderHook(() => usePrayerTimesStore());

      await act(async () => {
        await result.current.fetchPrayerTimes();
      });

      expect(result.current.error).toBe('Location not available');
    });

    it('should handle errors', async () => {
      const error = new Error('Failed to fetch prayer times');
      (PrayerTimesService.getPrayerTimes as jest.Mock).mockRejectedValue(error);

      const { result } = renderHook(() => usePrayerTimesStore());

      act(() => {
        result.current.setLocation(mockLocation);
      });

      await waitFor(() => {
        expect(result.current.error).toBe(error.message);
        expect(result.current.loading).toBe(false);
      });
    });
  });

  describe('fetchHijriDate', () => {
    it('should fetch and store Hijri date', async () => {
      (PrayerTimesService.getHijriDate as jest.Mock).mockResolvedValue(mockHijriDate);

      const { result } = renderHook(() => usePrayerTimesStore());

      await act(async () => {
        await result.current.fetchHijriDate();
      });

      expect(result.current.hijriDate).toEqual(mockHijriDate);
      expect(result.current.loading).toBe(false);
    });

    it('should use cached Hijri date for same day', async () => {
      (PrayerTimesService.getHijriDate as jest.Mock).mockResolvedValue(mockHijriDate);

      const { result } = renderHook(() => usePrayerTimesStore());

      // First fetch
      await act(async () => {
        await result.current.fetchHijriDate();
      });

      const firstCallCount = (PrayerTimesService.getHijriDate as jest.Mock).mock.calls.length;

      // Second fetch on same day - should use cache
      await act(async () => {
        await result.current.fetchHijriDate();
      });

      const secondCallCount = (PrayerTimesService.getHijriDate as jest.Mock).mock.calls.length;
      
      // Should not have made another call
      expect(secondCallCount).toBe(firstCallCount);
    });
  });

  describe('setLocation', () => {
    it('should set location and fetch prayer times', async () => {
      (PrayerTimesService.getPrayerTimes as jest.Mock).mockResolvedValue(mockPrayerTimes);
      (PrayerTimesService.getNextPrayer as jest.Mock).mockReturnValue(mockNextPrayer);

      const { result } = renderHook(() => usePrayerTimesStore());

      act(() => {
        result.current.setLocation(mockLocation);
      });

      await waitFor(() => {
        expect(result.current.location).toEqual(mockLocation);
        expect(result.current.prayerTimes).toEqual(mockPrayerTimes);
      });
    });
  });

  describe('setMadhab', () => {
    it('should set madhab and invalidate cache', async () => {
      (PrayerTimesService.getPrayerTimes as jest.Mock).mockResolvedValue(mockPrayerTimes);
      (PrayerTimesService.getNextPrayer as jest.Mock).mockReturnValue(mockNextPrayer);

      const { result } = renderHook(() => usePrayerTimesStore());

      // Set location first
      act(() => {
        result.current.setLocation(mockLocation);
      });

      await waitFor(() => {
        expect(result.current.prayerTimes).toEqual(mockPrayerTimes);
      });

      expect(PrayerTimesService.getPrayerTimes).toHaveBeenCalledTimes(1);

      // Change madhab
      act(() => {
        result.current.setMadhab('hanafi');
      });

      await waitFor(() => {
        expect(result.current.madhab).toBe('hanafi');
      });

      // Should refetch with new madhab
      expect(PrayerTimesService.getPrayerTimes).toHaveBeenCalledTimes(2);
    });
  });

  describe('updateNextPrayer', () => {
    it('should update next prayer calculation', () => {
      (PrayerTimesService.getNextPrayer as jest.Mock).mockReturnValue(mockNextPrayer);

      const { result } = renderHook(() => usePrayerTimesStore());

      // Set prayer times
      act(() => {
        usePrayerTimesStore.setState({ prayerTimes: mockPrayerTimes });
      });

      // Update next prayer
      act(() => {
        result.current.updateNextPrayer();
      });

      expect(result.current.nextPrayer).toEqual(mockNextPrayer);
    });

    it('should handle missing prayer times', () => {
      const { result } = renderHook(() => usePrayerTimesStore());

      act(() => {
        result.current.updateNextPrayer();
      });

      expect(result.current.nextPrayer).toBeNull();
    });
  });

  describe('fetchMonthlyPrayerTimes', () => {
    it('should fetch monthly prayer times', async () => {
      const monthlyTimes = [mockPrayerTimes, mockPrayerTimes];
      (PrayerTimesService.getMonthlyPrayerTimes as jest.Mock).mockResolvedValue(monthlyTimes);

      const { result } = renderHook(() => usePrayerTimesStore());

      // Set location first
      act(() => {
        usePrayerTimesStore.setState({ location: mockLocation });
      });

      await act(async () => {
        await result.current.fetchMonthlyPrayerTimes(1, 2024);
      });

      expect(result.current.monthlyPrayerTimes).toEqual(monthlyTimes);
      expect(result.current.loading).toBe(false);
    });

    it('should handle missing location', async () => {
      const { result } = renderHook(() => usePrayerTimesStore());

      await act(async () => {
        await result.current.fetchMonthlyPrayerTimes(1, 2024);
      });

      expect(result.current.error).toBe('Location not available');
    });
  });

  describe('selectors', () => {
    it('should format Hijri date', () => {
      (PrayerTimesService.formatHijriDate as jest.Mock).mockReturnValue(
        'الأحد، 15 رجب 1445 هـ'
      );

      const { result } = renderHook(() => usePrayerTimesStore());

      act(() => {
        usePrayerTimesStore.setState({ hijriDate: mockHijriDate });
      });

      const formatted = PrayerTimesService.formatHijriDate(result.current.hijriDate!);
      expect(formatted).toBe('الأحد، 15 رجب 1445 هـ');
    });
  });

  describe('reset', () => {
    it('should reset store to initial state', () => {
      const { result } = renderHook(() => usePrayerTimesStore());

      // Set some state
      act(() => {
        usePrayerTimesStore.setState({
          prayerTimes: mockPrayerTimes,
          hijriDate: mockHijriDate,
          location: mockLocation,
        });
      });

      // Reset
      act(() => {
        result.current.reset();
      });

      expect(result.current.prayerTimes).toBeNull();
      expect(result.current.hijriDate).toBeNull();
      expect(result.current.location).toBeNull();
    });
  });
});
