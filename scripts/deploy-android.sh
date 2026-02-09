#!/bin/bash

# Android Deployment Script for Sanad Islamic App
# This script automates the Android build and deployment process

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MOBILE_DIR="$PROJECT_DIR/frontend/mobile"
TRACK="${1:-internal}"  # Default to internal track

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Sanad Android Deployment Script${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Validate track parameter
if [[ ! "$TRACK" =~ ^(internal|beta|production)$ ]]; then
    echo -e "${RED}Error: Invalid track '$TRACK'${NC}"
    echo "Usage: $0 [internal|beta|production]"
    exit 1
fi

echo -e "${YELLOW}Deployment Track: $TRACK${NC}"
echo ""

# Check if Flutter is installed
if ! command -v flutter &> /dev/null; then
    echo -e "${RED}Error: Flutter is not installed${NC}"
    exit 1
fi

# Check if Fastlane is installed
if ! command -v fastlane &> /dev/null; then
    echo -e "${RED}Error: Fastlane is not installed${NC}"
    echo "Install with: gem install fastlane"
    exit 1
fi

# Navigate to mobile directory
cd "$MOBILE_DIR"

echo -e "${YELLOW}Step 1: Cleaning previous builds...${NC}"
flutter clean
echo -e "${GREEN}✓ Clean complete${NC}"
echo ""

echo -e "${YELLOW}Step 2: Getting dependencies...${NC}"
flutter pub get
echo -e "${GREEN}✓ Dependencies installed${NC}"
echo ""

echo -e "${YELLOW}Step 3: Running tests...${NC}"
flutter test
echo -e "${GREEN}✓ Tests passed${NC}"
echo ""

echo -e "${YELLOW}Step 4: Analyzing code...${NC}"
flutter analyze
echo -e "${GREEN}✓ Code analysis complete${NC}"
echo ""

# Build based on track
if [ "$TRACK" == "internal" ] || [ "$TRACK" == "beta" ]; then
    echo -e "${YELLOW}Step 5: Building APK...${NC}"
    flutter build apk --release
    echo -e "${GREEN}✓ APK build complete${NC}"
    echo ""
else
    echo -e "${YELLOW}Step 5: Building App Bundle...${NC}"
    flutter build appbundle --release
    echo -e "${GREEN}✓ App Bundle build complete${NC}"
    echo ""
fi

echo -e "${YELLOW}Step 6: Deploying to Google Play ($TRACK)...${NC}"
cd android
fastlane android $TRACK
echo -e "${GREEN}✓ Deployment complete${NC}"
echo ""

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Deployment Successful!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "Track: $TRACK"
echo "Build location: $MOBILE_DIR/build/app/outputs/"
echo ""
echo "Next steps:"
echo "1. Check Google Play Console for the new build"
echo "2. Monitor crash reports in Firebase Crashlytics"
echo "3. Review user feedback and ratings"
echo ""
