import { Metadata } from 'next';
import PrayerTimesClient from './PrayerTimesClient';

export const metadata: Metadata = {
  title: 'مواقيت الصلاة',
  description: 'مواقيت الصلاة الدقيقة حسب موقعك مع التقويم الهجري والتنبيهات. احصل على أوقات الفجر والظهر والعصر والمغرب والعشاء بدقة عالية',
  keywords: [
    'مواقيت الصلاة',
    'أوقات الصلاة',
    'التقويم الهجري',
    'الأذان',
    'Prayer Times',
    'Salah Times',
    'Islamic Calendar',
    'Adhan',
  ],
  openGraph: {
    title: 'مواقيت الصلاة | سند',
    description: 'مواقيت الصلاة الدقيقة حسب موقعك مع التقويم الهجري',
    type: 'website',
    locale: 'ar_SA',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'مواقيت الصلاة | سند',
    description: 'مواقيت الصلاة الدقيقة حسب موقعك مع التقويم الهجري',
  },
  alternates: {
    canonical: '/prayer-times',
  },
};

export default function PrayerTimesPage() {
  return <PrayerTimesClient />;
}
