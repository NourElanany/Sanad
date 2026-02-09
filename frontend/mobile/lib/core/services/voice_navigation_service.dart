import 'package:flutter/material.dart';
import 'package:speech_to_text/speech_to_text.dart' as stt;

/// Service for voice navigation and commands
class VoiceNavigationService {
  final stt.SpeechToText _speech = stt.SpeechToText();
  bool _isInitialized = false;
  bool _isListening = false;

  bool get isListening => _isListening;
  bool get isInitialized => _isInitialized;

  /// Initialize speech recognition
  Future<bool> initialize() async {
    if (_isInitialized) return true;
    
    _isInitialized = await _speech.initialize(
      onError: (error) => debugPrint('Speech recognition error: $error'),
      onStatus: (status) => debugPrint('Speech recognition status: $status'),
    );
    
    return _isInitialized;
  }

  /// Start listening for voice commands
  Future<void> startListening({
    required Function(String) onResult,
    String localeId = 'ar_SA',
  }) async {
    if (!_isInitialized) {
      final initialized = await initialize();
      if (!initialized) return;
    }

    if (_isListening) return;

    _isListening = true;
    await _speech.listen(
      onResult: (result) {
        if (result.finalResult) {
          onResult(result.recognizedWords);
        }
      },
      localeId: localeId,
      listenMode: stt.ListenMode.confirmation,
      cancelOnError: true,
      partialResults: false,
    );
  }

  /// Stop listening
  Future<void> stopListening() async {
    if (!_isListening) return;
    
    await _speech.stop();
    _isListening = false;
  }

  /// Cancel listening
  Future<void> cancelListening() async {
    if (!_isListening) return;
    
    await _speech.cancel();
    _isListening = false;
  }

  /// Process voice command and return navigation action
  VoiceCommand? processCommand(String text) {
    final command = text.toLowerCase().trim();

    // Navigation commands
    if (command.contains('الرئيسية') || command.contains('الصفحة الرئيسية')) {
      return VoiceCommand(type: VoiceCommandType.navigate, target: '/dashboard');
    } else if (command.contains('القرآن') || command.contains('المصحف')) {
      return VoiceCommand(type: VoiceCommandType.navigate, target: '/quran');
    } else if (command.contains('الأحاديث') || command.contains('الحديث')) {
      return VoiceCommand(type: VoiceCommandType.navigate, target: '/hadith');
    } else if (command.contains('المساعد') || command.contains('الذكاء الاصطناعي')) {
      return VoiceCommand(type: VoiceCommandType.navigate, target: '/ai-assistant');
    } else if (command.contains('القبلة') || command.contains('البوصلة')) {
      return VoiceCommand(type: VoiceCommandType.navigate, target: '/qibla');
    } else if (command.contains('المواقيت') || command.contains('الصلاة')) {
      return VoiceCommand(type: VoiceCommandType.navigate, target: '/prayer-times');
    } else if (command.contains('الإعدادات')) {
      return VoiceCommand(type: VoiceCommandType.navigate, target: '/settings');
    } else if (command.contains('البحث')) {
      return VoiceCommand(type: VoiceCommandType.navigate, target: '/search');
    }

    // Action commands
    else if (command.contains('ارجع') || command.contains('رجوع')) {
      return VoiceCommand(type: VoiceCommandType.back);
    } else if (command.contains('تشغيل') || command.contains('شغل')) {
      return VoiceCommand(type: VoiceCommandType.play);
    } else if (command.contains('إيقاف') || command.contains('وقف')) {
      return VoiceCommand(type: VoiceCommandType.pause);
    } else if (command.contains('التالي') || command.contains('التالية')) {
      return VoiceCommand(type: VoiceCommandType.next);
    } else if (command.contains('السابق') || command.contains('السابقة')) {
      return VoiceCommand(type: VoiceCommandType.previous);
    }

    // Surah commands
    else if (command.contains('اقرأ سورة') || command.contains('افتح سورة')) {
      final surahName = _extractSurahName(command);
      if (surahName != null) {
        return VoiceCommand(
          type: VoiceCommandType.openSurah,
          target: surahName,
        );
      }
    }

    return null;
  }

  /// Extract surah name from command
  String? _extractSurahName(String command) {
    final surahs = [
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

    for (final surah in surahs) {
      if (command.contains(surah)) {
        return surah;
      }
    }

    return null;
  }

  /// Dispose resources
  void dispose() {
    _speech.stop();
  }
}

/// Voice command types
enum VoiceCommandType {
  navigate,
  back,
  play,
  pause,
  next,
  previous,
  openSurah,
  search,
}

/// Voice command model
class VoiceCommand {
  final VoiceCommandType type;
  final String? target;

  VoiceCommand({
    required this.type,
    this.target,
  });
}
