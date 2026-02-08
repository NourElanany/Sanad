# Islamic UI Components Documentation

## Overview

This document describes the Islamic-themed UI components implemented for both the Flutter mobile app and Next.js web app. All components follow the Islamic design system with deep navy/emerald green primary colors, muted gold accents, and Arabic-first typography.

## Design System

### Color Palette

- **Primary**: `#1B365D` (Deep Navy) - Main brand color
- **Primary Light**: `#2E4A6B` - Hover states
- **Primary Dark**: `#0F1F35` - Pressed states
- **Secondary**: `#2D5A27` (Emerald Green)
- **Accent Gold**: `#B8860B` - Headings and active icons
- **Background**: `#FEFEFE` (Off-white) - Comfortable reading
- **Text Primary**: `#1A1A1A` - Main text
- **Text Secondary**: `#666666` - Secondary text

### Typography

- **Regular Text**: Tajawal, Alexandria (sans-serif)
- **Quranic Text**: KFGQPC Uthman Taha Naskh, Amiri (serif)
- **Font Sizes**: 12px (caption) to 48px (h1)

## Flutter Components

### Location
All components are located in `frontend/mobile/lib/core/widgets/`

### Components List

#### 1. IslamicButton
Islamic-themed button with multiple variants.

**Types:**
- `primary` - Solid primary color button
- `secondary` - Solid secondary color button
- `outlined` - Outlined button with border
- `text` - Text-only button
- `gradient` - Gradient background button

**Props:**
```dart
IslamicButton(
  text: 'Button Text',
  type: IslamicButtonType.primary,
  icon: Icons.check,
  isLoading: false,
  onPressed: () {},
)
```

#### 2. IslamicCard
Card component with Islamic styling.

**Variants:**
- `IslamicCard` - Basic card
- `IslamicCardWithHeader` - Card with header section
- `IslamicGradientCard` - Card with gradient background

**Props:**
```dart
IslamicCard(
  elevated: true,
  onTap: () {},
  child: Widget,
)
```

#### 3. IslamicTextField
Text input field with Islamic styling.

**Props:**
```dart
IslamicTextField(
  label: 'Label',
  hint: 'Placeholder',
  controller: TextEditingController(),
  prefixIcon: Icons.person,
  validator: (value) => null,
)
```

#### 4. IslamicDropdown
Dropdown selection field.

**Props:**
```dart
IslamicDropdown<String>(
  label: 'Select',
  value: selectedValue,
  items: [
    IslamicDropdownItem(value: '1', label: 'Option 1'),
  ],
  onChanged: (value) {},
)
```

#### 5. IslamicCheckbox
Checkbox with label.

**Props:**
```dart
IslamicCheckbox(
  label: 'Checkbox Label',
  value: isChecked,
  onChanged: (value) {},
)
```

#### 6. IslamicRadio
Radio button with label.

**Props:**
```dart
IslamicRadio<String>(
  value: 'option1',
  groupValue: selectedValue,
  label: 'Radio Label',
  onChanged: (value) {},
)
```

#### 7. IslamicSwitch
Toggle switch with label and subtitle.

**Props:**
```dart
IslamicSwitch(
  label: 'Switch Label',
  subtitle: 'Description',
  value: isEnabled,
  onChanged: (value) {},
)
```

#### 8. IslamicLoadingIndicator
Loading spinner with Islamic styling.

**Variants:**
- `IslamicLoadingIndicator` - Basic spinner
- `IslamicLoadingWithText` - Spinner with text
- `IslamicShimmer` - Shimmer loading effect
- `IslamicPulsingIndicator` - Pulsing circle indicator
- `IslamicLoadingOverlay` - Full-screen overlay

**Props:**
```dart
IslamicLoadingIndicator(
  size: 40,
  color: AppColors.primaryMain,
)
```

#### 9. IslamicModal
Modal dialogs with Islamic styling.

**Methods:**
- `IslamicModal.showDialog()` - Basic dialog
- `IslamicModal.showConfirmation()` - Confirmation dialog
- `IslamicModal.showSuccess()` - Success dialog
- `IslamicModal.showError()` - Error dialog

**Usage:**
```dart
IslamicModal.showSuccess(
  context: context,
  title: 'Success',
  message: 'Operation completed',
)
```

#### 10. IslamicBottomSheet
Bottom sheet with Islamic styling.

**Methods:**
- `IslamicBottomSheet.show()` - Basic bottom sheet
- `IslamicBottomSheet.showList()` - List selection bottom sheet

**Usage:**
```dart
IslamicBottomSheet.showList<String>(
  context: context,
  title: 'Select Option',
  items: [
    IslamicBottomSheetItem(
      value: 'option1',
      title: 'Option 1',
      icon: Icons.check,
    ),
  ],
)
```

#### 11. IslamicAppBar
App bar with Islamic styling.

**Props:**
```dart
IslamicAppBar(
  title: 'Page Title',
  actions: [Icon(Icons.search)],
  onBack: () {},
)
```

#### 12. IslamicBottomNavBar
Bottom navigation bar.

**Props:**
```dart
IslamicBottomNavBar(
  currentIndex: 0,
  onTap: (index) {},
  items: [
    IslamicNavItem(
      icon: Icons.home,
      label: 'Home',
    ),
  ],
)
```

#### 13. IslamicDrawer
Navigation drawer with Islamic styling.

**Props:**
```dart
IslamicDrawer(
  userName: 'User Name',
  userEmail: 'email@example.com',
  items: [
    IslamicDrawerItem(
      icon: Icons.settings,
      title: 'Settings',
      onTap: () {},
    ),
  ],
)
```

#### 14. IslamicTabBar
Tab bar with Islamic styling.

**Props:**
```dart
IslamicTabBar(
  tabs: ['Tab 1', 'Tab 2'],
  controller: TabController(),
)
```

## Next.js Components

### Location
All components are located in `frontend/nextjs-app/src/components/ui/`

### Components List

All Next.js components have similar functionality to their Flutter counterparts with React/TypeScript syntax.

#### Example Usage

```tsx
import { IslamicButton, IslamicCard } from '@/components/ui';

function MyComponent() {
  return (
    <div>
      <IslamicButton type="primary" onClick={() => {}}>
        Click Me
      </IslamicButton>
      
      <IslamicCard elevated>
        <h3>Card Title</h3>
        <p>Card content</p>
      </IslamicCard>
    </div>
  );
}
```

### Component Props

#### IslamicButton
```tsx
interface IslamicButtonProps {
  children: React.ReactNode;
  onClick?: () => void;
  type?: 'primary' | 'secondary' | 'outlined' | 'text' | 'gradient';
  icon?: React.ReactNode;
  isLoading?: boolean;
  disabled?: boolean;
  fullWidth?: boolean;
}
```

#### IslamicCard
```tsx
interface IslamicCardProps {
  children: React.ReactNode;
  onClick?: () => void;
  elevated?: boolean;
  className?: string;
  padding?: string;
}
```

#### IslamicTextField
```tsx
interface IslamicTextFieldProps {
  label?: string;
  placeholder?: string;
  value?: string;
  onChange?: (value: string) => void;
  error?: string;
  type?: 'text' | 'email' | 'password' | 'number' | 'tel';
  prefixIcon?: React.ReactNode;
  suffixIcon?: React.ReactNode;
  disabled?: boolean;
  required?: boolean;
  dir?: 'rtl' | 'ltr';
}
```

#### IslamicModal
```tsx
interface IslamicModalProps {
  isOpen: boolean;
  onClose: () => void;
  title: string;
  children: React.ReactNode;
  actions?: IslamicModalAction[];
}
```

## Demo Pages

### Flutter Demo
Location: `frontend/mobile/lib/core/widgets/examples/components_demo.dart`

Run the demo:
```bash
cd frontend/mobile
flutter run
```

### Next.js Demo
Location: `frontend/nextjs-app/src/app/components-demo/page.tsx`

Run the demo:
```bash
cd frontend/nextjs-app
npm run dev
```

Visit: `http://localhost:3000/components-demo`

## Styling Guidelines

### Flutter
- Use `AppColors` constants from `lib/core/theme/app_colors.dart`
- Use `AppTextStyles` from `lib/core/theme/app_text_styles.dart`
- Follow Material Design 3 principles
- Maintain 60fps performance

### Next.js
- Use Tailwind CSS utility classes
- Use custom color classes: `bg-primary-main`, `text-accent-gold`, etc.
- Use custom font classes: `font-tajawal`, `font-quran`
- Ensure responsive design with mobile-first approach

## Accessibility

All components support:
- Screen readers (semantic HTML/Flutter semantics)
- Keyboard navigation
- High contrast mode
- RTL text direction
- Touch targets (minimum 44x44 pixels)

## Testing

### Flutter
```bash
cd frontend/mobile
flutter test
```

### Next.js
```bash
cd frontend/nextjs-app
npm test
```

## Requirements Validation

These components satisfy the following requirements:

- **Requirement 18.1**: Deep navy/emerald green primary colors ✓
- **Requirement 18.2**: Muted gold for headings and active icons ✓
- **Requirement 18.3**: Off-white background for comfortable reading ✓
- **Requirement 18.4**: Tajawal/Alexandria fonts for regular text ✓
- **Requirement 18.5**: KFGQPC Uthman Taha Naskh for Quranic text ✓

## Next Steps

1. Integrate components into actual app screens
2. Add unit tests for all components
3. Add widget/component tests
4. Implement dark mode variants
5. Add animation polish
6. Optimize performance
7. Add accessibility tests

## Support

For questions or issues, please refer to:
- Design document: `.kiro/specs/sanad-frontend/design.md`
- Requirements: `.kiro/specs/sanad-frontend/requirements.md`
- Tasks: `.kiro/specs/sanad-frontend/tasks.md`
