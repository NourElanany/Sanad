#!/bin/bash

# Deployment Setup Script for Sanad Islamic App
# This script helps set up the deployment environment

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Sanad Deployment Setup${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to print status
print_status() {
    if [ $1 -eq 0 ]; then
        echo -e "${GREEN}✓${NC} $2"
    else
        echo -e "${RED}✗${NC} $2"
    fi
}

echo -e "${BLUE}Checking prerequisites...${NC}"
echo ""

# Check Flutter
if command_exists flutter; then
    FLUTTER_VERSION=$(flutter --version | head -n 1)
    print_status 0 "Flutter installed: $FLUTTER_VERSION"
else
    print_status 1 "Flutter not installed"
    echo -e "${YELLOW}  Install from: https://flutter.dev/docs/get-started/install${NC}"
fi

# Check Node.js
if command_exists node; then
    NODE_VERSION=$(node --version)
    print_status 0 "Node.js installed: $NODE_VERSION"
else
    print_status 1 "Node.js not installed"
    echo -e "${YELLOW}  Install from: https://nodejs.org/${NC}"
fi

# Check npm
if command_exists npm; then
    NPM_VERSION=$(npm --version)
    print_status 0 "npm installed: v$NPM_VERSION"
else
    print_status 1 "npm not installed"
fi

# Check Docker
if command_exists docker; then
    DOCKER_VERSION=$(docker --version)
    print_status 0 "Docker installed: $DOCKER_VERSION"
else
    print_status 1 "Docker not installed (optional)"
    echo -e "${YELLOW}  Install from: https://www.docker.com/get-started${NC}"
fi

# Check Fastlane
if command_exists fastlane; then
    FASTLANE_VERSION=$(fastlane --version | head -n 1)
    print_status 0 "Fastlane installed: $FASTLANE_VERSION"
else
    print_status 1 "Fastlane not installed"
    echo -e "${YELLOW}  Install with: gem install fastlane${NC}"
fi

# Check Git
if command_exists git; then
    GIT_VERSION=$(git --version)
    print_status 0 "Git installed: $GIT_VERSION"
else
    print_status 1 "Git not installed"
fi

echo ""
echo -e "${BLUE}Setting up deployment files...${NC}"
echo ""

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Make deployment scripts executable
chmod +x "$PROJECT_DIR/scripts/deploy-android.sh"
chmod +x "$PROJECT_DIR/scripts/deploy-ios.sh"
chmod +x "$PROJECT_DIR/scripts/deploy-web.sh"
print_status 0 "Made deployment scripts executable"

# Create necessary directories
mkdir -p "$PROJECT_DIR/frontend/mobile/android/fastlane"
mkdir -p "$PROJECT_DIR/frontend/mobile/ios/fastlane"
print_status 0 "Created Fastlane directories"

echo ""
echo -e "${BLUE}Next steps:${NC}"
echo ""
echo "1. Configure GitHub Secrets:"
echo "   - Go to your repository settings"
echo "   - Add all required secrets (see DEPLOYMENT_GUIDE.md)"
echo ""
echo "2. Set up Android:"
echo "   - Generate keystore: keytool -genkey -v -keystore keystore.jks ..."
echo "   - Create Google Play Console app"
echo "   - Set up service account"
echo ""
echo "3. Set up iOS:"
echo "   - Create App ID in Apple Developer Portal"
echo "   - Set up provisioning profiles"
echo "   - Create App Store Connect app"
echo ""
echo "4. Set up Web:"
echo "   - Create Vercel/Netlify account"
echo "   - Link your repository"
echo "   - Configure environment variables"
echo ""
echo "5. Set up Monitoring:"
echo "   - Create Firebase project"
echo "   - Set up Google Analytics"
echo "   - Configure Sentry"
echo ""
echo -e "${GREEN}For detailed instructions, see: DEPLOYMENT_GUIDE.md${NC}"
echo ""
