import 'package:flutter/material.dart';
import '../../data/models/ayah_model.dart';

/// Widget to display a single Ayah (verse) with highlighting support
class AyahWidget extends StatelessWidget {
  final AyahModel ayah;
  final bool isSelected;
  final double fontSize;
  final VoidCallback onTap;

  const AyahWidget({
    Key? key,
    required this.ayah,
    required this.isSelected,
    required this.fontSize,
    required this.onTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return GestureDetector(
      onTap: onTap,
      child: Container(
        margin: const EdgeInsets.only(bottom: 12),
        padding: const EdgeInsets.all(12),
        decoration: BoxDecoration(
          color: isSelected
              ? const Color(0xFFB8860B).withOpacity(0.15)
              : Colors.transparent,
          borderRadius: BorderRadius.circular(8),
          border: isSelected
              ? Border.all(
                  color: const Color(0xFFB8860B),
                  width: 2,
                )
              : null,
        ),
        child: RichText(
          textAlign: TextAlign.justify,
          textDirection: TextDirection.rtl,
          text: TextSpan(
            children: [
              // Ayah text in Uthmani script
              TextSpan(
                text: ayah.textUthmani,
                style: TextStyle(
                  fontSize: fontSize,
                  fontFamily: 'KFGQPC Uthman Taha Naskh',
                  color: const Color(0xFF0F1F35),
                  height: 2.0,
                  letterSpacing: 0.5,
                ),
              ),
              // Ayah number in decorative circle
              WidgetSpan(
                alignment: PlaceholderAlignment.middle,
                child: Container(
                  margin: const EdgeInsets.symmetric(horizontal: 4),
                  padding: const EdgeInsets.all(6),
                  decoration: BoxDecoration(
                    shape: BoxShape.circle,
                    border: Border.all(
                      color: const Color(0xFF1B365D),
                      width: 1.5,
                    ),
                  ),
                  child: Text(
                    '${ayah.numberInSurah}',
                    style: TextStyle(
                      fontSize: fontSize * 0.6,
                      fontFamily: 'Tajawal',
                      color: const Color(0xFF1B365D),
                      fontWeight: FontWeight.bold,
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
