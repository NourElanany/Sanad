# سند - تطبيق Next.js الإسلامي الشامل

تطبيق ويب إسلامي متكامل مبني باستخدام Next.js 14+ مع TypeScript، يوفر القرآن الكريم، الأحاديث النبوية، مواقيت الصلاة، والمساعد الذكي.

## 🚀 الميزات الرئيسية

- ⚡ **Next.js 14+** مع App Router و Server-Side Rendering
- 🎨 **Tailwind CSS** مع ثيم إسلامي مخصص
- 📱 **Progressive Web App (PWA)** مع دعم offline
- 🌐 **دعم كامل للغة العربية** مع RTL
- 🔍 **SEO Optimization** متقدم
- 🎭 **TypeScript** للأمان والجودة
- 🎯 **Zustand** لإدارة الحالة
- 🐳 **Docker** للنشر السهل

## 📋 المتطلبات

- Node.js 20.x أو أحدث
- npm أو yarn أو pnpm

## 🛠️ التثبيت

```bash
# تثبيت المكتبات
npm install

# نسخ ملف البيئة
cp .env.example .env.local

# تشغيل التطوير
npm run dev
```

افتح [http://localhost:3000](http://localhost:3000) في المتصفح.

## 📦 البناء والنشر

### البناء للإنتاج

```bash
npm run build
npm start
```

### النشر باستخدام Docker

```bash
# بناء الصورة
docker build -t sanad-nextjs-app .

# تشغيل الحاوية
docker run -p 3000:3000 sanad-nextjs-app
```

### النشر باستخدام Docker Compose

```bash
# للإنتاج
docker-compose up -d nextjs-app

# للتطوير
docker-compose up -d nextjs-dev
```

## 🎨 الثيم الإسلامي

التطبيق يستخدم نظام ألوان إسلامي مخصص:

- **الأساسي**: كحلي داكن (#1B365D)
- **الثانوي**: أخضر زمردي (#2D5A27)
- **التمييز**: ذهبي هادئ (#B8860B)

### الخطوط

- **النصوص العادية**: Tajawal
- **النصوص القرآنية**: KFGQPC Uthman Taha Naskh

## 📱 PWA Configuration

التطبيق مُكوّن كـ Progressive Web App مع:

- Service Workers للتخزين المؤقت
- دعم Offline
- إمكانية التثبيت على الأجهزة
- Push Notifications (قريباً)

## 🧪 الاختبارات

```bash
# تشغيل الاختبارات
npm test

# تشغيل الاختبارات مع المراقبة
npm run test:watch

# تقرير التغطية
npm run test:coverage
```

## 📁 هيكل المشروع

```
frontend/nextjs-app/
├── public/              # الملفات الثابتة
│   ├── manifest.json    # PWA manifest
│   └── icons/           # أيقونات التطبيق
├── src/
│   ├── app/             # Next.js App Router
│   │   ├── layout.tsx   # Layout الرئيسي
│   │   ├── page.tsx     # الصفحة الرئيسية
│   │   └── globals.css  # الأنماط العامة
│   ├── components/      # المكونات القابلة لإعادة الاستخدام
│   ├── lib/             # المكتبات والأدوات
│   ├── store/           # Zustand stores
│   ├── types/           # TypeScript types
│   └── utils/           # الدوال المساعدة
├── Dockerfile           # Docker configuration
├── docker-compose.yml   # Docker Compose
├── next.config.js       # Next.js configuration
├── tailwind.config.ts   # Tailwind configuration
└── tsconfig.json        # TypeScript configuration
```

## 🔧 التكوين

### متغيرات البيئة

انظر `.env.example` للمتغيرات المطلوبة:

- `NEXT_PUBLIC_API_URL`: عنوان API الخلفي
- `NEXT_PUBLIC_JWT_SECRET`: مفتاح JWT
- Feature flags للميزات المختلفة

### Tailwind CSS

يمكن تخصيص الثيم من `tailwind.config.ts`:

```typescript
theme: {
  extend: {
    colors: {
      primary: { ... },
      secondary: { ... },
      accent: { ... }
    }
  }
}
```

## 🌐 التدويل (i18n)

التطبيق يدعم:
- العربية (الافتراضي)
- الإنجليزية

التكوين في `next.config.js`:

```javascript
i18n: {
  locales: ['ar', 'en'],
  defaultLocale: 'ar',
}
```

## 📊 الأداء

- ⚡ تحميل أقل من 3 ثوان
- 🎯 Lighthouse Score > 90
- 📦 Code Splitting تلقائي
- 🖼️ Image Optimization مع next/image
- 🔄 Incremental Static Regeneration

## 🤝 المساهمة

نرحب بالمساهمات! يرجى:

1. Fork المشروع
2. إنشاء branch للميزة (`git checkout -b feature/AmazingFeature`)
3. Commit التغييرات (`git commit -m 'Add some AmazingFeature'`)
4. Push إلى Branch (`git push origin feature/AmazingFeature`)
5. فتح Pull Request

## 📄 الترخيص

MIT License - انظر ملف [LICENSE](LICENSE) للتفاصيل.

## 👥 الفريق

Sanad Development Team

## 📞 الدعم

للدعم والاستفسارات، يرجى فتح issue في GitHub.

---

صُنع بـ ❤️ للمسلمين في كل مكان
