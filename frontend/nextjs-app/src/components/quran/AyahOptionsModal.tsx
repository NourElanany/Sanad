'use client';

import { useState } from 'react';
import type { Ayah } from '@/types/quran';
import { QuranService } from '@/lib/services/quran-service';

interface AyahOptionsModalProps {
  ayah: Ayah;
  onClose: () => void;
}

/**
 * Modal to display options for a selected Ayah
 */
export default function AyahOptionsModal({ ayah, onClose }: AyahOptionsModalProps) {
  const [loading, setLoading] = useState(false);

  const handleAddBookmark = async () => {
    setLoading(true);
    try {
      await QuranService.addBookmark({
        surah_number: ayah.surah_number,
        ayah_number: ayah.number_in_surah,
        page_number: ayah.page_number,
      });
      alert('تمت إضافة العلامة المرجعية');
      onClose();
    } catch (error) {
      alert('فشل إضافة العلامة المرجعية');
    } finally {
      setLoading(false);
    }
  };

  const handleTafsir = () => {
    alert('التفسير قريباً');
    onClose();
  };

  const handleAudio = () => {
    alert('تشغيل الصوت قريباً');
    onClose();
  };

  const handleRecitation = () => {
    alert('مصحح التلاوة قريباً');
    onClose();
  };

  const handleShare = () => {
    if (navigator.share) {
      navigator.share({
        title: `سورة ${ayah.surah_number} - آية ${ayah.number_in_surah}`,
        text: ayah.text_arabic,
      });
    } else {
      navigator.clipboard.writeText(ayah.text_arabic);
      alert('تم نسخ الآية');
    }
    onClose();
  };

  return (
    <div 
      className="fixed inset-0 z-50 flex items-end justify-center bg-black/50"
      onClick={onClose}
    >
      <div 
        className="bg-white rounded-t-3xl w-full max-w-2xl p-6 animate-slide-up"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Handle Bar */}
        <div className="flex justify-center mb-4">
          <div className="w-12 h-1 bg-gray-300 rounded-full"></div>
        </div>

        {/* Ayah Info */}
        <div className="text-center mb-6">
          <h3 className="text-[#1B365D] font-tajawal font-bold text-xl">
            سورة {ayah.surah_number} - آية {ayah.number_in_surah}
          </h3>
        </div>

        {/* Options */}
        <div className="space-y-3">
          <OptionButton
            icon={
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
              </svg>
            }
            label="التفسير"
            onClick={handleTafsir}
          />

          <OptionButton
            icon={
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15.536 8.464a5 5 0 010 7.072m2.828-9.9a9 9 0 010 12.728M5.586 15H4a1 1 0 01-1-1v-4a1 1 0 011-1h1.586l4.707-4.707C10.923 3.663 12 4.109 12 5v14c0 .891-1.077 1.337-1.707.707L5.586 15z" />
              </svg>
            }
            label="استماع"
            onClick={handleAudio}
          />

          <OptionButton
            icon={
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z" />
              </svg>
            }
            label="صحح تلاوتي"
            onClick={handleRecitation}
          />

          <OptionButton
            icon={
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z" />
              </svg>
            }
            label="إضافة علامة"
            onClick={handleAddBookmark}
            loading={loading}
          />

          <OptionButton
            icon={
              <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z" />
              </svg>
            }
            label="مشاركة"
            onClick={handleShare}
          />
        </div>
      </div>
    </div>
  );
}

interface OptionButtonProps {
  icon: React.ReactNode;
  label: string;
  onClick: () => void;
  loading?: boolean;
}

function OptionButton({ icon, label, onClick, loading }: OptionButtonProps) {
  return (
    <button
      onClick={onClick}
      disabled={loading}
      className="w-full flex items-center gap-4 p-4 rounded-xl border border-[#1B365D]/20 hover:bg-[#1B365D]/5 transition-colors disabled:opacity-50"
    >
      <div className="text-[#1B365D]">{icon}</div>
      <span className="text-[#1B365D] font-tajawal text-lg">{label}</span>
      {loading && (
        <div className="ml-auto">
          <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-[#1B365D]"></div>
        </div>
      )}
    </button>
  );
}
