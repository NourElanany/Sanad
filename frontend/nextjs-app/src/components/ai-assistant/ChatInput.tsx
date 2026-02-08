'use client';

import { useState, useRef, useEffect } from 'react';

interface ChatInputProps {
  onSendMessage: (message: string) => void;
  disabled?: boolean;
}

export function ChatInput({ onSendMessage, disabled }: ChatInputProps) {
  const [message, setMessage] = useState('');
  const [isRecording, setIsRecording] = useState(false);
  const [mediaRecorder, setMediaRecorder] = useState<MediaRecorder | null>(null);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    // Auto-resize textarea
    if (textareaRef.current) {
      textareaRef.current.style.height = 'auto';
      textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`;
    }
  }, [message]);

  const handleSend = () => {
    if (message.trim() && !disabled) {
      onSendMessage(message.trim());
      setMessage('');
    }
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  const startRecording = async () => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      const recorder = new MediaRecorder(stream);
      const audioChunks: Blob[] = [];

      recorder.ondataavailable = (event) => {
        audioChunks.push(event.data);
      };

      recorder.onstop = async () => {
        const audioBlob = new Blob(audioChunks, { type: 'audio/wav' });
        
        // TODO: Send audio to backend for transcription
        // For now, show a message
        setMessage('تم تسجيل الصوت - جاري التحويل إلى نص...');
        
        // Stop all tracks
        stream.getTracks().forEach(track => track.stop());
      };

      recorder.start();
      setMediaRecorder(recorder);
      setIsRecording(true);
    } catch (error) {
      console.error('Error accessing microphone:', error);
      alert('فشل الوصول إلى الميكروفون. يرجى التحقق من الأذونات.');
    }
  };

  const stopRecording = () => {
    if (mediaRecorder && isRecording) {
      mediaRecorder.stop();
      setIsRecording(false);
      setMediaRecorder(null);
    }
  };

  return (
    <div className="flex items-end gap-3">
      {/* Voice Input Button */}
      <button
        onClick={isRecording ? stopRecording : startRecording}
        disabled={disabled}
        className={`flex-shrink-0 p-3 rounded-full transition-all ${
          isRecording
            ? 'bg-red-500 hover:bg-red-600 animate-pulse'
            : 'bg-gradient-to-br from-[#B8860B] to-[#DAA520] hover:from-[#DAA520] hover:to-[#B8860B]'
        } text-white shadow-lg disabled:opacity-50 disabled:cursor-not-allowed`}
        title={isRecording ? 'إيقاف التسجيل' : 'تسجيل صوتي'}
      >
        {isRecording ? (
          <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 10a1 1 0 011-1h4a1 1 0 011 1v4a1 1 0 01-1 1h-4a1 1 0 01-1-1v-4z" />
          </svg>
        ) : (
          <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 11a7 7 0 01-7 7m0 0a7 7 0 01-7-7m7 7v4m0 0H8m4 0h4m-4-8a3 3 0 01-3-3V5a3 3 0 116 0v6a3 3 0 01-3 3z" />
          </svg>
        )}
      </button>

      {/* Text Input */}
      <div className="flex-1 relative">
        <textarea
          ref={textareaRef}
          value={message}
          onChange={(e) => setMessage(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="اكتب سؤالك هنا..."
          disabled={disabled}
          dir="rtl"
          rows={1}
          className="w-full px-6 py-3 pr-12 bg-gray-50 border border-gray-300 rounded-2xl focus:outline-none focus:ring-2 focus:ring-[#1B365D] focus:border-transparent resize-none max-h-32 disabled:opacity-50 disabled:cursor-not-allowed"
        />
        
        {/* Character count (optional) */}
        {message.length > 0 && (
          <div className="absolute bottom-2 left-2 text-xs text-gray-400">
            {message.length}
          </div>
        )}
      </div>

      {/* Send Button */}
      <button
        onClick={handleSend}
        disabled={!message.trim() || disabled}
        className="flex-shrink-0 p-3 rounded-full bg-gradient-to-br from-[#1B365D] to-[#2E4A6B] hover:from-[#2E4A6B] hover:to-[#1B365D] text-white shadow-lg transition-all disabled:opacity-50 disabled:cursor-not-allowed"
        title="إرسال"
      >
        <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 19l9 2-9-18-9 18 9-2zm0 0v-8" />
        </svg>
      </button>
    </div>
  );
}
