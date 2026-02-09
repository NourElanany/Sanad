import { Metadata } from 'next';
import HadithLibraryClient from './HadithLibraryClient';

export const metadata: Metadata = {
  title: 'مكتبة الأحاديث النبوية',
  description: 'مكتبة شاملة من الأحاديث النبوية الصحيحة من صحيح البخاري ومسلم وغيرها. بحث متقدم في الأحاديث مع درجات الصحة والشروحات',
  keywords: [
    'أحاديث نبوية',
    'صحيح البخاري',
    'صحيح مسلم',
    'حديث شريف',
    'Hadith',
    'Sahih Bukhari',
    'Sahih Muslim',
  ],
  openGraph: {
    title: 'مكتبة الأحاديث النبوية | سند',
    description: 'مكتبة شاملة من الأحاديث النبوية الصحيحة مع البحث المتقدم',
    type: 'website',
    locale: 'ar_SA',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'مكتبة الأحاديث النبوية | سند',
    description: 'مكتبة شاملة من الأحاديث النبوية الصحيحة',
  },
  alternates: {
    canonical: '/hadith',
  },
};

export default function HadithLibraryPage() {
  return <HadithLibraryClient />;
}


