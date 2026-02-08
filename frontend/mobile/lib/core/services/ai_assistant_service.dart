import 'dart:async';
import 'dart:convert';
import 'package:dio/dio.dart';
import '../network/dio_client.dart';
import '../../features/ai_assistant/data/models/ai_message_model.dart';

/// Service for AI Assistant interactions
class AIAssistantService {
  final DioClient _dioClient;

  AIAssistantService(this._dioClient);

  /// Send a message and get streaming response
  Stream<AIMessageModel> sendMessageStream(String message, String sessionId) async* {
    try {
      final response = await _dioClient.dio.post(
        '/api/ai-assistant/chat',
        data: {
          'message': message,
          'session_id': sessionId,
        },
        options: Options(
          responseType: ResponseType.stream,
          headers: {
            'Accept': 'text/event-stream',
          },
        ),
      );

      final stream = response.data.stream as Stream<List<int>>;
      String buffer = '';
      String currentMessageId = DateTime.now().millisecondsSinceEpoch.toString();
      String accumulatedContent = '';

      await for (final chunk in stream) {
        buffer += utf8.decode(chunk);
        
        // Process complete lines
        final lines = buffer.split('\n');
        buffer = lines.last;
        
        for (int i = 0; i < lines.length - 1; i++) {
          final line = lines[i].trim();
          
          if (line.isEmpty || !line.startsWith('data: ')) continue;
          
          final data = line.substring(6); // Remove 'data: ' prefix
          
          if (data == '[DONE]') {
            // Stream complete
            continue;
          }
          
          try {
            final json = jsonDecode(data) as Map<String, dynamic>;
            
            if (json['type'] == 'content') {
              accumulatedContent += json['content'] as String;
              
              yield AIMessageModel(
                id: currentMessageId,
                content: accumulatedContent,
                role: MessageRole.assistant,
                timestamp: DateTime.now(),
                status: MessageStatus.streaming,
              );
            } else if (json['type'] == 'sources') {
              final sources = (json['sources'] as List)
                  .map((s) => SourceModel.fromJson(s as Map<String, dynamic>))
                  .toList();
              
              yield AIMessageModel(
                id: currentMessageId,
                content: accumulatedContent,
                role: MessageRole.assistant,
                timestamp: DateTime.now(),
                sources: sources,
                status: MessageStatus.sent,
              );
            }
          } catch (e) {
            print('Error parsing SSE data: $e');
          }
        }
      }
    } catch (e) {
      yield AIMessageModel(
        id: DateTime.now().millisecondsSinceEpoch.toString(),
        content: '',
        role: MessageRole.assistant,
        timestamp: DateTime.now(),
        status: MessageStatus.error,
        error: e.toString(),
      );
    }
  }

  /// Send a message and get complete response (non-streaming)
  Future<AIMessageModel> sendMessage(String message, String sessionId) async {
    try {
      final response = await _dioClient.dio.post(
        '/api/ai-assistant/chat',
        data: {
          'message': message,
          'session_id': sessionId,
          'stream': false,
        },
      );

      return AIMessageModel.fromJson(response.data as Map<String, dynamic>);
    } catch (e) {
      throw Exception('Failed to send message: $e');
    }
  }

  /// Get conversation history
  Future<List<AIMessageModel>> getHistory(String sessionId) async {
    try {
      final response = await _dioClient.dio.get(
        '/api/ai-assistant/chat/history/$sessionId',
      );

      return (response.data as List)
          .map((m) => AIMessageModel.fromJson(m as Map<String, dynamic>))
          .toList();
    } catch (e) {
      throw Exception('Failed to get history: $e');
    }
  }

  /// Clear conversation
  Future<void> clearConversation(String sessionId) async {
    try {
      await _dioClient.dio.delete(
        '/api/ai-assistant/chat/clear/$sessionId',
      );
    } catch (e) {
      throw Exception('Failed to clear conversation: $e');
    }
  }

  /// Get sources for verification
  Future<List<SourceModel>> getSources(String query) async {
    try {
      final response = await _dioClient.dio.post(
        '/api/ai-assistant/sources',
        data: {'query': query},
      );

      return (response.data as List)
          .map((s) => SourceModel.fromJson(s as Map<String, dynamic>))
          .toList();
    } catch (e) {
      throw Exception('Failed to get sources: $e');
    }
  }

  /// Convert speech to text
  Future<String> speechToText(String audioPath) async {
    try {
      final formData = FormData.fromMap({
        'audio': await MultipartFile.fromFile(audioPath),
      });

      final response = await _dioClient.dio.post(
        '/api/ai-assistant/speech-to-text',
        data: formData,
      );

      return response.data['text'] as String;
    } catch (e) {
      throw Exception('Failed to convert speech to text: $e');
    }
  }
}
