/**
 * Store Initializer Component
 * Initializes all Zustand stores on app mount
 * Handles hydration and synchronization
 * 
 * Requirements: 19.5 - State synchronization between pages
 */

'use client';

import { useEffect, useRef } from 'react';
import { useQuranStore } from './quran-store';
import { usePrayerTimesStore } from './prayer-times-store';
import { useAIAssistantStore } from './ai-assistant-store';
import { useSettingsStore } from './settings-store';

/**
 * Hook to initialize all stores
 * Call this in your root layout or app component
 */
export function useStoreInitializer() {
  const initialized = useRef(false);

  // Get store actions
  const loadPreferences = useSettingsStore((state) => state.loadPreferences);
  const updateLanguage = useSettingsStore((state) => state.updateLanguage);
  const language = useSettingsStore((state) => state.language);
  
  const fetchSurahs = useQuranStore((state) => state.fetchSurahs);
  const fetchJuzs = useQuranStore((state) => state.fetchJuzs);
  const fetchBookmarks = useQuranStore((state) => state.fetchBookmarks);
  const fetchReadingProgress = useQuranStore((state) => state.fetchReadingProgress);
  
  const fetchPrayerTimes = usePrayerTimesStore((state) => state.fetchPrayerTimes);
  const fetchHijriDate = usePrayerTimesStore((state) => state.fetchHijriDate);
  const updateNextPrayer = usePrayerTimesStore((state) => state.updateNextPrayer);
  const location = usePrayerTimesStore((state) => state.location);
  
  const loadSessions = useAIAssistantStore((state) => state.loadSessions);

  useEffect(() => {
    // Prevent double initialization in development mode
    if (initialized.current) return;
    initialized.current = true;

    // Initialize settings first
    loadPreferences();

    // Apply language settings
    if (typeof document !== 'undefined') {
      document.documentElement.dir = language === 'ar' ? 'rtl' : 'ltr';
      document.documentElement.lang = language;
    }

    // Initialize Quran data
    fetchSurahs();
    fetchJuzs();
    fetchBookmarks();
    fetchReadingProgress();

    // Initialize Prayer Times if location is available
    if (location) {
      fetchPrayerTimes();
      fetchHijriDate();
    }

    // Initialize AI Assistant sessions
    loadSessions();

    // Set up prayer time countdown updater (every minute)
    const prayerInterval = setInterval(() => {
      updateNextPrayer();
    }, 60000); // Update every minute

    // Clean up
    return () => {
      clearInterval(prayerInterval);
    };
  }, []);

  // Update language when it changes
  useEffect(() => {
    if (typeof document !== 'undefined') {
      document.documentElement.dir = language === 'ar' ? 'rtl' : 'ltr';
      document.documentElement.lang = language;
    }
  }, [language]);
}

/**
 * Store Initializer Component
 * Add this to your root layout
 */
export function StoreInitializer({ children }: { children: React.ReactNode }) {
  useStoreInitializer();
  return <>{children}</>;
}

/**
 * Hook to handle store hydration
 * Useful for preventing hydration mismatches in SSR
 */
export function useStoreHydration() {
  const [hydrated, setHydrated] = useState(false);

  useEffect(() => {
    setHydrated(true);
  }, []);

  return hydrated;
}

// Import useState
import { useState } from 'react';
