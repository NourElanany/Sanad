import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import '../../../../core/theme/app_theme.dart';
import '../../../ai_assistant/data/models/ai_message_model.dart';
import 'source_card.dart';

/// Message bubble widget for chat interface
class MessageBubble extends StatelessWidget {
  final AIMessageModel message;
  final Function(SourceModel)? onSourceTap;

  const MessageBubble({
    Key? key,
    required this.message,
    this.onSourceTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final isUser = message.role == MessageRole.user;

    return Padding(
      padding: const EdgeInsets.only(bottom: 16),
      child: Row(
        mainAxisAlignment:
            isUser ? MainAxisAlignment.end : MainAxisAlignment.start,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (!isUser) ...[
            _buildAvatar(isUser),
            const SizedBox(width: 8),
          ],
          Flexible(
            child: Column(
              crossAxisAlignment:
                  isUser ? CrossAxisAlignment.end : CrossAxisAlignment.start,
              children: [
                Container(
                  padding: const EdgeInsets.all(16),
                  decoration: BoxDecoration(
                    color: isUser ? AppColors.primary : Colors.white,
                    borderRadius: BorderRadius.only(
                      topLeft: const Radius.circular(20),
                      topRight: const Radius.circular(20),
                      bottomLeft: Radius.circular(isUser ? 20 : 4),
                      bottomRight: Radius.circular(isUser ? 4 : 20),
                    ),
                    boxShadow: [
                      BoxShadow(
                        color: Colors.black.withOpacity(0.05),
                        blurRadius: 8,
                        offset: const Offset(0, 2),
                      ),
                    ],
                  ),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      // Message content
                      SelectableText(
                        message.content,
                        style: TextStyle(
                          fontSize: 15,
                          color: isUser ? Colors.white : AppColors.textPrimary,
                          height: 1.5,
                        ),
                        textDirection: TextDirection.rtl,
                      ),

                      // Status indicator
                      if (message.status == MessageStatus.streaming)
                        Padding(
                          padding: const EdgeInsets.only(top: 8),
                          child: Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              SizedBox(
                                width: 12,
                                height: 12,
                                child: CircularProgressIndicator(
                                  strokeWidth: 2,
                                  valueColor: AlwaysStoppedAnimation<Color>(
                                    isUser
                                        ? Colors.white
                                        : AppColors.primary,
                                  ),
                                ),
                              ),
                              const SizedBox(width: 8),
                              Text(
                                'جاري الكتابة...',
                                style: TextStyle(
                                  fontSize: 12,
                                  color: isUser
                                      ? Colors.white70
                                      : Colors.grey.shade600,
                                ),
                              ),
                            ],
                          ),
                        ),

                      // Error indicator
                      if (message.status == MessageStatus.error &&
                          message.error != null)
                        Padding(
                          padding: const EdgeInsets.only(top: 8),
                          child: Row(
                            mainAxisSize: MainAxisSize.min,
                            children: [
                              const Icon(
                                Icons.error_outline,
                                size: 16,
                                color: Colors.red,
                              ),
                              const SizedBox(width: 4),
                              Flexible(
                                child: Text(
                                  message.error!,
                                  style: const TextStyle(
                                    fontSize: 12,
                                    color: Colors.red,
                                  ),
                                ),
                              ),
                            ],
                          ),
                        ),
                    ],
                  ),
                ),

                // Sources section
                if (!isUser && message.sources != null && message.sources!.isNotEmpty)
                  Padding(
                    padding: const EdgeInsets.only(top: 12),
                    child: Column(
                      crossAxisAlignment: CrossAxisAlignment.start,
                      children: [
                        Row(
                          children: [
                            Icon(
                              Icons.library_books,
                              size: 16,
                              color: AppColors.accent,
                            ),
                            const SizedBox(width: 4),
                            Text(
                              'المصادر (${message.sources!.length})',
                              style: TextStyle(
                                fontSize: 13,
                                fontWeight: FontWeight.bold,
                                color: AppColors.accent,
                              ),
                            ),
                          ],
                        ),
                        const SizedBox(height: 8),
                        ...message.sources!.map((source) => Padding(
                              padding: const EdgeInsets.only(bottom: 8),
                              child: SourceCard(
                                source: source,
                                onTap: onSourceTap != null
                                    ? () => onSourceTap!(source)
                                    : null,
                              ),
                            )),
                      ],
                    ),
                  ),

                // Timestamp
                Padding(
                  padding: const EdgeInsets.only(top: 4),
                  child: Text(
                    _formatTime(message.timestamp),
                    style: TextStyle(
                      fontSize: 11,
                      color: Colors.grey.shade500,
                    ),
                  ),
                ),
              ],
            ),
          ),
          if (isUser) ...[
            const SizedBox(width: 8),
            _buildAvatar(isUser),
          ],
        ],
      ),
    );
  }

  Widget _buildAvatar(bool isUser) {
    return Container(
      width: 36,
      height: 36,
      decoration: BoxDecoration(
        color: isUser ? AppColors.primary : AppColors.accent,
        shape: BoxShape.circle,
      ),
      child: Icon(
        isUser ? Icons.person : Icons.psychology,
        color: Colors.white,
        size: 20,
      ),
    );
  }

  String _formatTime(DateTime time) {
    final now = DateTime.now();
    final difference = now.difference(time);

    if (difference.inMinutes < 1) {
      return 'الآن';
    } else if (difference.inHours < 1) {
      return '${difference.inMinutes} دقيقة';
    } else if (difference.inDays < 1) {
      return '${difference.inHours} ساعة';
    } else {
      return '${time.day}/${time.month}/${time.year}';
    }
  }
}
