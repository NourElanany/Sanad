# Zustand State Management Documentation

## Overview

This directory contains the Zustand state management implementation for the Sanad web application. Zustand provides a lightweight, scalable solution for managing application state with built-in persistence, devtools integration, and optimistic updates.

## Architecture

### Store Structure

```
stores/
├── quran-store.ts           # Quran data, bookmarks, reading progress
├── prayer-times-store.ts    # Prayer times, Hijri calendar, location
├── ai-assistant-store.ts    # AI chat sessions, messages, streaming
├── settings-store.ts        # User preferences, theme, notifications
├── store-initializer.tsx    # Store initialization and hydration
├── index.ts                 # Central exports
└── __tests__/              # Comprehensive test suite
```

## Features

### 1. Persistence Middleware

All stores use Zustand's `persist` middleware to save state to localStorage:

```typescript
persist(
  (set, get) => ({
    // Store implementation
  }),
  {
    name: 'store-name',
    storage: createJSONStorage(() => localStorage),
    partialize: (state) => ({
      // Only persist essential data
    }),
  }
)
```

**Benefits:**
- Automatic state persistence across page reloads
- Selective persistence (only save what's needed)
- Hydration handling for SSR compatibility

### 2. DevTools Integration

All stores include Redux DevTools integration for debugging:

```typescript
devtools(
  persist(/* ... */),
  {
    name: 'StoreName',
  }
)
```

**Features:**
- Time-travel debugging
- State inspection
- Action tracking
- Performance monitoring

### 3. Optimistic Updates

Stores implement optimistic updates for better UX:

```typescript
// Add bookmark with optimistic update
addBookmark: async (data) => {
  // 1. Optimistic update
  const tempBookmark = { id: `temp-${Date.now()}`, ...data };
  set({ bookmarks: [...get().bookmarks, tempBookmark] });

  try {
    // 2. API call
    const bookmark = await QuranService.addBookmark(data);
    
    // 3. Replace temp with real data
    set({
      bookmarks: get().bookmarks.map(b => 
        b.id === tempBookmark.id ? bookmark : b
      ),
    });
  } catch (error) {
    // 4. Rollback on error
    set({
      bookmarks: get().bookmarks.filter(b => b.id !== tempBookmark.id),
      error: error.message,
    });
  }
}
```

### 4. Intelligent Caching

Stores implement caching strategies to minimize API calls:

```typescript
// Cache with size limit
fetchPage: async (pageNumber: number) => {
  // Check cache first
  const cached = get().cachedPages.get(pageNumber);
  if (cached) {
    set({ currentPage: cached });
    return;
  }

  // Fetch and cache
  const page = await QuranService.getPage(pageNumber);
  const newCache = new Map(get().cachedPages);
  newCache.set(pageNumber, page);
  
  // Limit cache size
  if (newCache.size > 20) {
    const firstKey = newCache.keys().next().value;
    newCache.delete(firstKey);
  }
  
  set({ cachedPages: newCache, currentPage: page });
}
```

### 5. State Synchronization

The `StoreInitializer` component ensures state is synchronized across pages:

```typescript
export function useStoreInitializer() {
  useEffect(() => {
    // Initialize all stores on mount
    loadPreferences();
    fetchSurahs();
    fetchPrayerTimes();
    loadSessions();
    
    // Set up periodic updates
    const interval = setInterval(() => {
      updateNextPrayer();
    }, 60000);
    
    return () => clearInterval(interval);
  }, []);
}
```

## Store Details

### Quran Store

**Purpose:** Manages Quran data, bookmarks, and reading progress

**Key Features:**
- Surah and Juz listings with caching
- Bookmark management with optimistic updates
- Reading progress tracking
- Page-based caching (limit: 20 pages)
- Surah ayahs caching (limit: 10 surahs)

**Usage:**
```typescript
import { useQuranStore } from '@/lib/stores';

function QuranComponent() {
  const surahs = useQuranStore((state) => state.surahs);
  const fetchSurahs = useQuranStore((state) => state.fetchSurahs);
  const addBookmark = useQuranStore((state) => state.addBookmark);
  
  useEffect(() => {
    fetchSurahs();
  }, []);
  
  return (
    <div>
      {surahs.map(surah => (
        <div key={surah.number}>{surah.name_arabic}</div>
      ))}
    </div>
  );
}
```

### Prayer Times Store

**Purpose:** Manages prayer times, Hijri calendar, and location

**Key Features:**
- Prayer times with 6-hour cache
- Hijri date with daily refresh
- Location-based calculations
- Madhab-specific calculations
- Next prayer countdown
- Monthly prayer times

**Usage:**
```typescript
import { usePrayerTimesStore } from '@/lib/stores';

function PrayerTimesComponent() {
  const prayerTimes = usePrayerTimesStore((state) => state.prayerTimes);
  const nextPrayer = usePrayerTimesStore((state) => state.nextPrayer);
  const setLocation = usePrayerTimesStore((state) => state.setLocation);
  
  useEffect(() => {
    navigator.geolocation.getCurrentPosition((position) => {
      setLocation({
        latitude: position.coords.latitude,
        longitude: position.coords.longitude,
      });
    });
  }, []);
  
  return (
    <div>
      <h2>Next Prayer: {nextPrayer?.name}</h2>
      <p>Time: {nextPrayer?.time}</p>
    </div>
  );
}
```

### AI Assistant Store

**Purpose:** Manages AI chat sessions, messages, and streaming

**Key Features:**
- Session management
- Message history
- Streaming responses with SSE
- Voice input support
- Source verification
- Optimistic message updates

**Usage:**
```typescript
import { useAIAssistantStore } from '@/lib/stores';

function AIAssistantComponent() {
  const messages = useAIAssistantStore((state) => state.currentMessages);
  const sendMessage = useAIAssistantStore((state) => state.sendMessage);
  const streaming = useAIAssistantStore((state) => state.streaming);
  
  const handleSend = async (text: string) => {
    await sendMessage(text, true); // true for streaming
  };
  
  return (
    <div>
      {messages.map(msg => (
        <div key={msg.id}>{msg.content}</div>
      ))}
      {streaming && <div>AI is typing...</div>}
    </div>
  );
}
```

### Settings Store

**Purpose:** Manages user preferences, theme, and app settings

**Key Features:**
- Display settings (theme, font size, animations)
- Notification preferences
- Audio settings
- Privacy settings
- Offline mode configuration
- Import/export settings
- Backend synchronization

**Usage:**
```typescript
import { useSettingsStore } from '@/lib/stores';

function SettingsComponent() {
  const theme = useSettingsStore((state) => state.display.theme);
  const updateDisplay = useSettingsStore((state) => state.updateDisplay);
  
  const handleThemeChange = (newTheme: 'light' | 'dark') => {
    updateDisplay({ theme: newTheme });
  };
  
  return (
    <div>
      <button onClick={() => handleThemeChange('light')}>Light</button>
      <button onClick={() => handleThemeChange('dark')}>Dark</button>
    </div>
  );
}
```

## Selectors

Stores provide optimized selectors to prevent unnecessary re-renders:

```typescript
// Basic selector
const surahs = useQuranStore((state) => state.surahs);

// Computed selector
const surah = useQuranStore(selectSurahByNumber(2));

// Multiple selectors
const { surahs, loading, error } = useQuranStore((state) => ({
  surahs: state.surahs,
  loading: state.loading,
  error: state.error,
}));
```

## Best Practices

### 1. Use Selectors for Optimization

```typescript
// ❌ Bad - Re-renders on any state change
const store = useQuranStore();

// ✅ Good - Only re-renders when surahs change
const surahs = useQuranStore((state) => state.surahs);
```

### 2. Batch Updates

```typescript
// ❌ Bad - Multiple re-renders
set({ loading: true });
set({ data: newData });
set({ loading: false });

// ✅ Good - Single re-render
set({ loading: false, data: newData });
```

### 3. Handle Errors Gracefully

```typescript
try {
  const data = await apiCall();
  set({ data, error: null });
} catch (error) {
  set({ error: error.message });
  // Optionally rollback optimistic updates
}
```

### 4. Clear Errors After Handling

```typescript
const error = useQuranStore((state) => state.error);
const clearError = useQuranStore((state) => state.clearError);

useEffect(() => {
  if (error) {
    toast.error(error);
    clearError();
  }
}, [error]);
```

## Testing

All stores have comprehensive test coverage:

```typescript
import { renderHook, act } from '@testing-library/react';
import { useQuranStore } from '../quran-store';

describe('Quran Store', () => {
  beforeEach(() => {
    useQuranStore.getState().reset();
  });

  it('should fetch surahs', async () => {
    const { result } = renderHook(() => useQuranStore());
    
    await act(async () => {
      await result.current.fetchSurahs();
    });
    
    expect(result.current.surahs).toHaveLength(114);
  });
});
```

## Performance Considerations

### 1. Caching Strategy

- **Quran Pages:** Cache up to 20 pages
- **Surah Ayahs:** Cache up to 10 surahs
- **Prayer Times:** Cache for 6 hours
- **Hijri Date:** Cache for 24 hours

### 2. Persistence Strategy

Only persist essential data to minimize localStorage usage:

```typescript
partialize: (state) => ({
  surahs: state.surahs,
  bookmarks: state.bookmarks,
  readingProgress: state.readingProgress,
  // Don't persist: loading, error, cachedPages
})
```

### 3. Selective Re-renders

Use specific selectors to prevent unnecessary re-renders:

```typescript
// Only re-renders when bookmarks change
const bookmarks = useQuranStore((state) => state.bookmarks);
```

## Migration from React Hooks

If you're migrating from useState/useContext:

```typescript
// Before (useState)
const [surahs, setSurahs] = useState([]);
const [loading, setLoading] = useState(false);

// After (Zustand)
const surahs = useQuranStore((state) => state.surahs);
const loading = useQuranStore((state) => state.loading);
const fetchSurahs = useQuranStore((state) => state.fetchSurahs);
```

## Troubleshooting

### Hydration Mismatch

If you encounter hydration mismatches in SSR:

```typescript
import { useStoreHydration } from '@/lib/stores/store-initializer';

function Component() {
  const hydrated = useStoreHydration();
  const data = useQuranStore((state) => state.surahs);
  
  if (!hydrated) {
    return <div>Loading...</div>;
  }
  
  return <div>{/* Render data */}</div>;
}
```

### DevTools Not Working

Ensure Redux DevTools extension is installed and stores are wrapped with `devtools()` middleware.

### Persistence Not Working

Check that:
1. localStorage is available
2. Store name is unique
3. `partialize` function returns serializable data

## Future Enhancements

- [ ] Add middleware for analytics tracking
- [ ] Implement undo/redo functionality
- [ ] Add store composition for complex features
- [ ] Implement optimistic UI patterns library
- [ ] Add performance monitoring middleware

## References

- [Zustand Documentation](https://github.com/pmndrs/zustand)
- [Zustand Best Practices](https://github.com/pmndrs/zustand/wiki/Best-Practices)
- [Redux DevTools](https://github.com/reduxjs/redux-devtools)
