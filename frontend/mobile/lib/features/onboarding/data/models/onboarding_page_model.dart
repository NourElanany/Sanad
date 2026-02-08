/// Model for onboarding page data
class OnboardingPageModel {
  final String title;
  final String description;
  final String iconPath;
  final String? lottieAnimation;

  const OnboardingPageModel({
    required this.title,
    required this.description,
    required this.iconPath,
    this.lottieAnimation,
  });
}
