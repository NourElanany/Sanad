# Sanad Mobile - التطبيق الإسلامي الشامل

تطبيق Flutter للهواتف المحمولة (Android و iOS) يوفر تجربة إسلامية شاملة مع تكامل كامل مع خدمات Rust Microservices.

## 📱 المميزات

- **القرآن الكريم**: قراءة المصحف الرقمي مع التفاسير المتعددة
- **المساعد الذكي**: مساعد AI للإجابة على الأسئلة الإسلامية مع المصادر
- **مصحح التلاوة**: تحليل التجويد والنطق باستخدام الذكاء الاصطناعي
- **مواقيت الصلاة**: حساب دقيق للمواقيت مع التنبيهات
- **بوصلة القبلة**: تحديد اتجاه القبلة بالواقع المعزز
- **الأحاديث النبوية**: مكتبة شاملة مع درجات الصحة
- **البحث الدلالي**: بحث ذكي عبر جميع المحتويات الإسلامية
- **الوضع دون اتصال**: عمل التطبيق بدون إنترنت

## 🏗️ البنية المعمارية

```
lib/
├── core/                      # الوظائف الأساسية المشتركة
│   ├── config/               # إعدادات التطبيق
│   ├── theme/                # الثيم والألوان
│   ├── router/               # التنقل والمسارات
│   ├── utils/                # أدوات مساعدة
│   └── widgets/              # مكونات UI مشتركة
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

## 🚀 البدء

### المتطلبات

- Flutter SDK 3.16 أو أحدث
- Dart SDK 3.2 أو أحدث
- Android Studio / Xcode للتطوير
- Android SDK 24+ / iOS 12+

### التثبيت

1. **استنساخ المشروع**
```bash
git clone <repository-url>
cd frontend/mobile
```

2. **تثبيت المكتبات**
```bash
flutter pub get
```

3. **توليد الملفات المطلوبة**
```bash
flutter pub run build_runner build --delete-conflicting-outputs
```

4. **تشغيل التطبيق**

للتطوير (Development):
```bash
flutter run --flavor development --dart-define=FLAVOR=development
```

للاختبار (Staging):
```bash
flutter run --flavor staging --dart-define=FLAVOR=staging
```

للإنتاج (Production):
```bash
flutter run --flavor production --dart-define=FLAVOR=production
```

## 🎨 نظام التصميم

### الألوان الأساسية

- **Primary (كحلي داكن)**: `#1B365D`
- **Secondary (أخضر زمردي)**: `#2D5A27`
- **Accent (ذهبي هادئ)**: `#B8860B`
- **Background**: `#FEFEFE`
- **Text**: `#1A1A1A`

### الخطوط

- **النصوص العادية**: Tajawal, Alexandria
- **النصوص القرآنية**: KFGQPC Uthman Taha Naskh

## 🔧 البيئات (Flavors)

التطبيق يدعم ثلاث بيئات:

### Development
- **Package ID**: `com.sanad.mobile.dev`
- **API URL**: `https://dev-api.sanad.app`
- **الاستخدام**: التطوير والاختبار المحلي

### Staging
- **Package ID**: `com.sanad.mobile.staging`
- **API URL**: `https://staging-api.sanad.app`
- **الاستخدام**: الاختبار قبل الإنتاج

### Production
- **Package ID**: `com.sanad.mobile`
- **API URL**: `https://api.sanad.app`
- **الاستخدام**: النسخة النهائية للمستخدمين

## 📦 المكتبات الرئيسية

### إدارة الحالة
- **flutter_riverpod**: إدارة الحالة المتقدمة

### الشبكة والAPI
- **dio**: HTTP client
- **retrofit**: REST API client generator
- **web_socket_channel**: WebSocket للـ real-time features

### التخزين المحلي
- **hive**: قاعدة بيانات محلية سريعة
- **flutter_secure_storage**: تخزين آمن للبيانات الحساسة
- **shared_preferences**: تخزين التفضيلات

### الصوت
- **flutter_sound**: تسجيل ومعالجة الصوت
- **audio_waveforms**: عرض الموجات الصوتية
- **just_audio**: تشغيل الصوت

### الموقع والحساسات
- **geolocator**: تحديد الموقع
- **flutter_compass**: البوصلة
- **sensors_plus**: مستشعرات الجهاز

### UI والرسوم المتحركة
- **cached_network_image**: تحميل وتخزين الصور
- **shimmer**: تأثيرات التحميل
- **lottie**: رسوم متحركة JSON

## 🧪 الاختبارات

### تشغيل الاختبارات

```bash
# Unit tests
flutter test

# Integration tests
flutter test integration_test

# Widget tests
flutter test test/widgets
```

### تغطية الاختبارات

```bash
flutter test --coverage
genhtml coverage/lcov.info -o coverage/html
```

## 📱 البناء للإنتاج

### Android

```bash
# Build APK
flutter build apk --flavor production --dart-define=FLAVOR=production

# Build App Bundle
flutter build appbundle --flavor production --dart-define=FLAVOR=production
```

### iOS

```bash
# Build IPA
flutter build ipa --flavor production --dart-define=FLAVOR=production
```

## 🔐 الأذونات

### Android
- `INTERNET`: الاتصال بالإنترنت
- `ACCESS_FINE_LOCATION`: تحديد الموقع الدقيق
- `RECORD_AUDIO`: تسجيل الصوت
- `WRITE_EXTERNAL_STORAGE`: حفظ الملفات
- `VIBRATE`: الاهتزاز للتنبيهات
- `POST_NOTIFICATIONS`: إرسال الإشعارات

### iOS
- `NSLocationWhenInUseUsageDescription`: تحديد الموقع
- `NSMicrophoneUsageDescription`: استخدام الميكروفون
- `NSPhotoLibraryUsageDescription`: الوصول للصور
- `NSMotionUsageDescription`: استخدام مستشعرات الحركة

## 🌐 التكامل مع Backend

التطبيق يتكامل مع خدمات Rust Microservices التالية:

- **Quran Service**: القرآن الكريم والتفاسير
- **Hadith Service**: الأحاديث النبوية
- **Prayer Times Service**: مواقيت الصلاة والتقويم الهجري
- **AI Service**: المساعد الذكي ونظام RAG
- **Audio Analysis Service**: تحليل التلاوة والتجويد
- **Search Service**: البحث الدلالي المتقدم
- **User Service**: إدارة المستخدمين والمصادقة
- **Notification Service**: الإشعارات والتنبيهات

## 📚 الموارد

- [Flutter Documentation](https://docs.flutter.dev/)
- [Riverpod Documentation](https://riverpod.dev/)
- [Material Design 3](https://m3.material.io/)

## 🤝 المساهمة

يرجى قراءة [CONTRIBUTING.md](../../CONTRIBUTING.md) للمزيد من المعلومات حول كيفية المساهمة في المشروع.

## 📄 الترخيص

هذا المشروع مرخص تحت [LICENSE](../../LICENSE).

## 📞 الدعم

للدعم والاستفسارات، يرجى فتح issue في المستودع أو التواصل عبر البريد الإلكتروني.

---

**ملاحظة**: هذا المشروع قيد التطوير النشط. بعض الميزات قد تكون غير مكتملة.
