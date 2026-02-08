import { MetadataRoute } from 'next'

export default function sitemap(): MetadataRoute.Sitemap {
  const baseUrl = 'https://sanad.app'
  
  return [
    {
      url: baseUrl,
      lastModified: new Date(),
      changeFrequency: 'daily',
      priority: 1,
      alternates: {
        languages: {
          ar: `${baseUrl}/ar`,
          en: `${baseUrl}/en`,
        },
      },
    },
    {
      url: `${baseUrl}/quran`,
      lastModified: new Date(),
      changeFrequency: 'weekly',
      priority: 0.9,
      alternates: {
        languages: {
          ar: `${baseUrl}/ar/quran`,
          en: `${baseUrl}/en/quran`,
        },
      },
    },
    {
      url: `${baseUrl}/hadith`,
      lastModified: new Date(),
      changeFrequency: 'weekly',
      priority: 0.8,
      alternates: {
        languages: {
          ar: `${baseUrl}/ar/hadith`,
          en: `${baseUrl}/en/hadith`,
        },
      },
    },
    {
      url: `${baseUrl}/prayer-times`,
      lastModified: new Date(),
      changeFrequency: 'daily',
      priority: 0.9,
      alternates: {
        languages: {
          ar: `${baseUrl}/ar/prayer-times`,
          en: `${baseUrl}/en/prayer-times`,
        },
      },
    },
    {
      url: `${baseUrl}/ai-assistant`,
      lastModified: new Date(),
      changeFrequency: 'weekly',
      priority: 0.8,
      alternates: {
        languages: {
          ar: `${baseUrl}/ar/ai-assistant`,
          en: `${baseUrl}/en/ai-assistant`,
        },
      },
    },
    {
      url: `${baseUrl}/qibla`,
      lastModified: new Date(),
      changeFrequency: 'monthly',
      priority: 0.7,
      alternates: {
        languages: {
          ar: `${baseUrl}/ar/qibla`,
          en: `${baseUrl}/en/qibla`,
        },
      },
    },
    {
      url: `${baseUrl}/stories`,
      lastModified: new Date(),
      changeFrequency: 'weekly',
      priority: 0.7,
      alternates: {
        languages: {
          ar: `${baseUrl}/ar/stories`,
          en: `${baseUrl}/en/stories`,
        },
      },
    },
    {
      url: `${baseUrl}/about`,
      lastModified: new Date(),
      changeFrequency: 'monthly',
      priority: 0.5,
      alternates: {
        languages: {
          ar: `${baseUrl}/ar/about`,
          en: `${baseUrl}/en/about`,
        },
      },
    },
  ]
}
