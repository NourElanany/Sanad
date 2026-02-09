// Google Analytics 4 integration

export const GA_TRACKING_ID = process.env.NEXT_PUBLIC_GA_ID || '';

// https://developers.google.com/analytics/devguides/collection/gtagjs/pages
export const pageview = (url: string) => {
  if (typeof window !== 'undefined' && window.gtag) {
    window.gtag('config', GA_TRACKING_ID, {
      page_path: url,
    });
  }
};

type GTagEvent = {
  action: string;
  category: string;
  label: string;
  value?: number;
};

// https://developers.google.com/analytics/devguides/collection/gtagjs/events
export const event = ({ action, category, label, value }: GTagEvent) => {
  if (typeof window !== 'undefined' && window.gtag) {
    window.gtag('event', action, {
      event_category: category,
      event_label: label,
      value: value,
    });
  }
};

// Custom events for Islamic app
export const trackQuranReading = (surahNumber: number, ayahNumber: number) => {
  event({
    action: 'quran_reading',
    category: 'Quran',
    label: `Surah ${surahNumber}, Ayah ${ayahNumber}`,
  });
};

export const trackPrayerTimeView = (prayerName: string) => {
  event({
    action: 'prayer_time_view',
    category: 'Prayer',
    label: prayerName,
  });
};

export const trackAIQuestion = (questionLength: number) => {
  event({
    action: 'ai_question',
    category: 'AI Assistant',
    label: 'Question Asked',
    value: questionLength,
  });
};

export const trackRecitationAnalysis = (surahNumber: number) => {
  event({
    action: 'recitation_analysis',
    category: 'Recitation',
    label: `Surah ${surahNumber}`,
  });
};

export const trackSearch = (searchTerm: string, resultCount: number) => {
  event({
    action: 'search',
    category: 'Search',
    label: searchTerm,
    value: resultCount,
  });
};

export const trackFeatureUsage = (featureName: string) => {
  event({
    action: 'feature_usage',
    category: 'Features',
    label: featureName,
  });
};

// Declare gtag on window
declare global {
  interface Window {
    gtag: (
      command: 'config' | 'event',
      targetId: string,
      config?: Record<string, any>
    ) => void;
  }
}
