# وثيقة التصميم - واجهات التطبيق الإسلامي الشامل

## نظرة عامة

تصميم شامل لواجهات المستخدم للتطبيق الإسلامي المتكامل، يشمل تطبيق Flutter للهواتف المحمولة وتطبيق Next.js للويب. التصميم يركز على الحداثة الإسلامية مع الأداء العالي والتكامل السلس مع خدمات Rust Microservices.

## الأهداف الرئيسية

- تقديم تجربة مستخدم احترافية تليق بقوة الـ Backend المُنفذ
- ضمان الأداء العالي (60fps) للنصوص القرآنية والرسوم المتحركة
- تطبيق مبادئ التصميم الإسلامي الحديث
- دعم كامل للغة العربية واتجاه النص RTL
- تكامل سلس مع جميع خدمات الـ Backend (400+ اختبار خاصية)

## الهندسة المعمارية

### نمط التصميم المعماري

#### تطبيق Flutter (Mobile)
```
┌─────────────────────────────────────────┐
│              Presentation Layer          │
│  ┌─────────────┐  ┌─────────────────┐   │
│  │   Screens   │  │    Widgets      │   │
│  │             │  │                 │   │
│  └─────────────┘  └─────────────────┘   │
└─────────────────────────────────────────┘
┌─────────────────────────────────────────┐
│            Business Logic Layer         │
│  ┌─────────────┐  ┌─────────────────┐   │
│  │  Providers  │  │   Use Cases     │   │
│  │ (Riverpod)  │  │                 │   │
│  └─────────────┘  └─────────────────┘   │
└─────────────────────────────────────────┘
┌─────────────────────────────────────────┐
│              Data Layer                 │
│  ┌─────────────┐  ┌─────────────────┐   │
│  │ Repositories│  │   Data Sources  │   │
│  │             │  │  (API, Local)   │   │
│  └─────────────┘  └─────────────────┘   │
└─────────────────────────────────────────┘
```

#### تطبيق Next.js (Web)
```
┌─────────────────────────────────────────┐
│               Pages Layer               │
│  ┌─────────────┐  ┌─────────────────┐   │
│  │    Pages    │  │   Components    │   │
│  │   (SSR)     │  │                 │   │
│  └─────────────┘  └─────────────────┘   │
└─────────────────────────────────────────┘
┌─────────────────────────────────────────┐
│              State Layer                │
│  ┌─────────────┐  ┌─────────────────┐   │
│  │   Zustand   │  │     Hooks       │   │
│  │   Stores    │  │                 │   │
│  └─────────────┘  └─────────────────┘   │
└─────────────────────────────────────────┘
┌─────────────────────────────────────────┐
│              Services Layer             │
│  ┌─────────────┐  ┌─────────────────┐   │
│  │ API Client  │  │   PWA Service   │   │
│  │             │  │    Workers      │   │
│  └─────────────┘  └─────────────────┘   │
└─────────────────────────────────────────┘
```
## نظام التصميم الإسلامي الحديث

### لوحة الألوان الأساسية

```typescript
const IslamicTheme = {
  primary: {
    main: '#1B365D',        // كحلي داكن - اللون الأساسي
    light: '#2E4A6B',       // كحلي فاتح للـ hover states
    dark: '#0F1F35',        // كحلي أغمق للـ pressed states
  },
  secondary: {
    main: '#2D5A27',        // أخضر زمردي
    light: '#4A7C59',       // أخضر فاتح
    dark: '#1A3318',        // أخضر داكن
  },
  accent: {
    gold: '#B8860B',        // ذهبي هادئ للعناوين والأيقونات النشطة
    lightGold: '#DAA520',   // ذهبي فاتح للـ highlights
  },
  background: {
    primary: '#FEFEFE',     // أبيض كريمي للخلفية الرئيسية
    secondary: '#F8F9FA',   // رمادي فاتح جداً للبطاقات
    paper: '#FFFFFF',       // أبيض نقي للـ modals والبطاقات المرفوعة
  },
  text: {
    primary: '#1A1A1A',     // أسود للنصوص الرئيسية
    secondary: '#666666',   // رمادي للنصوص الثانوية
    disabled: '#CCCCCC',    // رمادي فاتح للنصوص المعطلة
    quranic: '#0F1F35',     // كحلي داكن للنصوص القرآنية
  },
  status: {
    success: '#28A745',     // أخضر للنجاح
    warning: '#FFC107',     // أصفر للتحذير
    error: '#DC3545',       // أحمر للأخطاء
    info: '#17A2B8',        // أزرق للمعلومات
  }
};
```

### الخطوط والطباعة

```typescript
const Typography = {
  // للنصوص العادية والواجهة
  regular: {
    fontFamily: 'Tajawal, Alexandria, sans-serif',
    weights: {
      light: 300,
      regular: 400,
      medium: 500,
      bold: 700,
    }
  },
  
  // للنصوص القرآنية
  quranic: {
    fontFamily: 'KFGQPC Uthman Taha Naskh, Amiri, serif',
    weights: {
      regular: 400,
      bold: 700,
    },
    sizes: {
      small: '18px',
      medium: '24px',
      large: '32px',
      xlarge: '40px',
    }
  },
  
  // أحجام النصوص للواجهة
  sizes: {
    caption: '12px',
    body2: '14px',
    body1: '16px',
    subtitle2: '18px',
    subtitle1: '20px',
    h6: '24px',
    h5: '28px',
    h4: '32px',
    h3: '36px',
    h2: '40px',
    h1: '48px',
  }
};
```

## مكونات الواجهة الأساسية

### 1. Islamic Button Component

```dart
// Flutter Implementation
class IslamicButton extends StatelessWidget {
  final String text;
  final VoidCallback? onPressed;
  final IslamicButtonType type;
  final IconData? icon;
  
  const IslamicButton({
    Key? key,
    required this.text,
    this.onPressed,
    this.type = IslamicButtonType.primary,
    this.icon,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        gradient: _getGradient(),
        borderRadius: BorderRadius.circular(12),
        boxShadow: [
          BoxShadow(
            color: IslamicTheme.primary.main.withOpacity(0.2),
            blurRadius: 8,
            offset: const Offset(0, 4),
          ),
        ],
      ),
      child: Material(
        color: Colors.transparent,
        child: InkWell(
          onTap: onPressed,
          borderRadius: BorderRadius.circular(12),
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 24, vertical: 16),
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                if (icon != null) ...[
                  Icon(icon, color: Colors.white, size: 20),
                  const SizedBox(width: 8),
                ],
                Text(
                  text,
                  style: const TextStyle(
                    color: Colors.white,
                    fontSize: 16,
                    fontWeight: FontWeight.w600,
                    fontFamily: 'Tajawal',
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
```

### 2. Islamic Card Component

```dart
class IslamicCard extends StatelessWidget {
  final Widget child;
  final EdgeInsetsGeometry? padding;
  final VoidCallback? onTap;
  final bool elevated;
  
  const IslamicCard({
    Key? key,
    required this.child,
    this.padding,
    this.onTap,
    this.elevated = true,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: IslamicTheme.background.paper,
        borderRadius: BorderRadius.circular(16),
        border: Border.all(
          color: IslamicTheme.primary.main.withOpacity(0.1),
          width: 1,
        ),
        boxShadow: elevated ? [
          BoxShadow(
            color: IslamicTheme.primary.main.withOpacity(0.08),
            blurRadius: 16,
            offset: const Offset(0, 4),
          ),
        ] : null,
      ),
      child: Material(
        color: Colors.transparent,
        child: InkWell(
          onTap: onTap,
          borderRadius: BorderRadius.circular(16),
          child: Padding(
            padding: padding ?? const EdgeInsets.all(20),
            child: child,
          ),
        ),
      ),
    );
  }
}
```

## تخطيط الشاشات الرئيسية

### 1. الشاشة الرئيسية (Dashboard)

```
┌─────────────────────────────────────────┐
│  ☰  السلام عليكم، أحمد        🔔  ⚙️   │ Header
├─────────────────────────────────────────┤
│  📅 الأحد، 15 رجب 1445 هـ              │ Date Card
│      28 يناير 2024 م                   │
├─────────────────────────────────────────┤
│  🕌 الصلاة القادمة: المغرب             │ Prayer Card
│      ⏰ باقي 2:34:12                   │
│      📍 الرياض، السعودية               │
├─────────────────────────────────────────┤
│  📖 وردك اليومي                       │ Daily Wird
│      ▓▓▓▓▓▓▓░░░ 70% (7/10 صفحات)      │
├─────────────────────────────────────────┤
│  💎 آية اليوم                         │ Daily Verse
│      "وَمَن يَتَّقِ اللَّهَ يَجْعَل لَّهُ مَخْرَجًا" │
│      📚 اضغط للتفسير                   │
├─────────────────────────────────────────┤
│  🤖 المساعد  🧭 القبلة  📿 الأذكار    │ Quick Actions
└─────────────────────────────────────────┘
```

### 2. شاشة القرآن الكريم

```
┌─────────────────────────────────────────┐
│  ← القرآن الكريم                🔍 ⭐   │ Header
├─────────────────────────────────────────┤
│  🔍 ابحث في القرآن الكريم...           │ Search Bar
├─────────────────────────────────────────┤
│  📑 السور    📚 الأجزاء    🔖 المفضلة  │ Tabs
├─────────────────────────────────────────┤
│  1. الفاتحة                    7 آيات   │ Surah List
│  2. البقرة                   286 آية    │
│  3. آل عمران                 200 آية    │
│  4. النساء                   176 آية    │
│  ...                                   │
└─────────────────────────────────────────┘
```

### 3. شاشة المساعد الذكي

```
┌─────────────────────────────────────────┐
│  ← المساعد الإسلامي الذكي        🎤 ⚙️  │ Header
├─────────────────────────────────────────┤
│                                         │
│  👤 ما حكم الصلاة في الطائرة؟           │ User Message
│                                         │
│  🤖 يجوز الصلاة في الطائرة مع مراعاة   │ AI Response
│     الشروط التالية:                    │
│     1. استقبال القبلة إن أمكن           │
│     2. الوضوء قبل الصعود               │
│                                         │
│     📚 المصادر:                        │ Sources
│     • صحيح البخاري - كتاب الصلاة       │
│     • فتاوى اللجنة الدائمة             │
│                                         │
├─────────────────────────────────────────┤
│  💬 اكتب سؤالك هنا...           🎤 📤  │ Input Area
└─────────────────────────────────────────┘
```

## إدارة الحالة والبيانات

### Flutter State Management (Riverpod)

```dart
// Prayer Times Provider
final prayerTimesProvider = StateNotifierProvider<PrayerTimesNotifier, PrayerTimesState>((ref) {
  return PrayerTimesNotifier(ref.read(apiServiceProvider));
});

class PrayerTimesNotifier extends StateNotifier<PrayerTimesState> {
  final ApiService _apiService;
  
  PrayerTimesNotifier(this._apiService) : super(const PrayerTimesState.loading());
  
  Future<void> loadPrayerTimes(Location location) async {
    try {
      state = const PrayerTimesState.loading();
      final times = await _apiService.getPrayerTimes(location);
      state = PrayerTimesState.loaded(times);
    } catch (e) {
      state = PrayerTimesState.error(e.toString());
    }
  }
}

// Quran Reading Provider
final quranReadingProvider = StateNotifierProvider<QuranReadingNotifier, QuranReadingState>((ref) {
  return QuranReadingNotifier(
    ref.read(quranServiceProvider),
    ref.read(localStorageProvider),
  );
});

// Audio Recording Provider for Tajweed Analysis
final audioRecordingProvider = StateNotifierProvider<AudioRecordingNotifier, AudioRecordingState>((ref) {
  return AudioRecordingNotifier(ref.read(audioServiceProvider));
});
```

### Next.js State Management (Zustand)

```typescript
// Prayer Times Store
interface PrayerTimesStore {
  prayerTimes: PrayerTimes | null;
  loading: boolean;
  error: string | null;
  fetchPrayerTimes: (location: Location) => Promise<void>;
  setLocation: (location: Location) => void;
}

export const usePrayerTimesStore = create<PrayerTimesStore>((set, get) => ({
  prayerTimes: null,
  loading: false,
  error: null,
  
  fetchPrayerTimes: async (location: Location) => {
    set({ loading: true, error: null });
    try {
      const times = await apiService.getPrayerTimes(location);
      set({ prayerTimes: times, loading: false });
    } catch (error) {
      set({ error: error.message, loading: false });
    }
  },
  
  setLocation: (location: Location) => {
    // Update location and refetch prayer times
    get().fetchPrayerTimes(location);
  },
}));

// Quran Reading Store
interface QuranStore {
  currentSurah: number;
  currentAyah: number;
  bookmarks: Bookmark[];
  readingProgress: ReadingProgress;
  setCurrentPosition: (surah: number, ayah: number) => void;
  addBookmark: (bookmark: Bookmark) => void;
  updateProgress: (progress: ReadingProgress) => void;
}
```

## التكامل مع Backend Services

### API Client Configuration

```dart
// Flutter API Client
class ApiService {
  final Dio _dio;
  final String baseUrl;
  
  ApiService({required this.baseUrl}) : _dio = Dio() {
    _dio.options.baseUrl = baseUrl;
    _dio.interceptors.add(AuthInterceptor());
    _dio.interceptors.add(LoggingInterceptor());
  }
  
  // Quran Service Integration
  Future<List<Surah>> getSurahs() async {
    final response = await _dio.get('/api/quran/surahs');
    return (response.data as List)
        .map((json) => Surah.fromJson(json))
        .toList();
  }
  
  // AI Service Integration with RAG
  Stream<String> askAIQuestion(String question) async* {
    final response = await _dio.post(
      '/api/ai/ask',
      data: {'question': question},
      options: Options(responseType: ResponseType.stream),
    );
    
    await for (final chunk in response.data.stream) {
      yield utf8.decode(chunk);
    }
  }
  
  // Audio Analysis Service Integration
  Future<RecitationAnalysis> analyzeRecitation(
    File audioFile,
    int surahNumber,
    int ayahStart,
    int ayahEnd,
  ) async {
    final formData = FormData.fromMap({
      'audio': await MultipartFile.fromFile(audioFile.path),
      'surah_number': surahNumber,
      'ayah_start': ayahStart,
      'ayah_end': ayahEnd,
    });
    
    final response = await _dio.post('/api/audio/analyze', data: formData);
    return RecitationAnalysis.fromJson(response.data);
  }
}
```

### Real-time Features

```dart
// WebSocket Integration for AI Streaming
class AIStreamingService {
  late WebSocketChannel _channel;
  
  Stream<AIResponse> streamAIResponse(String question) {
    _channel = WebSocketChannel.connect(
      Uri.parse('wss://api.sanad.app/ai/stream'),
    );
    
    _channel.sink.add(jsonEncode({
      'question': question,
      'user_id': AuthService.currentUserId,
    }));
    
    return _channel.stream.map((data) {
      final json = jsonDecode(data);
      return AIResponse.fromJson(json);
    });
  }
}

// Server-Sent Events for Prayer Time Notifications
class NotificationService {
  Stream<PrayerNotification> getPrayerNotifications() {
    return EventSource('/api/notifications/prayer-times')
        .stream
        .map((event) => PrayerNotification.fromJson(jsonDecode(event.data)));
  }
}
```

## الأداء والتحسين

### Image Optimization

```dart
// Optimized Image Loading
class OptimizedImage extends StatelessWidget {
  final String imageUrl;
  final double? width;
  final double? height;
  
  const OptimizedImage({
    Key? key,
    required this.imageUrl,
    this.width,
    this.height,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return CachedNetworkImage(
      imageUrl: imageUrl,
      width: width,
      height: height,
      placeholder: (context, url) => const IslamicShimmer(),
      errorWidget: (context, url, error) => const IslamicErrorWidget(),
      memCacheWidth: width?.toInt(),
      memCacheHeight: height?.toInt(),
    );
  }
}
```

### Lazy Loading Implementation

```dart
// Lazy Loading for Quran Content
class LazyQuranList extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      itemCount: 114, // Number of Surahs
      itemBuilder: (context, index) {
        return Consumer(
          builder: (context, ref, child) {
            final surahAsync = ref.watch(surahProvider(index + 1));
            
            return surahAsync.when(
              data: (surah) => SurahTile(surah: surah),
              loading: () => const SurahTileShimmer(),
              error: (error, stack) => SurahTileError(error: error),
            );
          },
        );
      },
    );
  }
}
```

## إمكانية الوصول (Accessibility)

### Screen Reader Support

```dart
// Accessible Quran Text
class AccessibleQuranText extends StatelessWidget {
  final String arabicText;
  final String transliteration;
  final String translation;
  
  const AccessibleQuranText({
    Key? key,
    required this.arabicText,
    required this.transliteration,
    required this.translation,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Semantics(
      label: 'آية قرآنية: $transliteration. المعنى: $translation',
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Text(
            arabicText,
            style: Theme.of(context).textTheme.headlineMedium?.copyWith(
              fontFamily: 'KFGQPC Uthman Taha Naskh',
              height: 2.0,
            ),
            textAlign: TextAlign.center,
            textDirection: TextDirection.rtl,
          ),
          if (transliteration.isNotEmpty) ...[
            const SizedBox(height: 8),
            Text(
              transliteration,
              style: Theme.of(context).textTheme.bodyMedium?.copyWith(
                fontStyle: FontStyle.italic,
              ),
              textAlign: TextAlign.center,
            ),
          ],
          if (translation.isNotEmpty) ...[
            const SizedBox(height: 8),
            Text(
              translation,
              style: Theme.of(context).textTheme.bodyMedium,
              textAlign: TextAlign.center,
            ),
          ],
        ],
      ),
    );
  }
}
```

### Voice Navigation

```dart
// Voice Command Handler
class VoiceNavigationService {
  final SpeechToText _speechToText = SpeechToText();
  
  Future<void> startListening() async {
    if (await _speechToText.initialize()) {
      _speechToText.listen(
        onResult: _handleVoiceCommand,
        localeId: 'ar_SA', // Arabic (Saudi Arabia)
      );
    }
  }
  
  void _handleVoiceCommand(SpeechRecognitionResult result) {
    final command = result.recognizedWords.toLowerCase();
    
    if (command.contains('اقرأ سورة')) {
      final surahName = _extractSurahName(command);
      NavigationService.navigateToSurah(surahName);
    } else if (command.contains('صلي الآن')) {
      NavigationService.navigateToPrayerTimes();
    } else if (command.contains('اسأل الذكاء الاصطناعي')) {
      NavigationService.navigateToAI();
    }
  }
}
```

## الأمان والخصوصية

### Local Data Encryption

```dart
// Encrypted Local Storage
class SecureStorage {
  static const _storage = FlutterSecureStorage();
  
  static Future<void> storeUserData(String key, String value) async {
    await _storage.write(
      key: key,
      value: value,
      aOptions: const AndroidOptions(
        encryptedSharedPreferences: true,
      ),
      iOptions: const IOSOptions(
        accessibility: IOSAccessibility.first_unlock_this_device,
      ),
    );
  }
  
  static Future<String?> getUserData(String key) async {
    return await _storage.read(key: key);
  }
  
  static Future<void> clearAllData() async {
    await _storage.deleteAll();
  }
}
```

### JWT Token Management

```dart
// Secure Token Management
class AuthService {
  static const _accessTokenKey = 'access_token';
  static const _refreshTokenKey = 'refresh_token';
  
  static Future<void> saveTokens(String accessToken, String refreshToken) async {
    await Future.wait([
      SecureStorage.storeUserData(_accessTokenKey, accessToken),
      SecureStorage.storeUserData(_refreshTokenKey, refreshToken),
    ]);
  }
  
  static Future<String?> getAccessToken() async {
    final token = await SecureStorage.getUserData(_accessTokenKey);
    if (token != null && !_isTokenExpired(token)) {
      return token;
    }
    
    // Try to refresh token
    return await _refreshAccessToken();
  }
  
  static Future<String?> _refreshAccessToken() async {
    final refreshToken = await SecureStorage.getUserData(_refreshTokenKey);
    if (refreshToken == null) return null;
    
    try {
      final response = await ApiService.refreshToken(refreshToken);
      await saveTokens(response.accessToken, response.refreshToken);
      return response.accessToken;
    } catch (e) {
      // Refresh failed, user needs to login again
      await logout();
      return null;
    }
  }
}
```

هذا التصميم الشامل يضمن بناء واجهات مستخدم احترافية تليق بقوة الـ Backend المُنفذ، مع التركيز على الأداء العالي والأمان والتجربة الإسلامية الأصيلة.