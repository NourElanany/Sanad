import { describe, it, expect } from '@jest/globals';
import {
  hijriToGregorian,
  gregorianToHijri,
  formatDuration,
  parseDuration,
  calculateReadingProgress,
  deduplicateBookmarks,
  rankSearchResults,
} from '../data-transformations';

/**
 * Property-based tests for data transformations
 * **Validates: Requirements 20.4**
 */
describe('Data Transformations', () => {
  describe('Hijri Date Conversion', () => {
    it('should convert Hijri to Gregorian and back', () => {
      // Property: Round-trip conversion should preserve date
      const testCases = [
        { year: 1445, month: 7, day: 15 },
        { year: 1400, month: 1, day: 1 },
        { year: 1450, month: 12, day: 29 },
      ];

      testCases.forEach(({ year, month, day }) => {
        const gregorian = hijriToGregorian(year, month, day);
        const backToHijri = gregorianToHijri(
          gregorian.year,
          gregorian.month,
          gregorian.day
        );

        expect(backToHijri.year).toBe(year);
        expect(backToHijri.month).toBe(month);
        expect(backToHijri.day).toBe(day);
      });
    });

    it('should always return valid Hijri month (1-12)', () => {
      for (let i = 0; i < 100; i++) {
        const year = 2020 + Math.floor(Math.random() * 10);
        const month = 1 + Math.floor(Math.random() * 12);
        const day = 1 + Math.floor(Math.random() * 28);

        const hijri = gregorianToHijri(year, month, day);

        expect(hijri.month).toBeGreaterThanOrEqual(1);
        expect(hijri.month).toBeLessThanOrEqual(12);
      }
    });

    it('should always return valid Hijri day (1-30)', () => {
      for (let i = 0; i < 100; i++) {
        const year = 2020 + Math.floor(Math.random() * 10);
        const month = 1 + Math.floor(Math.random() * 12);
        const day = 1 + Math.floor(Math.random() * 28);

        const hijri = gregorianToHijri(year, month, day);

        expect(hijri.day).toBeGreaterThanOrEqual(1);
        expect(hijri.day).toBeLessThanOrEqual(30);
      }
    });
  });

  describe('Duration Formatting', () => {
    it('should format and parse duration correctly', () => {
      const testCases = [
        0, 30, 60, 90, 300, 600, 3600, 7200,
      ];

      testCases.forEach((seconds) => {
        const formatted = formatDuration(seconds);
        const parsed = parseDuration(formatted);

        expect(parsed).toBe(seconds);
      });
    });

    it('should format duration in MM:SS for < 1 hour', () => {
      expect(formatDuration(0)).toBe('00:00');
      expect(formatDuration(30)).toBe('00:30');
      expect(formatDuration(90)).toBe('01:30');
      expect(formatDuration(3599)).toBe('59:59');
    });

    it('should format duration in HH:MM:SS for >= 1 hour', () => {
      expect(formatDuration(3600)).toBe('01:00:00');
      expect(formatDuration(3661)).toBe('01:01:01');
      expect(formatDuration(7200)).toBe('02:00:00');
    });

    it('should handle edge cases', () => {
      expect(formatDuration(0)).toBe('00:00');
      expect(formatDuration(86400)).toBe('24:00:00'); // 24 hours
    });
  });

  describe('Reading Progress Calculation', () => {
    it('should return percentage between 0 and 100', () => {
      for (let i = 0; i < 100; i++) {
        const totalAyahs = 6236;
        const readAyahs = Math.floor(Math.random() * (totalAyahs + 1));

        const progress = calculateReadingProgress(readAyahs, totalAyahs);

        expect(progress).toBeGreaterThanOrEqual(0);
        expect(progress).toBeLessThanOrEqual(100);
      }
    });

    it('should return 0% for 0 ayahs read', () => {
      expect(calculateReadingProgress(0, 6236)).toBe(0);
    });

    it('should return 100% for all ayahs read', () => {
      expect(calculateReadingProgress(6236, 6236)).toBe(100);
    });

    it('should increase monotonically', () => {
      const totalAyahs = 6236;
      let previousProgress = 0;

      for (let readAyahs = 0; readAyahs <= totalAyahs; readAyahs += 100) {
        const progress = calculateReadingProgress(readAyahs, totalAyahs);
        expect(progress).toBeGreaterThanOrEqual(previousProgress);
        previousProgress = progress;
      }
    });

    it('should handle decimal precision correctly', () => {
      const progress = calculateReadingProgress(1, 3);
      expect(progress).toBeCloseTo(33.33, 2);
    });
  });

  describe('Bookmark Deduplication', () => {
    it('should preserve unique bookmarks', () => {
      const bookmarks = [
        { id: '1', surahNumber: 2, ayahNumber: 255 },
        { id: '2', surahNumber: 18, ayahNumber: 10 },
        { id: '3', surahNumber: 36, ayahNumber: 1 },
      ];

      const deduplicated = deduplicateBookmarks(bookmarks);

      expect(deduplicated).toHaveLength(3);
    });

    it('should remove exact duplicates', () => {
      const bookmarks = [
        { id: '1', surahNumber: 2, ayahNumber: 255 },
        { id: '2', surahNumber: 2, ayahNumber: 255 },
        { id: '3', surahNumber: 2, ayahNumber: 255 },
      ];

      const deduplicated = deduplicateBookmarks(bookmarks);

      expect(deduplicated).toHaveLength(1);
    });

    it('should handle empty array', () => {
      const deduplicated = deduplicateBookmarks([]);
      expect(deduplicated).toHaveLength(0);
    });

    it('should consider different IDs but same location as duplicates', () => {
      const bookmarks = [
        { id: '1', surahNumber: 2, ayahNumber: 255 },
        { id: '2', surahNumber: 2, ayahNumber: 255 },
      ];

      const deduplicated = deduplicateBookmarks(bookmarks);

      expect(deduplicated).toHaveLength(1);
    });
  });

  describe('Search Result Ranking', () => {
    it('should rank by relevance score', () => {
      const results = [
        { text: 'Result A', relevance: 0.5 },
        { text: 'Result B', relevance: 0.9 },
        { text: 'Result C', relevance: 0.7 },
      ];

      const ranked = rankSearchResults(results, 'query');

      expect(ranked[0].text).toBe('Result B');
      expect(ranked[1].text).toBe('Result C');
      expect(ranked[2].text).toBe('Result A');
    });

    it('should ensure relevance scores are between 0 and 1', () => {
      for (let i = 0; i < 100; i++) {
        const results = Array.from({ length: 10 }, (_, j) => ({
          text: `Result ${j}`,
          relevance: Math.random(),
        }));

        const ranked = rankSearchResults(results, 'query');

        ranked.forEach((result) => {
          expect(result.relevance).toBeGreaterThanOrEqual(0);
          expect(result.relevance).toBeLessThanOrEqual(1);
        });
      }
    });

    it('should be stable for same input', () => {
      const results = [
        { text: 'A', relevance: 0.9 },
        { text: 'B', relevance: 0.7 },
        { text: 'C', relevance: 0.8 },
      ];

      const ranked1 = rankSearchResults(results, 'query');
      const ranked2 = rankSearchResults(results, 'query');

      expect(ranked1.map((r) => r.text)).toEqual(ranked2.map((r) => r.text));
    });

    it('should handle empty results', () => {
      const ranked = rankSearchResults([], 'query');
      expect(ranked).toHaveLength(0);
    });

    it('should handle single result', () => {
      const results = [{ text: 'Only result', relevance: 0.8 }];
      const ranked = rankSearchResults(results, 'query');

      expect(ranked).toHaveLength(1);
      expect(ranked[0].text).toBe('Only result');
    });
  });

  describe('Edge Cases and Error Handling', () => {
    it('should handle invalid inputs gracefully', () => {
      expect(() => calculateReadingProgress(-1, 6236)).toThrow();
      expect(() => calculateReadingProgress(6237, 6236)).toThrow();
      expect(() => formatDuration(-1)).toThrow();
    });

    it('should handle null and undefined', () => {
      expect(deduplicateBookmarks(null as any)).toEqual([]);
      expect(deduplicateBookmarks(undefined as any)).toEqual([]);
    });

    it('should handle very large numbers', () => {
      const progress = calculateReadingProgress(1000000, 1000000);
      expect(progress).toBe(100);

      const formatted = formatDuration(999999);
      expect(formatted).toMatch(/^\d{2,}:\d{2}:\d{2}$/);
    });
  });
});
