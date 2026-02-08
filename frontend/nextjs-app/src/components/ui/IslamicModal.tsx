'use client';

import React, { useEffect } from 'react';
import { IslamicButton } from './IslamicButton';

export interface IslamicModalAction {
  label: string;
  onClick?: () => void;
  isPrimary?: boolean;
  dismissOnPress?: boolean;
}

export interface IslamicModalProps {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  children: React.ReactNode;
  actions?: IslamicModalAction[];
  className?: string;
}

export const IslamicModal: React.FC<IslamicModalProps> = ({
  isOpen,
  onClose,
  title,
  children,
  actions,
  className = '',
}) => {
  useEffect(() => {
    if (isOpen) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = 'unset';
    }
    return () => {
      document.body.style.overflow = 'unset';
    };
  }, [isOpen]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black bg-opacity-50 transition-opacity"
        onClick={onClose}
      />

      {/* Modal */}
      <div
        className={`relative bg-white rounded-2xl shadow-2xl max-w-md w-full p-6 transform transition-all ${className}`}
      >
        {/* Title */}
        <h2 className="text-xl font-semibold font-tajawal text-text-primary text-center mb-4">
          {title}
        </h2>

        {/* Content */}
        <div className="mb-6">{children}</div>

        {/* Actions */}
        {actions && actions.length > 0 && (
          <div className="flex items-center justify-end gap-2">
            {actions.map((action, index) => (
              <IslamicButton
                key={index}
                type={action.isPrimary ? 'primary' : 'text'}
                onClick={() => {
                  action.onClick?.();
                  if (action.dismissOnPress) {
                    onClose();
                  }
                }}
              >
                {action.label}
              </IslamicButton>
            ))}
          </div>
        )}
      </div>
    </div>
  );
};

export interface IslamicConfirmationModalProps {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  message: string;
  onConfirm: () => void;
  confirmText?: string;
  cancelText?: string;
  isDangerous?: boolean;
}

export const IslamicConfirmationModal: React.FC<IslamicConfirmationModalProps> = ({
  isOpen,
  onClose,
  title,
  message,
  onConfirm,
  confirmText = 'تأكيد',
  cancelText = 'إلغاء',
  isDangerous = false,
}) => {
  return (
    <IslamicModal
      isOpen={isOpen}
      onClose={onClose}
      title={title}
      actions={[
        {
          label: cancelText,
          isPrimary: false,
          dismissOnPress: true,
        },
        {
          label: confirmText,
          isPrimary: true,
          dismissOnPress: true,
          onClick: onConfirm,
        },
      ]}
    >
      <p className="text-base font-tajawal text-text-secondary text-center">
        {message}
      </p>
    </IslamicModal>
  );
};

export interface IslamicSuccessModalProps {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  message: string;
  buttonText?: string;
}

export const IslamicSuccessModal: React.FC<IslamicSuccessModalProps> = ({
  isOpen,
  onClose,
  title,
  message,
  buttonText = 'حسناً',
}) => {
  return (
    <IslamicModal
      isOpen={isOpen}
      onClose={onClose}
      title={title}
      actions={[
        {
          label: buttonText,
          isPrimary: true,
          dismissOnPress: true,
        },
      ]}
    >
      <div className="flex flex-col items-center gap-4">
        <div className="w-20 h-20 rounded-full bg-green-100 flex items-center justify-center">
          <svg
            className="w-12 h-12 text-green-600"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M5 13l4 4L19 7"
            />
          </svg>
        </div>
        <p className="text-base font-tajawal text-text-secondary text-center">
          {message}
        </p>
      </div>
    </IslamicModal>
  );
};

export interface IslamicErrorModalProps {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  message: string;
  buttonText?: string;
}

export const IslamicErrorModal: React.FC<IslamicErrorModalProps> = ({
  isOpen,
  onClose,
  title,
  message,
  buttonText = 'حسناً',
}) => {
  return (
    <IslamicModal
      isOpen={isOpen}
      onClose={onClose}
      title={title}
      actions={[
        {
          label: buttonText,
          isPrimary: true,
          dismissOnPress: true,
        },
      ]}
    >
      <div className="flex flex-col items-center gap-4">
        <div className="w-20 h-20 rounded-full bg-red-100 flex items-center justify-center">
          <svg
            className="w-12 h-12 text-red-600"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              strokeLinecap="round"
              strokeLinejoin="round"
              strokeWidth={2}
              d="M6 18L18 6M6 6l12 12"
            />
          </svg>
        </div>
        <p className="text-base font-tajawal text-text-secondary text-center">
          {message}
        </p>
      </div>
    </IslamicModal>
  );
};

export interface IslamicBottomSheetProps {
  isOpen: boolean;
  onClose: () => void;
  title?: string;
  children: React.ReactNode;
  className?: string;
}

export const IslamicBottomSheet: React.FC<IslamicBottomSheetProps> = ({
  isOpen,
  onClose,
  title,
  children,
  className = '',
}) => {
  useEffect(() => {
    if (isOpen) {
      document.body.style.overflow = 'hidden';
    } else {
      document.body.style.overflow = 'unset';
    }
    return () => {
      document.body.style.overflow = 'unset';
    };
  }, [isOpen]);

  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50">
      {/* Backdrop */}
      <div
        className="absolute inset-0 bg-black bg-opacity-50 transition-opacity"
        onClick={onClose}
      />

      {/* Bottom Sheet */}
      <div
        className={`absolute bottom-0 left-0 right-0 bg-white rounded-t-3xl shadow-2xl transform transition-transform max-h-[90vh] overflow-hidden ${className}`}
      >
        {/* Handle */}
        <div className="flex justify-center pt-3 pb-2">
          <div className="w-10 h-1 bg-gray-300 rounded-full" />
        </div>

        {/* Title */}
        {title && (
          <>
            <div className="px-5 py-4">
              <h2 className="text-xl font-semibold font-tajawal text-text-primary text-center">
                {title}
              </h2>
            </div>
            <hr className="border-primary-main border-opacity-10" />
          </>
        )}

        {/* Content */}
        <div className="overflow-y-auto p-5" style={{ maxHeight: 'calc(90vh - 100px)' }}>
          {children}
        </div>
      </div>
    </div>
  );
};

export default IslamicModal;
