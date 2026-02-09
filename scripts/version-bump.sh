#!/bin/bash

# Version Bump Script for Sanad Islamic App
# Updates version numbers across all platforms

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Get version type
VERSION_TYPE="${1:-patch}"

if [[ ! "$VERSION_TYPE" =~ ^(major|minor|patch)$ ]]; then
    echo -e "${RED}Error: Invalid version type '$VERSION_TYPE'${NC}"
    echo "Usage: $0 [major|minor|patch]"
    exit 1
fi

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Version Bump Script${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo -e "${YELLOW}Version Type: $VERSION_TYPE${NC}"
echo ""

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# Function to bump version
bump_version() {
    local current=$1
    local type=$2
    
    IFS='.' read -ra PARTS <<< "$current"
    local major=${PARTS[0]}
    local minor=${PARTS[1]}
    local patch=${PARTS[2]}
    
    case $type in
        major)
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        minor)
            minor=$((minor + 1))
            patch=0
            ;;
        patch)
            patch=$((patch + 1))
            ;;
    esac
    
    echo "$major.$minor.$patch"
}

# Get current version from Flutter pubspec.yaml
CURRENT_VERSION=$(grep "^version:" "$PROJECT_DIR/frontend/mobile/pubspec.yaml" | sed 's/version: //' | sed 's/+.*//')
echo -e "${YELLOW}Current version: $CURRENT_VERSION${NC}"

# Calculate new version
NEW_VERSION=$(bump_version "$CURRENT_VERSION" "$VERSION_TYPE")
echo -e "${GREEN}New version: $NEW_VERSION${NC}"
echo ""

# Confirm with user
read -p "Continue with version bump? (y/n) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborted."
    exit 1
fi

echo -e "${YELLOW}Updating version numbers...${NC}"
echo ""

# Update Flutter pubspec.yaml
echo "Updating Flutter pubspec.yaml..."
BUILD_NUMBER=$(date +%s)
sed -i.bak "s/^version: .*/version: $NEW_VERSION+$BUILD_NUMBER/" "$PROJECT_DIR/frontend/mobile/pubspec.yaml"
rm "$PROJECT_DIR/frontend/mobile/pubspec.yaml.bak"
echo -e "${GREEN}✓ Flutter pubspec.yaml updated${NC}"

# Update Next.js package.json
echo "Updating Next.js package.json..."
cd "$PROJECT_DIR/frontend/nextjs-app"
npm version "$NEW_VERSION" --no-git-tag-version
echo -e "${GREEN}✓ Next.js package.json updated${NC}"

# Update Android build.gradle
echo "Updating Android build.gradle..."
ANDROID_VERSION_CODE=$BUILD_NUMBER
sed -i.bak "s/versionCode .*/versionCode $ANDROID_VERSION_CODE/" "$PROJECT_DIR/frontend/mobile/android/app/build.gradle"
sed -i.bak "s/versionName .*/versionName \"$NEW_VERSION\"/" "$PROJECT_DIR/frontend/mobile/android/app/build.gradle"
rm "$PROJECT_DIR/frontend/mobile/android/app/build.gradle.bak"
echo -e "${GREEN}✓ Android build.gradle updated${NC}"

# Update iOS Info.plist
echo "Updating iOS Info.plist..."
/usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $NEW_VERSION" "$PROJECT_DIR/frontend/mobile/ios/Runner/Info.plist" 2>/dev/null || true
/usr/libexec/PlistBuddy -c "Set :CFBundleVersion $BUILD_NUMBER" "$PROJECT_DIR/frontend/mobile/ios/Runner/Info.plist" 2>/dev/null || true
echo -e "${GREEN}✓ iOS Info.plist updated${NC}"

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Version Bump Complete!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "Version: $CURRENT_VERSION → $NEW_VERSION"
echo "Build Number: $BUILD_NUMBER"
echo ""
echo "Next steps:"
echo "1. Review the changes"
echo "2. Update CHANGELOG.md"
echo "3. Commit the changes: git add . && git commit -m \"chore: bump version to $NEW_VERSION\""
echo "4. Create a git tag: git tag -a v$NEW_VERSION -m \"Release version $NEW_VERSION\""
echo "5. Push changes: git push && git push --tags"
echo ""
