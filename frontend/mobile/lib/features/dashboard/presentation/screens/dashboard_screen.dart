import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../../../core/theme/app_colors.dart';
import '../../../../core/theme/app_text_styles.dart';
import '../../../../core/widgets/islamic_loading_indicator.dart';
import '../../../../core/providers/dashboard_provider.dart';
import '../../../../core/providers/preferences_provider.dart';
import '../widgets/prayer_times_card.dart';
import '../widgets/hijri_date_card.dart';
import '../widgets/daily_wird_card.dart';
import '../widgets/daily_content_card.dart';
import '../widgets/quick_actions_card.dart';

class DashboardScreen extends ConsumerStatefulWidget {
  const DashboardScreen({Key? key}) : super(key: key);

  @override
  ConsumerState<DashboardScreen> createState() => _DashboardScreenState();
}

class _DashboardScreenState extends ConsumerState<DashboardScreen> {
  @override
  void initState() {
    super.initState();
    // Load dashboard data on init
    WidgetsBinding.instance.addPostFrameCallback((_) {
      _loadDashboardData();
    });
  }

  Future<void> _loadDashboardData() async {
    // Get user location from preferences or use default (Riyadh)
    final latitude = 24.7136; // TODO: Get from location service
    final longitude = 46.6753;

    await ref.read(dashboardNotifierProvider.notifier).loadDashboardData(
          latitude: latitude,
          longitude: longitude,
        );
  }

  Future<void> _refreshDashboard() async {
    final latitude = 24.7136; // TODO: Get from location service
    final longitude = 46.6753;

    await ref.read(dashboardNotifierProvider.notifier).refresh(
          latitude: latitude,
          longitude: longitude,
        );
  }

  @override
  Widget build(BuildContext context) {
    final dashboardState = ref.watch(dashboardNotifierProvider);
    final preferencesAsync = ref.watch(userPreferencesProvider);

    return Scaffold(
      backgroundColor: AppColors.backgroundPrimary,
      body: SafeArea(
        child: RefreshIndicator(
          onRefresh: _refreshDashboard,
          color: AppColors.primary,
          child: CustomScrollView(
            slivers: [
              // App Bar
              SliverAppBar(
                floating: true,
                backgroundColor: AppColors.backgroundPrimary,
                elevation: 0,
                title: Row(
                  children: [
                    Icon(
                      Icons.menu,
                      color: AppColors.primary,
                    ),
                    const SizedBox(width: 12),
                    Expanded(
                      child: preferencesAsync.when(
                        data: (prefs) => Text(
                          'السلام عليكم، ${prefs.userName ?? "مستخدم"}',
                          style: AppTextStyles.h6.copyWith(
                            color: AppColors.primary,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        loading: () => Text(
                          'السلام عليكم',
                          style: AppTextStyles.h6.copyWith(
                            color: AppColors.primary,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                        error: (_, __) => Text(
                          'السلام عليكم',
                          style: AppTextStyles.h6.copyWith(
                            color: AppColors.primary,
                            fontWeight: FontWeight.bold,
                          ),
                        ),
                      ),
                    ),
                  ],
                ),
                actions: [
                  IconButton(
                    icon: Icon(
                      Icons.notifications_outlined,
                      color: AppColors.primary,
                    ),
                    onPressed: () {
                      // TODO: Navigate to notifications
                    },
                  ),
                  IconButton(
                    icon: Icon(
                      Icons.settings_outlined,
                      color: AppColors.primary,
                    ),
                    onPressed: () {
                      // TODO: Navigate to settings
                    },
                  ),
                ],
              ),

              // Content
              if (dashboardState.isLoading)
                const SliverFillRemaining(
                  child: Center(
                    child: IslamicLoadingIndicator(),
                  ),
                )
              else if (dashboardState.error != null)
                SliverFillRemaining(
                  child: Center(
                    child: Column(
                      mainAxisAlignment: MainAxisAlignment.center,
                      children: [
                        Icon(
                          Icons.error_outline,
                          size: 64,
                          color: AppColors.error,
                        ),
                        const SizedBox(height: 16),
                        Text(
                          'حدث خطأ في تحميل البيانات',
                          style: AppTextStyles.bodyLarge.copyWith(
                            color: AppColors.textPrimary,
                          ),
                        ),
                        const SizedBox(height: 8),
                        Text(
                          dashboardState.error!,
                          style: AppTextStyles.bodySmall.copyWith(
                            color: AppColors.textSecondary,
                          ),
                          textAlign: TextAlign.center,
                        ),
                        const SizedBox(height: 24),
                        ElevatedButton(
                          onPressed: _loadDashboardData,
                          style: ElevatedButton.styleFrom(
                            backgroundColor: AppColors.primary,
                            foregroundColor: Colors.white,
                          ),
                          child: const Text('إعادة المحاولة'),
                        ),
                      ],
                    ),
                  ),
                )
              else
                SliverPadding(
                  padding: const EdgeInsets.all(16),
                  sliver: SliverList(
                    delegate: SliverChildListDelegate([
                      // Hijri Date Card
                      if (dashboardState.hijriDate != null)
                        HijriDateCard(hijriDate: dashboardState.hijriDate!),
                      const SizedBox(height: 16),

                      // Prayer Times Card
                      if (dashboardState.prayerTimes != null)
                        PrayerTimesCard(
                          prayerTimes: dashboardState.prayerTimes!,
                        ),
                      const SizedBox(height: 16),

                      // Daily Wird Card
                      if (dashboardState.dashboardData?.dailyWird != null)
                        DailyWirdCard(
                          dailyWird: dashboardState.dashboardData!.dailyWird,
                          onTap: () {
                            // TODO: Navigate to Quran reading
                          },
                        ),
                      const SizedBox(height: 16),

                      // Daily Content Card
                      if (dashboardState.dashboardData?.dailyContent != null)
                        DailyContentCard(
                          dailyContent:
                              dashboardState.dashboardData!.dailyContent,
                          onTap: () {
                            // TODO: Navigate to tafsir/explanation
                          },
                        ),
                      const SizedBox(height: 16),

                      // Quick Actions Card
                      QuickActionsCard(
                        actions: [
                          QuickAction(
                            title: 'المساعد الذكي',
                            icon: Icons.psychology,
                            color: AppColors.primary,
                            onTap: () {
                              // TODO: Navigate to AI assistant
                            },
                          ),
                          QuickAction(
                            title: 'القبلة',
                            icon: Icons.explore,
                            color: AppColors.secondary,
                            onTap: () {
                              // TODO: Navigate to Qibla compass
                            },
                          ),
                          QuickAction(
                            title: 'الأذكار',
                            icon: Icons.auto_stories,
                            color: AppColors.accent,
                            onTap: () {
                              // TODO: Navigate to Adhkar
                            },
                          ),
                        ],
                      ),
                      const SizedBox(height: 24),
                    ]),
                  ),
                ),
            ],
          ),
        ),
      ),
    );
  }
}
