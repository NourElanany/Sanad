'use client';

import { useState } from 'react';
import { useRouter } from 'next/navigation';
import { IslamicButton } from '@/components/ui/IslamicButton';
import { IslamicCard } from '@/components/ui/IslamicCard';

interface OnboardingPage {
  title: string;
  description: string;
  icon: string;
}

const onboardingPages: OnboardingPage[] = [
  {
    title: 'مرحباً بك في سَنَد',
    description: 'تطبيقك الإسلامي الشامل للقرآن الكريم والأحاديث النبوية والمساعد الذكي',
    icon: '🕌',
  },
  {
    title: 'القرآن الكريم',
    description: 'اقرأ القرآن الكريم بخط واضح مع التفاسير المتعددة والترجمات',
    icon: '📖',
  },
  {
    title: 'المساعد الذكي',
    description: 'اسأل أي سؤال إسلامي واحصل على إجابات موثوقة مع المصادر',
    icon: '🤖',
  },
  {
    title: 'مواقيت الصلاة',
    description: 'احصل على مواقيت الصلاة الدقيقة حسب موقعك ومذهبك الفقهي',
    icon: '🕰️',
  },
  {
    title: 'البحث الشامل',
    description: 'ابحث في القرآن والأحاديث والفتاوى بتقنية الذكاء الاصطناعي',
    icon: '🔍',
  },
];

export default function OnboardingPage() {
  const router = useRouter();
  const [currentPage, setCurrentPage] = useState(0);

  const handleNext = () => {
    if (currentPage < onboardingPages.length - 1) {
      setCurrentPage(currentPage + 1);
    } else {
      router.push('/onboarding/preferences');
    }
  };

  const handleSkip = () => {
    router.push('/onboarding/preferences');
  };

  const page = onboardingPages[currentPage];

  return (
    <div className="min-h-screen bg-gradient-to-b from-primary-main to-primary-dark flex flex-col">
      {/* Skip button */}
      <div className="p-6 flex justify-end">
        <button
          onClick={handleSkip}
          className="text-white/90 hover:text-white font-semibold transition-colors"
        >
          تخطي
        </button>
      </div>

      {/* Content */}
      <div className="flex-1 flex flex-col items-center justify-center px-6 pb-20">
        {/* Icon with animation */}
        <div className="mb-12 animate-fade-in-up">
          <div className="w-40 h-40 bg-white/10 backdrop-blur-sm rounded-3xl flex items-center justify-center text-8xl border-2 border-white/20 shadow-2xl">
            {page.icon}
          </div>
        </div>

        {/* Title */}
        <h1 className="text-4xl font-bold text-white text-center mb-6 animate-fade-in-up animation-delay-200">
          {page.title}
        </h1>

        {/* Description */}
        <p className="text-xl text-white/90 text-center max-w-2xl leading-relaxed animate-fade-in-up animation-delay-400">
          {page.description}
        </p>
      </div>

      {/* Bottom section */}
      <div className="p-6 pb-12">
        {/* Page indicators */}
        <div className="flex justify-center gap-2 mb-8">
          {onboardingPages.map((_, index) => (
            <div
              key={index}
              className={`h-2 rounded-full transition-all duration-300 ${
                index === currentPage
                  ? 'w-8 bg-accent-gold'
                  : 'w-2 bg-white/30'
              }`}
            />
          ))}
        </div>

        {/* Next button */}
        <div className="max-w-md mx-auto">
          <button
            onClick={handleNext}
            className="w-full bg-white text-primary-main font-bold py-4 px-8 rounded-xl hover:bg-white/90 transition-all duration-200 shadow-lg hover:shadow-xl transform hover:scale-105"
          >
            {currentPage === onboardingPages.length - 1 ? 'ابدأ الآن' : 'التالي'}
          </button>
        </div>
      </div>
    </div>
  );
}
