import 'package:flutter/material.dart';
import '../../../../core/theme/app_theme.dart';

class HadithSearchBar extends StatelessWidget {
  final TextEditingController controller;
  final Function(String) onSearch;
  final VoidCallback onClear;

  const HadithSearchBar({
    super.key,
    required this.controller,
    required this.onSearch,
    required this.onClear,
  });

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: Colors.grey[100],
        borderRadius: BorderRadius.circular(16),
        border: Border.all(
          color: AppTheme.primaryColor.withOpacity(0.1),
          width: 1,
        ),
      ),
      child: TextField(
        controller: controller,
        textDirection: TextDirection.rtl,
        style: const TextStyle(
          fontFamily: 'Tajawal',
          fontSize: 16,
        ),
        decoration: InputDecoration(
          hintText: 'ابحث في الأحاديث...',
          hintStyle: TextStyle(
            color: Colors.grey[400],
            fontFamily: 'Tajawal',
          ),
          prefixIcon: IconButton(
            icon: const Icon(Icons.search),
            onPressed: () => onSearch(controller.text),
            color: AppTheme.primaryColor,
          ),
          suffixIcon: controller.text.isNotEmpty
              ? IconButton(
                  icon: const Icon(Icons.clear),
                  onPressed: onClear,
                  color: Colors.grey[600],
                )
              : null,
          border: InputBorder.none,
          contentPadding: const EdgeInsets.symmetric(
            horizontal: 16,
            vertical: 14,
          ),
        ),
        onSubmitted: onSearch,
      ),
    );
  }
}
