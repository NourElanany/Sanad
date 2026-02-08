# Assets Directory

هذا المجلد يحتوي على جميع الموارد الثابتة للتطبيق.

## البنية

```
assets/
├── images/          # الصور والرسومات
├── icons/           # الأيقونات
├── fonts/           # الخطوط العربية
├── animations/      # ملفات Lottie JSON
└── audio/           # الملفات الصوتية
```

## الخطوط المطلوبة

يجب تحميل الخطوط التالية ووضعها في `assets/fonts/`:

### Tajawal
- Tajawal-Light.ttf (300)
- Tajawal-Regular.ttf (400)
- Tajawal-Medium.ttf (500)
- Tajawal-Bold.ttf (700)

تحميل من: https://fonts.google.com/specimen/Tajawal

### Alexandria
- Alexandria-Light.ttf (300)
- Alexandria-Regular.ttf (400)
- Alexandria-Medium.ttf (500)
- Alexandria-Bold.ttf (700)

تحميل من: https://fonts.google.com/specimen/Alexandria

### KFGQPC Uthman Taha Naskh
- KFGQPC-Uthman-Taha-Naskh.ttf (400)
- KFGQPC-Uthman-Taha-Naskh-Bold.ttf (700)

تحميل من مصادر الخطوط الإسلامية

## الأيقونات المطلوبة

### أيقونة التطبيق
- `icons/app_icon.png` - 1024x1024 بكسل
- `icons/app_icon_foreground.png` - للـ Adaptive Icon في Android

### شعار شاشة البداية
- `images/splash_logo.png` - 512x512 بكسل على الأقل

## ملاحظات

- جميع الصور يجب أن تكون بصيغة PNG أو WebP
- الرسوم المتحركة يجب أن تكون بصيغة JSON (Lottie)
- الملفات الصوتية يجب أن تكون بصيغة MP3 أو AAC
