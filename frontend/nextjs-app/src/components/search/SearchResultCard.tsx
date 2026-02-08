'use client';

import { useState } from 'react';
import type { SearchResult } from '@/types/search';
import {
  getContentTypeLabel,
  getContentTypeIcon,
  getAuthenticityLabel,
  getAuthenticityColor,
  getSimilarityColor,
} from '@/types/search';

interface SearchResultCardProps {
  result: SearchResult;
}

export function SearchResultCard({ result }: SearchResultCardProps) {
  const { document, similarity_score, highlighted_text, explanation } = result;
  const contentType = document.content_type;
  const isHadith = contentType.includes('hadith');
  const isQuran = contentType === 'quran';
  const isFatwa = contentType === 'fiqh_ruling' || contentType === 'scholar_opinion';
  
  const [showShareMenu, setShowShareMenu] = useState(false);
  const [copied, setCopied] = useState(false);
  
  const authenticityGrade = isHadith
    ? contentType.replace('_hadith', '') as any
    : null;

  // Enhanced text highlighting function
  const highlightText = (text: string): string => {
    if (!highlighted_text) return text;
    // The backend should provide highlighted text with <mark> tags
    return highlighted_text;
  };

  // Share functionality
  const handleShare = async (method: 'copy' | 'twitter' | 'whatsapp' | 'email') => {
    const shareText = `${document.text}\n\nالمصدر: ${document.source}${document.author ? ` - ${document.author}` : ''}`;
    const shareUrl = typeof window !== 'undefined' ? window.location.href : '';

    switch (method) {
      case 'copy':
        await navigator.clipboard.writeText(shareText);
        setCopied(true);
        setTimeout(() => setCopied(false), 2000);
        break;
      case 'twitter':
        window.open(
          `https://twitter.com/intent/tweet?text=${encodeURIComponent(shareText.substring(0, 280))}`,
          '_blank'
        );
        break;
      case 'whatsapp':
        window.open(
          `https://wa.me/?text=${encodeURIComponent(shareText)}`,
          '_blank'
        );
        break;
      case 'email':
        window.location.href = `mailto:?subject=${encodeURIComponent('محتوى إسلامي')}&body=${encodeURIComponent(shareText)}`;
        break;
    }
    setShowShareMenu(false);
  };

  // Render specialized card based on content type
  const renderContentSpecificDetails = () => {
    if (isQuran) {
      return (
        <div className="flex items-center gap-3 text-sm text-gray-600 mb-3">
          <div className="flex items-center gap-1 bg-[#1B365D]/5 px-3 py-1 rounded-lg">
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path d="M9 4.804A7.968 7.968 0 005.5 4c-1.255 0-2.443.29-3.5.804v10A7.969 7.969 0 015.5 14c1.669 0 3.218.51 4.5 1.385A7.962 7.962 0 0114.5 14c1.255 0 2.443.29 3.5.804v-10A7.968 7.968 0 0014.5 4c-1.255 0-2.443.29-3.5.804V12a1 1 0 11-2 0V4.804z" />
            </svg>
            <span className="font-semibold">القرآن الكريم</span>
          </div>
          {document.metadata?.surah_number && (
            <span>سورة {document.metadata.surah_name || document.metadata.surah_number}</span>
          )}
          {document.metadata?.ayah_number && (
            <span>آية {document.metadata.ayah_number}</span>
          )}
        </div>
      );
    }

    if (isHadith) {
      return (
        <div className="flex items-center gap-3 text-sm text-gray-600 mb-3">
          <div className="flex items-center gap-1 bg-[#2D5A27]/5 px-3 py-1 rounded-lg">
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path d="M9 2a1 1 0 000 2h2a1 1 0 100-2H9z" />
              <path fillRule="evenodd" d="M4 5a2 2 0 012-2 3 3 0 003 3h2a3 3 0 003-3 2 2 0 012 2v11a2 2 0 01-2 2H6a2 2 0 01-2-2V5zm3 4a1 1 0 000 2h.01a1 1 0 100-2H7zm3 0a1 1 0 000 2h3a1 1 0 100-2h-3zm-3 4a1 1 0 100 2h.01a1 1 0 100-2H7zm3 0a1 1 0 100 2h3a1 1 0 100-2h-3z" clipRule="evenodd" />
            </svg>
            <span className="font-semibold">الحديث النبوي</span>
          </div>
          {document.metadata?.hadith_number && (
            <span>رقم {document.metadata.hadith_number}</span>
          )}
          {document.metadata?.book && (
            <span>كتاب {document.metadata.book}</span>
          )}
        </div>
      );
    }

    if (isFatwa) {
      return (
        <div className="flex items-center gap-3 text-sm text-gray-600 mb-3">
          <div className="flex items-center gap-1 bg-[#B8860B]/5 px-3 py-1 rounded-lg">
            <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M10 2a1 1 0 011 1v1.323l3.954 1.582 1.599-.8a1 1 0 01.894 1.79l-1.233.616 1.738 5.42a1 1 0 01-.285 1.05A3.989 3.989 0 0115 15a3.989 3.989 0 01-2.667-1.019 1 1 0 01-.285-1.05l1.715-5.349L11 6.477V16h2a1 1 0 110 2H7a1 1 0 110-2h2V6.477L6.237 7.582l1.715 5.349a1 1 0 01-.285 1.05A3.989 3.989 0 015 15a3.989 3.989 0 01-2.667-1.019 1 1 0 01-.285-1.05l1.738-5.42-1.233-.617a1 1 0 01.894-1.788l1.599.799L9 4.323V3a1 1 0 011-1z" clipRule="evenodd" />
            </svg>
            <span className="font-semibold">فتوى شرعية</span>
          </div>
          {document.metadata?.fatwa_number && (
            <span>فتوى رقم {document.metadata.fatwa_number}</span>
          )}
        </div>
      );
    }

    return null;
  };

  return (
    <div className="bg-white rounded-xl border border-gray-200 p-6 hover:shadow-lg transition-shadow relative">
      {/* Header */}
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2 bg-[#1B365D]/10 px-3 py-1.5 rounded-lg">
          <span className="text-lg">{getContentTypeIcon(contentType as any)}</span>
          <span className="text-sm font-semibold text-[#1B365D]">
            {getContentTypeLabel(contentType as any)}
          </span>
        </div>
        <div className="flex items-center gap-2">
          {/* Similarity Score */}
          <div
            className="px-3 py-1 rounded-lg flex items-center gap-1"
            style={{ backgroundColor: `${getSimilarityColor(similarity_score)}20` }}
          >
            <svg
              className="w-4 h-4"
              fill="currentColor"
              viewBox="0 0 20 20"
              style={{ color: getSimilarityColor(similarity_score) }}
            >
              <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
            </svg>
            <span
              className="text-sm font-semibold"
              style={{ color: getSimilarityColor(similarity_score) }}
            >
              {Math.round(similarity_score * 100)}%
            </span>
          </div>

          {/* Share Button */}
          <div className="relative">
            <button
              onClick={() => setShowShareMenu(!showShareMenu)}
              className="p-2 hover:bg-gray-100 rounded-lg transition-colors"
              title="مشاركة"
            >
              <svg className="w-5 h-5 text-gray-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z" />
              </svg>
            </button>

            {/* Share Menu */}
            {showShareMenu && (
              <div className="absolute left-0 mt-2 w-48 bg-white rounded-lg shadow-xl border border-gray-200 z-10">
                <div className="py-2">
                  <button
                    onClick={() => handleShare('copy')}
                    className="w-full px-4 py-2 text-right hover:bg-gray-50 flex items-center gap-3 text-sm"
                  >
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
                    </svg>
                    {copied ? 'تم النسخ!' : 'نسخ النص'}
                  </button>
                  <button
                    onClick={() => handleShare('whatsapp')}
                    className="w-full px-4 py-2 text-right hover:bg-gray-50 flex items-center gap-3 text-sm"
                  >
                    <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                      <path d="M17.472 14.382c-.297-.149-1.758-.867-2.03-.967-.273-.099-.471-.148-.67.15-.197.297-.767.966-.94 1.164-.173.199-.347.223-.644.075-.297-.15-1.255-.463-2.39-1.475-.883-.788-1.48-1.761-1.653-2.059-.173-.297-.018-.458.13-.606.134-.133.298-.347.446-.52.149-.174.198-.298.298-.497.099-.198.05-.371-.025-.52-.075-.149-.669-1.612-.916-2.207-.242-.579-.487-.5-.669-.51-.173-.008-.371-.01-.57-.01-.198 0-.52.074-.792.372-.272.297-1.04 1.016-1.04 2.479 0 1.462 1.065 2.875 1.213 3.074.149.198 2.096 3.2 5.077 4.487.709.306 1.262.489 1.694.625.712.227 1.36.195 1.871.118.571-.085 1.758-.719 2.006-1.413.248-.694.248-1.289.173-1.413-.074-.124-.272-.198-.57-.347m-5.421 7.403h-.004a9.87 9.87 0 01-5.031-1.378l-.361-.214-3.741.982.998-3.648-.235-.374a9.86 9.86 0 01-1.51-5.26c.001-5.45 4.436-9.884 9.888-9.884 2.64 0 5.122 1.03 6.988 2.898a9.825 9.825 0 012.893 6.994c-.003 5.45-4.437 9.884-9.885 9.884m8.413-18.297A11.815 11.815 0 0012.05 0C5.495 0 .16 5.335.157 11.892c0 2.096.547 4.142 1.588 5.945L.057 24l6.305-1.654a11.882 11.882 0 005.683 1.448h.005c6.554 0 11.89-5.335 11.893-11.893a11.821 11.821 0 00-3.48-8.413Z"/>
                    </svg>
                    واتساب
                  </button>
                  <button
                    onClick={() => handleShare('twitter')}
                    className="w-full px-4 py-2 text-right hover:bg-gray-50 flex items-center gap-3 text-sm"
                  >
                    <svg className="w-4 h-4" fill="currentColor" viewBox="0 0 24 24">
                      <path d="M23.953 4.57a10 10 0 01-2.825.775 4.958 4.958 0 002.163-2.723c-.951.555-2.005.959-3.127 1.184a4.92 4.92 0 00-8.384 4.482C7.69 8.095 4.067 6.13 1.64 3.162a4.822 4.822 0 00-.666 2.475c0 1.71.87 3.213 2.188 4.096a4.904 4.904 0 01-2.228-.616v.06a4.923 4.923 0 003.946 4.827 4.996 4.996 0 01-2.212.085 4.936 4.936 0 004.604 3.417 9.867 9.867 0 01-6.102 2.105c-.39 0-.779-.023-1.17-.067a13.995 13.995 0 007.557 2.209c9.053 0 13.998-7.496 13.998-13.985 0-.21 0-.42-.015-.63A9.935 9.935 0 0024 4.59z"/>
                    </svg>
                    تويتر
                  </button>
                  <button
                    onClick={() => handleShare('email')}
                    className="w-full px-4 py-2 text-right hover:bg-gray-50 flex items-center gap-3 text-sm"
                  >
                    <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
                    </svg>
                    بريد إلكتروني
                  </button>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Content-specific details */}
      {renderContentSpecificDetails()}

      {/* Authenticity Badge */}
      {authenticityGrade && (
        <div className="mb-3">
          <span
            className="inline-block px-3 py-1 rounded-lg text-sm font-semibold border"
            style={{
              backgroundColor: `${getAuthenticityColor(authenticityGrade)}20`,
              borderColor: getAuthenticityColor(authenticityGrade),
              color: getAuthenticityColor(authenticityGrade),
            }}
          >
            {getAuthenticityLabel(authenticityGrade)}
          </span>
        </div>
      )}

      {/* Content with highlighting */}
      <p
        className="text-gray-900 text-lg leading-relaxed mb-4"
        style={{ fontFamily: 'Amiri, serif' }}
        dir="rtl"
        dangerouslySetInnerHTML={{ 
          __html: highlightText(document.text).replace(
            /<mark>/g, 
            '<mark style="background-color: #B8860B; color: white; padding: 2px 4px; border-radius: 3px; font-weight: 600;">'
          )
        }}
      />

      {/* Footer */}
      <div className="flex items-center gap-4 text-sm text-gray-600">
        <div className="flex items-center gap-1">
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"
            />
          </svg>
          <span>{document.source}</span>
        </div>
        {document.author && (
          <div className="flex items-center gap-1">
            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
              />
            </svg>
            <span>{document.author}</span>
          </div>
        )}
      </div>

      {/* Explanation */}
      {explanation && (
        <div className="mt-4 bg-[#B8860B]/5 border border-[#B8860B]/20 rounded-lg p-3">
          <div className="flex items-start gap-2">
            <svg
              className="w-5 h-5 text-[#B8860B] flex-shrink-0 mt-0.5"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"
              />
            </svg>
            <p className="text-sm text-gray-700 italic">{explanation}</p>
          </div>
        </div>
      )}
    </div>
  );
}
