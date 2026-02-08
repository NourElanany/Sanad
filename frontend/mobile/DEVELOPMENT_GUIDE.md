# دليل التطوير - Sanad Mobile

## 🚀 البدء السريع

### المتطلبات الأساسية

1. **Flutter SDK 3.16+**
   ```bash
   flutter --version
   ```

2. **Android Studio** (للتطوير على Android)
   - Android SDK 24+
   - Android Emulator أو جهاز حقيقي

3. **Xcode** (للتطوير على iOS - macOS فقط)
   - iOS 12+
   - iOS Simulator أو جهاز حقيقي

### خطوات الإعداد

1. **استنساخ المشروع**
   ```bash
   git clone <repository-url>
   cd frontend/mobile
   ```

2. **تثبيت المكتبات**
   ```bash
   flutter pub get
   ```

3. **إضافة الخطوط العربية**
   - قم بتحميل الخطوط من الروابط في `assets/README.md`
   - ضعها في `assets/fonts/`

4. **إضافة الأيقونات**
   - أنشئ أيقونة التطبيق (1024x1024)
   - ضعها في `assets/icons/app_icon.png`

5. **توليد الملفات**
   ```bash
   flutter pub run build_runner build --delete-conflicting-outputs
   ```

6. **تشغيل التطبيق**
   ```bash
   # Development
   flutter run --flavor development --dart-define=FLAVOR=development
   ```

## 📁 هيكل المشروع

```
lib/
├── core/                      # الوظائف الأساسية المشتركة
│   ├── config/               # إعدادات التطبيق
│   │   └── app_config.dart   # تكوين البيئات والـ API
│   ├── theme/                # نظام التصميم
│   │   ├── app_theme.dart    # الثيم الرئيسي
│   │   ├── app_colors.dart   # الألوان
│   │   └── app_text_styles.dart  # أنماط النصوص
│   ├── router/               # التنقل
│   │   └── app_router.dart   # GoRouter configuration
│   ├── utils/                # أدوات مساعدة
│   │   └── logger.dart       # نظام التسجيل
│   └── widgets/              # مكونات UI مشتركة
│       ├── buttons/          # الأزرار
│       ├── cards/            # البطاقات
│       └── loading/          # مؤشرات التحميل
├── features/                  # الميزات الرئيسية
│   ├── splash/               # شاشة البداية
│   ├── onboarding/           # الإعداد الأولي
│   ├── home/                 # الشاشة الرئيسية
│   ├── quran/                # القرآن الكريم
│   ├── ai_assistant/         # المساعد الذكي
│   ├── prayer_times/         # مواقيت الصلاة
│   ├── qibla/                # بوصلة القبلة
│   ├── hadith/               # الأحاديث
│   ├── recitation/           # مصحح التلاوة
│   ├── search/               # البحث
│   └── settings/             # الإعدادات
└── main.dart                 # نقطة البداية
```

### بنية Feature

كل feature يتبع Clean Architecture:

```
feature_name/
├── data/
│   ├── models/              # نماذج البيانات
│   ├── repositories/        # تطبيق المستودعات
│   └── datasources/         # مصادر البيانات (API, Local)
├── domain/
│   ├── entities/            # كيانات الأعمال
│   ├── repositories/        # واجهات المستودعات
│   └── usecases/            # حالات الاستخدام
└── presentation/
    ├── screens/             # الشاشات
    ├── widgets/             # المكونات
    └── providers/           # Riverpod providers
```

## 🎨 نظام التصميم

### الألوان

استخدم `AppColors` للألوان:

```dart
import 'package:sanad_mobile/core/theme/app_colors.dart';

Container(
  color: AppColors.primaryMain,
  child: Text(
    'مرحباً',
    style: TextStyle(color: AppColors.textPrimary),
  ),
)
```

### الخطوط

استخدم `AppTextStyles` للنصوص:

```dart
import 'package:sanad_mobile/core/theme/app_text_styles.dart';

Text(
  'عنوان رئيسي',
  style: AppTextStyles.h1,
)

Text(
  'نص قرآني',
  style: AppTextStyles.quranicMedium,
)
```

### المكونات

استخدم المكونات الجاهزة:

```dart
// زر إسلامي
IslamicButton(
  text: 'تسجيل الدخول',
  icon: Icons.login,
  onPressed: () {},
)

// بطاقة إسلامية
IslamicCard(
  child: Text('محتوى البطاقة'),
  onTap: () {},
)
```

## 🔧 إدارة الحالة (Riverpod)

### إنشاء Provider

```dart
import 'package:flutter_riverpod/flutter_riverpod.dart';

// State Notifier
class PrayerTimesNotifier extends StateNotifier<PrayerTimesState> {
  PrayerTimesNotifier() : super(const PrayerTimesState.loading());
  
  Future<void> loadPrayerTimes() async {
    try {
      state = const PrayerTimesState.loading();
      final times = await _fetchPrayerTimes();
      state = PrayerTimesState.loaded(times);
    } catch (e) {
      state = PrayerTimesState.error(e.toString());
    }
  }
}

// Provider
final prayerTimesProvider = StateNotifierProvider<PrayerTimesNotifier, PrayerTimesState>(
  (ref) => PrayerTimesNotifier(),
);
```

### استخدام Provider

```dart
class PrayerTimesScreen extends ConsumerWidget {
  const PrayerTimesScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final state = ref.watch(prayerTimesProvider);
    
    return state.when(
      loading: () => const CircularProgressIndicator(),
      loaded: (times) => PrayerTimesList(times: times),
      error: (error) => ErrorWidget(error: error),
    );
  }
}
```

## 🌐 التكامل مع API

### إنشاء API Client

```dart
import 'package:dio/dio.dart';
import 'package:retrofit/retrofit.dart';

part 'quran_api.g.dart';

@RestApi()
abstract class QuranApi {
  factory QuranApi(Dio dio, {String baseUrl}) = _QuranApi;
  
  @GET('/surahs')
  Future<List<Surah>> getSurahs();
  
  @GET('/surahs/{id}')
  Future<Surah> getSurah(@Path('id') int id);
  
  @GET('/ayahs/{surahId}/{ayahNumber}')
  Future<Ayah> getAyah(
    @Path('surahId') int surahId,
    @Path('ayahNumber') int ayahNumber,
  );
}
```

### استخدام API Client

```dart
final dioProvider = Provider<Dio>((ref) {
  final dio = Dio(BaseOptions(
    baseUrl: AppConfig.apiBaseUrl + AppConfig.quranServicePath,
    connectTimeout: const Duration(milliseconds: AppConfig.connectTimeout),
    receiveTimeout: const Duration(milliseconds: AppConfig.apiTimeout),
  ));
  
  // Add interceptors
  dio.interceptors.add(AuthInterceptor());
  dio.interceptors.add(LoggingInterceptor());
  
  return dio;
});

final quranApiProvider = Provider<QuranApi>((ref) {
  final dio = ref.watch(dioProvider);
  return QuranApi(dio);
});
```

## 💾 التخزين المحلي

### Hive

```dart
import 'package:hive_flutter/hive_flutter.dart';

// تعريف النموذج
@HiveType(typeId: 0)
class Bookmark extends HiveObject {
  @HiveField(0)
  final int surahId;
  
  @HiveField(1)
  final int ayahNumber;
  
  @HiveField(2)
  final DateTime createdAt;
  
  Bookmark({
    required this.surahId,
    required this.ayahNumber,
    required this.createdAt,
  });
}

// فتح الصندوق
final bookmarksBox = await Hive.openBox<Bookmark>('bookmarks');

// الحفظ
await bookmarksBox.add(bookmark);

// القراءة
final bookmarks = bookmarksBox.values.toList();

// الحذف
await bookmarksBox.deleteAt(index);
```

### Secure Storage

```dart
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

const storage = FlutterSecureStorage();

// حفظ
await storage.write(key: 'access_token', value: token);

// قراءة
final token = await storage.read(key: 'access_token');

// حذف
await storage.delete(key: 'access_token');

// حذف الكل
await storage.deleteAll();
```

## 🧪 الاختبارات

### Unit Tests

```dart
import 'package:flutter_test/flutter_test.dart';

void main() {
  group('PrayerTimesCalculator', () {
    test('calculates Fajr time correctly', () {
      final calculator = PrayerTimesCalculator();
      final fajrTime = calculator.calculateFajr(
        latitude: 24.7136,
        longitude: 46.6753,
        date: DateTime(2024, 1, 1),
      );
      
      expect(fajrTime.hour, equals(5));
      expect(fajrTime.minute, inInclusiveRange(20, 30));
    });
  });
}
```

### Widget Tests

```dart
import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

void main() {
  testWidgets('PrayerTimeCard displays correct time', (tester) async {
    await tester.pumpWidget(
      ProviderScope(
        child: MaterialApp(
          home: PrayerTimeCard(
            name: 'الفجر',
            time: TimeOfDay(hour: 5, minute: 30),
          ),
        ),
      ),
    );
    
    expect(find.text('الفجر'), findsOneWidget);
    expect(find.text('05:30'), findsOneWidget);
  });
}
```

### Integration Tests

```dart
import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();
  
  testWidgets('Complete user flow', (tester) async {
    // Launch app
    await tester.pumpWidget(const SanadApp());
    await tester.pumpAndSettle();
    
    // Navigate to Quran
    await tester.tap(find.text('القرآن'));
    await tester.pumpAndSettle();
    
    // Select surah
    await tester.tap(find.text('الفاتحة'));
    await tester.pumpAndSettle();
    
    // Verify content
    expect(find.text('بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ'), findsOneWidget);
  });
}
```

## 🔍 التصحيح (Debugging)

### استخدام Logger

```dart
import 'package:sanad_mobile/core/utils/logger.dart';

// Debug
AppLogger.debug('User tapped button');

// Info
AppLogger.info('Prayer times loaded successfully');

// Warning
AppLogger.warning('Network connection slow');

// Error
AppLogger.error('Failed to load data', error, stackTrace);
```

### Flutter DevTools

```bash
# تشغيل DevTools
flutter pub global activate devtools
flutter pub global run devtools
```

## 📦 البناء للإنتاج

### Android

```bash
# APK
flutter build apk --release --flavor production --dart-define=FLAVOR=production

# App Bundle (للنشر على Google Play)
flutter build appbundle --release --flavor production --dart-define=FLAVOR=production
```

### iOS

```bash
# IPA
flutter build ipa --release --flavor production --dart-define=FLAVOR=production
```

## 🚀 النشر

### Android (Google Play)

1. إنشاء keystore:
   ```bash
   keytool -genkey -v -keystore sanad-release-key.jks -keyalg RSA -keysize 2048 -validity 10000 -alias sanad
   ```

2. إنشاء `android/key.properties`:
   ```properties
   storePassword=<password>
   keyPassword=<password>
   keyAlias=sanad
   storeFile=<path-to-keystore>
   ```

3. بناء App Bundle:
   ```bash
   flutter build appbundle --release --flavor production
   ```

4. رفع على Google Play Console

### iOS (App Store)

1. فتح Xcode:
   ```bash
   open ios/Runner.xcworkspace
   ```

2. تكوين Signing & Capabilities

3. بناء Archive:
   - Product > Archive

4. رفع على App Store Connect

## 💡 نصائح التطوير

### الأداء

1. **استخدم const constructors**
   ```dart
   const Text('مرحباً')  // ✅
   Text('مرحباً')        // ❌
   ```

2. **تجنب rebuild غير الضروري**
   ```dart
   // استخدم ConsumerWidget بدلاً من Consumer
   class MyWidget extends ConsumerWidget {
     @override
     Widget build(BuildContext context, WidgetRef ref) {
       final state = ref.watch(myProvider);
       return Text(state.value);
     }
   }
   ```

3. **استخدم ListView.builder للقوائم الطويلة**
   ```dart
   ListView.builder(
     itemCount: items.length,
     itemBuilder: (context, index) => ItemWidget(items[index]),
   )
   ```

### الأمان

1. **لا تحفظ البيانات الحساسة في SharedPreferences**
   ```dart
   // ✅ استخدم FlutterSecureStorage
   await secureStorage.write(key: 'token', value: token);
   
   // ❌ لا تستخدم SharedPreferences للـ tokens
   await prefs.setString('token', token);
   ```

2. **تحقق من الأذونات قبل الاستخدام**
   ```dart
   final status = await Permission.location.request();
   if (status.isGranted) {
     // استخدم الموقع
   }
   ```

### الصيانة

1. **اتبع قواعد الكود (Linting)**
   ```bash
   flutter analyze
   ```

2. **اكتب اختبارات للكود الجديد**

3. **وثق الكود المعقد**
   ```dart
   /// يحسب وقت صلاة الفجر بناءً على الموقع والتاريخ
   /// 
   /// [latitude] خط العرض
   /// [longitude] خط الطول
   /// [date] التاريخ المطلوب
   /// 
   /// Returns وقت صلاة الفجر
   TimeOfDay calculateFajr({
     required double latitude,
     required double longitude,
     required DateTime date,
   }) {
     // ...
   }
   ```

## 📚 موارد إضافية

- [Flutter Documentation](https://docs.flutter.dev/)
- [Riverpod Documentation](https://riverpod.dev/)
- [Dio Documentation](https://pub.dev/packages/dio)
- [Hive Documentation](https://docs.hivedb.dev/)
- [GoRouter Documentation](https://pub.dev/packages/go_router)

---

**ملاحظة**: هذا الدليل يتطور مع المشروع. يرجى المساهمة بإضافة معلومات جديدة عند اكتشافها.
