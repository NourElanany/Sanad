/// Search result card widget with enhanced features
/// Requirements: 8.1, 8.2, 8.3, 8.4, 8.5

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:share_plus/share_plus.dart';
import '../../../../core/theme/app_theme.dart';
import '../../../../core/widgets/islamic_card.dart';
import '../../data/models/search_models.dart';

class SearchResultCard extends StatefulWidget {
  final SearchResult result;
  final VoidCallback onTap;

  const SearchResultCard({
    Key? key,
    required this.result,
    required this.onTap,
  }) : super(key: key);

  @override
  State<SearchResultCard> createState() => _SearchResultCardState();
}

class _SearchResultCardState extends State<SearchResultCard> {
  bool _showShareMenu = false;
  bool _copied = false;

  @override
  Widget build(BuildContext context) {
    final contentType = _parseContentType(widget.result.document.contentType);
    final authenticityGrade = _getAuthenticityGrade(widget.result.document.contentType);
    final isQuran = widget.result.document.contentType == 'quran';
    final isHadith = widget.result.document.contentType.contains('hadith');
    final isFatwa = widget.result.document.contentType == 'fiqh_ruling' || 
                     widget.result.document.contentType == 'scholar_opinion';

    return Padding(
      padding: const EdgeInsets.only(bottom: 12),
      child: IslamicCard(
        onTap: widget.onTap,
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            // Header with content type, score, and share button
            Row(
              children: [
                // Content type icon and label
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
                  decoration: BoxDecoration(
                    color: AppTheme.primary.main.withOpacity(0.1),
                    borderRadius: BorderRadius.circular(6),
                  ),
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Text(
                        contentType?.iconEmoji ?? '📄',
                        style: const TextStyle(fontSize: 14),
                      ),
                      const SizedBox(width: 6),
                      Text(
                        contentType?.displayName ?? widget.result.document.contentType,
                        style: TextStyle(
                          fontSize: 12,
                          fontWeight: FontWeight.w600,
                          color: AppTheme.primary.main,
                        ),
                      ),
                    ],
                  ),
                ),
                const Spacer(),
                // Similarity score
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
                  decoration: BoxDecoration(
                    color: _getScoreColor(widget.result.similarityScore).withOpacity(0.1),
                    borderRadius: BorderRadius.circular(4),
                  ),
                  child: Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Icon(
                        Icons.star,
                        size: 14,
                        color: _getScoreColor(widget.result.similarityScore),
                      ),
                      const SizedBox(width: 4),
                      Text(
                        '${(widget.result.similarityScore * 100).toStringAsFixed(0)}%',
                        style: TextStyle(
                          fontSize: 12,
                          fontWeight: FontWeight.w600,
                          color: _getScoreColor(widget.result.similarityScore),
                        ),
                      ),
                    ],
                  ),
                ),
                const SizedBox(width: 8),
                // Share button
                GestureDetector(
                  onTap: () => _showShareOptions(context),
                  child: Container(
                    padding: const EdgeInsets.all(6),
                    decoration: BoxDecoration(
                      color: Colors.grey.shade100,
                      borderRadius: BorderRadius.circular(6),
                    ),
                    child: Icon(
                      Icons.share,
                      size: 18,
                      color: AppTheme.text.secondary,
                    ),
                  ),
                ),
              ],
            ),

            // Content-specific details
            if (isQuran) _buildQuranDetails(),
            if (isHadith) _buildHadithDetails(),
            if (isFatwa) _buildFatwaDetails(),

            const SizedBox(height: 12),

            // Content text with highlighting
            _buildHighlightedText(),

            const SizedBox(height: 12),

            // Footer with source and metadata
            Row(
              children: [
                // Source
                Expanded(
                  child: Row(
                    children: [
                      Icon(
                        Icons.source,
                        size: 14,
                        color: AppTheme.text.secondary,
                      ),
                      const SizedBox(width: 4),
                      Expanded(
                        child: Text(
                          widget.result.document.source,
                          style: TextStyle(
                            fontSize: 12,
                            color: AppTheme.text.secondary,
                          ),
                          maxLines: 1,
                          overflow: TextOverflow.ellipsis,
                        ),
                      ),
                    ],
                  ),
                ),
                // Author if available
                if (widget.result.document.author != null) ...[
                  const SizedBox(width: 12),
                  Row(
                    children: [
                      Icon(
                        Icons.person,
                        size: 14,
                        color: AppTheme.text.secondary,
                      ),
                      const SizedBox(width: 4),
                      Text(
                        widget.result.document.author!,
                        style: TextStyle(
                          fontSize: 12,
                          color: AppTheme.text.secondary,
                        ),
                      ),
                    ],
                  ),
                ],
              ],
            ),

            // Explanation if available
            if (widget.result.explanation != null) ...[
              const SizedBox(height: 8),
              Container(
                padding: const EdgeInsets.all(8),
                decoration: BoxDecoration(
                  color: AppTheme.accent.gold.withOpacity(0.05),
                  borderRadius: BorderRadius.circular(6),
                  border: Border.all(
                    color: AppTheme.accent.gold.withOpacity(0.2),
                  ),
                ),
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Icon(
                      Icons.lightbulb_outline,
                      size: 16,
                      color: AppTheme.accent.gold,
                    ),
                    const SizedBox(width: 8),
                    Expanded(
                      child: Text(
                        widget.result.explanation!,
                        style: TextStyle(
                          fontSize: 12,
                          color: AppTheme.text.secondary,
                          fontStyle: FontStyle.italic,
                        ),
                      ),
                    ),
                  ],
                ),
              ),
            ],
          ],
        ),
      ),
    );
  }

  Widget _buildQuranDetails() {
    final metadata = widget.result.document.metadata;
    return Padding(
      padding: const EdgeInsets.only(top: 8),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
        decoration: BoxDecoration(
          color: AppTheme.primary.main.withOpacity(0.05),
          borderRadius: BorderRadius.circular(6),
        ),
        child: Row(
          children: [
            Icon(Icons.book, size: 14, color: AppTheme.primary.main),
            const SizedBox(width: 6),
            Text(
              'القرآن الكريم',
              style: TextStyle(
                fontSize: 12,
                fontWeight: FontWeight.w600,
                color: AppTheme.primary.main,
              ),
            ),
            if (metadata?['surah_number'] != null) ...[
              const SizedBox(width: 8),
              Text(
                'سورة ${metadata?['surah_name'] ?? metadata?['surah_number']}',
                style: TextStyle(fontSize: 12, color: AppTheme.text.secondary),
              ),
            ],
            if (metadata?['ayah_number'] != null) ...[
              const SizedBox(width: 8),
              Text(
                'آية ${metadata?['ayah_number']}',
                style: TextStyle(fontSize: 12, color: AppTheme.text.secondary),
              ),
            ],
          ],
        ),
      ),
    );
  }

  Widget _buildHadithDetails() {
    final metadata = widget.result.document.metadata;
    final authenticityGrade = _getAuthenticityGrade(widget.result.document.contentType);
    
    return Padding(
      padding: const EdgeInsets.only(top: 8),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          if (authenticityGrade != null)
            Container(
              padding: const EdgeInsets.symmetric(horizontal: 8, vertical: 4),
              decoration: BoxDecoration(
                color: _parseColor(authenticityGrade.colorHex).withOpacity(0.1),
                borderRadius: BorderRadius.circular(4),
                border: Border.all(
                  color: _parseColor(authenticityGrade.colorHex),
                  width: 1,
                ),
              ),
              child: Text(
                authenticityGrade.displayName,
                style: TextStyle(
                  fontSize: 12,
                  fontWeight: FontWeight.w600,
                  color: _parseColor(authenticityGrade.colorHex),
                ),
              ),
            ),
          const SizedBox(height: 6),
          Container(
            padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
            decoration: BoxDecoration(
              color: AppTheme.secondary.main.withOpacity(0.05),
              borderRadius: BorderRadius.circular(6),
            ),
            child: Row(
              children: [
                Icon(Icons.article, size: 14, color: AppTheme.secondary.main),
                const SizedBox(width: 6),
                Text(
                  'الحديث النبوي',
                  style: TextStyle(
                    fontSize: 12,
                    fontWeight: FontWeight.w600,
                    color: AppTheme.secondary.main,
                  ),
                ),
                if (metadata?['hadith_number'] != null) ...[
                  const SizedBox(width: 8),
                  Text(
                    'رقم ${metadata?['hadith_number']}',
                    style: TextStyle(fontSize: 12, color: AppTheme.text.secondary),
                  ),
                ],
                if (metadata?['book'] != null) ...[
                  const SizedBox(width: 8),
                  Text(
                    'كتاب ${metadata?['book']}',
                    style: TextStyle(fontSize: 12, color: AppTheme.text.secondary),
                  ),
                ],
              ],
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildFatwaDetails() {
    final metadata = widget.result.document.metadata;
    return Padding(
      padding: const EdgeInsets.only(top: 8),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 6),
        decoration: BoxDecoration(
          color: AppTheme.accent.gold.withOpacity(0.05),
          borderRadius: BorderRadius.circular(6),
        ),
        child: Row(
          children: [
            Icon(Icons.gavel, size: 14, color: AppTheme.accent.gold),
            const SizedBox(width: 6),
            Text(
              'فتوى شرعية',
              style: TextStyle(
                fontSize: 12,
                fontWeight: FontWeight.w600,
                color: AppTheme.accent.gold,
              ),
            ),
            if (metadata?['fatwa_number'] != null) ...[
              const SizedBox(width: 8),
              Text(
                'فتوى رقم ${metadata?['fatwa_number']}',
                style: TextStyle(fontSize: 12, color: AppTheme.text.secondary),
              ),
            ],
          ],
        ),
      ),
    );
  }

  Widget _buildHighlightedText() {
    final text = widget.result.highlightedText ?? widget.result.document.text;
    
    // Simple highlighting - in production, parse HTML <mark> tags properly
    return Text(
      text,
      style: TextStyle(
        fontSize: 16,
        height: 1.8,
        color: AppTheme.text.primary,
        fontFamily: _isArabic(widget.result.document.text) ? 'Amiri' : null,
      ),
      maxLines: 4,
      overflow: TextOverflow.ellipsis,
      textDirection: _isArabic(widget.result.document.text)
          ? TextDirection.rtl
          : TextDirection.ltr,
    );
  }

  void _showShareOptions(BuildContext context) {
    final shareText = '${widget.result.document.text}\n\nالمصدر: ${widget.result.document.source}${widget.result.document.author != null ? ' - ${widget.result.document.author}' : ''}';
    
    showModalBottomSheet(
      context: context,
      shape: const RoundedRectangleBorder(
        borderRadius: BorderRadius.vertical(top: Radius.circular(20)),
      ),
      builder: (context) => Container(
        padding: const EdgeInsets.all(20),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Container(
              width: 40,
              height: 4,
              decoration: BoxDecoration(
                color: Colors.grey.shade300,
                borderRadius: BorderRadius.circular(2),
              ),
            ),
            const SizedBox(height: 20),
            Text(
              'مشاركة النتيجة',
              style: TextStyle(
                fontSize: 18,
                fontWeight: FontWeight.bold,
                color: AppTheme.text.primary,
              ),
            ),
            const SizedBox(height: 20),
            ListTile(
              leading: Icon(Icons.copy, color: AppTheme.primary.main),
              title: Text(_copied ? 'تم النسخ!' : 'نسخ النص'),
              onTap: () async {
                await Clipboard.setData(ClipboardData(text: shareText));
                setState(() => _copied = true);
                Future.delayed(const Duration(seconds: 2), () {
                  if (mounted) setState(() => _copied = false);
                });
                Navigator.pop(context);
              },
            ),
            ListTile(
              leading: Icon(Icons.share, color: AppTheme.primary.main),
              title: const Text('مشاركة عبر التطبيقات'),
              onTap: () {
                Share.share(shareText);
                Navigator.pop(context);
              },
            ),
          ],
        ),
      ),
    );
  }

  ContentType? _parseContentType(String contentType) {
    try {
      return ContentType.values.firstWhere(
        (e) => e.toString().split('.').last == contentType.replaceAll('_', ''),
        orElse: () => ContentType.quran,
      );
    } catch (e) {
      return null;
    }
  }

  AuthenticityGrade? _getAuthenticityGrade(String contentType) {
    if (contentType.contains('hadith')) {
      if (contentType.contains('sahih')) return AuthenticityGrade.sahih;
      if (contentType.contains('hasan')) return AuthenticityGrade.hasan;
      if (contentType.contains('daif')) return AuthenticityGrade.daif;
      if (contentType.contains('mawdu')) return AuthenticityGrade.mawdu;
    }
    return null;
  }

  Color _getScoreColor(double score) {
    if (score >= 0.8) return AppTheme.status.success;
    if (score >= 0.6) return AppTheme.accent.gold;
    return AppTheme.status.warning;
  }

  Color _parseColor(String hex) {
    return Color(int.parse(hex.substring(1), radix: 16) + 0xFF000000);
  }

  bool _isArabic(String text) {
    if (text.isEmpty) return false;
    final arabicRegex = RegExp(r'[\u0600-\u06FF]');
    return arabicRegex.hasMatch(text);
  }
}
