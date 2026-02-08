import React from 'react';

export interface IslamicLoadingIndicatorProps {
  size?: 'sm' | 'md' | 'lg';
  color?: string;
  className?: string;
}

export const IslamicLoadingIndicator: React.FC<IslamicLoadingIndicatorProps> = ({
  size = 'md',
  color = 'border-primary-main',
  className = '',
}) => {
  const sizeClasses = {
    sm: 'w-6 h-6 border-2',
    md: 'w-10 h-10 border-3',
    lg: 'w-16 h-16 border-4',
  };

  return (
    <div
      className={`${sizeClasses[size]} ${color} border-t-transparent rounded-full animate-spin ${className}`}
    />
  );
};

export interface IslamicLoadingWithTextProps {
  text: string;
  size?: 'sm' | 'md' | 'lg';
  color?: string;
  className?: string;
}

export const IslamicLoadingWithText: React.FC<IslamicLoadingWithTextProps> = ({
  text,
  size = 'md',
  color,
  className = '',
}) => {
  return (
    <div className={`flex flex-col items-center gap-4 ${className}`}>
      <IslamicLoadingIndicator size={size} color={color} />
      <p className="text-base font-medium font-tajawal text-text-secondary text-center">
        {text}
      </p>
    </div>
  );
};

export interface IslamicShimmerProps {
  width?: string;
  height?: string;
  borderRadius?: string;
  className?: string;
}

export const IslamicShimmer: React.FC<IslamicShimmerProps> = ({
  width = 'w-full',
  height = 'h-24',
  borderRadius = 'rounded-xl',
  className = '',
}) => {
  return (
    <div
      className={`${width} ${height} ${borderRadius} bg-gradient-to-r from-background-secondary via-background-paper to-background-secondary bg-[length:200%_100%] animate-shimmer ${className}`}
    />
  );
};

export interface IslamicPulsingIndicatorProps {
  size?: 'sm' | 'md' | 'lg';
  color?: string;
  className?: string;
}

export const IslamicPulsingIndicator: React.FC<IslamicPulsingIndicatorProps> = ({
  size = 'md',
  color = 'bg-primary-main',
  className = '',
}) => {
  const sizeClasses = {
    sm: 'w-12 h-12',
    md: 'w-16 h-16',
    lg: 'w-24 h-24',
  };

  const innerSizeClasses = {
    sm: 'w-7 h-7',
    md: 'w-10 h-10',
    lg: 'w-14 h-14',
  };

  return (
    <div className={`relative ${sizeClasses[size]} ${className}`}>
      <div
        className={`absolute inset-0 ${color} opacity-30 rounded-full animate-pulse`}
      />
      <div
        className={`absolute inset-0 flex items-center justify-center`}
      >
        <div className={`${innerSizeClasses[size]} ${color} rounded-full`} />
      </div>
    </div>
  );
};

export interface IslamicLoadingOverlayProps {
  message?: string;
  className?: string;
}

export const IslamicLoadingOverlay: React.FC<IslamicLoadingOverlayProps> = ({
  message,
  className = '',
}) => {
  return (
    <div
      className={`fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 ${className}`}
    >
      <div className="bg-white rounded-2xl shadow-2xl p-8 max-w-sm mx-4">
        <div className="flex flex-col items-center gap-6">
          <IslamicPulsingIndicator size="lg" />
          {message && (
            <p className="text-base font-medium font-tajawal text-text-primary text-center">
              {message}
            </p>
          )}
        </div>
      </div>
    </div>
  );
};

export default IslamicLoadingIndicator;
