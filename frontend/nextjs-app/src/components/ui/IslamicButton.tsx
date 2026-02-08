import React from 'react';

export type IslamicButtonType = 'primary' | 'secondary' | 'outlined' | 'text' | 'gradient';

export interface IslamicButtonProps {
  children: React.ReactNode;
  onClick?: () => void;
  type?: IslamicButtonType;
  icon?: React.ReactNode;
  isLoading?: boolean;
  disabled?: boolean;
  className?: string;
  fullWidth?: boolean;
}

export const IslamicButton: React.FC<IslamicButtonProps> = ({
  children,
  onClick,
  type = 'primary',
  icon,
  isLoading = false,
  disabled = false,
  className = '',
  fullWidth = false,
}) => {
  const baseClasses = 'inline-flex items-center justify-center gap-2 px-6 py-4 rounded-xl font-tajawal font-semibold text-base transition-all duration-200 disabled:opacity-50 disabled:cursor-not-allowed';
  
  const typeClasses = {
    primary: 'bg-primary-main text-white hover:bg-primary-light shadow-md hover:shadow-lg',
    secondary: 'bg-secondary-main text-white hover:bg-secondary-light shadow-md hover:shadow-lg',
    outlined: 'border-2 border-primary-main text-primary-main hover:bg-primary-main hover:text-white',
    text: 'text-primary-main hover:bg-primary-main hover:bg-opacity-10',
    gradient: 'bg-gradient-to-br from-primary-main to-primary-light text-white shadow-lg hover:shadow-xl',
  };

  const widthClass = fullWidth ? 'w-full' : '';

  return (
    <button
      onClick={onClick}
      disabled={disabled || isLoading}
      className={`${baseClasses} ${typeClasses[type]} ${widthClass} ${className}`}
    >
      {isLoading ? (
        <div className="w-5 h-5 border-2 border-white border-t-transparent rounded-full animate-spin" />
      ) : (
        <>
          {icon && <span className="text-xl">{icon}</span>}
          {children}
        </>
      )}
    </button>
  );
};

export default IslamicButton;
