import type { Config } from 'tailwindcss'

const config: Config = {
  content: [
    './src/pages/**/*.{js,ts,jsx,tsx,mdx}',
    './src/components/**/*.{js,ts,jsx,tsx,mdx}',
    './src/app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        // Islamic Theme Colors
        primary: {
          DEFAULT: '#1B365D', // كحلي داكن
          light: '#2E4A6B',
          dark: '#0F1F35',
          50: '#E8EBF0',
          100: '#D1D7E1',
          200: '#A3AFC3',
          300: '#7587A5',
          400: '#475F87',
          500: '#1B365D',
          600: '#162B4A',
          700: '#102038',
          800: '#0B1525',
          900: '#050B13',
        },
        secondary: {
          DEFAULT: '#2D5A27', // أخضر زمردي
          light: '#4A7C59',
          dark: '#1A3318',
          50: '#E9F2E8',
          100: '#D3E5D1',
          200: '#A7CBA3',
          300: '#7BB175',
          400: '#4F9747',
          500: '#2D5A27',
          600: '#24481F',
          700: '#1B3617',
          800: '#122410',
          900: '#091208',
        },
        accent: {
          gold: '#B8860B',
          lightGold: '#DAA520',
          50: '#FDF8E8',
          100: '#FBF1D1',
          200: '#F7E3A3',
          300: '#F3D575',
          400: '#EFC747',
          500: '#B8860B',
          600: '#936B09',
          700: '#6E5007',
          800: '#4A3604',
          900: '#251B02',
        },
        background: {
          primary: '#FEFEFE',
          secondary: '#F8F9FA',
          paper: '#FFFFFF',
        },
        text: {
          primary: '#1A1A1A',
          secondary: '#666666',
          disabled: '#CCCCCC',
          quranic: '#0F1F35',
        },
        status: {
          success: '#28A745',
          warning: '#FFC107',
          error: '#DC3545',
          info: '#17A2B8',
        },
      },
      fontFamily: {
        // للنصوص العادية والواجهة
        sans: ['Tajawal', 'Alexandria', 'system-ui', 'sans-serif'],
        // للنصوص القرآنية
        quran: ['KFGQPC Uthman Taha Naskh', 'Amiri', 'serif'],
      },
      fontSize: {
        'caption': '0.75rem',     // 12px
        'body2': '0.875rem',      // 14px
        'body1': '1rem',          // 16px
        'subtitle2': '1.125rem',  // 18px
        'subtitle1': '1.25rem',   // 20px
        'h6': '1.5rem',           // 24px
        'h5': '1.75rem',          // 28px
        'h4': '2rem',             // 32px
        'h3': '2.25rem',          // 36px
        'h2': '2.5rem',           // 40px
        'h1': '3rem',             // 48px
        // أحجام النصوص القرآنية
        'quran-sm': '1.125rem',   // 18px
        'quran-md': '1.5rem',     // 24px
        'quran-lg': '2rem',       // 32px
        'quran-xl': '2.5rem',     // 40px
      },
      spacing: {
        '18': '4.5rem',
        '88': '22rem',
        '128': '32rem',
      },
      borderRadius: {
        'islamic': '0.75rem',
        'islamic-lg': '1rem',
      },
      boxShadow: {
        'islamic': '0 4px 16px rgba(27, 54, 93, 0.08)',
        'islamic-lg': '0 8px 24px rgba(27, 54, 93, 0.12)',
        'islamic-xl': '0 12px 32px rgba(27, 54, 93, 0.16)',
      },
      animation: {
        'fade-in': 'fadeIn 0.3s ease-in-out',
        'slide-up': 'slideUp 0.3s ease-out',
        'slide-down': 'slideDown 0.3s ease-out',
        'scale-in': 'scaleIn 0.2s ease-out',
      },
      keyframes: {
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        slideDown: {
          '0%': { transform: 'translateY(-10px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        },
        scaleIn: {
          '0%': { transform: 'scale(0.95)', opacity: '0' },
          '100%': { transform: 'scale(1)', opacity: '1' },
        },
      },
    },
  },
  plugins: [],
}

export default config
