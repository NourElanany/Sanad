import { Metadata } from 'next';
import StoriesLibraryClient from './StoriesLibraryClient';

export const metadata: Metadata = {
  title: 'مكتبة القصص الإسلامية',
  description: 'مكتبة شاملة من القصص الإسلامية التربوية تشمل قصص الأنبياء والصحابة والتابعين مع الدروس المستفادة والمراجع الموثوقة',
  keywords: [
    'قصص الأنبياء',
    'قصص الصحابة',
    'قصص إسلامية',
    'قصص تربوية',
    'قصص القرآن',
    'Islamic Stories',
    'Prophets Stories',
    'Sahaba Stories',
  ],
  openGraph: {
    title: 'مكتبة القصص الإسلامية | سند',
    description: 'مكتبة شاملة من القصص الإسلامية التربوية مع الدروس المستفادة',
    type: 'website',
    locale: 'ar_SA',
  },
  twitter: {
    card: 'summary_large_image',
    title: 'مكتبة القصص الإسلامية | سند',
    description: 'مكتبة شاملة من القصص الإسلامية التربوية مع الدروس المستفادة',
  },
  alternates: {
    canonical: '/stories',
  },
};

export default function StoriesPage() {
  return <StoriesLibraryClient />;
}
