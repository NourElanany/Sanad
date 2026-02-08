'use client';

import React, { useState } from 'react';
import {
  IslamicButton,
  IslamicCard,
  IslamicCardWithHeader,
  IslamicGradientCard,
  IslamicTextField,
  IslamicDropdown,
  IslamicCheckbox,
  IslamicRadio,
  IslamicSwitch,
  IslamicLoadingIndicator,
  IslamicLoadingWithText,
  IslamicShimmer,
  IslamicPulsingIndicator,
  IslamicModal,
  IslamicConfirmationModal,
  IslamicSuccessModal,
  IslamicErrorModal,
  IslamicBottomSheet,
  IslamicAppBar,
} from '@/components/ui';

export default function ComponentsDemo() {
  const [textValue, setTextValue] = useState('');
  const [dropdownValue, setDropdownValue] = useState('');
  const [checkboxValue, setCheckboxValue] = useState(false);
  const [radioValue, setRadioValue] = useState('option1');
  const [switchValue, setSwitchValue] = useState(false);
  
  const [showModal, setShowModal] = useState(false);
  const [showConfirmModal, setShowConfirmModal] = useState(false);
  const [showSuccessModal, setShowSuccessModal] = useState(false);
  const [showErrorModal, setShowErrorModal] = useState(false);
  const [showBottomSheet, setShowBottomSheet] = useState(false);

  return (
    <div className="min-h-screen bg-background-primary" dir="rtl">
      <IslamicAppBar
        title="مكونات الواجهة الإسلامية"
        actions={
          <>
            <button className="p-2 hover:bg-white hover:bg-opacity-10 rounded-lg transition-colors">
              🔍
            </button>
            <button className="p-2 hover:bg-white hover:bg-opacity-10 rounded-lg transition-colors">
              🔔
            </button>
          </>
        }
      />

      <div className="max-w-4xl mx-auto p-6 space-y-12">
        {/* Buttons Section */}
        <section>
          <h2 className="text-2xl font-bold font-tajawal text-text-primary mb-6">
            الأزرار
          </h2>
          <div className="space-y-4">
            <IslamicButton type="primary" icon="✓">
              زر أساسي
            </IslamicButton>
            <IslamicButton type="secondary">زر ثانوي</IslamicButton>
            <IslamicButton type="outlined">زر محدد</IslamicButton>
            <IslamicButton type="gradient" icon="⭐">
              زر متدرج
            </IslamicButton>
            <IslamicButton type="primary" isLoading>
              جاري التحميل...
            </IslamicButton>
          </div>
        </section>

        {/* Cards Section */}
        <section>
          <h2 className="text-2xl font-bold font-tajawal text-text-primary mb-6">
            البطاقات
          </h2>
          <div className="space-y-4">
            <IslamicCard>
              <div className="text-center">
                <h3 className="text-lg font-semibold font-tajawal text-text-primary mb-2">
                  بطاقة إسلامية بسيطة
                </h3>
                <p className="text-sm font-tajawal text-text-secondary">
                  هذا مثال على بطاقة إسلامية بتصميم حديث وأنيق
                </p>
              </div>
            </IslamicCard>

            <IslamicCardWithHeader
              title="بطاقة مع عنوان"
              icon="🕌"
              trailing={<span className="text-sm">←</span>}
            >
              <p className="text-sm font-tajawal text-text-secondary">
                محتوى البطاقة يظهر هنا مع تصميم إسلامي جميل
              </p>
            </IslamicCardWithHeader>

            <IslamicGradientCard>
              <div className="text-center">
                <div className="text-4xl mb-2">⭐</div>
                <h3 className="text-lg font-bold font-tajawal mb-1">
                  بطاقة متدرجة
                </h3>
                <p className="text-sm font-tajawal">
                  تصميم جذاب مع خلفية متدرجة
                </p>
              </div>
            </IslamicGradientCard>
          </div>
        </section>

        {/* Form Components Section */}
        <section>
          <h2 className="text-2xl font-bold font-tajawal text-text-primary mb-6">
            مكونات النماذج
          </h2>
          <div className="space-y-4">
            <IslamicTextField
              label="حقل نصي"
              placeholder="أدخل النص هنا"
              value={textValue}
              onChange={setTextValue}
              prefixIcon={<span>👤</span>}
            />

            <IslamicDropdown
              label="قائمة منسدلة"
              placeholder="اختر خياراً"
              value={dropdownValue}
              onChange={setDropdownValue}
              options={[
                { value: 'option1', label: 'الخيار الأول' },
                { value: 'option2', label: 'الخيار الثاني' },
                { value: 'option3', label: 'الخيار الثالث' },
              ]}
              prefixIcon={<span>📋</span>}
            />

            <IslamicCheckbox
              label="خيار اختيار"
              checked={checkboxValue}
              onChange={setCheckboxValue}
            />

            <div className="space-y-2">
              <IslamicRadio
                value="option1"
                groupValue={radioValue}
                label="خيار راديو 1"
                onChange={setRadioValue}
              />
              <IslamicRadio
                value="option2"
                groupValue={radioValue}
                label="خيار راديو 2"
                onChange={setRadioValue}
              />
            </div>

            <IslamicSwitch
              label="مفتاح تبديل"
              subtitle="وصف إضافي للمفتاح"
              checked={switchValue}
              onChange={setSwitchValue}
            />
          </div>
        </section>

        {/* Loading Indicators Section */}
        <section>
          <h2 className="text-2xl font-bold font-tajawal text-text-primary mb-6">
            مؤشرات التحميل
          </h2>
          <div className="space-y-6">
            <div className="flex items-center justify-around">
              <IslamicLoadingIndicator size="md" />
              <IslamicPulsingIndicator size="md" />
            </div>
            <IslamicLoadingWithText text="جاري التحميل..." />
            <IslamicShimmer height="h-24" />
          </div>
        </section>

        {/* Modals Section */}
        <section>
          <h2 className="text-2xl font-bold font-tajawal text-text-primary mb-6">
            النوافذ المنبثقة
          </h2>
          <div className="space-y-4">
            <IslamicButton
              type="outlined"
              onClick={() => setShowModal(true)}
              fullWidth
            >
              عرض نافذة عادية
            </IslamicButton>
            <IslamicButton
              type="outlined"
              onClick={() => setShowSuccessModal(true)}
              fullWidth
            >
              عرض نافذة نجاح
            </IslamicButton>
            <IslamicButton
              type="outlined"
              onClick={() => setShowErrorModal(true)}
              fullWidth
            >
              عرض نافذة خطأ
            </IslamicButton>
            <IslamicButton
              type="outlined"
              onClick={() => setShowConfirmModal(true)}
              fullWidth
            >
              عرض نافذة تأكيد
            </IslamicButton>
            <IslamicButton
              type="outlined"
              onClick={() => setShowBottomSheet(true)}
              fullWidth
            >
              عرض قائمة سفلية
            </IslamicButton>
          </div>
        </section>
      </div>

      {/* Modals */}
      <IslamicModal
        isOpen={showModal}
        onClose={() => setShowModal(false)}
        title="نافذة منبثقة"
        actions={[
          {
            label: 'إلغاء',
            isPrimary: false,
            dismissOnPress: true,
          },
          {
            label: 'تأكيد',
            isPrimary: true,
            dismissOnPress: true,
          },
        ]}
      >
        <p className="text-base font-tajawal text-text-secondary text-center">
          هذا مثال على نافذة منبثقة إسلامية
        </p>
      </IslamicModal>

      <IslamicConfirmationModal
        isOpen={showConfirmModal}
        onClose={() => setShowConfirmModal(false)}
        title="تأكيد"
        message="هل أنت متأكد من تنفيذ هذا الإجراء؟"
        onConfirm={() => console.log('Confirmed')}
      />

      <IslamicSuccessModal
        isOpen={showSuccessModal}
        onClose={() => setShowSuccessModal(false)}
        title="نجح!"
        message="تمت العملية بنجاح"
      />

      <IslamicErrorModal
        isOpen={showErrorModal}
        onClose={() => setShowErrorModal(false)}
        title="خطأ!"
        message="حدث خطأ أثناء تنفيذ العملية"
      />

      <IslamicBottomSheet
        isOpen={showBottomSheet}
        onClose={() => setShowBottomSheet(false)}
        title="اختر خياراً"
      >
        <div className="space-y-2">
          {['الخيار الأول', 'الخيار الثاني', 'الخيار الثالث'].map(
            (option, index) => (
              <button
                key={index}
                onClick={() => {
                  console.log('Selected:', option);
                  setShowBottomSheet(false);
                }}
                className="w-full p-4 text-right hover:bg-primary-main hover:bg-opacity-5 rounded-lg transition-colors"
              >
                <span className="text-base font-tajawal text-text-primary">
                  {option}
                </span>
              </button>
            )
          )}
        </div>
      </IslamicBottomSheet>
    </div>
  );
}
