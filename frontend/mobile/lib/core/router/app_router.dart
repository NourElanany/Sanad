import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

// Splash and onboarding screens
import '../../features/splash/presentation/screens/splash_screen.dart';
import '../../features/onboarding/presentation/screens/onboarding_screen.dart';
import '../../features/onboarding/presentation/screens/permissions_screen.dart';
import '../../features/onboarding/presentation/screens/madhab_selection_screen.dart';
import '../../features/onboarding/presentation/screens/theme_selection_screen.dart';

/// Provider for the app router
final appRouterProvider = Provider<GoRouter>((ref) {
  return GoRouter(
    initialLocation: '/',
    debugLogDiagnostics: true,
    routes: [
      // Splash screen
      GoRoute(
        path: '/',
        name: 'splash',
        builder: (context, state) => const SplashScreen(),
      ),
      
      // Onboarding routes
      GoRoute(
        path: '/onboarding',
        name: 'onboarding',
        builder: (context, state) => const OnboardingScreen(),
        routes: [
          GoRoute(
            path: 'permissions',
            name: 'permissions',
            builder: (context, state) => const PermissionsScreen(),
          ),
          GoRoute(
            path: 'madhab',
            name: 'madhab',
            builder: (context, state) => const MadhabSelectionScreen(),
          ),
          GoRoute(
            path: 'theme',
            name: 'theme',
            builder: (context, state) => const ThemeSelectionScreen(),
          ),
        ],
      ),
      
      // Main app routes
      GoRoute(
        path: '/home',
        name: 'home',
        builder: (context, state) => const Placeholder(),
      ),
      
      // Quran routes
      GoRoute(
        path: '/quran',
        name: 'quran',
        builder: (context, state) => const Placeholder(),
        routes: [
          GoRoute(
            path: 'surah/:surahId',
            name: 'surah',
            builder: (context, state) {
              final surahId = state.pathParameters['surahId']!;
              return const Placeholder();
            },
          ),
        ],
      ),
      
      // AI Assistant routes
      GoRoute(
        path: '/ai-assistant',
        name: 'ai-assistant',
        builder: (context, state) => const Placeholder(),
      ),
      
      // Prayer times routes
      GoRoute(
        path: '/prayer-times',
        name: 'prayer-times',
        builder: (context, state) => const Placeholder(),
      ),
      
      // Qibla compass routes
      GoRoute(
        path: '/qibla',
        name: 'qibla',
        builder: (context, state) => const Placeholder(),
      ),
      
      // Hadith routes
      GoRoute(
        path: '/hadith',
        name: 'hadith',
        builder: (context, state) => const Placeholder(),
      ),
      
      // Recitation analysis routes
      GoRoute(
        path: '/recitation',
        name: 'recitation',
        builder: (context, state) => const Placeholder(),
      ),
      
      // Search routes
      GoRoute(
        path: '/search',
        name: 'search',
        builder: (context, state) => const Placeholder(),
      ),
      
      // Settings routes
      GoRoute(
        path: '/settings',
        name: 'settings',
        builder: (context, state) => const Placeholder(),
      ),
    ],
    
    // Error handling
    errorBuilder: (context, state) => Scaffold(
      body: Center(
        child: Text('خطأ: ${state.error}'),
      ),
    ),
  );
});
