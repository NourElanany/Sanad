/**
 * Voice Navigation Service for web speech recognition
 */

export enum VoiceCommandType {
  NAVIGATE = 'navigate',
  BACK = 'back',
  PLAY = 'play',
  PAUSE = 'pause',
  NEXT = 'next',
  PREVIOUS = 'previous',
  OPEN_SURAH = 'openSurah',
  SEARCH = 'search',
}

export interface VoiceCommand {
  type: VoiceCommandType;
  target?: string;
}

// Extend Window interface for Web Speech API
declare global {
  interface Window {
    SpeechRecognition: any;
    webkitSpeechRecognition: any;
  }
}

class VoiceNavigationService {
  private recognition: any = null;
  private isInitialized = false;
  private isListening = false;

  /**
   * Initialize speech recognition
   */
  async initialize(): Promise<boolean> {
    if (this.isInitialized) return true;
    if (typeof window === 'undefined') return false;

    const SpeechRecognition =
      window.SpeechRecognition || window.webkitSpeechRecognition;

    if (!SpeechRecognition) {
      console.warn('Speech recognition not supported in this browser');
      return false;
    }

    try {
      this.recognition = new SpeechRecognition();
      this.recognition.lang = 'ar-SA';
      this.recognition.continuous = false;
      this.recognition.interimResults = false;
      this.recognition.maxAlternatives = 1;

      this.isInitialized = true;
      return true;
    } catch (error) {
      console.error('Failed to initialize speech recognition:', error);
      return false;
    }
  }

  /**
   * Start listening for voice commands
   */
  async startListening(
    onResult: (command: VoiceCommand | null) => void,
    onError?: (error: string) => void
  ): Promise<void> {
    if (!this.isInitialized) {
      const initialized = await this.initialize();
      if (!initialized) {
        onError?.('Speech recognition not available');
        return;
      }
    }

    if (this.isListening) return;

    this.recognition.onresult = (event: any) => {
      const transcript = event.results[0][0].transcript;
      const command = this.processCommand(transcript);
      onResult(command);
    };

    this.recognition.onerror = (event: any) => {
      console.error('Speech recognition error:', event.error);
      onError?.(event.error);
      this.isListening = false;
    };

    this.recognition.onend = () => {
      this.isListening = false;
    };

    try {
      this.recognition.start();
      this.isListening = true;
    } catch (error) {
      console.error('Failed to start speech recognition:', error);
      onError?.('Failed to start listening');
    }
  }

  /**
   * Stop listening
   */
  stopListening(): void {
    if (!this.isListening || !this.recognition) return;

    try {
      this.recognition.stop();
      this.isListening = false;
    } catch (error) {
      console.error('Failed to stop speech recognition:', error);
    }
  }

  /**
   * Process voice command
   */
  processCommand(text: string): VoiceCommand | null {
    const command = text.toLowerCase().trim();

    // Navigation commands
    if (command.includes('الرئيسية') || command.includes('الصفحة الرئيسية')) {
      return { type: VoiceCommandType.NAVIGATE, target: '/dashboard' };
    } else if (command.includes('القرآن') || command.includes('المصحف')) {
      return { type: VoiceCommandType.NAVIGATE, target: '/quran' };
    } else if (command.includes('الأحاديث') || command.includes('الحديث')) {
      return { type: VoiceCommandType.NAVIGATE, target: '/hadith' };
    } else if (command.includes('المساعد') || command.includes('الذكاء الاصطناعي')) {
      return { type: VoiceCommandType.NAVIGATE, target: '/ai-assistant' };
    } else if (command.includes('القبلة') || command.includes('البوصلة')) {
      return { type: VoiceCommandType.NAVIGATE, target: '/qibla' };
    } else if (command.includes('المواقيت') || command.includes('الصلاة')) {
      return { type: VoiceCommandType.NAVIGATE, target: '/dashboard' };
    } else if (command.includes('الإعدادات')) {
      return { type: VoiceCommandType.NAVIGATE, target: '/settings' };
    } else if (command.includes('البحث')) {
      return { type: VoiceCommandType.NAVIGATE, target: '/search' };
    }

    // Action commands
    else if (command.includes('ارجع') || command.includes('رجوع')) {
      return { type: VoiceCommandType.BACK };
    } else if (command.includes('تشغيل') || command.includes('شغل')) {
      return { type: VoiceCommandType.PLAY };
    } else if (command.includes('إيقاف') || command.includes('وقف')) {
      return { type: VoiceCommandType.PAUSE };
    } else if (command.includes('التالي') || command.includes('التالية')) {
      return { type: VoiceCommandType.NEXT };
    } else if (command.includes('السابق') || command.includes('السابقة')) {
      return { type: VoiceCommandType.PREVIOUS };
    }

    // Surah commands
    else if (command.includes('اقرأ سورة') || command.includes('افتح سورة')) {
      const surahName = this.extractSurahName(command);
      if (surahName) {
        return {
          type: VoiceCommandType.OPEN_SURAH,
          target: surahName,
        };
      }
    }

    return null;
  }

  /**
   * Extract surah name from command
   */
  private extractSurahName(command: string): string | null {
    const surahs = [
      'الفاتحة', 'البقرة', 'آل عمران', 'النساء', 'المائدة', 'الأنعام',
      'الأعراف', 'الأنفال', 'التوبة', 'يونس', 'هود', 'يوسف',
      'الرعد', 'إبراهيم', 'الحجر', 'النحل', 'الإسراء', 'الكهف',
      'مريم', 'طه', 'الأنبياء', 'الحج', 'المؤمنون', 'النور',
      'الفرقان', 'الشعراء', 'النمل', 'القصص', 'العنكبوت', 'الروم',
      'لقمان', 'السجدة', 'الأحزاب', 'سبأ', 'فاطر', 'يس',
      'الصافات', 'ص', 'الزمر', 'غافر', 'فصلت', 'الشورى',
      'الزخرف', 'الدخان', 'الجاثية', 'الأحقاف', 'محمد', 'الفتح',
      'الحجرات', 'ق', 'الذاريات', 'الطور', 'النجم', 'القمر',
      'الرحمن', 'الواقعة', 'الحديد', 'المجادلة', 'الحشر', 'الممتحنة',
      'الصف', 'الجمعة', 'المنافقون', 'التغابن', 'الطلاق', 'التحريم',
      'الملك', 'القلم', 'الحاقة', 'المعارج', 'نوح', 'الجن',
      'المزمل', 'المدثر', 'القيامة', 'الإنسان', 'المرسلات', 'النبأ',
      'النازعات', 'عبس', 'التكوير', 'الانفطار', 'المطففين', 'الانشقاق',
      'البروج', 'الطارق', 'الأعلى', 'الغاشية', 'الفجر', 'البلد',
      'الشمس', 'الليل', 'الضحى', 'الشرح', 'التين', 'العلق',
      'القدر', 'البينة', 'الزلزلة', 'العاديات', 'القارعة', 'التكاثر',
      'العصر', 'الهمزة', 'الفيل', 'قريش', 'الماعون', 'الكوثر',
      'الكافرون', 'النصر', 'المسد', 'الإخلاص', 'الفلق', 'الناس',
    ];

    for (const surah of surahs) {
      if (command.includes(surah)) {
        return surah;
      }
    }

    return null;
  }

  /**
   * Check if listening
   */
  getIsListening(): boolean {
    return this.isListening;
  }

  /**
   * Check if initialized
   */
  getIsInitialized(): boolean {
    return this.isInitialized;
  }
}

// Export singleton instance
export const voiceNavigationService = new VoiceNavigationService();
