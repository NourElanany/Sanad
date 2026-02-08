'use client';

interface QuickAction {
  title: string;
  icon: string;
  color: string;
  onTap: () => void;
}

interface QuickActionsCardProps {
  actions: QuickAction[];
}

export function QuickActionsCard({ actions }: QuickActionsCardProps) {
  return (
    <div className="bg-white rounded-2xl shadow-lg border border-primary/10 p-6">
      <h3 className="text-xl font-bold text-primary mb-4">الوصول السريع</h3>
      
      <div className="grid grid-cols-3 gap-4">
        {actions.map((action, index) => (
          <button
            key={index}
            onClick={action.onTap}
            className="flex flex-col items-center gap-3 p-4 rounded-xl border-2 transition-all hover:shadow-md"
            style={{
              borderColor: `${action.color}30`,
              background: `linear-gradient(to bottom, ${action.color}15, ${action.color}05)`,
            }}
          >
            <div
              className="w-14 h-14 rounded-full flex items-center justify-center text-2xl"
              style={{ backgroundColor: `${action.color}30` }}
            >
              {action.icon}
            </div>
            <span className="text-sm font-semibold text-gray-800 text-center">
              {action.title}
            </span>
          </button>
        ))}
      </div>
    </div>
  );
}
