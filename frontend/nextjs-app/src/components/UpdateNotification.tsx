'use client';

import { useEffect, useState } from 'react';
import updateService, { UpdateInfo } from '@/lib/services/update-service';

export default function UpdateNotification() {
  const [updateInfo, setUpdateInfo] = useState<UpdateInfo | null>(null);
  const [showNotification, setShowNotification] = useState(false);

  useEffect(() => {
    // Listen for update events
    const handleUpdateAvailable = (event: Event) => {
      const customEvent = event as CustomEvent<UpdateInfo>;
      setUpdateInfo(customEvent.detail || null);
      setShowNotification(true);
    };

    window.addEventListener('app-update-available', handleUpdateAvailable);

    // Check for updates on mount
    updateService.checkForUpdates().then((update) => {
      if (update) {
        setUpdateInfo(update);
        setShowNotification(true);
      }
    });

    return () => {
      window.removeEventListener('app-update-available', handleUpdateAvailable);
    };
  }, []);

  const handleUpdate = async () => {
    await updateService.applyUpdate();
  };

  const handleDismiss = () => {
    if (!updateInfo?.isMandatory) {
      setShowNotification(false);
    }
  };

  if (!showNotification || !updateInfo) {
    return null;
  }

  return (
    <div className="fixed bottom-4 right-4 left-4 md:left-auto md:w-96 z-50 animate-slide-up">
      <div
        className={`rounded-lg shadow-lg p-4 ${
          updateInfo.isMandatory
            ? 'bg-red-50 border-2 border-red-200'
            : 'bg-blue-50 border-2 border-blue-200'
        }`}
      >
        <div className="flex items-start justify-between mb-3">
          <div className="flex items-center gap-2">
            {updateInfo.isMandatory ? (
              <svg
                className="w-6 h-6 text-red-600"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"
                />
              </svg>
            ) : (
              <svg
                className="w-6 h-6 text-blue-600"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  strokeLinecap="round"
                  strokeLinejoin="round"
                  strokeWidth={2}
                  d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                />
              </svg>
            )}
            <h3
              className={`font-bold text-lg ${
                updateInfo.isMandatory ? 'text-red-700' : 'text-blue-700'
              }`}
            >
              {updateInfo.isMandatory ? 'تحديث مطلوب' : 'تحديث متوفر'}
            </h3>
          </div>
          {!updateInfo.isMandatory && (
            <button
              onClick={handleDismiss}
              className="text-gray-400 hover:text-gray-600"
              aria-label="إغلاق"
            >
              <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path
                  fillRule="evenodd"
                  d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                  clipRule="evenodd"
                />
              </svg>
            </button>
          )}
        </div>

        <div className="mb-3">
          <p className="font-semibold text-gray-900 mb-1">
            الإصدار {updateInfo.version}
          </p>
          <p className="text-sm text-gray-700">{updateInfo.releaseNotes}</p>
        </div>

        {updateInfo.features && updateInfo.features.length > 0 && (
          <div className="mb-3">
            <p className="font-semibold text-sm text-gray-900 mb-1">
              ميزات جديدة:
            </p>
            <ul className="text-sm text-gray-700 space-y-1">
              {updateInfo.features.map((feature, index) => (
                <li key={index} className="flex items-start">
                  <span className="mr-2">•</span>
                  <span>{feature}</span>
                </li>
              ))}
            </ul>
          </div>
        )}

        <button
          onClick={handleUpdate}
          className={`w-full py-2 px-4 rounded-lg font-semibold text-white transition-colors ${
            updateInfo.isMandatory
              ? 'bg-red-600 hover:bg-red-700'
              : 'bg-blue-600 hover:bg-blue-700'
          }`}
        >
          تحديث الآن
        </button>
      </div>
    </div>
  );
}
