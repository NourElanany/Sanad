# Zustand Store Usage Examples

This document provides practical examples of using Zustand stores in the Sanad application.

## Table of Contents

1. [Basic Usage](#basic-usage)
2. [Quran Store Examples](#quran-store-examples)
3. [Prayer Times Store Examples](#prayer-times-store-examples)
4. [AI Assistant Store Examples](#ai-assistant-store-examples)
5. [Settings Store Examples](#settings-store-examples)
6. [Advanced Patterns](#advanced-patterns)

## Basic Usage

### Simple Component

```typescript
'use client';

import { useQuranStore } from '@/lib/stores';

export function SurahList() {
  const surahs = useQuranStore((state) => state.surahs);
  const loading = useQuranStore((state) => state.loading);
  const fetchSurahs = useQuranStore((state) => state.fetchSurahs);

  useEffect(() => {
    fetchSurahs();
  }, [fetchSurahs]);

  if (loading) return <div>Loading...</div>;

  return (
    <div>
      {surahs.map((surah) => (
        <div key={surah.number}>
          {surah.number}. {surah.name_arabic}
        </div>
      ))}
    </div>
  );
}
```

### Using Multiple Stores

```typescript
'use client';

import { useQuranStore, usePrayerTimesStore, useSettingsStore } from '@/lib/stores';

export function Dashboard() {
  const readingProgress = useQuranStore((state) => state.readingProgress);
  const nextPrayer = usePrayerTimesStore((state) => state.nextPrayer);
  const language = useSettingsStore((state) => state.language);

  return (
    <div dir={language === 'ar' ? 'rtl' : 'ltr'}>
      <h1>Dashboard</h1>
      <div>Next Prayer: {nextPrayer?.name}</div>
      <div>Reading Progress: Surah {readingProgress?.surah_number}</div>
    </div>
  );
}
```

## Quran Store Examples

### Displaying Surahs with Filters

```typescript
'use client';

import { useState } from 'react';
import { useQuranStore } from '@/lib/stores';

export function FilteredSurahList() {
  const surahs = useQuranStore((state) => state.surahs);
  const fetchSurahs = useQuranStore((state) => state.fetchSurahs);
  const [filter, setFilter] = useState<'all' | 'meccan' | 'medinan'>('all');

  useEffect(() => {
    fetchSurahs();
  }, []);

  const filteredSurahs = surahs.filter((surah) => {
    if (filter === 'all') return true;
    return surah.revelation_type.toLowerCase() === filter;
  });

  return (
    <div>
      <div className="filters">
        <button onClick={() => setFilter('all')}>All</button>
        <button onClick={() => setFilter('meccan')}>Meccan</button>
        <button onClick={() => setFilter('medinan')}>Medinan</button>
      </div>
      
      <div className="surah-list">
        {filteredSurahs.map((surah) => (
          <div key={surah.number} className="surah-card">
            <h3>{surah.name_arabic}</h3>
            <p>{surah.name_english}</p>
            <span>{surah.ayah_count} verses</span>
          </div>
        ))}
      </div>
    </div>
  );
}
```

### Managing Bookmarks

```typescript
'use client';

import { useQuranStore } from '@/lib/stores';
import { toast } from 'react-hot-toast';

export function BookmarkManager() {
  const bookmarks = useQuranStore((state) => state.bookmarks);
  const addBookmark = useQuranStore((state) => state.addBookmark);
  const deleteBookmark = useQuranStore((state) => state.deleteBookmark);
  const error = useQuranStore((state) => state.error);
  const clearError = useQuranStore((state) => state.clearError);

  useEffect(() => {
    if (error) {
      toast.error(error);
      clearError();
    }
  }, [error, clearError]);

  const handleAddBookmark = async () => {
    try {
      await addBookmark({
        surah_number: 2,
        ayah_number: 255,
        page_number: 42,
        note: 'Ayat Al-Kursi',
      });
      toast.success('Bookmark added!');
    } catch (err) {
      // Error is handled by store
    }
  };

  const handleDeleteBookmark = async (id: string) => {
    try {
      await deleteBookmark(id);
      toast.success('Bookmark deleted!');
    } catch (err) {
      // Error is handled by store
    }
  };

  return (
    <div>
      <button onClick={handleAddBookmark}>Add Bookmark</button>
      
      <div className="bookmarks">
        {bookmarks.map((bookmark) => (
          <div key={bookmark.id} className="bookmark-card">
            <h4>Surah {bookmark.surah_number}, Ayah {bookmark.ayah_number}</h4>
            {bookmark.note && <p>{bookmark.note}</p>}
            <button onClick={() => handleDeleteBookmark(bookmark.id)}>
              Delete
            </button>
          </div>
        ))}
      </div>
    </div>
  );
}
```

### Reading Progress Tracker

```typescript
'use client';

import { useQuranStore } from '@/lib/stores';

export function ReadingProgressTracker() {
  const readingProgress = useQuranStore((state) => state.readingProgress);
  const updateReadingProgress = useQuranStore((state) => state.updateReadingProgress);
  const surahs = useQuranStore((state) => state.surahs);

  const currentSurah = surahs.find(
    (s) => s.number === readingProgress?.surah_number
  );

  const handleUpdateProgress = async (surahNumber: number, ayahNumber: number) => {
    await updateReadingProgress({
      surah_number: surahNumber,
      ayah_number: ayahNumber,
      page_number: Math.floor(ayahNumber / 15) + 1, // Approximate
    });
  };

  return (
    <div className="progress-tracker">
      <h2>Your Reading Progress</h2>
      {readingProgress && currentSurah ? (
        <div>
          <p>Currently reading: {currentSurah.name_arabic}</p>
          <p>Ayah: {readingProgress.ayah_number} / {currentSurah.ayah_count}</p>
          <div className="progress-bar">
            <div
              style={{
                width: `${(readingProgress.ayah_number / currentSurah.ayah_count) * 100}%`,
              }}
            />
          </div>
        </div>
      ) : (
        <p>Start reading to track your progress</p>
      )}
    </div>
  );
}
```

## Prayer Times Store Examples

### Prayer Times Display with Countdown

```typescript
'use client';

import { useEffect, useState } from 'react';
import { usePrayerTimesStore } from '@/lib/stores';

export function PrayerTimesDisplay() {
  const prayerTimes = usePrayerTimesStore((state) => state.prayerTimes);
  const nextPrayer = usePrayerTimesStore((state) => state.nextPrayer);
  const updateNextPrayer = usePrayerTimesStore((state) => state.updateNextPrayer);
  const fetchPrayerTimes = usePrayerTimesStore((state) => state.fetchPrayerTimes);
  const setLocation = usePrayerTimesStore((state) => state.setLocation);

  useEffect(() => {
    // Get user location
    navigator.geolocation.getCurrentPosition((position) => {
      setLocation({
        latitude: position.coords.latitude,
        longitude: position.coords.longitude,
      });
    });

    // Update countdown every second
    const interval = setInterval(() => {
      updateNextPrayer();
    }, 1000);

    return () => clearInterval(interval);
  }, []);

  if (!prayerTimes) return <div>Loading prayer times...</div>;

  return (
    <div className="prayer-times">
      <div className="next-prayer">
        <h2>Next Prayer: {nextPrayer?.name}</h2>
        <p className="countdown">
          {nextPrayer?.timeRemaining.hours}h{' '}
          {nextPrayer?.timeRemaining.minutes}m{' '}
          {nextPrayer?.timeRemaining.seconds}s
        </p>
      </div>

      <div className="all-prayers">
        <div>Fajr: {prayerTimes.fajr}</div>
        <div>Sunrise: {prayerTimes.sunrise}</div>
        <div>Dhuhr: {prayerTimes.dhuhr}</div>
        <div>Asr: {prayerTimes.asr}</div>
        <div>Maghrib: {prayerTimes.maghrib}</div>
        <div>Isha: {prayerTimes.isha}</div>
      </div>
    </div>
  );
}
```

### Hijri Calendar Display

```typescript
'use client';

import { usePrayerTimesStore } from '@/lib/stores';

export function HijriCalendar() {
  const hijriDate = usePrayerTimesStore((state) => state.hijriDate);
  const fetchHijriDate = usePrayerTimesStore((state) => state.fetchHijriDate);
  const formattedDate = usePrayerTimesStore((state) => {
    if (!state.hijriDate) return null;
    return `${state.hijriDate.weekday}، ${state.hijriDate.day} ${state.hijriDate.monthName} ${state.hijriDate.year} هـ`;
  });

  useEffect(() => {
    fetchHijriDate();
  }, []);

  return (
    <div className="hijri-calendar">
      <h3>التاريخ الهجري</h3>
      <p className="hijri-date">{formattedDate}</p>
    </div>
  );
}
```

### Madhab Selector

```typescript
'use client';

import { usePrayerTimesStore } from '@/lib/stores';

export function MadhabSelector() {
  const madhab = usePrayerTimesStore((state) => state.madhab);
  const setMadhab = usePrayerTimesStore((state) => state.setMadhab);

  const madhabs = [
    { value: 'shafi', label: 'Shafi' },
    { value: 'hanafi', label: 'Hanafi' },
    { value: 'maliki', label: 'Maliki' },
    { value: 'hanbali', label: 'Hanbali' },
  ];

  return (
    <div className="madhab-selector">
      <label>Select Madhab:</label>
      <select value={madhab} onChange={(e) => setMadhab(e.target.value)}>
        {madhabs.map((m) => (
          <option key={m.value} value={m.value}>
            {m.label}
          </option>
        ))}
      </select>
    </div>
  );
}
```

## AI Assistant Store Examples

### Chat Interface with Streaming

```typescript
'use client';

import { useState } from 'react';
import { useAIAssistantStore } from '@/lib/stores';

export function AIChat() {
  const [input, setInput] = useState('');
  const messages = useAIAssistantStore((state) => state.currentMessages);
  const sendMessage = useAIAssistantStore((state) => state.sendMessage);
  const streaming = useAIAssistantStore((state) => state.streaming);
  const createSession = useAIAssistantStore((state) => state.createSession);
  const currentSessionId = useAIAssistantStore((state) => state.currentSessionId);

  useEffect(() => {
    if (!currentSessionId) {
      createSession();
    }
  }, [currentSessionId]);

  const handleSend = async () => {
    if (!input.trim()) return;
    
    await sendMessage(input, true); // true for streaming
    setInput('');
  };

  return (
    <div className="ai-chat">
      <div className="messages">
        {messages.map((msg) => (
          <div key={msg.id} className={`message ${msg.role}`}>
            <div className="content">{msg.content}</div>
            {msg.sources && (
              <div className="sources">
                {msg.sources.map((source) => (
                  <div key={source.id} className="source-card">
                    <h4>{source.title}</h4>
                    <p>{source.reference}</p>
                  </div>
                ))}
              </div>
            )}
          </div>
        ))}
        {streaming && <div className="typing-indicator">AI is typing...</div>}
      </div>

      <div className="input-area">
        <input
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyPress={(e) => e.key === 'Enter' && handleSend()}
          placeholder="Ask a question..."
          disabled={streaming}
        />
        <button onClick={handleSend} disabled={streaming}>
          Send
        </button>
      </div>
    </div>
  );
}
```

### Session Management

```typescript
'use client';

import { useAIAssistantStore } from '@/lib/stores';

export function SessionManager() {
  const sessions = useAIAssistantStore((state) => state.sessions);
  const currentSessionId = useAIAssistantStore((state) => state.currentSessionId);
  const loadSession = useAIAssistantStore((state) => state.loadSession);
  const deleteSession = useAIAssistantStore((state) => state.deleteSession);
  const createSession = useAIAssistantStore((state) => state.createSession);

  return (
    <div className="session-manager">
      <button onClick={createSession}>New Chat</button>
      
      <div className="sessions-list">
        {sessions.map((session) => (
          <div
            key={session.id}
            className={`session ${session.id === currentSessionId ? 'active' : ''}`}
          >
            <div onClick={() => loadSession(session.id)}>
              <h4>Chat {session.id.slice(0, 8)}</h4>
              <p>{session.messages.length} messages</p>
              <small>{new Date(session.updatedAt).toLocaleString()}</small>
            </div>
            <button onClick={() => deleteSession(session.id)}>Delete</button>
          </div>
        ))}
      </div>
    </div>
  );
}
```

## Settings Store Examples

### Theme Switcher

```typescript
'use client';

import { useSettingsStore } from '@/lib/stores';

export function ThemeSwitcher() {
  const theme = useSettingsStore((state) => state.display.theme);
  const updateDisplay = useSettingsStore((state) => state.updateDisplay);

  const themes = [
    { value: 'light', label: 'Light', icon: '☀️' },
    { value: 'dark', label: 'Dark', icon: '🌙' },
    { value: 'auto', label: 'Auto', icon: '🔄' },
  ];

  return (
    <div className="theme-switcher">
      {themes.map((t) => (
        <button
          key={t.value}
          onClick={() => updateDisplay({ theme: t.value as any })}
          className={theme === t.value ? 'active' : ''}
        >
          <span>{t.icon}</span>
          <span>{t.label}</span>
        </button>
      ))}
    </div>
  );
}
```

### Comprehensive Settings Panel

```typescript
'use client';

import { useSettingsStore } from '@/lib/stores';

export function SettingsPanel() {
  const settings = useSettingsStore();

  return (
    <div className="settings-panel">
      {/* Display Settings */}
      <section>
        <h2>Display</h2>
        <div>
          <label>Font Size</label>
          <select
            value={settings.display.fontSize}
            onChange={(e) =>
              settings.updateDisplay({ fontSize: e.target.value as any })
            }
          >
            <option value="small">Small</option>
            <option value="medium">Medium</option>
            <option value="large">Large</option>
            <option value="xlarge">Extra Large</option>
          </select>
        </div>
        <div>
          <label>
            <input
              type="checkbox"
              checked={settings.display.enableAnimations}
              onChange={(e) =>
                settings.updateDisplay({ enableAnimations: e.target.checked })
              }
            />
            Enable Animations
          </label>
        </div>
      </section>

      {/* Notification Settings */}
      <section>
        <h2>Notifications</h2>
        <div>
          <label>
            <input
              type="checkbox"
              checked={settings.notifications.prayerTimes}
              onChange={(e) =>
                settings.updateNotifications({ prayerTimes: e.target.checked })
              }
            />
            Prayer Times
          </label>
        </div>
        <div>
          <label>
            <input
              type="checkbox"
              checked={settings.notifications.dailyReminders}
              onChange={(e) =>
                settings.updateNotifications({ dailyReminders: e.target.checked })
              }
            />
            Daily Reminders
          </label>
        </div>
      </section>

      {/* Audio Settings */}
      <section>
        <h2>Audio</h2>
        <div>
          <label>Recitation Volume</label>
          <input
            type="range"
            min="0"
            max="100"
            value={settings.audio.recitationVolume}
            onChange={(e) =>
              settings.updateAudio({ recitationVolume: parseInt(e.target.value) })
            }
          />
        </div>
      </section>

      {/* Actions */}
      <div className="actions">
        <button onClick={() => settings.exportSettings()}>
          Export Settings
        </button>
        <button onClick={() => settings.resetToDefaults()}>
          Reset to Defaults
        </button>
      </div>
    </div>
  );
}
```

## Advanced Patterns

### Combining Multiple Stores

```typescript
'use client';

import { useQuranStore, usePrayerTimesStore, useSettingsStore } from '@/lib/stores';

export function IntegratedDashboard() {
  // Quran data
  const readingProgress = useQuranStore((state) => state.readingProgress);
  const bookmarks = useQuranStore((state) => state.bookmarks);
  
  // Prayer times
  const nextPrayer = usePrayerTimesStore((state) => state.nextPrayer);
  const hijriDate = usePrayerTimesStore((state) => state.hijriDate);
  
  // Settings
  const language = useSettingsStore((state) => state.language);
  const theme = useSettingsStore((state) => state.display.theme);

  return (
    <div className={`dashboard theme-${theme}`} dir={language === 'ar' ? 'rtl' : 'ltr'}>
      <div className="prayer-widget">
        <h3>Next Prayer</h3>
        <p>{nextPrayer?.name} at {nextPrayer?.time}</p>
      </div>

      <div className="reading-widget">
        <h3>Reading Progress</h3>
        <p>Surah {readingProgress?.surah_number}</p>
        <p>{bookmarks.length} bookmarks</p>
      </div>

      <div className="date-widget">
        <h3>Hijri Date</h3>
        <p>{hijriDate?.day} {hijriDate?.monthName} {hijriDate?.year}</p>
      </div>
    </div>
  );
}
```

### Custom Hook for Store Logic

```typescript
import { useEffect } from 'react';
import { useQuranStore } from '@/lib/stores';

export function useQuranData(surahNumber?: number) {
  const surahs = useQuranStore((state) => state.surahs);
  const currentSurah = useQuranStore((state) => state.currentSurah);
  const fetchSurahs = useQuranStore((state) => state.fetchSurahs);
  const fetchSurah = useQuranStore((state) => state.fetchSurah);
  const loading = useQuranStore((state) => state.loading);
  const error = useQuranStore((state) => state.error);

  useEffect(() => {
    if (surahs.length === 0) {
      fetchSurahs();
    }
  }, [surahs.length, fetchSurahs]);

  useEffect(() => {
    if (surahNumber) {
      fetchSurah(surahNumber);
    }
  }, [surahNumber, fetchSurah]);

  return {
    surahs,
    currentSurah,
    loading,
    error,
  };
}

// Usage
function SurahViewer({ surahNumber }: { surahNumber: number }) {
  const { currentSurah, loading } = useQuranData(surahNumber);

  if (loading) return <div>Loading...</div>;
  if (!currentSurah) return <div>Surah not found</div>;

  return <div>{currentSurah.name_arabic}</div>;
}
```

These examples demonstrate the full power and flexibility of the Zustand state management system in the Sanad application.
