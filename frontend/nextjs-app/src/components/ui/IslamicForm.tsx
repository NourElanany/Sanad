'use client';

import React from 'react';

export interface IslamicTextFieldProps {
  label?: string;
  placeholder?: string;
  value?: string;
  onChange?: (value: string) => void;
  error?: string;
  type?: 'text' | 'email' | 'password' | 'number' | 'tel';
  prefixIcon?: React.ReactNode;
  suffixIcon?: React.ReactNode;
  disabled?: boolean;
  required?: boolean;
  maxLength?: number;
  rows?: number;
  className?: string;
  dir?: 'rtl' | 'ltr';
}

export const IslamicTextField: React.FC<IslamicTextFieldProps> = ({
  label,
  placeholder,
  value,
  onChange,
  error,
  type = 'text',
  prefixIcon,
  suffixIcon,
  disabled = false,
  required = false,
  maxLength,
  rows,
  className = '',
  dir = 'rtl',
}) => {
  const InputComponent = rows ? 'textarea' : 'input';

  return (
    <div className={`flex flex-col gap-2 ${className}`}>
      {label && (
        <label className="text-sm font-semibold font-tajawal text-text-primary">
          {label}
          {required && <span className="text-red-500 mr-1">*</span>}
        </label>
      )}
      <div className="relative">
        {prefixIcon && (
          <div className="absolute right-3 top-1/2 -translate-y-1/2 text-primary-main">
            {prefixIcon}
          </div>
        )}
        <InputComponent
          type={type}
          value={value}
          onChange={(e) => onChange?.(e.target.value)}
          placeholder={placeholder}
          disabled={disabled}
          required={required}
          maxLength={maxLength}
          rows={rows}
          dir={dir}
          className={`w-full px-4 py-4 ${prefixIcon ? 'pr-12' : ''} ${
            suffixIcon ? 'pl-12' : ''
          } bg-background-secondary border ${
            error
              ? 'border-red-500 focus:border-red-500'
              : 'border-primary-main border-opacity-20 focus:border-primary-main'
          } rounded-xl font-tajawal text-base text-text-primary placeholder:text-text-disabled transition-colors focus:outline-none focus:ring-2 focus:ring-primary-main focus:ring-opacity-20 disabled:opacity-50 disabled:cursor-not-allowed ${
            rows ? 'resize-y' : ''
          }`}
        />
        {suffixIcon && (
          <div className="absolute left-3 top-1/2 -translate-y-1/2 text-primary-main">
            {suffixIcon}
          </div>
        )}
      </div>
      {error && (
        <p className="text-sm font-tajawal text-red-500">{error}</p>
      )}
    </div>
  );
};

export interface IslamicDropdownOption<T = string> {
  value: T;
  label: string;
}

export interface IslamicDropdownProps<T = string> {
  label?: string;
  value?: T;
  onChange?: (value: T) => void;
  options: IslamicDropdownOption<T>[];
  placeholder?: string;
  error?: string;
  prefixIcon?: React.ReactNode;
  disabled?: boolean;
  required?: boolean;
  className?: string;
}

export function IslamicDropdown<T = string>({
  label,
  value,
  onChange,
  options,
  placeholder,
  error,
  prefixIcon,
  disabled = false,
  required = false,
  className = '',
}: IslamicDropdownProps<T>) {
  return (
    <div className={`flex flex-col gap-2 ${className}`}>
      {label && (
        <label className="text-sm font-semibold font-tajawal text-text-primary">
          {label}
          {required && <span className="text-red-500 mr-1">*</span>}
        </label>
      )}
      <div className="relative">
        {prefixIcon && (
          <div className="absolute right-3 top-1/2 -translate-y-1/2 text-primary-main z-10">
            {prefixIcon}
          </div>
        )}
        <select
          value={value as any}
          onChange={(e) => onChange?.(e.target.value as T)}
          disabled={disabled}
          required={required}
          className={`w-full px-4 py-4 ${
            prefixIcon ? 'pr-12' : ''
          } bg-background-secondary border ${
            error
              ? 'border-red-500 focus:border-red-500'
              : 'border-primary-main border-opacity-20 focus:border-primary-main'
          } rounded-xl font-tajawal text-base text-text-primary transition-colors focus:outline-none focus:ring-2 focus:ring-primary-main focus:ring-opacity-20 disabled:opacity-50 disabled:cursor-not-allowed appearance-none`}
          dir="rtl"
        >
          {placeholder && (
            <option value="" disabled>
              {placeholder}
            </option>
          )}
          {options.map((option, index) => (
            <option key={index} value={option.value as any}>
              {option.label}
            </option>
          ))}
        </select>
        <div className="absolute left-3 top-1/2 -translate-y-1/2 text-primary-main pointer-events-none">
          <svg
            className="w-5 h-5"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M19 9l-7 7-7-7"
            />
          </svg>
        </div>
      </div>
      {error && (
        <p className="text-sm font-tajawal text-red-500">{error}</p>
      )}
    </div>
  );
}

export interface IslamicCheckboxProps {
  label: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
  disabled?: boolean;
  className?: string;
}

export const IslamicCheckbox: React.FC<IslamicCheckboxProps> = ({
  label,
  checked,
  onChange,
  disabled = false,
  className = '',
}) => {
  return (
    <label
      className={`flex items-center gap-3 cursor-pointer py-2 ${
        disabled ? 'opacity-50 cursor-not-allowed' : ''
      } ${className}`}
    >
      <input
        type="checkbox"
        checked={checked}
        onChange={(e) => onChange(e.target.checked)}
        disabled={disabled}
        className="w-5 h-5 text-primary-main bg-background-secondary border-primary-main border-opacity-20 rounded focus:ring-2 focus:ring-primary-main focus:ring-opacity-20 cursor-pointer"
      />
      <span className="text-base font-tajawal text-text-primary">{label}</span>
    </label>
  );
};

export interface IslamicRadioProps<T = string> {
  value: T;
  groupValue: T;
  label: string;
  onChange: (value: T) => void;
  disabled?: boolean;
  className?: string;
}

export function IslamicRadio<T = string>({
  value,
  groupValue,
  label,
  onChange,
  disabled = false,
  className = '',
}: IslamicRadioProps<T>) {
  const isChecked = value === groupValue;

  return (
    <label
      className={`flex items-center gap-3 cursor-pointer py-2 ${
        disabled ? 'opacity-50 cursor-not-allowed' : ''
      } ${className}`}
    >
      <input
        type="radio"
        checked={isChecked}
        onChange={() => onChange(value)}
        disabled={disabled}
        className="w-5 h-5 text-primary-main bg-background-secondary border-primary-main border-opacity-20 focus:ring-2 focus:ring-primary-main focus:ring-opacity-20 cursor-pointer"
      />
      <span className="text-base font-tajawal text-text-primary">{label}</span>
    </label>
  );
}

export interface IslamicSwitchProps {
  label: string;
  subtitle?: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
  disabled?: boolean;
  className?: string;
}

export const IslamicSwitch: React.FC<IslamicSwitchProps> = ({
  label,
  subtitle,
  checked,
  onChange,
  disabled = false,
  className = '',
}) => {
  return (
    <label
      className={`flex items-center justify-between cursor-pointer py-3 ${
        disabled ? 'opacity-50 cursor-not-allowed' : ''
      } ${className}`}
    >
      <div className="flex-1">
        <div className="text-base font-medium font-tajawal text-text-primary">
          {label}
        </div>
        {subtitle && (
          <div className="text-sm font-tajawal text-text-secondary mt-1">
            {subtitle}
          </div>
        )}
      </div>
      <div className="relative">
        <input
          type="checkbox"
          checked={checked}
          onChange={(e) => onChange(e.target.checked)}
          disabled={disabled}
          className="sr-only peer"
        />
        <div
          className={`w-11 h-6 bg-gray-300 rounded-full peer peer-checked:bg-primary-main transition-colors ${
            disabled ? 'cursor-not-allowed' : 'cursor-pointer'
          }`}
        />
        <div
          className={`absolute top-0.5 ${
            checked ? 'left-0.5' : 'right-0.5'
          } w-5 h-5 bg-white rounded-full shadow transition-all`}
        />
      </div>
    </label>
  );
};

export default IslamicTextField;
