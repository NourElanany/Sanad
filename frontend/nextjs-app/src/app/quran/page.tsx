import { Metadata } from 'next';
import QuranIndexClient from './QuranIndexClient';

export const metadata: Metadata = {
  title: 'القرآن الكريم',
  description: 'قراءة القرآن الكريم كاملاً بخط عثمان طه مع التفاسير المتعددة والترجمات. تصفح السور والأجزاء والأحزاب مع إمكانية البحث والحفظ',
  keywords: [
    'قرآن كريم',
    'قراءة القرآن',
    'تفسير القرآن',
    'سور القرآن',
    'أجزاء القرآن',
    'Quran',
    'Quran Reading',
    'Tafsir',
  ],
  openGraph: {
    title: 'القرآن الكريم | سند',
    description: 'قراءة القرآن الكريم كاملاً مع التفاسير والترجمات',
    type: 'website',
    locale: 'ar_SA',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'القرآن الكريم | سند',
    description: 'قراءة القرآن الكريم كاملاً مع التفاسير والترجمات',
  },
  alternates: {
    canonical: '/quran',
  },
};

export default function QuranIndexPage() {
  return <QuranIndexClient />;
}


