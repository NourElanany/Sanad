'use client';

export function TypingIndicator() {
  return (
    <div className="flex items-center gap-1">
      <div className="flex gap-1">
        <div className="w-2 h-2 bg-current rounded-full animate-bounce" style={{ animationDelay: '0ms' }} />
        <div className="w-2 h-2 bg-current rounded-full animate-bounce" style={{ animationDelay: '150ms' }} />
        <div className="w-2 h-2 bg-current rounded-full animate-bounce" style={{ animationDelay: '300ms' }} />
      </div>
      <span className="text-sm opacity-70 mr-2">جاري الكتابة...</span>
    </div>
  );
}
