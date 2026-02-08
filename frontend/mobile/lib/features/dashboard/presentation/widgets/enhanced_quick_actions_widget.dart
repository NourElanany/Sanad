import 'package:flutter/material.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_card.dart';

/// Model for quick action item
class QuickAction {
  final String id;
  final String title;
  final IconData icon;
  final Color color;
  final VoidCallback onTap;
  final String? badge;

  QuickAction({
    required this.id,
    required this.title,
    required this.icon,
    required this.color,
    required this.onTap,
    this.badge,
  });
}

/// Enhanced Quick Actions Widget with more features
class EnhancedQuickActionsWidget extends StatelessWidget {
  final List<QuickAction>? customActions;

  const EnhancedQuickActionsWidget({
    Key? key,
    this.customActions,
  }) : super(key: key);

  List<QuickAction> _getDefaultActions(BuildContext context) {
    return [
      QuickAction(
        id: 'ai_assistant',
        title: 'المساعد الذكي',
        icon: Icons.psychology,
        color: AppColors.primary,
        onTap: () {
          // TODO: Navigate to AI assistant
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('المساعد الذكي قريباً')),
          );
        },
      ),
      QuickAction(
        id: 'qibla',
        title: 'القبلة',
        icon: Icons.explore,
        color: AppColors.secondary,
        onTap: () {
          // TODO: Navigate to Qibla compass
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('بوصلة القبلة قريباً')),
          );
        },
      ),
      QuickAction(
        id: 'adhkar',
        title: 'الأذكار',
        icon: Icons.auto_stories,
        color: AppColors.accent,
        onTap: () {
          // TODO: Navigate to Adhkar
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('الأذكار قريباً')),
          );
        },
      ),
      QuickAction(
        id: 'quran',
        title: 'القرآن',
        icon: Icons.menu_book,
        color: AppColors.success,
        onTap: () {
          // TODO: Navigate to Quran
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('القرآن الكريم قريباً')),
          );
        },
      ),
      QuickAction(
        id: 'hadith',
        title: 'الأحاديث',
        icon: Icons.library_books,
        color: const Color(0xFF8B4513),
        onTap: () {
          // TODO: Navigate to Hadith
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('الأحاديث قريباً')),
          );
        },
      ),
      QuickAction(
        id: 'tasbih',
        title: 'المسبحة',
        icon: Icons.circle_outlined,
        color: AppColors.info,
        onTap: () {
          // TODO: Navigate to Tasbih counter
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('المسبحة الإلكترونية قريباً')),
          );
        },
      ),
      QuickAction(
        id: 'dua',
        title: 'الأدعية',
        icon: Icons.favorite_border,
        color: const Color(0xFFE91E63),
        onTap: () {
          // TODO: Navigate to Dua collection
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('الأدعية قريباً')),
          );
        },
      ),
      QuickAction(
        id: 'mosque_finder',
        title: 'المساجد القريبة',
        icon: Icons.mosque,
        color: const Color(0xFF9C27B0),
        onTap: () {
          // TODO: Navigate to mosque finder
          ScaffoldMessenger.of(context).showSnackBar(
            const SnackBar(content: Text('البحث عن المساجد قريباً')),
          );
        },
      ),
    ];
  }

  @override
  Widget build(BuildContext context) {
    final actions = customActions ?? _getDefaultActions(context);

    return IslamicCard(
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          // Header
          Row(
            children: [
              Container(
                padding: const EdgeInsets.all(10),
                decoration: BoxDecoration(
                  gradient: LinearGradient(
                    colors: [
                      AppColors.primary,
                      AppColors.primary.withOpacity(0.7),
                    ],
                  ),
                  borderRadius: BorderRadius.circular(10),
                ),
                child: const Icon(
                  Icons.dashboard_customize,
                  color: Colors.white,
                  size: 20,
                ),
              ),
              const SizedBox(width: 12),
              Text(
                'الوصول السريع',
                style: AppTextStyles.h6.copyWith(
                  color: AppColors.primary,
                  fontWeight: FontWeight.bold,
                ),
              ),
            ],
          ),
          const SizedBox(height: 16),

          // Grid of actions
          GridView.builder(
            shrinkWrap: true,
            physics: const NeverScrollableScrollPhysics(),
            gridDelegate: const SliverGridDelegateWithFixedCrossAxisCount(
              crossAxisCount: 4,
              crossAxisSpacing: 12,
              mainAxisSpacing: 16,
              childAspectRatio: 0.85,
            ),
            itemCount: actions.length,
            itemBuilder: (context, index) {
              final action = actions[index];
              return _buildActionButton(context, action);
            },
          ),
        ],
      ),
    );
  }

  Widget _buildActionButton(BuildContext context, QuickAction action) {
    return InkWell(
      onTap: action.onTap,
      borderRadius: BorderRadius.circular(12),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          // Icon container with badge
          Stack(
            clipBehavior: Clip.none,
            children: [
              Container(
                width: 56,
                height: 56,
                decoration: BoxDecoration(
                  gradient: LinearGradient(
                    colors: [
                      action.color,
                      action.color.withOpacity(0.7),
                    ],
                    begin: Alignment.topLeft,
                    end: Alignment.bottomRight,
                  ),
                  borderRadius: BorderRadius.circular(12),
                  boxShadow: [
                    BoxShadow(
                      color: action.color.withOpacity(0.3),
                      blurRadius: 8,
                      offset: const Offset(0, 4),
                    ),
                  ],
                ),
                child: Icon(
                  action.icon,
                  color: Colors.white,
                  size: 28,
                ),
              ),
              // Badge
              if (action.badge != null)
                Positioned(
                  top: -4,
                  right: -4,
                  child: Container(
                    padding: const EdgeInsets.symmetric(
                      horizontal: 6,
                      vertical: 2,
                    ),
                    decoration: BoxDecoration(
                      color: AppColors.error,
                      borderRadius: BorderRadius.circular(10),
                      border: Border.all(
                        color: Colors.white,
                        width: 2,
                      ),
                    ),
                    child: Text(
                      action.badge!,
                      style: AppTextStyles.caption.copyWith(
                        color: Colors.white,
                        fontSize: 10,
                        fontWeight: FontWeight.bold,
                      ),
                    ),
                  ),
                ),
            ],
          ),
          const SizedBox(height: 8),
          // Title
          Text(
            action.title,
            style: AppTextStyles.caption.copyWith(
              color: AppColors.textPrimary,
              fontWeight: FontWeight.w600,
            ),
            textAlign: TextAlign.center,
            maxLines: 2,
            overflow: TextOverflow.ellipsis,
          ),
        ],
      ),
    );
  }
}

/// Compact version for smaller spaces
class CompactQuickActionsWidget extends StatelessWidget {
  final List<QuickAction>? actions;
  final int maxActions;

  const CompactQuickActionsWidget({
    Key? key,
    this.actions,
    this.maxActions = 3,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final defaultActions = [
      QuickAction(
        id: 'ai_assistant',
        title: 'المساعد الذكي',
        icon: Icons.psychology,
        color: AppColors.primary,
        onTap: () {},
      ),
      QuickAction(
        id: 'qibla',
        title: 'القبلة',
        icon: Icons.explore,
        color: AppColors.secondary,
        onTap: () {},
      ),
      QuickAction(
        id: 'adhkar',
        title: 'الأذكار',
        icon: Icons.auto_stories,
        color: AppColors.accent,
        onTap: () {},
      ),
    ];

    final displayActions = (actions ?? defaultActions).take(maxActions).toList();

    return IslamicCard(
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
        children: displayActions.map((action) {
          return Expanded(
            child: InkWell(
              onTap: action.onTap,
              borderRadius: BorderRadius.circular(12),
              child: Padding(
                padding: const EdgeInsets.symmetric(vertical: 8),
                child: Column(
                  children: [
                    Container(
                      width: 48,
                      height: 48,
                      decoration: BoxDecoration(
                        gradient: LinearGradient(
                          colors: [
                            action.color,
                            action.color.withOpacity(0.7),
                          ],
                        ),
                        borderRadius: BorderRadius.circular(10),
                      ),
                      child: Icon(
                        action.icon,
                        color: Colors.white,
                        size: 24,
                      ),
                    ),
                    const SizedBox(height: 6),
                    Text(
                      action.title,
                      style: AppTextStyles.caption.copyWith(
                        color: AppColors.textPrimary,
                        fontWeight: FontWeight.w600,
                      ),
                      textAlign: TextAlign.center,
                      maxLines: 1,
                      overflow: TextOverflow.ellipsis,
                    ),
                  ],
                ),
              ),
            ),
          );
        }).toList(),
      ),
    );
  }
}
