import type { Ayah } from '@/types/quran';

interface AyahViewProps {
  ayah: Ayah;
  isSelected: boolean;
  fontSize: number;
  onClick: () => void;
}

/**
 * Component to display a single Ayah (verse) with highlighting support
 */
export default function AyahView({
  ayah,
  isSelected,
  fontSize,
  onClick,
}: AyahViewProps) {
  return (
    <div
      onClick={onClick}
      className={`
        p-4 rounded-lg cursor-pointer transition-all duration-200
        ${isSelected 
          ? 'bg-[#B8860B]/15 border-2 border-[#B8860B]' 
          : 'hover:bg-[#1B365D]/5'
        }
      `}
    >
      <div 
        className="text-[#0F1F35] font-uthmani leading-loose text-justify"
        style={{ fontSize: `${fontSize}px`, direction: 'rtl' }}
      >
        {ayah.text_uthmani}
        {' '}
        <span className="inline-flex items-center justify-center w-8 h-8 rounded-full border-2 border-[#1B365D] text-[#1B365D] font-tajawal text-sm font-bold mx-1">
          {ayah.number_in_surah}
        </span>
      </div>
    </div>
  );
}
