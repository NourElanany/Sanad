'use client';

import React from 'react';
import Link from 'next/link';

export interface IslamicNavItem {
  icon: React.ReactNode;
  label: string;
  href: string;
}

export interface IslamicBottomNavBarProps {
  items: IslamicNavItem[];
  currentPath: string;
  className?: string;
}

export const IslamicBottomNavBar: React.FC<IslamicBottomNavBarProps> = ({
  items,
  currentPath,
  className = '',
}) => {
  return (
    <nav
      className={`fixed bottom-0 left-0 right-0 bg-white border-t border-primary-main border-opacity-10 shadow-lg z-40 ${className}`}
    >
      <div className="flex items-center justify-around py-2 safe-area-inset-bottom">
        {items.map((item, index) => {
          const isActive = currentPath === item.href;
          return (
            <Link
              key={index}
              href={item.href}
              className={`flex flex-col items-center gap-1 px-4 py-2 rounded-xl transition-all duration-200 ${
                isActive
                  ? 'bg-primary-main bg-opacity-10 text-primary-main'
                  : 'text-text-secondary hover:text-primary-main'
              }`}
            >
              <span className="text-2xl">{item.icon}</span>
              <span
                className={`text-xs font-tajawal ${
                  isActive ? 'font-semibold' : 'font-normal'
                }`}
              >
                {item.label}
              </span>
            </Link>
          );
        })}
      </div>
    </nav>
  );
};

export interface IslamicAppBarProps {
  title: string;
  actions?: React.ReactNode;
  onBack?: () => void;
  className?: string;
}

export const IslamicAppBar: React.FC<IslamicAppBarProps> = ({
  title,
  actions,
  onBack,
  className = '',
}) => {
  return (
    <header
      className={`sticky top-0 bg-primary-main text-white shadow-md z-30 ${className}`}
    >
      <div className="flex items-center justify-between px-4 py-4">
        {onBack && (
          <button
            onClick={onBack}
            className="p-2 hover:bg-white hover:bg-opacity-10 rounded-lg transition-colors"
            aria-label="رجوع"
          >
            <svg
              className="w-6 h-6"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M15 19l-7-7 7-7"
              />
            </svg>
          </button>
        )}
        <h1 className="flex-1 text-xl font-semibold font-tajawal text-center">
          {title}
        </h1>
        {actions && <div className="flex items-center gap-2">{actions}</div>}
      </div>
    </header>
  );
};

export interface IslamicDrawerItem {
  icon?: React.ReactNode;
  title: string;
  href?: string;
  onClick?: () => void;
  trailing?: React.ReactNode;
  isDivider?: boolean;
}

export interface IslamicDrawerProps {
  isOpen: boolean;
  onClose: () => void;
  userName: string;
  userEmail?: string;
  userAvatar?: string;
  items: IslamicDrawerItem[];
  onProfileClick?: () => void;
  className?: string;
}

export const IslamicDrawer: React.FC<IslamicDrawerProps> = ({
  isOpen,
  onClose,
  userName,
  userEmail,
  userAvatar,
  items,
  onProfileClick,
  className = '',
}) => {
  return (
    <>
      {/* Backdrop */}
      {isOpen && (
        <div
          className="fixed inset-0 bg-black bg-opacity-50 z-40 transition-opacity"
          onClick={onClose}
        />
      )}

      {/* Drawer */}
      <div
        className={`fixed top-0 right-0 h-full w-80 bg-white shadow-2xl z-50 transform transition-transform duration-300 ${
          isOpen ? 'translate-x-0' : 'translate-x-full'
        } ${className}`}
      >
        {/* Header */}
        <div className="bg-gradient-to-br from-primary-main to-primary-light text-white p-6 pt-16">
          <div
            className="flex items-center gap-4 cursor-pointer"
            onClick={onProfileClick}
          >
            <div className="w-16 h-16 rounded-full bg-white flex items-center justify-center overflow-hidden">
              {userAvatar ? (
                <img
                  src={userAvatar}
                  alt={userName}
                  className="w-full h-full object-cover"
                />
              ) : (
                <svg
                  className="w-10 h-10 text-primary-main"
                  fill="currentColor"
                  viewBox="0 0 20 20"
                >
                  <path
                    fillRule="evenodd"
                    d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z"
                    clipRule="evenodd"
                  />
                </svg>
              )}
            </div>
            <div>
              <h2 className="text-xl font-semibold font-tajawal">{userName}</h2>
              {userEmail && (
                <p className="text-sm text-white text-opacity-80">{userEmail}</p>
              )}
            </div>
          </div>
        </div>

        {/* Menu Items */}
        <nav className="py-2 overflow-y-auto" style={{ maxHeight: 'calc(100vh - 200px)' }}>
          {items.map((item, index) => {
            if (item.isDivider) {
              return (
                <hr
                  key={index}
                  className="my-2 border-primary-main border-opacity-10"
                />
              );
            }

            const content = (
              <div className="flex items-center gap-3 px-6 py-4 hover:bg-primary-main hover:bg-opacity-5 transition-colors cursor-pointer">
                {item.icon && (
                  <span className="text-primary-main text-xl">{item.icon}</span>
                )}
                <span className="flex-1 text-base font-tajawal text-text-primary">
                  {item.title}
                </span>
                {item.trailing && <div>{item.trailing}</div>}
              </div>
            );

            if (item.href) {
              return (
                <Link key={index} href={item.href} onClick={onClose}>
                  {content}
                </Link>
              );
            }

            return (
              <div key={index} onClick={item.onClick}>
                {content}
              </div>
            );
          })}
        </nav>
      </div>
    </>
  );
};

export interface IslamicTabBarProps {
  tabs: string[];
  activeTab: number;
  onTabChange: (index: number) => void;
  className?: string;
}

export const IslamicTabBar: React.FC<IslamicTabBarProps> = ({
  tabs,
  activeTab,
  onTabChange,
  className = '',
}) => {
  return (
    <div
      className={`flex items-center bg-white border-b border-primary-main border-opacity-10 ${className}`}
    >
      {tabs.map((tab, index) => (
        <button
          key={index}
          onClick={() => onTabChange(index)}
          className={`flex-1 py-3 text-base font-tajawal font-semibold transition-colors relative ${
            activeTab === index
              ? 'text-primary-main'
              : 'text-text-secondary hover:text-primary-main'
          }`}
        >
          {tab}
          {activeTab === index && (
            <div className="absolute bottom-0 left-0 right-0 h-0.5 bg-primary-main" />
          )}
        </button>
      ))}
    </div>
  );
};

export default IslamicBottomNavBar;
