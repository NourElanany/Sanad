'use client';

import { IslamicModal, IslamicButton } from '@/components/ui';

interface CalibrationModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export default function CalibrationModal({
  isOpen,
  onClose,
}: CalibrationModalProps) {
  const steps = [
    {
      number: '1',
      text: 'أمسك الجهاز بشكل مستوٍ أمامك',
    },
    {
      number: '2',
      text: 'حرك الجهاز على شكل رقم 8 في الهواء',
    },
    {
      number: '3',
      text: 'كرر الحركة عدة مرات حتى تتحسن الدقة',
    },
  ];

  return (
    <IslamicModal isOpen={isOpen} onClose={onClose} title="معايرة البوصلة">
      <div className="space-y-6">
        {/* Animation placeholder */}
        <div className="flex justify-center">
          <div className="w-48 h-48 bg-[#1B365D]/10 rounded-2xl flex items-center justify-center">
            <svg
              className="w-20 h-20 text-[#1B365D]/50"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M12 18h.01M8 21h8a2 2 0 002-2V5a2 2 0 00-2-2H8a2 2 0 00-2 2v14a2 2 0 002 2z"
              />
            </svg>
          </div>
        </div>

        {/* Instructions */}
        <div className="space-y-4">
          {steps.map((step) => (
            <div key={step.number} className="flex items-start gap-3">
              <div className="flex-shrink-0 w-8 h-8 bg-[#1B365D] rounded-full flex items-center justify-center">
                <span className="text-white font-bold font-['Tajawal']">
                  {step.number}
                </span>
              </div>
              <p className="flex-1 pt-1 text-gray-700 font-['Tajawal']">
                {step.text}
              </p>
            </div>
          ))}
        </div>

        {/* Tips */}
        <div className="bg-[#B8860B]/10 border border-[#B8860B]/30 rounded-lg p-4">
          <div className="flex items-start gap-3">
            <svg
              className="w-6 h-6 text-[#B8860B] flex-shrink-0"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path
                strokeLinecap="round"
                strokeLinejoin="round"
                strokeWidth={2}
                d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"
              />
            </svg>
            <p className="text-sm text-gray-600 font-['Tajawal']">
              ابتعد عن الأجهزة الإلكترونية والمعادن للحصول على أفضل دقة
            </p>
          </div>
        </div>

        {/* Close button */}
        <IslamicButton onClick={onClose} className="w-full">
          فهمت
        </IslamicButton>
      </div>
    </IslamicModal>
  );
}
