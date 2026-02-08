/**
 * Bookmark List Component
 */
import type { QuranBookmark } from '@/types/quran';
import { BookmarkCard } from './BookmarkCard';

interface BookmarkListProps {
  bookmarks: QuranBookmark[];
  onDelete: (bookmarkId: string) => void;
}

export function BookmarkList({ bookmarks, onDelete }: BookmarkListProps) {
  if (bookmarks.length === 0) {
    return (
      <div className="text-center py-16">
        <div className="text-6xl mb-4">🔖</div>
        <p className="text-gray-500 text-lg">لا توجد علامات مرجعية</p>
        <p className="text-gray-400 text-sm mt-2">
          أضف علامات مرجعية أثناء قراءة القرآن للعودة إليها لاحقاً
        </p>
      </div>
    );
  }

  return (
    <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      {bookmarks.map((bookmark) => (
        <BookmarkCard
          key={bookmark.id}
          bookmark={bookmark}
          onDelete={() => onDelete(bookmark.id)}
        />
      ))}
    </div>
  );
}
