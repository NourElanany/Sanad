import 'package:flutter/material.dart';
import '../theme/app_colors.dart';

/// Islamic-themed app bar with customizable actions
class IslamicAppBar extends StatelessWidget implements PreferredSizeWidget {
  final String title;
  final List<Widget>? actions;
  final Widget? leading;
  final bool centerTitle;
  final Color? backgroundColor;
  final PreferredSizeWidget? bottom;

  const IslamicAppBar({
    Key? key,
    required this.title,
    this.actions,
    this.leading,
    this.centerTitle = true,
    this.backgroundColor,
    this.bottom,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return AppBar(
      title: Text(
        title,
        style: const TextStyle(
          fontSize: 20,
          fontWeight: FontWeight.w600,
          fontFamily: 'Tajawal',
          color: Colors.white,
        ),
      ),
      centerTitle: centerTitle,
      backgroundColor: backgroundColor ?? AppColors.primaryMain,
      elevation: 0,
      leading: leading,
      actions: actions,
      bottom: bottom,
    );
  }

  @override
  Size get preferredSize => Size.fromHeight(
        kToolbarHeight + (bottom?.preferredSize.height ?? 0),
      );
}

/// Islamic-themed bottom navigation bar
class IslamicBottomNavBar extends StatelessWidget {
  final int currentIndex;
  final Function(int) onTap;
  final List<IslamicNavItem> items;

  const IslamicBottomNavBar({
    Key? key,
    required this.currentIndex,
    required this.onTap,
    required this.items,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: AppColors.backgroundPaper,
        boxShadow: [
          BoxShadow(
            color: AppColors.primaryMain.withOpacity(0.1),
            blurRadius: 8,
            offset: const Offset(0, -2),
          ),
        ],
      ),
      child: SafeArea(
        child: Padding(
          padding: const EdgeInsets.symmetric(vertical: 8),
          child: Row(
            mainAxisAlignment: MainAxisAlignment.spaceAround,
            children: List.generate(
              items.length,
              (index) => _buildNavItem(
                items[index],
                index == currentIndex,
                () => onTap(index),
              ),
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildNavItem(IslamicNavItem item, bool isSelected, VoidCallback onTap) {
    return InkWell(
      onTap: onTap,
      borderRadius: BorderRadius.circular(12),
      child: Container(
        padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
        decoration: BoxDecoration(
          color: isSelected
              ? AppColors.primaryMain.withOpacity(0.1)
              : Colors.transparent,
          borderRadius: BorderRadius.circular(12),
        ),
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Icon(
              item.icon,
              color: isSelected ? AppColors.primaryMain : AppColors.textSecondary,
              size: 24,
            ),
            const SizedBox(height: 4),
            Text(
              item.label,
              style: TextStyle(
                fontSize: 12,
                fontWeight: isSelected ? FontWeight.w600 : FontWeight.w400,
                fontFamily: 'Tajawal',
                color: isSelected ? AppColors.primaryMain : AppColors.textSecondary,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

/// Navigation item model
class IslamicNavItem {
  final IconData icon;
  final String label;

  const IslamicNavItem({
    required this.icon,
    required this.label,
  });
}

/// Islamic-themed drawer
class IslamicDrawer extends StatelessWidget {
  final String userName;
  final String? userEmail;
  final String? userAvatar;
  final List<IslamicDrawerItem> items;
  final VoidCallback? onProfileTap;

  const IslamicDrawer({
    Key? key,
    required this.userName,
    this.userEmail,
    this.userAvatar,
    required this.items,
    this.onProfileTap,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Drawer(
      child: Column(
        children: [
          // Header
          Container(
            width: double.infinity,
            padding: const EdgeInsets.fromLTRB(24, 60, 24, 24),
            decoration: BoxDecoration(
              gradient: LinearGradient(
                colors: [
                  AppColors.primaryMain,
                  AppColors.primaryLight,
                ],
                begin: Alignment.topLeft,
                end: Alignment.bottomRight,
              ),
            ),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                CircleAvatar(
                  radius: 40,
                  backgroundColor: Colors.white,
                  backgroundImage:
                      userAvatar != null ? NetworkImage(userAvatar!) : null,
                  child: userAvatar == null
                      ? Icon(
                          Icons.person,
                          size: 40,
                          color: AppColors.primaryMain,
                        )
                      : null,
                ),
                const SizedBox(height: 16),
                Text(
                  userName,
                  style: const TextStyle(
                    fontSize: 20,
                    fontWeight: FontWeight.w600,
                    fontFamily: 'Tajawal',
                    color: Colors.white,
                  ),
                ),
                if (userEmail != null) ...[
                  const SizedBox(height: 4),
                  Text(
                    userEmail!,
                    style: const TextStyle(
                      fontSize: 14,
                      fontFamily: 'Tajawal',
                      color: Colors.white70,
                    ),
                  ),
                ],
              ],
            ),
          ),
          // Menu items
          Expanded(
            child: ListView.builder(
              padding: const EdgeInsets.symmetric(vertical: 8),
              itemCount: items.length,
              itemBuilder: (context, index) {
                final item = items[index];
                if (item.isDivider) {
                  return Divider(
                    color: AppColors.primaryMain.withOpacity(0.1),
                    thickness: 1,
                    height: 1,
                  );
                }
                return ListTile(
                  leading: Icon(
                    item.icon,
                    color: AppColors.primaryMain,
                  ),
                  title: Text(
                    item.title,
                    style: const TextStyle(
                      fontSize: 16,
                      fontFamily: 'Tajawal',
                      color: AppColors.textPrimary,
                    ),
                  ),
                  trailing: item.trailing,
                  onTap: item.onTap,
                );
              },
            ),
          ),
        ],
      ),
    );
  }
}

/// Drawer item model
class IslamicDrawerItem {
  final IconData? icon;
  final String title;
  final VoidCallback? onTap;
  final Widget? trailing;
  final bool isDivider;

  const IslamicDrawerItem({
    this.icon,
    required this.title,
    this.onTap,
    this.trailing,
    this.isDivider = false,
  });

  const IslamicDrawerItem.divider()
      : icon = null,
        title = '',
        onTap = null,
        trailing = null,
        isDivider = true;
}

/// Islamic-themed tab bar
class IslamicTabBar extends StatelessWidget implements PreferredSizeWidget {
  final List<String> tabs;
  final TabController controller;

  const IslamicTabBar({
    Key? key,
    required this.tabs,
    required this.controller,
  }) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      decoration: BoxDecoration(
        color: AppColors.backgroundPaper,
        border: Border(
          bottom: BorderSide(
            color: AppColors.primaryMain.withOpacity(0.1),
            width: 1,
          ),
        ),
      ),
      child: TabBar(
        controller: controller,
        tabs: tabs
            .map((tab) => Tab(
                  child: Text(
                    tab,
                    style: const TextStyle(
                      fontFamily: 'Tajawal',
                      fontWeight: FontWeight.w600,
                    ),
                  ),
                ))
            .toList(),
        labelColor: AppColors.primaryMain,
        unselectedLabelColor: AppColors.textSecondary,
        indicatorColor: AppColors.primaryMain,
        indicatorWeight: 3,
      ),
    );
  }

  @override
  Size get preferredSize => const Size.fromHeight(48);
}
