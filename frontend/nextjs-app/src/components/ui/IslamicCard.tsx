import React from 'react';

export interface IslamicCardProps {
  children: React.ReactNode;
  onClick?: () => void;
  elevated?: boolean;
  className?: string;
  padding?: string;
}

export const IslamicCard: React.FC<IslamicCardProps> = ({
  children,
  onClick,
  elevated = true,
  className = '',
  padding = 'p-5',
}) => {
  const baseClasses = 'bg-white rounded-2xl border border-primary-main border-opacity-10 transition-all duration-200';
  const elevatedClasses = elevated ? 'shadow-md hover:shadow-lg' : '';
  const clickableClasses = onClick ? 'cursor-pointer hover:border-opacity-20' : '';

  return (
    <div
      onClick={onClick}
      className={`${baseClasses} ${elevatedClasses} ${clickableClasses} ${padding} ${className}`}
    >
      {children}
    </div>
  );
};

export interface IslamicCardWithHeaderProps {
  title: string;
  children: React.ReactNode;
  icon?: React.ReactNode;
  trailing?: React.ReactNode;
  onClick?: () => void;
  elevated?: boolean;
  className?: string;
}

export const IslamicCardWithHeader: React.FC<IslamicCardWithHeaderProps> = ({
  title,
  children,
  icon,
  trailing,
  onClick,
  elevated = true,
  className = '',
}) => {
  return (
    <IslamicCard onClick={onClick} elevated={elevated} padding="p-0" className={className}>
      {/* Header */}
      <div className="flex items-center gap-3 px-4 py-4 bg-primary-main bg-opacity-5 rounded-t-2xl">
        {icon && <span className="text-accent-gold text-2xl">{icon}</span>}
        <h3 className="flex-1 text-lg font-semibold font-tajawal text-text-primary">
          {title}
        </h3>
        {trailing && <div>{trailing}</div>}
      </div>
      {/* Content */}
      <div className="p-5">{children}</div>
    </IslamicCard>
  );
};

export interface IslamicGradientCardProps {
  children: React.ReactNode;
  onClick?: () => void;
  className?: string;
  gradientFrom?: string;
  gradientTo?: string;
}

export const IslamicGradientCard: React.FC<IslamicGradientCardProps> = ({
  children,
  onClick,
  className = '',
  gradientFrom = 'from-primary-main',
  gradientTo = 'to-primary-light',
}) => {
  const baseClasses = 'bg-gradient-to-br rounded-2xl shadow-lg transition-all duration-200';
  const clickableClasses = onClick ? 'cursor-pointer hover:shadow-xl' : '';

  return (
    <div
      onClick={onClick}
      className={`${baseClasses} ${gradientFrom} ${gradientTo} ${clickableClasses} p-5 ${className}`}
    >
      <div className="text-white">{children}</div>
    </div>
  );
};

export default IslamicCard;
