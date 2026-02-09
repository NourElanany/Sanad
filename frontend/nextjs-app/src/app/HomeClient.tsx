'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';
import Link from 'next/link';
import { PreferencesService } from '@/lib/services/preferences-service';

export default function HomeClient() {
  const router = useRouter();

  useEffect(() => {
    // Check if user has completed onboarding
    const completed = PreferencesService.getOnboardingCompleted();
    if (!completed) {
      router.push('/onboarding');
    }
  }, [router]);

  return (
    <>
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
          <Link href="/quran" className="islamic-card p-8 hover:shadow-2xl transition-shadow">
            <div className="text-5xl mb-4">📖</div>
            <h4 className="text-2xl font-semibold text-primary mb-4">
              القرآن الكريم
            </h4>
            <p className="text-text-secondary leading-relaxed">
              قراءة القرآن الكريم بخط عثمان طه، مع التفاسير المتعددة والترجمات
            </p>
          </Link>

          {/* Feature 2 */}
          <Link href="/hadith" className="islamic-card p-8 hover:shadow-2xl transition-shadow">
            <div className="text-5xl mb-4">📚</div>
            <h4 className="text-2xl font-semibold text-primary mb-4">
              الأحاديث النبوية
            </h4>
            <p className="text-text-secondary leading-relaxed">
              مكتبة شاملة من الأحاديث الصحيحة مع درجات الصحة والشروحات
            </p>
          </Link>

          {/* Feature 3 */}
          <Link href="/prayer-times" className="islamic-card p-8 hover:shadow-2xl transition-shadow">
            <div className="text-5xl mb-4">🕌</div>
            <h4 className="text-2xl font-semibold text-primary mb-4">
              مواقيت الصلاة
            </h4>
            <p className="text-text-secondary leading-relaxed">
              مواقيت دقيقة للصلاة حسب موقعك مع التنبيهات والتقويم الهجري
            </p>
          </Link>

          {/* Feature 4 */}
          <Link href="/ai-assistant" className="islamic-card p-8 hover:shadow-2xl transition-shadow">
            <div className="text-5xl mb-4">🤖</div>
            <h4 className="text-2xl font-semibold text-primary mb-4">
              المساعد الذكي
            </h4>
            <p className="text-text-secondary leading-relaxed">
              اسأل أي سؤال إسلامي واحصل على إجابات موثوقة مع المصادر
            </p>
          </Link>

          {/* Feature 5 */}
          <Link href="/recording" className="islamic-card p-8 hover:shadow-2xl transition-shadow">
            <div className="text-5xl mb-4">🎤</div>
            <h4 className="text-2xl font-semibold text-primary mb-4">
              مصحح التلاوة
            </h4>
            <p className="text-text-secondary leading-relaxed">
              سجل تلاوتك واحصل على تحليل دقيق لأحكام التجويد
            </p>
          </Link>

          {/* Feature 6 */}
          <Link href="/qibla" className="islamic-card p-8 hover:shadow-2xl transition-shadow">
            <div className="text-5xl mb-4">🧭</div>
            <h4 className="text-2xl font-semibold text-primary mb-4">
              بوصلة القبلة
            </h4>
            <p className="text-text-secondary leading-relaxed">
              حدد اتجاه القبلة بدقة باستخدام تقنية الواقع المعزز
            </p>
          </Link>

          {/* Feature 7 */}
          <Link href="/stories" className="islamic-card p-8 hover:shadow-2xl transition-shadow">
            <div className="text-5xl mb-4">📜</div>
            <h4 className="text-2xl font-semibold text-primary mb-4">
              القصص الإسلامية
            </h4>
            <p className="text-text-secondary leading-relaxed">
              قصص الأنبياء والصحابة والتابعين مع الدروس المستفادة
            </p>
          </Link>

          {/* Feature 8 */}
          <Link href="/search" className="islamic-card p-8 hover:shadow-2xl transition-shadow">
            <div className="text-5xl mb-4">🔍</div>
            <h4 className="text-2xl font-semibold text-primary mb-4">
              البحث الذكي
            </h4>
            <p className="text-text-secondary leading-relaxed">
              بحث متقدم في القرآن والأحاديث والفتاوى بتقنية الذكاء الاصطناعي
            </p>
          </Link>

          {/* Feature 9 */}
          <Link href="/statistics" className="islamic-card p-8 hover:shadow-2xl transition-shadow">
            <div className="text-5xl mb-4">📊</div>
            <h4 className="text-2xl font-semibold text-primary mb-4">
              تتبع التقدم
            </h4>
            <p className="text-text-secondary leading-relaxed">
              تتبع تقدمك في القراءة والحفظ والعبادات مع إحصائيات تفصيلية
            </p>
          </Link>
        </div>
      </section>

      {/* Call to Action Section */}
      <section className="container mx-auto px-4 py-16">
        <div className="bg-gradient-to-r from-[#1B365D] to-[#2D5A27] rounded-2xl p-12 text-center text-white">
          <h3 className="text-3xl font-bold mb-4">ابدأ رحلتك الإيمانية اليوم</h3>
          <p className="text-xl mb-8 opacity-90">
            انضم إلى آلاف المستخدمين الذين يستفيدون من سند يومياً
          </p>
          <div className="flex flex-col sm:flex-row gap-4 justify-center">
            <Link
              href="/onboarding"
              className="bg-white text-[#1B365D] px-8 py-4 rounded-lg font-bold text-lg hover:bg-gray-100 transition-colors"
            >
              ابدأ الآن
            </Link>
            <Link
              href="/dashboard"
              className="bg-transparent border-2 border-white px-8 py-4 rounded-lg font-bold text-lg hover:bg-white/10 transition-colors"
            >
              استكشف المزيد
            </Link>
          </div>
        </div>
      </section>

      {/* Footer */}
      <footer className="bg-primary text-white py-8 mt-16">
        <div className="container mx-auto px-4">
          <div className="grid grid-cols-1 md:grid-cols-3 gap-8 mb-8">
            <div>
              <h4 className="text-xl font-bold mb-4">سند</h4>
              <p className="text-sm opacity-80">
                التطبيق الإسلامي الشامل لكل مسلم
              </p>
            </div>
            <div>
              <h4 className="text-xl font-bold mb-4">روابط سريعة</h4>
              <ul className="space-y-2 text-sm">
                <li>
                  <Link href="/quran" className="opacity-80 hover:opacity-100">
                    القرآن الكريم
                  </Link>
                </li>
                <li>
                  <Link href="/hadith" className="opacity-80 hover:opacity-100">
                    الأحاديث النبوية
                  </Link>
                </li>
                <li>
                  <Link href="/prayer-times" className="opacity-80 hover:opacity-100">
                    مواقيت الصلاة
                  </Link>
                </li>
                <li>
                  <Link href="/ai-assistant" className="opacity-80 hover:opacity-100">
                    المساعد الذكي
                  </Link>
                </li>
              </ul>
            </div>
            <div>
              <h4 className="text-xl font-bold mb-4">تواصل معنا</h4>
              <p className="text-sm opacity-80">
                نسعد بتواصلكم واقتراحاتكم
              </p>
            </div>
          </div>
          <div className="text-center pt-8 border-t border-white/20">
            <p className="text-sm opacity-80">
              جميع الحقوق محفوظة © {new Date().getFullYear()} - سند
            </p>
          </div>
        </div>
      </footer>
    </>
  );
}
