import 'dart:async';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:uuid/uuid.dart';
import '../services/ai_assistant_service.dart';
import '../../features/ai_assistant/data/models/ai_message_model.dart';
import '../network/dio_client.dart';
import 'cache_provider.dart';
import 'offline_provider.dart';
import 'error_handler_provider.dart';
import 'app_state_provider.dart';

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

/// Notifier for AI Assistant with cache and offline support
class AIAssistantNotifier extends StateNotifier<AIAssistantState> {
  final AIAssistantService _service;
  final CacheService _cacheService;
  final OfflineManager _offlineManager;
  final ErrorHandlerNotifier _errorHandler;
  final bool _isOnline;
  StreamSubscription<AIMessageModel>? _streamSubscription;

  AIAssistantNotifier(
    this._service,
    this._cacheService,
    this._offlineManager,
    this._errorHandler,
    this._isOnline,
  ) : super(AIAssistantState(sessionId: const Uuid().v4())) {
    _loadCachedHistory();
  }

  /// Load cached conversation history
  Future<void> _loadCachedHistory() async {
    final cached = _cacheService.get<List<AIMessageModel>>(
      'ai_conversation_${state.sessionId}',
      (json) => (json as List).map((e) => AIMessageModel.fromJson(e)).toList(),
    );
    
    if (cached != null) {
      state = state.copyWith(messages: cached);
    }
  }

  /// Cache conversation
  Future<void> _cacheConversation() async {
    await _cacheService.put(
      'ai_conversation_${state.sessionId}',
      state.messages.map((m) => m.toJson()).toList(),
      ttl: const Duration(hours: 24),
    );
  }

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

    // Cache immediately
    await _cacheConversation();

    if (!_isOnline) {
      // Queue for offline processing
      await _offlineManager.queueOperation('ai_message', {
        'session_id': state.sessionId,
        'message': message,
        'timestamp': DateTime.now().toIso8601String(),
      });
      
      // Add offline indicator message
      final offlineMessage = AIMessageModel(
        id: const Uuid().v4(),
        content: 'سيتم إرسال رسالتك عند الاتصال بالإنترنت',
        role: MessageRole.assistant,
        timestamp: DateTime.now(),
        status: MessageStatus.sent,
      );
      
      state = state.copyWith(
        messages: [...state.messages, offlineMessage],
        isStreaming: false,
      );
      
      await _cacheConversation();
      return;
    }

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
            _cacheConversation();
          }
        },
        onError: (error) {
          _errorHandler.handleError(error);
          state = state.copyWith(
            isStreaming: false,
            error: AppError.fromException(error).userFriendlyMessage,
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
            _cacheConversation();
          }
        },
      );
    } catch (e) {
      _errorHandler.handleError(e);
      state = state.copyWith(
        isStreaming: false,
        error: AppError.fromException(e).userFriendlyMessage,
      );
    }
  }

  /// Send a message from voice input
  Future<void> sendVoiceMessage(String audioPath) async {
    if (!_isOnline) {
      _errorHandler.handleError(AppError(
        type: ErrorType.network,
        message: 'يتطلب الإدخال الصوتي اتصالاً بالإنترنت',
      ));
      return;
    }

    state = state.copyWith(isLoading: true, error: null);

    try {
      // Convert speech to text
      final text = await _service.speechToText(audioPath);
      
      state = state.copyWith(isLoading: false);
      
      // Send the transcribed text
      await sendMessage(text);
    } catch (e) {
      _errorHandler.handleError(e);
      state = state.copyWith(
        isLoading: false,
        error: AppError.fromException(e).userFriendlyMessage,
      );
    }
  }

  /// Load conversation history
  Future<void> loadHistory() async {
    state = state.copyWith(isLoading: true, error: null);

    try {
      // Try cache first
      final cached = _cacheService.get<List<AIMessageModel>>(
        'ai_conversation_${state.sessionId}',
        (json) => (json as List).map((e) => AIMessageModel.fromJson(e)).toList(),
      );
      
      if (cached != null) {
        state = state.copyWith(messages: cached, isLoading: false);
      }

      // Fetch from server if online
      if (_isOnline) {
        final messages = await _service.getHistory(state.sessionId);
        await _cacheService.put(
          'ai_conversation_${state.sessionId}',
          messages.map((m) => m.toJson()).toList(),
        );
        state = state.copyWith(messages: messages, isLoading: false);
      } else if (cached == null) {
        throw AppError(
          type: ErrorType.network,
          message: 'لا يوجد اتصال بالإنترنت',
        );
      }
    } catch (e) {
      _errorHandler.handleError(e);
      state = state.copyWith(
        isLoading: false,
        error: AppError.fromException(e).userFriendlyMessage,
      );
    }
  }

  /// Clear conversation
  Future<void> clearConversation() async {
    try {
      if (_isOnline) {
        await _service.clearConversation(state.sessionId);
      }
      
      // Clear cache
      await _cacheService.remove('ai_conversation_${state.sessionId}');
      
      // Create new session
      state = AIAssistantState(sessionId: const Uuid().v4());
    } catch (e) {
      _errorHandler.handleError(e);
      state = state.copyWith(error: AppError.fromException(e).userFriendlyMessage);
    }
  }

  /// Get sources for a query
  Future<List<SourceModel>> getSources(String query) async {
    if (!_isOnline) {
      _errorHandler.handleError(AppError(
        type: ErrorType.network,
        message: 'يتطلب عرض المصادر اتصالاً بالإنترنت',
      ));
      return [];
    }

    try {
      // Try cache first
      final cached = _cacheService.get<List<SourceModel>>(
        'ai_sources_$query',
        (json) => (json as List).map((e) => SourceModel.fromJson(e)).toList(),
      );
      
      if (cached != null) {
        return cached;
      }

      final sources = await _service.getSources(query);
      
      // Cache sources
      await _cacheService.put(
        'ai_sources_$query',
        sources.map((s) => s.toJson()).toList(),
        ttl: const Duration(hours: 1),
      );
      
      return sources;
    } catch (e) {
      _errorHandler.handleError(e);
      state = state.copyWith(error: AppError.fromException(e).userFriendlyMessage);
      return [];
    }
  }

  @override
  void dispose() {
    _streamSubscription?.cancel();
    super.dispose();
  }
}

/// Provider for AI Assistant Notifier with integrated state management
final aiAssistantProvider =
    StateNotifierProvider<AIAssistantNotifier, AIAssistantState>((ref) {
  final service = ref.watch(aiAssistantServiceProvider);
  final cacheService = ref.watch(configuredCacheServiceProvider);
  final offlineManager = ref.watch(offlineManagerProvider.notifier);
  final errorHandler = ref.watch(errorHandlerProvider.notifier);
  final isOnline = ref.watch(isOnlineProvider);
  
  return AIAssistantNotifier(
    service,
    cacheService,
    offlineManager,
    errorHandler,
    isOnline,
  );
});
