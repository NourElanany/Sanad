import 'dart:async';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:uuid/uuid.dart';
import '../services/ai_assistant_service.dart';
import '../../features/ai_assistant/data/models/ai_message_model.dart';
import '../network/dio_client.dart';

/// Provider for AI Assistant Service
final aiAssistantServiceProvider = Provider<AIAssistantService>((ref) {
  final dioClient = ref.watch(dioClientProvider);
  return AIAssistantService(dioClient);
});

/// State for AI Assistant
class AIAssistantState {
  final String sessionId;
  final List<AIMessageModel> messages;
  final bool isLoading;
  final bool isStreaming;
  final String? error;
  final AIMessageModel? streamingMessage;

  const AIAssistantState({
    required this.sessionId,
    this.messages = const [],
    this.isLoading = false,
    this.isStreaming = false,
    this.error,
    this.streamingMessage,
  });

  AIAssistantState copyWith({
    String? sessionId,
    List<AIMessageModel>? messages,
    bool? isLoading,
    bool? isStreaming,
    String? error,
    AIMessageModel? streamingMessage,
  }) {
    return AIAssistantState(
      sessionId: sessionId ?? this.sessionId,
      messages: messages ?? this.messages,
      isLoading: isLoading ?? this.isLoading,
      isStreaming: isStreaming ?? this.isStreaming,
      error: error,
      streamingMessage: streamingMessage,
    );
  }
}

/// Notifier for AI Assistant
class AIAssistantNotifier extends StateNotifier<AIAssistantState> {
  final AIAssistantService _service;
  StreamSubscription<AIMessageModel>? _streamSubscription;

  AIAssistantNotifier(this._service)
      : super(AIAssistantState(sessionId: const Uuid().v4()));

  /// Send a message with streaming response
  Future<void> sendMessage(String message) async {
    if (message.trim().isEmpty) return;

    // Add user message
    final userMessage = AIMessageModel(
      id: const Uuid().v4(),
      content: message,
      role: MessageRole.user,
      timestamp: DateTime.now(),
      status: MessageStatus.sent,
    );

    state = state.copyWith(
      messages: [...state.messages, userMessage],
      isStreaming: true,
      error: null,
    );

    try {
      // Cancel any existing stream
      await _streamSubscription?.cancel();

      // Start streaming response
      final stream = _service.sendMessageStream(message, state.sessionId);
      
      _streamSubscription = stream.listen(
        (aiMessage) {
          // Update streaming message
          state = state.copyWith(
            streamingMessage: aiMessage,
            isStreaming: aiMessage.status == MessageStatus.streaming,
          );

          // If streaming is complete, add to messages
          if (aiMessage.status == MessageStatus.sent) {
            state = state.copyWith(
              messages: [...state.messages, aiMessage],
              streamingMessage: null,
              isStreaming: false,
            );
          }
        },
        onError: (error) {
          state = state.copyWith(
            isStreaming: false,
            error: error.toString(),
            streamingMessage: null,
          );
        },
        onDone: () {
          // Ensure streaming message is added if not already
          if (state.streamingMessage != null) {
            state = state.copyWith(
              messages: [...state.messages, state.streamingMessage!],
              streamingMessage: null,
              isStreaming: false,
            );
          }
        },
      );
    } catch (e) {
      state = state.copyWith(
        isStreaming: false,
        error: e.toString(),
      );
    }
  }

  /// Send a message from voice input
  Future<void> sendVoiceMessage(String audioPath) async {
    state = state.copyWith(isLoading: true, error: null);

    try {
      // Convert speech to text
      final text = await _service.speechToText(audioPath);
      
      state = state.copyWith(isLoading: false);
      
      // Send the transcribed text
      await sendMessage(text);
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        error: 'فشل تحويل الصوت إلى نص: ${e.toString()}',
      );
    }
  }

  /// Load conversation history
  Future<void> loadHistory() async {
    state = state.copyWith(isLoading: true, error: null);

    try {
      final messages = await _service.getHistory(state.sessionId);
      state = state.copyWith(
        messages: messages,
        isLoading: false,
      );
    } catch (e) {
      state = state.copyWith(
        isLoading: false,
        error: e.toString(),
      );
    }
  }

  /// Clear conversation
  Future<void> clearConversation() async {
    try {
      await _service.clearConversation(state.sessionId);
      
      // Create new session
      state = AIAssistantState(sessionId: const Uuid().v4());
    } catch (e) {
      state = state.copyWith(error: e.toString());
    }
  }

  /// Get sources for a query
  Future<List<SourceModel>> getSources(String query) async {
    try {
      return await _service.getSources(query);
    } catch (e) {
      state = state.copyWith(error: e.toString());
      return [];
    }
  }

  @override
  void dispose() {
    _streamSubscription?.cancel();
    super.dispose();
  }
}

/// Provider for AI Assistant Notifier
final aiAssistantProvider =
    StateNotifierProvider<AIAssistantNotifier, AIAssistantState>((ref) {
  final service = ref.watch(aiAssistantServiceProvider);
  return AIAssistantNotifier(service);
});
