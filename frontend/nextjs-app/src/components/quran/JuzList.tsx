/**
 * Juz List Component
 */
import type { Juz } from '@/types/quran';
import { JuzCard } from './JuzCard';

interface JuzListProps {
  juzs: Juz[];
}

export function JuzList({ juzs }: JuzListProps) {
  if (juzs.length === 0) {
    return (
      <div className="text-center py-16">
        <div className="text-6xl mb-4">📚</div>
        <p className="text-gray-500 text-lg">لا توجد أجزاء</p>
      </div>
    );
  }

  return (
    <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      {juzs.map((juz) => (
        <JuzCard key={juz.number} juz={juz} />
      ))}
    </div>
  );
}
