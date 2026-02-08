import 'package:flutter/material.dart';
import '../../data/models/hadith_model.dart';
import '../../../../core/theme/app_theme.dart';

class HadithBookCard extends StatelessWidget {
  final HadithBookModel book;
  final VoidCallback onTap;

  const HadithBookCard({
    super.key,
    required this.book,
    required this.onTap,
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: const EdgeInsets.only(bottom: 16),
      elevation: 2,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(16),
        side: BorderSide(
          color: AppTheme.primaryColor.withOpacity(0.1),
          width: 1,
        ),
      ),
      child: InkWell(
        onTap: onTap,
        borderRadius: BorderRadius.circular(16),
        child: Padding(
          padding: const EdgeInsets.all(20),
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              // Header with book type and authenticity
              Row(
                children: [
                  _buildBookTypeBadge(),
                  const Spacer(),
                  _buildAuthenticityBadge(),
                ],
              ),
              const SizedBox(height: 16),

              // Book title
              Text(
                book.arabicName,
                style: const TextStyle(
                  fontSize: 20,
                  fontWeight: FontWeight.bold,
                  fontFamily: 'Tajawal',
                  height: 1.5,
                ),
                textDirection: TextDirection.rtl,
              ),
              const SizedBox(height: 8),

              // Author
              Row(
                children: [
                  Icon(
                    Icons.person,
                    size: 16,
                    color: Colors.grey[600],
                  ),
                  const SizedBox(width: 4),
                  Expanded(
                    child: Text(
                      book.authorArabicName,
                      style: TextStyle(
                        fontSize: 14,
                        color: Colors.grey[700],
                        fontFamily: 'Tajawal',
                      ),
                      textDirection: TextDirection.rtl,
                    ),
                  ),
                ],
              ),
              const SizedBox(height: 12),

              // Description (if available)
              if (book.description != null) ...[
                Text(
                  book.description!,
                  style: TextStyle(
                    fontSize: 13,
                    color: Colors.grey[600],
                    height: 1.6,
                  ),
                  maxLines: 2,
                  overflow: TextOverflow.ellipsis,
                  textDirection: TextDirection.rtl,
                ),
                const SizedBox(height: 12),
              ],

              // Footer with stats
              Row(
                children: [
                  _buildStatItem(
                    icon: Icons.book,
                    label: '${book.totalHadiths} حديث',
                  ),
                  if (book.compilationYear != null) ...[
                    const SizedBox(width: 16),
                    _buildStatItem(
                      icon: Icons.calendar_today,
                      label: '${book.compilationYear} هـ',
                    ),
                  ],
                  const Spacer(),
                  Icon(
                    Icons.arrow_forward_ios,
                    size: 16,
                    color: AppTheme.primaryColor,
                  ),
                ],
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildBookTypeBadge() {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
      decoration: BoxDecoration(
        color: AppTheme.secondaryColor.withOpacity(0.1),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Text(
        book.bookType.arabicName,
        style: TextStyle(
          color: AppTheme.secondaryColor,
          fontSize: 12,
          fontWeight: FontWeight.bold,
          fontFamily: 'Tajawal',
        ),
      ),
    );
  }

  Widget _buildAuthenticityBadge() {
    Color color;
    switch (book.authenticityLevel) {
      case BookAuthenticityLevel.highest:
        color = Colors.green;
        break;
      case BookAuthenticityLevel.high:
        color = Colors.lightGreen;
        break;
      case BookAuthenticityLevel.moderate:
        color = Colors.amber;
        break;
      case BookAuthenticityLevel.variable:
        color = Colors.orange;
        break;
    }

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
      decoration: BoxDecoration(
        color: color.withOpacity(0.2),
        borderRadius: BorderRadius.circular(12),
        border: Border.all(color: color, width: 1),
      ),
      child: Row(
        mainAxisSize: MainAxisSize.min,
        children: [
          Icon(Icons.verified, size: 14, color: color),
          const SizedBox(width: 4),
          Text(
            book.authenticityLevel.arabicName,
            style: TextStyle(
              color: color,
              fontSize: 11,
              fontWeight: FontWeight.bold,
              fontFamily: 'Tajawal',
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildStatItem({required IconData icon, required String label}) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [
        Icon(icon, size: 14, color: Colors.grey[600]),
        const SizedBox(width: 4),
        Text(
          label,
          style: TextStyle(
            fontSize: 12,
            color: Colors.grey[600],
            fontFamily: 'Tajawal',
          ),
        ),
      ],
    );
  }
}
