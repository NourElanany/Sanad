import '../models/onboarding_page_model.dart';

/// Static data for onboarding pages
class OnboardingData {
  static const List<OnboardingPageModel> pages = [
    OnboardingPageModel(
      title: 'مرحباً بك في سَنَد',
      description: 'تطبيقك الإسلامي الشامل للقرآن الكريم والأحاديث النبوية والمساعد الذكي',
      iconPath: 'mosque',
    ),
    OnboardingPageModel(
      title: 'القرآن الكريم',
      description: 'اقرأ القرآن الكريم بخط واضح مع التفاسير المتعددة والترجمات',
      iconPath: 'book',
    ),
    OnboardingPageModel(
      title: 'مصحح التلاوة',
      description: 'حسّن تلاوتك مع نظام التحليل الصوتي الذكي وتصحيح التجويد',
      iconPath: 'mic',
    ),
    OnboardingPageModel(
      title: 'المساعد الذكي',
      description: 'اسأل أي سؤال إسلامي واحصل على إجابات موثوقة مع المصادر',
      iconPath: 'chat',
    ),
    OnboardingPageModel(
      title: 'مواقيت الصلاة',
      description: 'احصل على مواقيت الصلاة الدقيقة حسب موقعك ومذهبك الفقهي',
      iconPath: 'schedule',
    ),
  ];
}
