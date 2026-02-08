import type { QuranPage, Ayah } from '@/types/quran';
import AyahView from './AyahView';

interface MushafPageViewProps {
  page: QuranPage;
  selectedAyahNumber?: number;
  fontSize: number;
  onAyahClick: (ayah: Ayah) => void;
}

/**
 * Component to display a single page of the Mushaf
 */
export default function MushafPageView({
  page,
  selectedAyahNumber,
  fontSize,
  onAyahClick,
}: MushafPageViewProps) {
  const isFirstAyahOfSurah = page.ayahs.length > 0 && page.ayahs[0].number_in_surah === 1;
  const shouldShowBismillah = isFirstAyahOfSurah && page.surah_number !== 9;

  return (
    <div className="bg-white rounded-xl shadow-lg p-8 border border-[#1B365D]/10">
      {/* Page Header */}
      <div className="flex items-center justify-between pb-4 mb-6 border-b border-[#1B365D]/20">
        <div className="text-[#1B365D] font-tajawal text-sm">
          جزء {page.juz_number}
        </div>
        <div className="text-[#1B365D] font-tajawal font-bold text-lg">
          {page.surah_name}
        </div>
      </div>

      {/* Surah Header with Bismillah */}
      {shouldShowBismillah && (
        <div className="mb-8 p-6 bg-[#1B365D]/5 rounded-lg border border-[#1B365D]/20">
          <div className="text-center text-[#1B365D] font-uthmani text-3xl leading-loose">
            بِسْمِ اللَّهِ الرَّحْمَٰنِ الرَّحِيمِ
          </div>
        </div>
      )}

      {/* Ayahs */}
      <div className="space-y-4">
        {page.ayahs.map((ayah) => (
          <AyahView
            key={ayah.number}
            ayah={ayah}
            isSelected={ayah.number === selectedAyahNumber}
            fontSize={fontSize}
            onClick={() => onAyahClick(ayah)}
          />
        ))}
      </div>

      {/* Page Footer */}
      <div className="flex items-center justify-center pt-6 mt-6 border-t border-[#1B365D]/20">
        <div className="px-6 py-2 bg-[#B8860B]/10 rounded-full">
          <span className="text-[#B8860B] font-tajawal font-bold text-lg">
            {page.page_number}
          </span>
        </div>
      </div>
    </div>
  );
}
