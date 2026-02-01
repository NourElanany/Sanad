# اختبارات واجهة سند - Sanad Interface Tests

## نظرة عامة

هذا المجلد يحتوي على مجموعة شاملة من الاختبارات لواجهة تطبيق سند الإسلامي. الاختبارات تغطي جميع جوانب الواجهة بما في ذلك التنقل، تبديل اللغات، والتصميم المتجاوب.

## هيكل الاختبارات

```
tests/
├── integration/                    # اختبارات التكامل
│   ├── interface-integration.test.js    # الاختبارات الأساسية
│   ├── interface-property.test.js       # اختبارات الخصائص
│   └── enhanced-interface-tests.js      # اختبارات متقدمة
├── setup/                          # إعدادات الاختبارات
│   ├── test-setup.js               # الإعداد العام
│   └── custom-matchers.js          # مطابقات مخصصة
├── coverage/                       # تقارير التغطية
├── reports/                        # تقارير الاختبارات
├── test-runner.html               # مشغل الاختبارات التفاعلي
├── jest.config.js                 # إعدادات Jest
└── README.md                      # هذا الملف
```

## أنواع الاختبارات

### 1. اختبارات التكامل (Integration Tests)

تختبر التفاعل بين مكونات الواجهة المختلفة:

- **اختبارات التنقل**: التنقل بين الأقسام، اختصارات لوحة المفاتيح، القائمة المحمولة
- **اختبارات اللغة**: تبديل اللغات، اتجاه النص، حفظ التفضيلات
- **اختبارات التصميم المتجاوب**: التكيف مع أحجام الشاشات المختلفة

### 2. اختبارات الخصائص (Property-Based Tests)

تستخدم مكتبة `fast-check` لاختبار الخصائص العامة:

- **خاصية اتساق التنقل**: أي تنقل صحيح يجب أن ينتج عنه القسم المطلوب
- **خاصية اتساق اللغة**: تبديل اللغة يجب أن يحدث جميع العناصر المرتبطة
- **خاصية التصميم المتجاوب**: التخطيط يجب أن يتكيف مع جميع أحجام الشاشات

### 3. اختبارات متقدمة (Enhanced Tests)

تغطي سيناريوهات معقدة ومتقدمة:

- **اختبارات الأداء**: التفاعلات السريعة، إدارة الذاكرة
- **اختبارات إمكانية الوصول**: معايير WCAG، أهداف اللمس
- **اختبارات معالجة الأخطاء**: سيناريوهات الفشل، الاستعادة

## تشغيل الاختبارات

### المتطلبات

```bash
# تثبيت Node.js (الإصدار 16 أو أحدث)
# تثبيت التبعيات
npm install
```

### الأوامر المتاحة

```bash
# تشغيل جميع الاختبارات
npm test

# تشغيل الاختبارات مع المراقبة
npm run test:watch

# تشغيل الاختبارات مع تقرير التغطية
npm run test:coverage

# تشغيل اختبارات التكامل فقط
npm run test:integration

# تشغيل اختبارات الخصائص فقط
npm run test:property

# تشغيل الاختبارات المتقدمة فقط
npm run test:enhanced

# فتح مشغل الاختبارات التفاعلي
npm run test:runner

# تشغيل جميع الاختبارات (Jest + مشغل تفاعلي)
npm run test:all
```

### مشغل الاختبارات التفاعلي

يمكن فتح `test-runner.html` في المتصفح لتشغيل الاختبارات بشكل تفاعلي:

```bash
# تشغيل خادم محلي
npm run serve:test

# ثم فتح http://localhost:8080/test-runner.html
```

## كتابة اختبارات جديدة

### مثال على اختبار تكامل

```javascript
describe('اختبار ميزة جديدة', () => {
  beforeEach(async () => {
    document.body.innerHTML = await testUtils.fixtures.loadFixture('basic');
    await initializeApp();
  });

  test('يجب أن تعمل الميزة بشكل صحيح', async () => {
    // ترتيب
    const element = document.getElementById('myElement');
    
    // تنفيذ
    testUtils.dom.simulateClick(element);
    
    // تحقق
    await testUtils.async.waitFor(() => {
      return element.classList.contains('active');
    });
    
    expect(element).toHaveClass('active');
  });
});
```

### مثال على اختبار خاصية

```javascript
test('خاصية عامة للميزة', () => {
  fc.assert(fc.property(
    fc.constantFrom('value1', 'value2', 'value3'),
    (inputValue) => {
      // تنفيذ العملية
      const result = myFunction(inputValue);
      
      // التحقق من الخاصية
      expect(result).toBeDefined();
      expect(typeof result).toBe('string');
      
      return true;
    }
  ), { numRuns: 100 });
});
```

## المطابقات المخصصة

تم إنشاء مطابقات مخصصة لتسهيل اختبار الواجهة:

```javascript
// اختبار الرؤية في منطقة العرض
expect(element).toBeVisibleInViewport();

// اختبار التصميم المتجاوب
expect(element).toBeResponsive('mobile');

// اختبار اتجاه النص
expect(element).toHaveCorrectTextDirection('rtl');

// اختبار إمكانية الوصول
expect(element).toBeAccessible();

// اختبار اتساق حالة التنقل
expect(document).toHaveConsistentNavigationState('quran');

// اختبار اتساق حالة اللغة
expect(document).toHaveConsistentLanguageState('ar');

// اختبار حالة التحميل
expect(element).toHaveLoadingState();

// اختبار حالة الخطأ
expect(element).toHaveErrorState('رسالة الخطأ');
```

## تقارير التغطية

بعد تشغيل `npm run test:coverage`، يمكن عرض تقرير التغطية:

```bash
# فتح تقرير HTML
open tests/coverage/lcov-report/index.html
```

## أفضل الممارسات

### 1. تنظيم الاختبارات

- استخدم أسماء وصفية للاختبارات
- جمع الاختبارات المترابطة في مجموعات
- استخدم `beforeEach` و `afterEach` للإعداد والتنظيف

### 2. كتابة اختبارات موثوقة

- تجنب الاعتماد على التوقيتات الثابتة
- استخدم `waitFor` للعمليات غير المتزامنة
- اختبر السلوك وليس التنفيذ

### 3. اختبار إمكانية الوصول

- تحقق من أحجام أهداف اللمس (44px كحد أدنى)
- اختبر التنقل بلوحة المفاتيح
- تأكد من وجود نصوص بديلة للصور

### 4. اختبار التصميم المتجاوب

- اختبر على أحجام شاشات متعددة
- تحقق من سلوك القوائم المحمولة
- اختبر تغيير الاتجاه (عمودي/أفقي)

### 5. اختبار تعدد اللغات

- اختبر جميع اللغات المدعومة
- تحقق من اتجاه النص الصحيح
- اختبر حفظ واستعادة تفضيلات اللغة

## استكشاف الأخطاء

### مشاكل شائعة

1. **فشل الاختبارات بسبب التوقيت**
   ```javascript
   // بدلاً من setTimeout
   await testUtils.async.wait(100);
   
   // استخدم waitFor
   await testUtils.async.waitFor(() => condition);
   ```

2. **عناصر DOM غير موجودة**
   ```javascript
   // تأكد من تحميل الـ fixture
   document.body.innerHTML = await testUtils.fixtures.loadFixture('basic');
   ```

3. **مشاكل في localStorage**
   ```javascript
   // تنظيف localStorage قبل كل اختبار
   beforeEach(() => {
     localStorage.clear();
   });
   ```

### تشغيل اختبار واحد

```bash
# تشغيل ملف اختبار محدد
npx jest interface-integration.test.js

# تشغيل اختبار محدد
npx jest -t "اسم الاختبار"
```

### وضع التصحيح

```bash
# تشغيل Jest في وضع التصحيح
node --inspect-brk node_modules/.bin/jest --runInBand
```

## المساهمة

عند إضافة اختبارات جديدة:

1. اتبع نمط التسمية الموجود
2. أضف تعليقات باللغة العربية
3. تأكد من تغطية الحالات الحدية
4. اختبر على أجهزة متعددة
5. حدث هذا الملف إذا لزم الأمر

## الموارد

- [Jest Documentation](https://jestjs.io/docs/getting-started)
- [Testing Library](https://testing-library.com/docs/)
- [fast-check Documentation](https://fast-check.dev/)
- [WCAG Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)

---

**ملاحظة**: هذه الاختبارات تدعم المتطلبات 1.3 و 10.2 من مواصفات التطبيق وتغطي المهمة 14.3 "كتابة اختبارات التكامل للواجهة".