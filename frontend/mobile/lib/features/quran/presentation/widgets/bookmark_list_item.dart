import 'package:flutter/material.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_card.dart';
import '../../data/models/surah_model.dart';
import 'package:intl/intl.dart';

/// List item widget for displaying a bookmark
class BookmarkListItem extends StatelessWidget {
  final QuranBookmark bookmark;
  final VoidCallback onTap;
  final VoidCallback onDelete;

  const BookmarkListItem({
    Key? key,
    required this.bookmark,
    required this.onTap,
    required this.onDelete,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return IslamicCard(
      onTap: onTap,
      padding: const EdgeInsets.all(16),
      child: Row(
        children: [
          // Bookmark icon
          Container(
            width: 48,
            height: 48,
            decoration: BoxDecoration(
              color: AppColors.accentGold.withOpacity(0.1),
              borderRadius: BorderRadius.circular(12),
              border: Border.all(
                color: AppColors.accentGold,
                width: 2,
              ),
            ),
            child: Icon(
              Icons.bookmark,
              color: AppColors.accentGold,
              size: 24,
            ),
          ),
          const SizedBox(width: 16),
          
          // Bookmark info
          Expanded(
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                // Position
                Text(
                  'سورة ${bookmark.surahNumber} - آية ${bookmark.ayahNumber}',
                  style: AppTextStyles.subtitle1.copyWith(
                    color: AppColors.textPrimary,
                    fontWeight: FontWeight.bold,
                  ),
                ),
                const SizedBox(height: 4),
                
                // Page number
                Text(
                  'صفحة ${bookmark.pageNumber}',
                  style: AppTextStyles.body2.copyWith(
                    color: AppColors.textSecondary,
                  ),
                ),
                
                // Note if available
                if (bookmark.note != null && bookmark.note!.isNotEmpty) ...[
                  const SizedBox(height: 4),
                  Text(
                    bookmark.note!,
                    style: AppTextStyles.caption.copyWith(
                      color: AppColors.textSecondary,
                      fontStyle: FontStyle.italic,
                    ),
                    maxLines: 2,
                    overflow: TextOverflow.ellipsis,
                  ),
                ],
                
                const SizedBox(height: 4),
                
                // Date
                Text(
                  _formatDate(bookmark.createdAt),
                  style: AppTextStyles.caption.copyWith(
                    color: AppColors.textDisabled,
                  ),
                ),
              ],
            ),
          ),
          
          // Delete button
          IconButton(
            icon: Icon(
              Icons.delete_outline,
              color: AppColors.statusError,
            ),
            onPressed: () {
              _showDeleteConfirmation(context);
            },
            tooltip: 'حذف العلامة المرجعية',
          ),
        ],
      ),
    );
  }

  void _showDeleteConfirmation(BuildContext context) {
    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: Text(
          'حذف العلامة المرجعية',
          style: AppTextStyles.h6.copyWith(
            color: AppColors.textPrimary,
          ),
        ),
        content: Text(
          'هل أنت متأكد من حذف هذه العلامة المرجعية؟',
          style: AppTextStyles.body1.copyWith(
            color: AppColors.textSecondary,
          ),
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: Text(
              'إلغاء',
              style: AppTextStyles.button.copyWith(
                color: AppColors.textSecondary,
              ),
            ),
          ),
          TextButton(
            onPressed: () {
              Navigator.of(context).pop();
              onDelete();
            },
            child: Text(
              'حذف',
              style: AppTextStyles.button.copyWith(
                color: AppColors.statusError,
              ),
            ),
          ),
        ],
      ),
    );
  }

  String _formatDate(DateTime date) {
    final now = DateTime.now();
    final difference = now.difference(date);
    
    if (difference.inDays == 0) {
      return 'اليوم';
    } else if (difference.inDays == 1) {
      return 'أمس';
    } else if (difference.inDays < 7) {
      return 'منذ ${difference.inDays} أيام';
    } else {
      return DateFormat('dd/MM/yyyy').format(date);
    }
  }
}
