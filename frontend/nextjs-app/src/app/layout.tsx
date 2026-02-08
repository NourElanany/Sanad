import type { Metadata, Viewport } from 'next'
import { Tajawal } from 'next/font/google'
import './globals.css'

const tajawal = Tajawal({
  weight: ['300', '400', '500', '700'],
  subsets: ['arabic', 'latin'],
  display: 'swap',
  variable: '--font-tajawal',
})

export const metadata: Metadata = {
  title: {
    default: 'سند - التطبيق الإسلامي الشامل',
    template: '%s | سند'
  },
  description: 'تطبيق إسلامي شامل يوفر القرآن الكريم، الأحاديث النبوية، مواقيت الصلاة، والمساعد الذكي للإجابة على الأسئلة الإسلامية',
  keywords: [
    'قرآن',
    'حديث',
    'صلاة',
    'إسلام',
    'مواقيت الصلاة',
    'القبلة',
    'تفسير',
    'Quran',
    'Hadith',
    'Prayer Times',
    'Islamic App'
  ],
  authors: [{ name: 'Sanad Development Team' }],
  creator: 'Sanad Development Team',
  publisher: 'Sanad',
  formatDetection: {
    email: false,
    address: false,
    telephone: false,
  },
  metadataBase: new URL('https://sanad.app'),
  alternates: {
    canonical: '/',
    languages: {
      'ar': '/ar',
      'en': '/en',
    },
  },
  openGraph: {
    type: 'website',
    locale: 'ar_SA',
    alternateLocale: ['en_US'],
    url: 'https://sanad.app',
    title: 'سند - التطبيق الإسلامي الشامل',
    description: 'تطبيق إسلامي شامل يوفر القرآن الكريم، الأحاديث النبوية، مواقيت الصلاة، والمساعد الذكي',
    siteName: 'سند',
    images: [
      {
        url: '/og-image.png',
        width: 1200,
        height: 630,
        alt: 'سند - التطبيق الإسلامي الشامل',
      },
    ],
  },
  twitter: {
    card: 'summary_large_image',
    title: 'سند - التطبيق الإسلامي الشامل',
    description: 'تطبيق إسلامي شامل يوفر القرآن الكريم، الأحاديث النبوية، مواقيت الصلاة، والمساعد الذكي',
    images: ['/twitter-image.png'],
  },
  robots: {
    index: true,
    follow: true,
    googleBot: {
      index: true,
      follow: true,
      'max-video-preview': -1,
      'max-image-preview': 'large',
      'max-snippet': -1,
    },
  },
  icons: {
    icon: [
      { url: '/favicon.ico' },
      { url: '/icon-192.png', sizes: '192x192', type: 'image/png' },
      { url: '/icon-512.png', sizes: '512x512', type: 'image/png' },
    ],
    apple: [
      { url: '/apple-icon.png' },
    ],
  },
  manifest: '/manifest.json',
}

export const viewport: Viewport = {
  width: 'device-width',
  initialScale: 1,
  maximumScale: 5,
  userScalable: true,
  themeColor: [
    { media: '(prefers-color-scheme: light)', color: '#1B365D' },
    { media: '(prefers-color-scheme: dark)', color: '#0F1F35' },
  ],
}

export default function RootLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <html lang="ar" dir="rtl" className={tajawal.variable}>
      <head>
        {/* Preconnect to external domains */}
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossOrigin="anonymous" />
        
        {/* DNS Prefetch */}
        <link rel="dns-prefetch" href="https://fonts.googleapis.com" />
        
        {/* Structured Data */}
        <script
          type="application/ld+json"
          dangerouslySetInnerHTML={{
            __html: JSON.stringify({
              '@context': 'https://schema.org',
              '@type': 'WebApplication',
              name: 'سند',
              description: 'تطبيق إسلامي شامل',
              applicationCategory: 'LifestyleApplication',
              operatingSystem: 'All',
              offers: {
                '@type': 'Offer',
                price: '0',
                priceCurrency: 'USD',
              },
              aggregateRating: {
                '@type': 'AggregateRating',
                ratingValue: '4.8',
                ratingCount: '1000',
              },
            }),
          }}
        />
      </head>
      <body className="antialiased">
        {children}
      </body>
    </html>
  )
}
