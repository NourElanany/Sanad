import 'package:flutter/material.dart';
import '../widgets.dart';

/// Demo screen showcasing all Islamic UI components
class ComponentsDemo extends StatefulWidget {
  const ComponentsDemo({Key? key}) : super(key: key);

  @override
  State<ComponentsDemo> createState() => _ComponentsDemoState();
}

class _ComponentsDemoState extends State<ComponentsDemo> {
  bool _switchValue = false;
  bool _checkboxValue = false;
  String _radioValue = 'option1';
  String? _dropdownValue;
  final _textController = TextEditingController();

  @override
  void dispose() {
    _textController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: const IslamicAppBar(
        title: 'مكونات الواجهة الإسلامية',
        actions: [
          Icon(Icons.search),
          SizedBox(width: 8),
          Icon(Icons.notifications),
          SizedBox(width: 16),
        ],
      ),
      body: SingleChildScrollView(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          children: [
            // Buttons Section
            const Text(
              'الأزرار',
              style: TextStyle(
                fontSize: 20,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 16),
            IslamicButton(
              text: 'زر أساسي',
              type: IslamicButtonType.primary,
              icon: Icons.check,
              onPressed: () => _showSuccessDialog(),
            ),
            const SizedBox(height: 12),
            IslamicButton(
              text: 'زر ثانوي',
              type: IslamicButtonType.secondary,
              onPressed: () {},
            ),
            const SizedBox(height: 12),
            IslamicButton(
              text: 'زر محدد',
              type: IslamicButtonType.outlined,
              onPressed: () {},
            ),
            const SizedBox(height: 12),
            IslamicButton(
              text: 'زر متدرج',
              type: IslamicButtonType.gradient,
              icon: Icons.star,
              onPressed: () {},
            ),
            const SizedBox(height: 12),
            IslamicButton(
              text: 'جاري التحميل...',
              type: IslamicButtonType.primary,
              isLoading: true,
            ),
            const SizedBox(height: 32),

            // Cards Section
            const Text(
              'البطاقات',
              style: TextStyle(
                fontSize: 20,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 16),
            IslamicCard(
              child: Column(
                children: const [
                  Text(
                    'بطاقة إسلامية بسيطة',
                    style: TextStyle(
                      fontSize: 16,
                      fontWeight: FontWeight.w600,
                      fontFamily: 'Tajawal',
                    ),
                  ),
                  SizedBox(height: 8),
                  Text(
                    'هذا مثال على بطاقة إسلامية بتصميم حديث وأنيق',
                    style: TextStyle(
                      fontSize: 14,
                      fontFamily: 'Tajawal',
                    ),
                    textAlign: TextAlign.center,
                  ),
                ],
              ),
            ),
            const SizedBox(height: 16),
            IslamicCardWithHeader(
              title: 'بطاقة مع عنوان',
              icon: Icons.mosque,
              trailing: const Icon(Icons.arrow_forward_ios, size: 16),
              child: const Text(
                'محتوى البطاقة يظهر هنا مع تصميم إسلامي جميل',
                style: TextStyle(
                  fontSize: 14,
                  fontFamily: 'Tajawal',
                ),
              ),
            ),
            const SizedBox(height: 16),
            IslamicGradientCard(
              child: Column(
                children: const [
                  Icon(Icons.star, color: Colors.white, size: 32),
                  SizedBox(height: 8),
                  Text(
                    'بطاقة متدرجة',
                    style: TextStyle(
                      fontSize: 18,
                      fontWeight: FontWeight.bold,
                      fontFamily: 'Tajawal',
                    ),
                  ),
                  SizedBox(height: 4),
                  Text(
                    'تصميم جذاب مع خلفية متدرجة',
                    style: TextStyle(
                      fontSize: 14,
                      fontFamily: 'Tajawal',
                    ),
                  ),
                ],
              ),
            ),
            const SizedBox(height: 32),

            // Form Components Section
            const Text(
              'مكونات النماذج',
              style: TextStyle(
                fontSize: 20,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 16),
            IslamicTextField(
              label: 'حقل نصي',
              hint: 'أدخل النص هنا',
              controller: _textController,
              prefixIcon: Icons.person,
            ),
            const SizedBox(height: 16),
            IslamicDropdown<String>(
              label: 'قائمة منسدلة',
              hint: 'اختر خياراً',
              value: _dropdownValue,
              items: const [
                IslamicDropdownItem(value: 'option1', label: 'الخيار الأول'),
                IslamicDropdownItem(value: 'option2', label: 'الخيار الثاني'),
                IslamicDropdownItem(value: 'option3', label: 'الخيار الثالث'),
              ],
              onChanged: (value) => setState(() => _dropdownValue = value),
              prefixIcon: Icons.list,
            ),
            const SizedBox(height: 16),
            IslamicCheckbox(
              label: 'خيار اختيار',
              value: _checkboxValue,
              onChanged: (value) => setState(() => _checkboxValue = value ?? false),
            ),
            const SizedBox(height: 8),
            IslamicRadio<String>(
              value: 'option1',
              groupValue: _radioValue,
              label: 'خيار راديو 1',
              onChanged: (value) => setState(() => _radioValue = value ?? 'option1'),
            ),
            IslamicRadio<String>(
              value: 'option2',
              groupValue: _radioValue,
              label: 'خيار راديو 2',
              onChanged: (value) => setState(() => _radioValue = value ?? 'option1'),
            ),
            const SizedBox(height: 16),
            IslamicSwitch(
              label: 'مفتاح تبديل',
              subtitle: 'وصف إضافي للمفتاح',
              value: _switchValue,
              onChanged: (value) => setState(() => _switchValue = value),
            ),
            const SizedBox(height: 32),

            // Loading Indicators Section
            const Text(
              'مؤشرات التحميل',
              style: TextStyle(
                fontSize: 20,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 16),
            const Row(
              mainAxisAlignment: MainAxisAlignment.spaceAround,
              children: [
                IslamicLoadingIndicator(size: 30),
                IslamicPulsingIndicator(size: 50),
              ],
            ),
            const SizedBox(height: 16),
            const IslamicShimmer(height: 100),
            const SizedBox(height: 32),

            // Modals Section
            const Text(
              'النوافذ المنبثقة',
              style: TextStyle(
                fontSize: 20,
                fontWeight: FontWeight.bold,
                fontFamily: 'Tajawal',
              ),
            ),
            const SizedBox(height: 16),
            IslamicButton(
              text: 'عرض نافذة نجاح',
              type: IslamicButtonType.outlined,
              onPressed: () => _showSuccessDialog(),
            ),
            const SizedBox(height: 12),
            IslamicButton(
              text: 'عرض نافذة خطأ',
              type: IslamicButtonType.outlined,
              onPressed: () => _showErrorDialog(),
            ),
            const SizedBox(height: 12),
            IslamicButton(
              text: 'عرض نافذة تأكيد',
              type: IslamicButtonType.outlined,
              onPressed: () => _showConfirmationDialog(),
            ),
            const SizedBox(height: 12),
            IslamicButton(
              text: 'عرض قائمة سفلية',
              type: IslamicButtonType.outlined,
              onPressed: () => _showBottomSheet(),
            ),
            const SizedBox(height: 32),
          ],
        ),
      ),
    );
  }

  void _showSuccessDialog() {
    IslamicModal.showSuccess(
      context: context,
      title: 'نجح!',
      message: 'تمت العملية بنجاح',
    );
  }

  void _showErrorDialog() {
    IslamicModal.showError(
      context: context,
      title: 'خطأ!',
      message: 'حدث خطأ أثناء تنفيذ العملية',
    );
  }

  void _showConfirmationDialog() {
    IslamicModal.showConfirmation(
      context: context,
      title: 'تأكيد',
      message: 'هل أنت متأكد من تنفيذ هذا الإجراء؟',
    );
  }

  void _showBottomSheet() {
    IslamicBottomSheet.showList<String>(
      context: context,
      title: 'اختر خياراً',
      items: const [
        IslamicBottomSheetItem(
          value: 'option1',
          title: 'الخيار الأول',
          icon: Icons.check_circle,
        ),
        IslamicBottomSheetItem(
          value: 'option2',
          title: 'الخيار الثاني',
          icon: Icons.star,
        ),
        IslamicBottomSheetItem(
          value: 'option3',
          title: 'الخيار الثالث',
          icon: Icons.favorite,
        ),
      ],
    );
  }
}
