#!/bin/bash

# iOS Deployment Script for Sanad Islamic App
# This script automates the iOS build and deployment process

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
MOBILE_DIR="$PROJECT_DIR/frontend/mobile"
DEPLOYMENT_TYPE="${1:-beta}"  # Default to beta (TestFlight)

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Sanad iOS Deployment Script${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Validate deployment type
if [[ ! "$DEPLOYMENT_TYPE" =~ ^(beta|release)$ ]]; then
    echo -e "${RED}Error: Invalid deployment type '$DEPLOYMENT_TYPE'${NC}"
    echo "Usage: $0 [beta|release]"
    exit 1
fi

echo -e "${YELLOW}Deployment Type: $DEPLOYMENT_TYPE${NC}"
echo ""

# Check if running on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo -e "${RED}Error: iOS deployment must be run on macOS${NC}"
    exit 1
fi

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

# Check if CocoaPods is installed
if ! command -v pod &> /dev/null; then
    echo -e "${RED}Error: CocoaPods is not installed${NC}"
    echo "Install with: sudo gem install cocoapods"
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

echo -e "${YELLOW}Step 3: Installing CocoaPods...${NC}"
cd ios
pod install
cd ..
echo -e "${GREEN}✓ CocoaPods installed${NC}"
echo ""

echo -e "${YELLOW}Step 4: Running tests...${NC}"
flutter test
echo -e "${GREEN}✓ Tests passed${NC}"
echo ""

echo -e "${YELLOW}Step 5: Analyzing code...${NC}"
flutter analyze
echo -e "${GREEN}✓ Code analysis complete${NC}"
echo ""

echo -e "${YELLOW}Step 6: Building iOS app...${NC}"
flutter build ios --release --no-codesign
echo -e "${GREEN}✓ iOS build complete${NC}"
echo ""

echo -e "${YELLOW}Step 7: Deploying to $DEPLOYMENT_TYPE...${NC}"
cd ios
fastlane ios $DEPLOYMENT_TYPE
echo -e "${GREEN}✓ Deployment complete${NC}"
echo ""

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Deployment Successful!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "Deployment Type: $DEPLOYMENT_TYPE"
echo ""
if [ "$DEPLOYMENT_TYPE" == "beta" ]; then
    echo "Next steps:"
    echo "1. Check TestFlight for the new build"
    echo "2. Add release notes in App Store Connect"
    echo "3. Invite testers to test the build"
else
    echo "Next steps:"
    echo "1. Check App Store Connect for the new build"
    echo "2. Submit for review"
    echo "3. Monitor review status"
fi
echo ""
