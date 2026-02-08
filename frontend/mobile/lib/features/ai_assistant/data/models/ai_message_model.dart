import 'package:equatable/equatable.dart';

/// Model for AI chat messages
class AIMessageModel extends Equatable {
  final String id;
  final String content;
  final MessageRole role;
  final DateTime timestamp;
  final List<SourceModel>? sources;
  final MessageStatus status;
  final String? error;

  const AIMessageModel({
    required this.id,
    required this.content,
    required this.role,
    required this.timestamp,
    this.sources,
    this.status = MessageStatus.sent,
    this.error,
  });

  factory AIMessageModel.fromJson(Map<String, dynamic> json) {
    return AIMessageModel(
      id: json['id'] as String,
      content: json['content'] as String,
      role: MessageRole.values.firstWhere(
        (e) => e.name == json['role'],
        orElse: () => MessageRole.assistant,
      ),
      timestamp: DateTime.parse(json['timestamp'] as String),
      sources: json['sources'] != null
          ? (json['sources'] as List)
              .map((s) => SourceModel.fromJson(s as Map<String, dynamic>))
              .toList()
          : null,
      status: MessageStatus.values.firstWhere(
        (e) => e.name == (json['status'] ?? 'sent'),
        orElse: () => MessageStatus.sent,
      ),
      error: json['error'] as String?,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'content': content,
      'role': role.name,
      'timestamp': timestamp.toIso8601String(),
      'sources': sources?.map((s) => s.toJson()).toList(),
      'status': status.name,
      'error': error,
    };
  }

  AIMessageModel copyWith({
    String? id,
    String? content,
    MessageRole? role,
    DateTime? timestamp,
    List<SourceModel>? sources,
    MessageStatus? status,
    String? error,
  }) {
    return AIMessageModel(
      id: id ?? this.id,
      content: content ?? this.content,
      role: role ?? this.role,
      timestamp: timestamp ?? this.timestamp,
      sources: sources ?? this.sources,
      status: status ?? this.status,
      error: error ?? this.error,
    );
  }

  @override
  List<Object?> get props => [id, content, role, timestamp, sources, status, error];
}

/// Message role enum
enum MessageRole {
  user,
  assistant,
  system,
}

/// Message status enum
enum MessageStatus {
  sending,
  sent,
  streaming,
  error,
}

/// Model for source citations
class SourceModel extends Equatable {
  final String id;
  final String title;
  final String type; // 'quran', 'hadith', 'fatwa', 'tafsir'
  final String reference;
  final String? excerpt;
  final String? url;
  final double? confidence;

  const SourceModel({
    required this.id,
    required this.title,
    required this.type,
    required this.reference,
    this.excerpt,
    this.url,
    this.confidence,
  });

  factory SourceModel.fromJson(Map<String, dynamic> json) {
    return SourceModel(
      id: json['id'] as String,
      title: json['title'] as String,
      type: json['type'] as String,
      reference: json['reference'] as String,
      excerpt: json['excerpt'] as String?,
      url: json['url'] as String?,
      confidence: json['confidence'] != null
          ? (json['confidence'] as num).toDouble()
          : null,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'title': title,
      'type': type,
      'reference': reference,
      'excerpt': excerpt,
      'url': url,
      'confidence': confidence,
    };
  }

  @override
  List<Object?> get props => [id, title, type, reference, excerpt, url, confidence];
}

/// Model for chat session
class ChatSessionModel extends Equatable {
  final String id;
  final DateTime createdAt;
  final DateTime updatedAt;
  final List<AIMessageModel> messages;

  const ChatSessionModel({
    required this.id,
    required this.createdAt,
    required this.updatedAt,
    required this.messages,
  });

  factory ChatSessionModel.fromJson(Map<String, dynamic> json) {
    return ChatSessionModel(
      id: json['id'] as String,
      createdAt: DateTime.parse(json['created_at'] as String),
      updatedAt: DateTime.parse(json['updated_at'] as String),
      messages: (json['messages'] as List)
          .map((m) => AIMessageModel.fromJson(m as Map<String, dynamic>))
          .toList(),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'created_at': createdAt.toIso8601String(),
      'updated_at': updatedAt.toIso8601String(),
      'messages': messages.map((m) => m.toJson()).toList(),
    };
  }

  ChatSessionModel copyWith({
    String? id,
    DateTime? createdAt,
    DateTime? updatedAt,
    List<AIMessageModel>? messages,
  }) {
    return ChatSessionModel(
      id: id ?? this.id,
      createdAt: createdAt ?? this.createdAt,
      updatedAt: updatedAt ?? this.updatedAt,
      messages: messages ?? this.messages,
    );
  }

  @override
  List<Object?> get props => [id, createdAt, updatedAt, messages];
}
