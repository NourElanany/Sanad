# Sanad - التطبيق الإسلامي الشامل

<div align="center">

![Sanad Logo](https://via.placeholder.com/200x100/2E8B57/FFFFFF?text=Sanad)

**A comprehensive Islamic application built with Rust microservices architecture**

[![Rust](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://www.docker.com)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

[العربية](#العربية) | [English](#english)

</div>

## العربية

### نظرة عامة

سند هو تطبيق إسلامي شامل يجمع جميع المصادر الإسلامية الأساسية في منصة واحدة موثوقة. يتميز التطبيق بهندسة معمارية حديثة تدعم الأداء العالي والبحث المتقدم والذكاء الاصطناعي المتخصص.

### الميزات الرئيسية

- 📖 **المصحف الشريف** - عرض القرآن الكريم بالرسم العثماني مع التفاسير
- 📚 **الأحاديث النبوية** - مجموعة شاملة من الأحاديث مع درجات الصحة
- 📜 **القصص الإسلامية** - قصص الأنبياء والصحابة والسلف الصالح
- 🤖 **المساعد الذكي** - نظام RAG لمنع الاختلاق والإجابات الموثوقة
- 🔍 **البحث الدلالي** - بحث متقدم يفهم المعنى وليس فقط الكلمات
- 🕐 **مواقيت الصلاة** - حساب دقيق لمواقيت الصلاة حسب الموقع
- 📅 **التقويم الهجري** - تحويل التواريخ والمناسبات الإسلامية
- 🎵 **مصحح التلاوة** - تحليل صوتي لتصحيح التجويد
- 📊 **الختمة الذكية** - تخطيط تفاعلي لختم القرآن
- 🔔 **الإشعارات الذكية** - تذكيرات مخصصة للعبادات

### التقنيات المستخدمة

- **Rust** - لغة البرمجة الأساسية للأداء والأمان
- **Microservices** - هندسة معمارية قابلة للتوسع
- **PostgreSQL** - قاعدة البيانات الرئيسية
- **Redis** - التخزين المؤقت عالي الأداء
- **Qdrant** - قاعدة بيانات الـ Vector للبحث الدلالي
- **Docker** - للحاويات والنشر
- **Axum** - إطار عمل الويب السريع

## English

### Overview

Sanad is a comprehensive Islamic application that brings together all essential Islamic resources in one trusted platform. The application features modern architecture supporting high performance, advanced search, and specialized artificial intelligence.

### Key Features

- 📖 **Holy Quran** - Display the Quran with Uthmanic script and interpretations
- 📚 **Prophetic Hadiths** - Comprehensive collection with authenticity grades
- 📜 **Islamic Stories** - Stories of prophets, companions, and righteous predecessors
- 🤖 **AI Assistant** - RAG system to prevent hallucination and provide reliable answers
- 🔍 **Semantic Search** - Advanced search that understands meaning, not just words
- 🕐 **Prayer Times** - Accurate prayer time calculations based on location
- 📅 **Hijri Calendar** - Date conversion and Islamic occasions
- 🎵 **Recitation Corrector** - Audio analysis for Tajweed correction
- 📊 **Smart Khatma** - Interactive planning for Quran completion
- 🔔 **Smart Notifications** - Personalized reminders for worship

### Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   API Gateway   │────│  Microservices  │────│   Databases     │
│                 │    │                 │    │                 │
│ • Authentication│    │ • Quran Service │    │ • PostgreSQL    │
│ • Rate Limiting │    │ • Hadith Service│    │ • Redis Cache   │
│ • Load Balancing│    │ • AI Service    │    │ • Qdrant Vector │
│ • Request Routing│   │ • Search Service│    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## Quick Start

### Prerequisites

- Rust 1.75+
- Docker & Docker Compose
- PostgreSQL 15+
- Redis 7+

### Installation

1. **Clone the repository**
   ```bash
   git clone https://github.com/your-org/sanad.git
   cd sanad
   ```

2. **Set up environment variables**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Start with Docker Compose**
   ```bash
   docker-compose up -d
   ```

4. **Or run locally**
   ```bash
   # Start databases
   docker-compose up -d postgres redis qdrant
   
   # Run the gateway
   cargo run --bin gateway
   ```

### API Endpoints

- **Health Check**: `GET /api/v1/health`
- **Authentication**: `POST /api/v1/auth/login`
- **Quran**: `GET /api/v1/quran/surahs`
- **Hadith**: `GET /api/v1/hadith/search`
- **AI Assistant**: `POST /api/v1/ai/ask`
- **Search**: `GET /api/v1/search`

## Development

### Project Structure

```
sanad/
├── gateway/                 # API Gateway
├── services/               # Microservices
│   ├── quran-service/      # Quran management
│   ├── hadith-service/     # Hadith management
│   ├── ai-service/         # AI with RAG
│   ├── search-service/     # Semantic search
│   └── ...
├── shared/                 # Shared libraries
├── database/              # Database schemas
├── config/                # Configuration files
└── docker-compose.yml     # Docker setup
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific service tests
cargo test --package quran-service

# Run property-based tests
cargo test --features proptest
```

### Building Services

```bash
# Build all services
cargo build --release

# Build specific service
cargo build --release --bin quran-service
```

## Configuration

### Environment Variables

Key environment variables (see `.env.example` for full list):

- `SANAD_DATABASE_URL` - PostgreSQL connection string
- `SANAD_REDIS_URL` - Redis connection string
- `SANAD_QDRANT_URL` - Qdrant vector database URL
- `SANAD_SECURITY_JWT_SECRET` - JWT signing secret
- `SANAD_EXTERNAL_APIS_HUGGING_FACE_API_KEY` - Hugging Face API key

### Database Setup

The application automatically creates the required database schema on startup. Sample data is included for development.

## Security Features

- 🔐 **JWT Authentication** - Secure token-based authentication
- 🛡️ **Rate Limiting** - Protection against abuse
- 🔒 **Content Integrity** - SHA-256 hashing for Islamic texts
- 🚫 **Anti-Hallucination** - RAG system prevents AI from generating false Islamic content
- 📝 **Digital Signing** - Cryptographic verification of religious texts

## Performance

- ⚡ **Sub-3s Response Times** - All API calls complete within 3 seconds
- 🚀 **High Throughput** - Handles thousands of concurrent requests
- 💾 **Smart Caching** - Redis-based caching for frequently accessed data
- 📱 **Offline Support** - Core content available without internet

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- 📧 Email: support@sanad.app
- 💬 Discord: [Join our community](https://discord.gg/sanad)
- 📖 Documentation: [docs.sanad.app](https://docs.sanad.app)

## Acknowledgments

- Islamic scholars and institutions for providing authentic sources
- The Rust community for excellent tools and libraries
- Contributors and beta testers

---

<div align="center">

**Built with ❤️ for the Muslim Ummah**

*"And whoever saves a life, it is as if he has saved all of mankind"* - Quran 5:32

</div>