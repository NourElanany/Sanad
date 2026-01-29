# Sanad - التطبيق الإسلامي الشامل

## نظرة عامة

**Sanad** هو تطبيق إسلامي شامل ومتكامل يهدف إلى توفير جميع المصادر والأدوات الإسلامية الأساسية في مكان واحد. يتميز التطبيق بالذكاء الاصطناعي المتقدم، البحث الدلالي، ومصحح التلاوة.

## الميزات الرئيسية

### 📖 المحتوى الإسلامي
- **المصحف الشريف** بالرسم العثماني مع التفاسير المعتمدة
- **الأحاديث النبوية** مع درجات الصحة والشروح
- **القصص الإسلامية** الموثوقة مع الدروس والعبر
- **التقويم الهجري** مع المناسبات الإسلامية

### 🤖 الذكاء الاصطناعي المتقدم
- **نظام RAG** لمنع اختلاق الآيات والأحاديث
- **البحث الدلالي** يفهم المعنى وليس فقط الكلمات
- **مساعد ذكي** متخصص في الشؤون الإسلامية
- **مصحح التلاوة** بالذكاء الاصطناعي

### 📱 الميزات الذكية
- **الختمة الذكية** تتكيف مع سرعة قراءتك
- **مواقيت الصلاة** الدقيقة مع الإشعارات الذكية
- **الودجات التفاعلية** للشاشة الرئيسية
- **التزامن الذكي** عبر جميع أجهزتك

### 🔒 الأمان والموثوقية
- **التوثيق الرقمي** للنصوص الإسلامية
- **تشفير متقدم** لحماية البيانات
- **نسخ احتياطية** آمنة ومشفرة

## التقنيات المستخدمة

- **Backend**: Rust مع Microservices Architecture
- **Database**: PostgreSQL + Redis + Qdrant (Vector DB)
- **AI/ML**: Hugging Face + RAG System
- **Frontend**: React Native / Web
- **Infrastructure**: Docker + API Gateway

## بدء التطوير

### المتطلبات الأساسية
- Rust 1.70+
- Node.js 18+
- Docker & Docker Compose
- PostgreSQL 15+
- Redis 7+

### إعداد البيئة

```bash
# استنساخ المشروع
git clone https://github.com/NourElanany/Sanad.git
cd Sanad

# إعداد قواعد البيانات
docker-compose up -d postgres redis qdrant

# تشغيل الخدمات
cargo run --bin api-gateway
```

## هيكل المشروع

```
Sanad/
├── .kiro/
│   └── specs/
│       └── islamic-app-comprehensive/
│           ├── requirements.md    # وثيقة المتطلبات
│           ├── design.md         # وثيقة التصميم
│           └── tasks.md          # خطة التنفيذ
├── services/
│   ├── quran-service/           # خدمة القرآن الكريم
│   ├── hadith-service/          # خدمة الأحاديث
│   ├── ai-service/              # خدمة الذكاء الاصطناعي
│   ├── search-service/          # خدمة البحث الدلالي
│   ├── audio-service/           # خدمة تحليل الصوت
│   └── prayer-times-service/    # خدمة المواقيت
├── frontend/
│   ├── web/                     # تطبيق الويب
│   └── mobile/                  # تطبيق الهاتف
└── infrastructure/
    ├── docker-compose.yml
    └── kubernetes/
```

## المساهمة

نرحب بمساهماتكم في تطوير هذا المشروع الإسلامي. يرجى قراءة [دليل المساهمة](CONTRIBUTING.md) قبل البدء.

### خطوات المساهمة
1. Fork المشروع
2. إنشاء branch جديد (`git checkout -b feature/amazing-feature`)
3. Commit التغييرات (`git commit -m 'Add amazing feature'`)
4. Push إلى البranch (`git push origin feature/amazing-feature`)
5. فتح Pull Request

## الترخيص

هذا المشروع مرخص تحت رخصة MIT - راجع ملف [LICENSE](LICENSE) للتفاصيل.

## التواصل

- **المطور**: Nour Elanany
- **GitHub**: [@NourElanany](https://github.com/NourElanany)
- **المشروع**: [Sanad](https://github.com/NourElanany/Sanad)

## الشكر والتقدير

- جميع المساهمين في المشروع
- مجتمع المطورين المسلمين
- مصادر البيانات الإسلامية الموثوقة

---

**"وَقُل رَّبِّ زِدْنِي عِلْمًا"** - طه: 114