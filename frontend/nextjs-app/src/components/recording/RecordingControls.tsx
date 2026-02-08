'use client';

import React from 'react';
import { RecordingState, AudioQuality, AudioQualitySettings } from '@/types/recording';
import { Mic, Pause, Play, Square, X, Clock, HelpCircle } from 'lucide-react';

interface RecordingControlsProps {
  state: RecordingState;
  onRecord: () => void;
  onPause: () => void;
  onResume: () => void;
  onStop: () => void;
  onCancel: () => void;
}

export const RecordingControls: React.FC<RecordingControlsProps> = ({
  state,
  onRecord,
  onPause,
  onResume,
  onStop,
  onCancel,
}) => {
  const getMainButton = () => {
    switch (state) {
      case RecordingState.IDLE:
      case RecordingState.STOPPED:
        return {
          icon: Mic,
          label: 'تسجيل',
          color: 'bg-green-600 hover:bg-green-700',
          onClick: onRecord,
        };
      case RecordingState.RECORDING:
        return {
          icon: Pause,
          label: 'إيقاف مؤقت',
          color: 'bg-yellow-500 hover:bg-yellow-600',
          onClick: onPause,
        };
      case RecordingState.PAUSED:
        return {
          icon: Play,
          label: 'استئناف',
          color: 'bg-green-600 hover:bg-green-700',
          onClick: onResume,
        };
      case RecordingState.PREPARING:
      case RecordingState.PROCESSING:
        return {
          icon: Clock,
          label: 'جاري المعالجة...',
          color: 'bg-gray-400',
          onClick: undefined,
        };
      default:
        return {
          icon: Mic,
          label: 'تسجيل',
          color: 'bg-gray-400',
          onClick: undefined,
        };
    }
  };

  const mainButton = getMainButton();
  const MainIcon = mainButton.icon;

  const showSecondaryButtons =
    state === RecordingState.RECORDING || state === RecordingState.PAUSED;

  return (
    <div className="flex items-center justify-center gap-4">
      {/* Cancel button */}
      {showSecondaryButtons && (
        <ControlButton
          icon={X}
          label="إلغاء"
          color="bg-red-600 hover:bg-red-700"
          onClick={onCancel}
        />
      )}

      {/* Main control button */}
      <div className="flex flex-col items-center gap-2">
        <button
          onClick={mainButton.onClick}
          disabled={!mainButton.onClick}
          className={`
            w-18 h-18 rounded-full flex items-center justify-center
            text-white shadow-lg transition-all duration-200
            ${mainButton.color}
            ${mainButton.onClick ? 'hover:scale-105 active:scale-95' : 'cursor-not-allowed'}
          `}
        >
          <MainIcon className="w-8 h-8" />
        </button>
        <span className="text-sm font-medium text-gray-700 font-tajawal">
          {mainButton.label}
        </span>
      </div>

      {/* Stop button */}
      {showSecondaryButtons && (
        <ControlButton
          icon={Square}
          label="إيقاف"
          color="bg-navy hover:bg-navy-dark"
          onClick={onStop}
        />
      )}
    </div>
  );
};

interface ControlButtonProps {
  icon: React.ElementType;
  label: string;
  color: string;
  onClick: () => void;
}

const ControlButton: React.FC<ControlButtonProps> = ({
  icon: Icon,
  label,
  color,
  onClick,
}) => {
  return (
    <div className="flex flex-col items-center gap-2">
      <button
        onClick={onClick}
        className={`
          w-14 h-14 rounded-full flex items-center justify-center
          text-white shadow-md transition-all duration-200
          hover:scale-105 active:scale-95
          ${color}
        `}
      >
        <Icon className="w-6 h-6" />
      </button>
      <span className="text-sm font-medium text-gray-700 font-tajawal">{label}</span>
    </div>
  );
};

interface RecordingDurationProps {
  duration: number; // in milliseconds
  maxDuration?: number; // in milliseconds
}

export const RecordingDuration: React.FC<RecordingDurationProps> = ({
  duration,
  maxDuration,
}) => {
  const formatTime = (ms: number) => {
    const totalSeconds = Math.floor(ms / 1000);
    const minutes = Math.floor(totalSeconds / 60);
    const seconds = totalSeconds % 60;
    return `${minutes.toString().padStart(2, '0')}:${seconds.toString().padStart(2, '0')}`;
  };

  return (
    <div className="inline-flex items-center gap-2 px-5 py-3 bg-navy/10 border border-navy/30 rounded-full">
      <Clock className="w-5 h-5 text-gold" />
      <span className="text-2xl font-bold text-navy font-tajawal">
        {formatTime(duration)}
      </span>
      {maxDuration && (
        <span className="text-base text-navy/60 font-tajawal">
          / {formatTime(maxDuration)}
        </span>
      )}
    </div>
  );
};

interface AudioQualitySelectorProps {
  selectedQuality: AudioQuality;
  onQualityChanged: (quality: AudioQuality) => void;
  enabled?: boolean;
}

export const AudioQualitySelector: React.FC<AudioQualitySelectorProps> = ({
  selectedQuality,
  onQualityChanged,
  enabled = true,
}) => {
  return (
    <div className="space-y-3">
      <h3 className="text-base font-semibold text-gray-900 font-tajawal">
        جودة التسجيل
      </h3>
      <div className="flex flex-wrap gap-2">
        {Object.entries(AudioQualitySettings).map(([key, settings]) => {
          const quality = key as AudioQuality;
          const isSelected = quality === selectedQuality;

          return (
            <button
              key={quality}
              onClick={() => enabled && onQualityChanged(quality)}
              disabled={!enabled}
              className={`
                px-4 py-3 rounded-xl border-2 transition-all duration-200
                ${
                  isSelected
                    ? 'bg-navy border-navy text-white'
                    : 'bg-gray-50 border-gray-200 text-gray-900 hover:border-navy/50'
                }
                ${enabled ? 'cursor-pointer' : 'cursor-not-allowed opacity-50'}
              `}
            >
              <div className="text-sm font-semibold font-tajawal">
                {settings.label}
              </div>
              <div
                className={`text-xs font-tajawal ${
                  isSelected ? 'text-white/80' : 'text-gray-500'
                }`}
              >
                {settings.sampleRate / 1000}kHz
              </div>
            </button>
          );
        })}
      </div>
    </div>
  );
};
