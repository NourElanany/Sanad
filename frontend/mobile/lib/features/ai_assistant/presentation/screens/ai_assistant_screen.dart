import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:speech_to_text/speech_to_text.dart' as stt;
import '../../../../core/providers/ai_assistant_provider.dart';
import '../../../../core/theme/app_theme.dart';
import '../widgets/message_bubble.dart';
import '../widgets/source_card.dart';
import '../widgets/voice_input_button.dart';
import '../widgets/typing_indicator.dart';

/// AI Assistant Chat Screen
class AIAssistantScreen extends ConsumerStatefulWidget {
  const AIAssistantScreen({Key? key}) : super(key: key);

  @override
  ConsumerState<AIAssistantScreen> createState() => _AIAssistantScreenState();
}

class _AIAssistantScreenState extends ConsumerState<AIAssistantScreen> {
  final TextEditingController _messageController = TextEditingController();
  final ScrollController _scrollController = ScrollController();
  final stt.SpeechToText _speech = stt.SpeechToText();
  bool _isListening = false;

  @override
  void initState() {
    super.initState();
    _initSpeech();
  }

  Future<void> _initSpeech() async {
    await _speech.initialize(
      onError: (error) => print('Speech recognition error: $error'),
      onStatus: (status) => print('Speech recognition status: $status'),
    );
  }

  @override
  void dispose() {
    _messageController.dispose();
    _scrollController.dispose();
    _speech.stop();
    super.dispose();
  }

  void _scrollToBottom() {
    if (_scrollController.hasClients) {
      Future.delayed(const Duration(milliseconds: 100), () {
        _scrollController.animateTo(
          _scrollController.position.maxScrollExtent,
          duration: const Duration(milliseconds: 300),
          curve: Curves.easeOut,
        );
      });
    }
  }

  Future<void> _sendMessage() async {
    final message = _messageController.text.trim();
    if (message.isEmpty) return;

    _messageController.clear();
    await ref.read(aiAssistantProvider.notifier).sendMessage(message);
    _scrollToBottom();
  }

  Future<void> _startListening() async {
    if (!_speech.isAvailable) {
      ScaffoldMessenger.of(context).showSnackBar(
        const SnackBar(content: Text('التعرف على الصوت غير متاح')),
      );
      return;
    }

    setState(() => _isListening = true);

    await _speech.listen(
      onResult: (result) {
        setState(() {
          _messageController.text = result.recognizedWords;
        });
      },
      localeId: 'ar_SA',
    );
  }

  Future<void> _stopListening() async {
    await _speech.stop();
    setState(() => _isListening = false);
  }

  @override
  Widget build(BuildContext context) {
    final state = ref.watch(aiAssistantProvider);

    return Scaffold(
      backgroundColor: AppColors.background,
      appBar: AppBar(
        backgroundColor: AppColors.primary,
        elevation: 0,
        title: Row(
          children: [
            Container(
              padding: const EdgeInsets.all(8),
              decoration: BoxDecoration(
                color: Colors.white.withOpacity(0.2),
                borderRadius: BorderRadius.circular(8),
              ),
              child: const Icon(
                Icons.psychology,
                color: Colors.white,
                size: 24,
              ),
            ),
            const SizedBox(width: 12),
            const Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    'المساعد الإسلامي الذكي',
                    style: TextStyle(
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                      color: Colors.white,
                    ),
                  ),
                  Text(
                    'اسأل عن أي شيء في الإسلام',
                    style: TextStyle(
                      fontSize: 12,
                      color: Colors.white70,
                    ),
                  ),
                ],
              ),
            ),
          ],
        ),
        actions: [
          IconButton(
            icon: const Icon(Icons.delete_outline, color: Colors.white),
            onPressed: () async {
              final confirm = await showDialog<bool>(
                context: context,
                builder: (context) => AlertDialog(
                  title: const Text('مسح المحادثة'),
                  content: const Text('هل تريد مسح جميع الرسائل؟'),
                  actions: [
                    TextButton(
                      onPressed: () => Navigator.pop(context, false),
                      child: const Text('إلغاء'),
                    ),
                    TextButton(
                      onPressed: () => Navigator.pop(context, true),
                      child: const Text('مسح'),
                    ),
                  ],
                ),
              );

              if (confirm == true) {
                await ref.read(aiAssistantProvider.notifier).clearConversation();
              }
            },
          ),
        ],
      ),
      body: Column(
        children: [
          // Messages list
          Expanded(
            child: state.messages.isEmpty && state.streamingMessage == null
                ? _buildEmptyState()
                : ListView.builder(
                    controller: _scrollController,
                    padding: const EdgeInsets.all(16),
                    itemCount: state.messages.length +
                        (state.streamingMessage != null ? 1 : 0),
                    itemBuilder: (context, index) {
                      if (index < state.messages.length) {
                        final message = state.messages[index];
                        return MessageBubble(
                          message: message,
                          onSourceTap: (source) {
                            _showSourceDetails(context, source);
                          },
                        );
                      } else {
                        // Streaming message
                        return Column(
                          children: [
                            MessageBubble(
                              message: state.streamingMessage!,
                              onSourceTap: (source) {
                                _showSourceDetails(context, source);
                              },
                            ),
                            if (state.isStreaming) const TypingIndicator(),
                          ],
                        );
                      }
                    },
                  ),
          ),

          // Error message
          if (state.error != null)
            Container(
              padding: const EdgeInsets.all(12),
              margin: const EdgeInsets.symmetric(horizontal: 16),
              decoration: BoxDecoration(
                color: Colors.red.shade50,
                borderRadius: BorderRadius.circular(8),
              ),
              child: Row(
                children: [
                  const Icon(Icons.error_outline, color: Colors.red),
                  const SizedBox(width: 8),
                  Expanded(
                    child: Text(
                      state.error!,
                      style: const TextStyle(color: Colors.red),
                    ),
                  ),
                ],
              ),
            ),

          // Input area
          Container(
            padding: const EdgeInsets.all(16),
            decoration: BoxDecoration(
              color: Colors.white,
              boxShadow: [
                BoxShadow(
                  color: Colors.black.withOpacity(0.05),
                  blurRadius: 10,
                  offset: const Offset(0, -2),
                ),
              ],
            ),
            child: SafeArea(
              child: Row(
                children: [
                  // Voice input button
                  VoiceInputButton(
                    isListening: _isListening,
                    onStartListening: _startListening,
                    onStopListening: _stopListening,
                  ),
                  const SizedBox(width: 12),

                  // Text input
                  Expanded(
                    child: Container(
                      decoration: BoxDecoration(
                        color: AppColors.background,
                        borderRadius: BorderRadius.circular(24),
                        border: Border.all(
                          color: AppColors.primary.withOpacity(0.2),
                        ),
                      ),
                      child: TextField(
                        controller: _messageController,
                        textDirection: TextDirection.rtl,
                        maxLines: null,
                        textInputAction: TextInputAction.send,
                        onSubmitted: (_) => _sendMessage(),
                        decoration: const InputDecoration(
                          hintText: 'اكتب سؤالك هنا...',
                          hintTextDirection: TextDirection.rtl,
                          border: InputBorder.none,
                          contentPadding: EdgeInsets.symmetric(
                            horizontal: 20,
                            vertical: 12,
                          ),
                        ),
                      ),
                    ),
                  ),
                  const SizedBox(width: 12),

                  // Send button
                  Container(
                    decoration: BoxDecoration(
                      gradient: LinearGradient(
                        colors: [AppColors.primary, AppColors.primaryDark],
                      ),
                      shape: BoxShape.circle,
                    ),
                    child: IconButton(
                      icon: const Icon(Icons.send, color: Colors.white),
                      onPressed: state.isStreaming ? null : _sendMessage,
                    ),
                  ),
                ],
              ),
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildEmptyState() {
    return Center(
      child: Padding(
        padding: const EdgeInsets.all(32),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Container(
              padding: const EdgeInsets.all(24),
              decoration: BoxDecoration(
                color: AppColors.primary.withOpacity(0.1),
                shape: BoxShape.circle,
              ),
              child: Icon(
                Icons.psychology,
                size: 64,
                color: AppColors.primary,
              ),
            ),
            const SizedBox(height: 24),
            const Text(
              'مرحباً بك في المساعد الإسلامي الذكي',
              style: TextStyle(
                fontSize: 20,
                fontWeight: FontWeight.bold,
              ),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 12),
            Text(
              'اسأل أي سؤال عن الإسلام وسأجيبك بإذن الله\nمع المصادر الموثوقة',
              style: TextStyle(
                fontSize: 14,
                color: Colors.grey.shade600,
              ),
              textAlign: TextAlign.center,
            ),
            const SizedBox(height: 32),
            Wrap(
              spacing: 8,
              runSpacing: 8,
              alignment: WrapAlignment.center,
              children: [
                _buildSuggestionChip('ما حكم الصلاة في الطائرة؟'),
                _buildSuggestionChip('كيف أحسب زكاة المال؟'),
                _buildSuggestionChip('ما هي أركان الإسلام؟'),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildSuggestionChip(String text) {
    return ActionChip(
      label: Text(text),
      onPressed: () {
        _messageController.text = text;
        _sendMessage();
      },
      backgroundColor: AppColors.background,
      side: BorderSide(color: AppColors.primary.withOpacity(0.3)),
    );
  }

  void _showSourceDetails(BuildContext context, source) {
    showModalBottomSheet(
      context: context,
      isScrollControlled: true,
      backgroundColor: Colors.transparent,
      builder: (context) => SourceCard(source: source, isExpanded: true),
    );
  }
}
