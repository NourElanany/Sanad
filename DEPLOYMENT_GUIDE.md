# Sanad Frontend Deployment Guide

This guide covers the complete deployment process for both the Flutter mobile application and Next.js web application.

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Environment Setup](#environment-setup)
3. [CI/CD Pipeline Configuration](#cicd-pipeline-configuration)
4. [Android Deployment](#android-deployment)
5. [iOS Deployment](#ios-deployment)
6. [Web Deployment](#web-deployment)
7. [Monitoring and Analytics](#monitoring-and-analytics)
8. [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Required Accounts

- **GitHub Account**: For CI/CD pipelines
- **Google Play Console**: For Android app distribution
- **Apple Developer Account**: For iOS app distribution
- **Firebase Account**: For analytics, crashlytics, and app distribution
- **Vercel/Netlify Account**: For web hosting (choose one)
- **Sentry Account**: For error tracking (optional but recommended)

### Required Tools

- **Flutter SDK**: 3.16.0 or higher
- **Node.js**: 20.x or higher
- **Fastlane**: For automated deployments
- **Docker**: For containerized deployments

---

## Environment Setup

### 1. GitHub Secrets Configuration

Add the following secrets to your GitHub repository:

#### Android Secrets
```
ANDROID_KEYSTORE_BASE64          # Base64 encoded keystore file
ANDROID_KEYSTORE_PASSWORD        # Keystore password
ANDROID_KEY_PASSWORD             # Key password
ANDROID_KEY_ALIAS                # Key alias
GOOGLE_PLAY_SERVICE_ACCOUNT_JSON # Service account JSON for Play Store
FIREBASE_ANDROID_APP_ID          # Firebase Android app ID
```

#### iOS Secrets
```
IOS_CERTIFICATES_P12             # Base64 encoded P12 certificate
IOS_CERTIFICATES_PASSWORD        # Certificate password
APPSTORE_ISSUER_ID              # App Store Connect API issuer ID
APPSTORE_KEY_ID                 # App Store Connect API key ID
APPSTORE_PRIVATE_KEY            # App Store Connect API private key
FIREBASE_IOS_APP_ID             # Firebase iOS app ID
APPLE_ID                        # Apple ID email
ITC_TEAM_ID                     # App Store Connect team ID
TEAM_ID                         # Developer Portal team ID
```

#### Web Secrets
```
NEXT_PUBLIC_API_URL             # Backend API URL
VERCEL_TOKEN                    # Vercel deployment token
VERCEL_PRODUCTION_URL           # Production URL
NEXT_PUBLIC_GA_ID               # Google Analytics ID
NEXT_PUBLIC_SENTRY_DSN          # Sentry DSN
```

#### Shared Secrets
```
FIREBASE_SERVICE_ACCOUNT_JSON   # Firebase service account JSON
CODECOV_TOKEN                   # Codecov token for coverage reports
```

### 2. Local Environment Files

#### Next.js (.env.local)
```env
NEXT_PUBLIC_API_URL=http://localhost:8080
NEXT_PUBLIC_APP_ENV=development
NEXT_PUBLIC_GA_ID=G-XXXXXXXXXX
NEXT_PUBLIC_SENTRY_DSN=https://xxx@xxx.ingest.sentry.io/xxx
```

#### Flutter (android/key.properties)
```properties
storePassword=your_keystore_password
keyPassword=your_key_password
keyAlias=your_key_alias
storeFile=keystore.jks
```

---

## CI/CD Pipeline Configuration

### GitHub Actions Workflows

Three main workflows are configured:

1. **Next.js Deployment** (`.github/workflows/nextjs-deploy.yml`)
   - Runs on push to `main` or `develop` branches
   - Tests, builds, and deploys to Vercel
   - Runs Lighthouse performance audits

2. **Flutter Android Deployment** (`.github/workflows/flutter-android-deploy.yml`)
   - Builds APK for develop branch
   - Builds AAB for main branch
   - Deploys to Firebase App Distribution (develop)
   - Deploys to Google Play Store (main)

3. **Flutter iOS Deployment** (`.github/workflows/flutter-ios-deploy.yml`)
   - Builds IPA for main branch
   - Deploys to TestFlight (main)
   - Deploys to Firebase App Distribution (develop)

### Workflow Triggers

- **Pull Requests**: Run tests only
- **Push to develop**: Deploy to staging/testing environments
- **Push to main**: Deploy to production

---

## Android Deployment

### 1. Generate Keystore

```bash
keytool -genkey -v -keystore keystore.jks -keyalg RSA -keysize 2048 -validity 10000 -alias sanad
```

### 2. Configure Signing

Edit `android/app/build.gradle`:

```gradle
android {
    signingConfigs {
        release {
            keyAlias keystoreProperties['keyAlias']
            keyPassword keystoreProperties['keyPassword']
            storeFile keystoreProperties['storeFile'] ? file(keystoreProperties['storeFile']) : null
            storePassword keystoreProperties['storePassword']
        }
    }
    buildTypes {
        release {
            signingConfig signingConfigs.release
        }
    }
}
```

### 3. Google Play Console Setup

1. Create app in Google Play Console
2. Complete store listing:
   - App name: Sanad - Islamic Companion
   - Short description: (see `fastlane/metadata/android/en-US/short_description.txt`)
   - Full description: (see `fastlane/metadata/android/en-US/full_description.txt`)
   - Screenshots: Add at least 2 screenshots per device type
   - Feature graphic: 1024x500 px
   - App icon: 512x512 px

3. Set up internal testing track
4. Create service account and download JSON key
5. Grant access to the service account in Play Console

### 4. Manual Deployment

```bash
cd frontend/mobile
flutter build appbundle --release
fastlane android production
```

### 5. Automated Deployment

Push to `main` branch triggers automatic deployment to Play Store internal testing track.

---

## iOS Deployment

### 1. Apple Developer Setup

1. Create App ID in Apple Developer Portal
   - Bundle ID: `com.sanad.islamicApp`
   - Enable capabilities: Push Notifications, Background Modes

2. Create App Store Connect app
   - App name: Sanad - Islamic Companion
   - Primary language: English
   - Bundle ID: Select the created App ID

### 2. Code Signing

#### Option A: Manual Signing
1. Create provisioning profiles in Apple Developer Portal
2. Download and install certificates
3. Configure Xcode project

#### Option B: Fastlane Match (Recommended)
```bash
cd frontend/mobile/ios
fastlane match appstore
```

### 3. App Store Connect Setup

1. Complete app information:
   - Name: (see `fastlane/metadata/en-US/name.txt`)
   - Subtitle: (see `fastlane/metadata/en-US/subtitle.txt`)
   - Description: (see `fastlane/metadata/en-US/description.txt`)
   - Keywords: (see `fastlane/metadata/en-US/keywords.txt`)
   - Screenshots: Add screenshots for all required device sizes
   - App icon: 1024x1024 px

2. Set up TestFlight
   - Add internal testers
   - Configure external testing (optional)

### 4. Manual Deployment

```bash
cd frontend/mobile
flutter build ios --release
cd ios
fastlane beta  # For TestFlight
fastlane release  # For App Store
```

### 5. Automated Deployment

Push to `main` branch triggers automatic deployment to TestFlight.

---

## Web Deployment

### Vercel Deployment (Recommended)

#### 1. Vercel Setup

1. Install Vercel CLI:
```bash
npm install -g vercel
```

2. Link project:
```bash
cd frontend/nextjs-app
vercel link
```

3. Configure environment variables in Vercel dashboard

#### 2. Manual Deployment

```bash
cd frontend/nextjs-app
vercel --prod
```

#### 3. Automated Deployment

Push to `main` branch triggers automatic deployment to Vercel production.

### Netlify Deployment (Alternative)

#### 1. Netlify Setup

1. Install Netlify CLI:
```bash
npm install -g netlify-cli
```

2. Link project:
```bash
cd frontend/nextjs-app
netlify link
```

3. Configure environment variables in Netlify dashboard

#### 2. Manual Deployment

```bash
cd frontend/nextjs-app
netlify deploy --prod
```

### Docker Deployment (Self-Hosted)

#### 1. Build Docker Image

```bash
cd frontend/nextjs-app
docker build -t sanad-nextjs:latest .
```

#### 2. Run Container

```bash
docker run -d \
  -p 3000:3000 \
  -e NEXT_PUBLIC_API_URL=https://api.sanad.app \
  -e NEXT_PUBLIC_APP_ENV=production \
  --name sanad-nextjs \
  sanad-nextjs:latest
```

#### 3. Docker Compose

```bash
cd frontend/nextjs-app
docker-compose up -d
```

---

## Monitoring and Analytics

### Firebase Analytics

#### Mobile Setup

1. Add Firebase configuration files:
   - Android: `google-services.json`
   - iOS: `GoogleService-Info.plist`

2. Initialize in app:
```dart
await AnalyticsService().initialize();
```

3. Track events:
```dart
AnalyticsService().trackQuranReading(surahNumber, ayahNumber);
```

#### Web Setup

Add Firebase SDK to Next.js app and configure analytics.

### Google Analytics 4

#### Setup

1. Create GA4 property
2. Add tracking ID to environment variables
3. Add GA script to `_app.tsx`:

```typescript
import { useEffect } from 'react';
import { useRouter } from 'next/router';
import * as ga from '../lib/analytics/google-analytics';

function MyApp({ Component, pageProps }) {
  const router = useRouter();

  useEffect(() => {
    const handleRouteChange = (url: string) => {
      ga.pageview(url);
    };
    router.events.on('routeChangeComplete', handleRouteChange);
    return () => {
      router.events.off('routeChangeComplete', handleRouteChange);
    };
  }, [router.events]);

  return <Component {...pageProps} />;
}
```

### Sentry Error Tracking

#### Setup

1. Create Sentry project
2. Add DSN to environment variables
3. Initialize in app:

```typescript
import { initSentry } from '../lib/analytics/sentry';

initSentry();
```

4. Track errors:
```typescript
import { captureException } from '../lib/analytics/sentry';

try {
  // code
} catch (error) {
  captureException(error, { context: 'additional info' });
}
```

### Firebase Crashlytics

#### Mobile Setup

1. Enable Crashlytics in Firebase Console
2. Initialize in app:
```dart
FlutterError.onError = FirebaseCrashlytics.instance.recordFlutterFatalError;
```

3. Record errors:
```dart
await AnalyticsService().recordError(exception, stackTrace);
```

---

## Troubleshooting

### Common Issues

#### Android Build Failures

**Issue**: Keystore not found
```
Solution: Ensure keystore.jks is in android/app/ directory
```

**Issue**: Gradle build fails
```
Solution: Clean build and rebuild
flutter clean
flutter pub get
flutter build appbundle --release
```

#### iOS Build Failures

**Issue**: Code signing error
```
Solution: Verify provisioning profiles and certificates
fastlane match appstore --readonly
```

**Issue**: CocoaPods issues
```
Solution: Update and reinstall pods
cd ios
pod deintegrate
pod install
```

#### Web Deployment Issues

**Issue**: Build fails on Vercel
```
Solution: Check Node.js version and dependencies
Ensure package-lock.json is committed
```

**Issue**: Environment variables not working
```
Solution: Verify variables are prefixed with NEXT_PUBLIC_
Redeploy after adding variables
```

### Getting Help

- **GitHub Issues**: Report bugs and request features
- **Documentation**: Check official Flutter and Next.js docs
- **Community**: Join our Discord/Slack channel

---

## Deployment Checklist

### Pre-Deployment

- [ ] All tests passing
- [ ] Code reviewed and approved
- [ ] Version numbers updated
- [ ] Release notes prepared
- [ ] Environment variables configured
- [ ] Secrets added to CI/CD

### Android

- [ ] Keystore generated and secured
- [ ] Play Store listing complete
- [ ] Screenshots uploaded
- [ ] Service account configured
- [ ] Internal testing track set up

### iOS

- [ ] Certificates and profiles configured
- [ ] App Store Connect listing complete
- [ ] Screenshots uploaded for all devices
- [ ] TestFlight configured
- [ ] App Store Connect API keys set up

### Web

- [ ] Domain configured
- [ ] SSL certificate active
- [ ] CDN configured
- [ ] Environment variables set
- [ ] Analytics tracking verified

### Post-Deployment

- [ ] Verify app functionality
- [ ] Check analytics data
- [ ] Monitor error reports
- [ ] Review user feedback
- [ ] Update documentation

---

## Version Management

### Semantic Versioning

Follow semantic versioning: `MAJOR.MINOR.PATCH`

- **MAJOR**: Breaking changes
- **MINOR**: New features (backward compatible)
- **PATCH**: Bug fixes

### Version Update Process

1. Update version in:
   - `pubspec.yaml` (Flutter)
   - `package.json` (Next.js)
   - `android/app/build.gradle`
   - `ios/Runner.xcodeproj/project.pbxproj`

2. Create git tag:
```bash
git tag -a v1.0.0 -m "Release version 1.0.0"
git push origin v1.0.0
```

3. Update release notes

---

## Security Best Practices

1. **Never commit secrets**: Use environment variables and CI/CD secrets
2. **Rotate keys regularly**: Update API keys and certificates periodically
3. **Use HTTPS only**: Ensure all API calls use HTTPS
4. **Implement certificate pinning**: For mobile apps
5. **Enable ProGuard/R8**: For Android release builds
6. **Enable bitcode**: For iOS release builds (if applicable)
7. **Regular security audits**: Review dependencies and code

---

## Support and Maintenance

### Monitoring

- Check Firebase Crashlytics daily
- Review Sentry error reports
- Monitor Google Analytics metrics
- Track app store ratings and reviews

### Updates

- Security patches: As needed
- Bug fixes: Weekly or as needed
- Feature updates: Monthly
- Major releases: Quarterly

### Rollback Procedure

If issues are detected after deployment:

1. **Android**: Halt rollout in Play Console
2. **iOS**: Remove build from TestFlight/App Store
3. **Web**: Revert to previous deployment in Vercel/Netlify
4. Fix issues and redeploy

---

## Additional Resources

- [Flutter Deployment Documentation](https://docs.flutter.dev/deployment)
- [Next.js Deployment Documentation](https://nextjs.org/docs/deployment)
- [Fastlane Documentation](https://docs.fastlane.tools/)
- [Firebase Documentation](https://firebase.google.com/docs)
- [Google Play Console Help](https://support.google.com/googleplay/android-developer)
- [App Store Connect Help](https://developer.apple.com/app-store-connect/)

---

**Last Updated**: 2024
**Maintained By**: Sanad Development Team
