/**
 * Zustand Store for Prayer Times State Management
 * Handles prayer times, location, notifications, and Hijri calendar
 * 
 * Requirements: 19.1, 19.2, 19.3, 19.4, 19.5
 */

import { create } from 'zustand';
import { persist, createJSONStorage } from 'zustand/middleware';
import { devtools } from 'zustand/middleware';
import { PrayerTimesService, type PrayerTimes, type HijriDate, type NextPrayer } from '../services/prayer-times-service';

// ============================================================================
// Types
// ============================================================================

interface Location {
  latitude: number;
  longitude: number;
  city?: string;
  country?: string;
}

interface PrayerTimesState {
  // Data
  prayerTimes: PrayerTimes | null;
  hijriDate: HijriDate | null;
  nextPrayer: NextPrayer | null;
  location: Location | null;
  madhab: string;
  monthlyPrayerTimes: PrayerTimes[];
  
  // UI State
  loading: boolean;
  error: string | null;
  
  // Cache
  lastFetchTime: number | null;
  
  // Actions
  fetchPrayerTimes: (latitude?: number, longitude?: number) => Promise<void>;
  fetchHijriDate: () => Promise<void>;
  fetchMonthlyPrayerTimes: (month: number, year: number) => Promise<void>;
  setLocation: (location: Location) => void;
  setMadhab: (madhab: string) => void;
  updateNextPrayer: () => void;
  
  // Utility
  clearError: () => void;
  reset: () => void;
}

// ============================================================================
// Initial State
// ============================================================================

const initialState = {
  prayerTimes: null,
  hijriDate: null,
  nextPrayer: null,
  location: null,
  madhab: 'shafi',
  monthlyPrayerTimes: [],
  loading: false,
  error: null,
  lastFetchTime: null,
};

// ============================================================================
// Store Implementation
// ============================================================================

export const usePrayerTimesStore = create<PrayerTimesState>()(
  devtools(
    persist(
      (set, get) => ({
        ...initialState,

        // Fetch prayer times
        fetchPrayerTimes: async (latitude?: number, longitude?: number) => {
          const state = get();
          
          // Use provided coordinates or stored location
          const lat = latitude ?? state.location?.latitude;
          const lon = longitude ?? state.location?.longitude;
          
          if (!lat || !lon) {
            set({ error: 'Location not available' });
            return;
          }

          // Check cache (refresh every 6 hours)
          const now = Date.now();
          const sixHours = 6 * 60 * 60 * 1000;
          if (
            state.prayerTimes &&
            state.lastFetchTime &&
            now - state.lastFetchTime < sixHours
          ) {
            // Update next prayer calculation
            get().updateNextPrayer();
            return;
          }

          set({ loading: true, error: null });
          try {
            const times = await PrayerTimesService.getPrayerTimes(
              lat,
              lon,
              state.madhab
            );
            
            const nextPrayer = PrayerTimesService.getNextPrayer(times);
            
            set({
              prayerTimes: times,
              nextPrayer,
              lastFetchTime: now,
              loading: false,
            });
          } catch (error: any) {
            set({ error: error.message, loading: false });
          }
        },

        // Fetch Hijri date
        fetchHijriDate: async () => {
          // Check cache (refresh daily)
          const state = get();
          if (state.hijriDate) {
            const today = new Date().toDateString();
            const cachedDate = new Date(state.hijriDate.day).toDateString();
            if (today === cachedDate) {
              return;
            }
          }

          set({ loading: true, error: null });
          try {
            const hijriDate = await PrayerTimesService.getHijriDate();
            set({ hijriDate, loading: false });
          } catch (error: any) {
            set({ error: error.message, loading: false });
          }
        },

        // Fetch monthly prayer times
        fetchMonthlyPrayerTimes: async (month: number, year: number) => {
          const state = get();
          const lat = state.location?.latitude;
          const lon = state.location?.longitude;
          
          if (!lat || !lon) {
            set({ error: 'Location not available' });
            return;
          }

          set({ loading: true, error: null });
          try {
            const times = await PrayerTimesService.getMonthlyPrayerTimes(
              lat,
              lon,
              month,
              year,
              state.madhab
            );
            set({ monthlyPrayerTimes: times, loading: false });
          } catch (error: any) {
            set({ error: error.message, loading: false });
          }
        },

        // Set location and fetch prayer times
        setLocation: (location: Location) => {
          set({ location });
          // Automatically fetch prayer times for new location
          get().fetchPrayerTimes(location.latitude, location.longitude);
        },

        // Set madhab and refetch prayer times
        setMadhab: (madhab: string) => {
          set({ madhab, lastFetchTime: null }); // Invalidate cache
          // Refetch with new madhab
          const state = get();
          if (state.location) {
            get().fetchPrayerTimes();
          }
        },

        // Update next prayer calculation (call this every minute)
        updateNextPrayer: () => {
          const state = get();
          if (state.prayerTimes) {
            const nextPrayer = PrayerTimesService.getNextPrayer(state.prayerTimes);
            set({ nextPrayer });
          }
        },

        // Clear error
        clearError: () => set({ error: null }),

        // Reset store
        reset: () => set(initialState),
      }),
      {
        name: 'prayer-times-storage',
        storage: createJSONStorage(() => localStorage),
        // Persist essential data
        partialize: (state) => ({
          prayerTimes: state.prayerTimes,
          hijriDate: state.hijriDate,
          location: state.location,
          madhab: state.madhab,
          lastFetchTime: state.lastFetchTime,
        }),
      }
    ),
    {
      name: 'PrayerTimesStore',
    }
  )
);

// ============================================================================
// Selectors
// ============================================================================

export const selectPrayerTimes = (state: PrayerTimesState) => state.prayerTimes;
export const selectHijriDate = (state: PrayerTimesState) => state.hijriDate;
export const selectNextPrayer = (state: PrayerTimesState) => state.nextPrayer;
export const selectLocation = (state: PrayerTimesState) => state.location;
export const selectMadhab = (state: PrayerTimesState) => state.madhab;
export const selectMonthlyPrayerTimes = (state: PrayerTimesState) => state.monthlyPrayerTimes;
export const selectLoading = (state: PrayerTimesState) => state.loading;
export const selectError = (state: PrayerTimesState) => state.error;

// Formatted selectors
export const selectFormattedHijriDate = (state: PrayerTimesState) => {
  if (!state.hijriDate) return null;
  return PrayerTimesService.formatHijriDate(state.hijriDate);
};

export const selectTimeUntilNextPrayer = (state: PrayerTimesState) => {
  if (!state.nextPrayer) return null;
  const { hours, minutes, seconds } = state.nextPrayer.timeRemaining;
  return `${hours.toString().padStart(2, '0')}:${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
};
