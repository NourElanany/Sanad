/// Smart search bar with voice search support
/// Requirements: 8.1, 8.3

import 'package:flutter/material.dart';
import 'package:speech_to_text/speech_to_text.dart' as stt;
import '../../../../core/theme/app_theme.dart';

class SearchBarWidget extends StatefulWidget {
  final TextEditingController controller;
  final VoidCallback onSearch;
  final VoidCallback onFilterTap;
  final bool hasActiveFilters;

  const SearchBarWidget({
    Key? key,
    required this.controller,
    required this.onSearch,
    required this.onFilterTap,
    this.hasActiveFilters = false,
  }) : super(key: key);

  @override
  State<SearchBarWidget> createState() => _SearchBarWidgetState();
}

class _SearchBarWidgetState extends State<SearchBarWidget> {
  final stt.SpeechToText _speech = stt.SpeechToText();
  bool _isListening = false;
  bool _speechAvailable = false;

  @override
  void initState() {
    super.initState();
    _initSpeech();
  }

  Future<void> _initSpeech() async {
    _speechAvailable = await _speech.initialize(
      onError: (error) => setState(() => _isListening = false),
      onStatus: (status) {
        if (status == 'done' || status == 'notListening') {
          setState(() => _isListening = false);
        }
      },
    );
    setState(() {});
  }

  Future<void> _startListening() async {
    if (!_speechAvailable) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('البحث الصوتي غير متاح')),
      );
      return;
    }

    setState(() => _isListening = true);

    await _speech.listen(
      onResult: (result) {
        setState(() {
          widget.controller.text = result.recognizedWords;
        });

        if (result.finalResult) {
          widget.onSearch();
        }
      },
      localeId: 'ar_SA', // Arabic (Saudi Arabia)
      listenMode: stt.ListenMode.confirmation,
    );
  }

  Future<void> _stopListening() async {
    await _speech.stop();
    setState(() => _isListening = false);
  }

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: AppTheme.background.secondary,
        borderRadius: BorderRadius.circular(12),
        border: Border.all(
          color: AppTheme.primary.main.withOpacity(0.2),
        ),
      ),
      child: Row(
        children: [
          // Search icon
          Padding(
            padding: const EdgeInsets.symmetric(horizontal: 12),
            child: Icon(
              Icons.search,
              color: AppTheme.text.secondary,
              size: 24,
            ),
          ),

          // Text field
          Expanded(
            child: TextField(
              controller: widget.controller,
              textDirection: TextDirection.rtl,
              style: TextStyle(
                fontSize: 16,
                color: AppTheme.text.primary,
              ),
              decoration: InputDecoration(
                hintText: 'ابحث في القرآن والحديث والفتاوى...',
                hintStyle: TextStyle(
                  color: AppTheme.text.secondary,
                ),
                border: InputBorder.none,
                contentPadding: const EdgeInsets.symmetric(vertical: 14),
              ),
              onSubmitted: (_) => widget.onSearch(),
            ),
          ),

          // Voice search button
          if (_speechAvailable)
            IconButton(
              icon: Icon(
                _isListening ? Icons.mic : Icons.mic_none,
                color: _isListening ? AppTheme.status.error : AppTheme.primary.main,
              ),
              onPressed: _isListening ? _stopListening : _startListening,
              tooltip: 'البحث الصوتي',
            ),

          // Filter button
          Stack(
            children: [
              IconButton(
                icon: Icon(
                  Icons.tune,
                  color: widget.hasActiveFilters
                      ? AppTheme.accent.gold
                      : AppTheme.primary.main,
                ),
                onPressed: widget.onFilterTap,
                tooltip: 'الفلاتر',
              ),
              if (widget.hasActiveFilters)
                Positioned(
                  right: 8,
                  top: 8,
                  child: Container(
                    width: 8,
                    height: 8,
                    decoration: BoxDecoration(
                      color: AppTheme.accent.gold,
                      shape: BoxShape.circle,
                    ),
                  ),
                ),
            ],
          ),
        ],
      ),
    );
  }

  @override
  void dispose() {
    _speech.stop();
    super.dispose();
  }
}
