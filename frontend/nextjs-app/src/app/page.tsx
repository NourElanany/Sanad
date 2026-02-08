import Link from 'next/link'

export default function Home() {
  return (
    <main className="min-h-screen bg-background-primary">
      {/* Hero Section */}
      <section className="container mx-auto px-4 py-16 text-center">
        <div className="max-w-4xl mx-auto">
          <h1 className="text-5xl md:text-6xl font-bold text-primary mb-6 animate-fade-in">
            بسم الله الرحمن الرحيم
          </h1>
          <h2 className="text-3xl md:text-4xl font-semibold text-secondary mb-8 animate-slide-up">
            سند - التطبيق الإسلامي الشامل
          </h2>
          <p className="text-xl text-text-secondary mb-12 leading-relaxed animate-slide-up">
            تطبيق إسلامي متكامل يوفر القرآن الكريم، الأحاديث النبوية، مواقيت الصلاة،
            والمساعد الذكي للإجابة على أسئلتك الإسلامية
          </p>
          
          {/* CTA Buttons */}
          <div className="flex flex-col sm:flex-row gap-4 justify-center items-center animate-scale-in">
            <Link
              href="/quran"
              className="islamic-button w-full sm:w-auto px-8 py-4 text-lg"
            >
              📖 القرآن الكريم
            </Link>
            <Link
              href="/prayer-times"
              className="islamic-button w-full sm:w-auto px-8 py-4 text-lg"
            >
              🕌 مواقيت الصلاة
            </Link>
            <Link
              href="/ai-assistant"
              className="islamic-button w-full sm:w-auto px-8 py-4 text-lg"
            >
              🤖 المساعد الذكي
            </Link>
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="container mx-auto px-4 py-16">
        <h3 className="text-3xl font-bold text-center text-primary mb-12">
          الميزات الرئيسية
        </h3>
        
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
          {/* Feature 1 */}
          <div className="islamic-card p-8">
            <div className="text-5xl mb-4">📖</div>
            <h4 className="text-2xl font-semibold text-primary mb-4">
              القرآن الكريم
            </h4>
            <p className="text-text-secondary leading-relaxed">
              قراءة القرآن الكريم بخط عثمان طه، مع التفاسير المتعددة والترجمات
            </p>
          </div>

          {/* Feature 2 */}
          <div className="islamic-card p-8">
            <div className="text-5xl mb-4">📚</div>
            <h4 className="text-2xl font-semibold text-primary mb-4">
              الأحاديث النبوية
            </h4>
            <p className="text-text-secondary leading-relaxed">
              مكتبة شاملة من الأحاديث الصحيحة مع درجات الصحة والشروحات
            </p>
          </div>

          {/* Feature 3 */}
          <div className="islamic-card p-8">
            <div className="text-5xl mb-4">🕌</div>
            <h4 className="text-2xl font-semibold text-primary mb-4">
              مواقيت الصلاة
            </h4>
            <p className="text-text-secondary leading-relaxed">
              مواقيت دقيقة للصلاة حسب موقعك مع التنبيهات والتقويم الهجري
            </p>
          </div>

          {/* Feature 4 */}
          <div className="islamic-card p-8">
            <div className="text-5xl mb-4">🤖</div>
            <h4 className="text-2xl font-semibold text-primary mb-4">
              المساعد الذكي
            </h4>
            <p className="text-text-secondary leading-relaxed">
              اسأل أي سؤال إسلامي واحصل على إجابات موثوقة مع المصادر
            </p>
          </div>

          {/* Feature 5 */}
          <div className="islamic-card p-8">
            <div className="text-5xl mb-4">🎤</div>
            <h4 className="text-2xl font-semibold text-primary mb-4">
              مصحح التلاوة
            </h4>
            <p className="text-text-secondary leading-relaxed">
              سجل تلاوتك واحصل على تحليل دقيق لأحكام التجويد
            </p>
          </div>

          {/* Feature 6 */}
          <div className="islamic-card p-8">
            <div className="text-5xl mb-4">🧭</div>
            <h4 className="text-2xl font-semibold text-primary mb-4">
              بوصلة القبلة
            </h4>
            <p className="text-text-secondary leading-relaxed">
              حدد اتجاه القبلة بدقة باستخدام تقنية الواقع المعزز
            </p>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-primary text-white py-8 mt-16">
        <div className="container mx-auto px-4 text-center">
          <p className="text-lg mb-4">
            سند - التطبيق الإسلامي الشامل
          </p>
          <p className="text-sm opacity-80">
            جميع الحقوق محفوظة © {new Date().getFullYear()}
          </p>
        </div>
      </footer>
    </main>
  )
}
