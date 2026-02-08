'use client';

interface QuickAction {
  id: string;
  title: string;
  icon: string;
  color: string;
  onTap: () => void;
  badge?: string;
}

interface EnhancedQuickActionsWidgetProps {
  customActions?: QuickAction[];
}

export function EnhancedQuickActionsWidget({ customActions }: EnhancedQuickActionsWidgetProps) {
  const defaultActions: QuickAction[] = [
    {
      id: 'ai_assistant',
      title: 'المساعد الذكي',
      icon: '🤖',
      color: '#1B365D',
      onTap: () => alert('المساعد الذكي قريباً'),
    },
    {
      id: 'qibla',
      title: 'القبلة',
      icon: '🧭',
      color: '#2D5A27',
      onTap: () => alert('بوصلة القبلة قريباً'),
    },
    {
      id: 'adhkar',
      title: 'الأذكار',
      icon: '📿',
      color: '#B8860B',
      onTap: () => alert('الأذكار قريباً'),
    },
    {
      id: 'quran',
      title: 'القرآن',
      icon: '📖',
      color: '#28A745',
      onTap: () => alert('القرآن الكريم قريباً'),
    },
    {
      id: 'hadith',
      title: 'الأحاديث',
      icon: '📚',
      color: '#8B4513',
      onTap: () => alert('الأحاديث قريباً'),
    },
    {
      id: 'tasbih',
      title: 'المسبحة',
      icon: '⭕',
      color: '#17A2B8',
      onTap: () => alert('المسبحة الإلكترونية قريباً'),
    },
    {
      id: 'dua',
      title: 'الأدعية',
      icon: '🤲',
      color: '#E91E63',
      onTap: () => alert('الأدعية قريباً'),
    },
    {
      id: 'mosque_finder',
      title: 'المساجد القريبة',
      icon: '🕌',
      color: '#9C27B0',
      onTap: () => alert('البحث عن المساجد قريباً'),
    },
  ];

  const actions = customActions || defaultActions;

  return (
    <div className="bg-white rounded-2xl shadow-lg border border-primary/10 p-6">
      {/* Header */}
      <div className="flex items-center gap-3 mb-6">
        <div className="bg-gradient-to-br from-primary to-primary/70 p-2.5 rounded-lg">
          <span className="text-xl">⚡</span>
        </div>
        <h3 className="text-xl font-bold text-primary">الوصول السريع</h3>
      </div>

      {/* Grid of actions */}
      <div className="grid grid-cols-4 gap-4">
        {actions.map((action) => (
          <button
            key={action.id}
            onClick={action.onTap}
            className="flex flex-col items-center gap-2 p-3 rounded-xl hover:bg-gray-50 transition-all group"
          >
            {/* Icon container with badge */}
            <div className="relative">
              <div
                className="w-14 h-14 rounded-xl flex items-center justify-center shadow-lg group-hover:scale-110 transition-transform"
                style={{
                  background: `linear-gradient(135deg, ${action.color}, ${action.color}B3)`,
                }}
              >
                <span className="text-2xl">{action.icon}</span>
              </div>
              {action.badge && (
                <div className="absolute -top-1 -right-1 bg-red-500 text-white text-xs font-bold rounded-full w-5 h-5 flex items-center justify-center border-2 border-white">
                  {action.badge}
                </div>
              )}
            </div>
            {/* Title */}
            <span className="text-xs font-semibold text-gray-900 text-center leading-tight">
              {action.title}
            </span>
          </button>
        ))}
      </div>
    </div>
  );
}

interface CompactQuickActionsWidgetProps {
  actions?: QuickAction[];
  maxActions?: number;
}

export function CompactQuickActionsWidget({
  actions,
  maxActions = 3,
}: CompactQuickActionsWidgetProps) {
  const defaultActions: QuickAction[] = [
    {
      id: 'ai_assistant',
      title: 'المساعد الذكي',
      icon: '🤖',
      color: '#1B365D',
      onTap: () => {},
    },
    {
      id: 'qibla',
      title: 'القبلة',
      icon: '🧭',
      color: '#2D5A27',
      onTap: () => {},
    },
    {
      id: 'adhkar',
      title: 'الأذكار',
      icon: '📿',
      color: '#B8860B',
      onTap: () => {},
    },
  ];

  const displayActions = (actions || defaultActions).slice(0, maxActions);

  return (
    <div className="bg-white rounded-2xl shadow-lg border border-primary/10 p-4">
      <div className="flex justify-around">
        {displayActions.map((action) => (
          <button
            key={action.id}
            onClick={action.onTap}
            className="flex flex-col items-center gap-2 p-2 rounded-lg hover:bg-gray-50 transition-all"
          >
            <div
              className="w-12 h-12 rounded-lg flex items-center justify-center"
              style={{
                background: `linear-gradient(135deg, ${action.color}, ${action.color}B3)`,
              }}
            >
              <span className="text-xl">{action.icon}</span>
            </div>
            <span className="text-xs font-semibold text-gray-900 text-center">
              {action.title}
            </span>
          </button>
        ))}
      </div>
    </div>
  );
}
