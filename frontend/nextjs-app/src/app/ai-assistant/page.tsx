import { Metadata } from 'next';
import AIAssistantClient from './AIAssistantClient';

export const metadata: Metadata = {
  title: 'المساعد الإسلامي الذكي',
  description: 'اسأل أي سؤال إسلامي واحصل على إجابات موثوقة مع المصادر من القرآن والسنة. مساعد ذكي يعتمد على الذكاء الاصطناعي للإجابة على أسئلتك الشرعية',
  keywords: [
    'مساعد إسلامي',
    'فتاوى',
    'أسئلة شرعية',
    'ذكاء اصطناعي إسلامي',
    'Islamic AI',
    'Islamic Assistant',
    'Fatwa',
  ],
  openGraph: {
    title: 'المساعد الإسلامي الذكي | سند',
    description: 'اسأل أي سؤال إسلامي واحصل على إجابات موثوقة مع المصادر',
    type: 'website',
    locale: 'ar_SA',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'المساعد الإسلامي الذكي | سند',
    description: 'اسأل أي سؤال إسلامي واحصل على إجابات موثوقة',
  },
  alternates: {
    canonical: '/ai-assistant',
  },
};

export default function AIAssistantPage() {
  return <AIAssistantClient />;
}


