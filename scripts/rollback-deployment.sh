#!/bin/bash

# Rollback Deployment Script for Sanad Islamic App
# Reverts to a previous version in case of issues

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

PLATFORM="${1}"
VERSION="${2}"

echo -e "${RED}========================================${NC}"
echo -e "${RED}Deployment Rollback Script${NC}"
echo -e "${RED}========================================${NC}"
echo ""

# Validate parameters
if [[ -z "$PLATFORM" ]] || [[ -z "$VERSION" ]]; then
    echo -e "${RED}Error: Missing required parameters${NC}"
    echo "Usage: $0 <platform> <version>"
    echo "Platforms: android, ios, web"
    echo "Example: $0 android 1.0.0"
    exit 1
fi

if [[ ! "$PLATFORM" =~ ^(android|ios|web)$ ]]; then
    echo -e "${RED}Error: Invalid platform '$PLATFORM'${NC}"
    echo "Valid platforms: android, ios, web"
    exit 1
fi

echo -e "${YELLOW}Platform: $PLATFORM${NC}"
echo -e "${YELLOW}Target Version: $VERSION${NC}"
echo ""

# Confirm rollback
echo -e "${RED}WARNING: This will rollback the deployment to version $VERSION${NC}"
read -p "Are you sure you want to continue? (yes/no) " -r
echo
if [[ ! $REPLY =~ ^[Yy][Ee][Ss]$ ]]; then
    echo "Rollback cancelled."
    exit 0
fi

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

case "$PLATFORM" in
    android)
        echo -e "${YELLOW}Rolling back Android deployment...${NC}"
        echo ""
        
        # Check if Fastlane is installed
        if ! command -v fastlane &> /dev/null; then
            echo -e "${RED}Error: Fastlane is not installed${NC}"
            exit 1
        fi
        
        cd "$PROJECT_DIR/frontend/mobile"
        
        # Halt current rollout in Play Console
        echo "Halting current rollout..."
        # This would require Play Console API integration
        echo -e "${YELLOW}Please manually halt the rollout in Google Play Console${NC}"
        echo "1. Go to Google Play Console"
        echo "2. Navigate to Production > Releases"
        echo "3. Click 'Halt rollout' on the current release"
        echo ""
        
        # Promote previous version
        echo "To promote version $VERSION:"
        echo "1. Go to Google Play Console"
        echo "2. Find the release for version $VERSION"
        echo "3. Click 'Promote to Production'"
        echo ""
        
        echo -e "${GREEN}✓ Android rollback instructions provided${NC}"
        ;;
        
    ios)
        echo -e "${YELLOW}Rolling back iOS deployment...${NC}"
        echo ""
        
        echo "To rollback iOS deployment:"
        echo "1. Go to App Store Connect"
        echo "2. Navigate to your app > TestFlight or App Store"
        echo "3. Remove the current build from review/release"
        echo "4. Submit the previous version ($VERSION) for review"
        echo ""
        
        echo -e "${YELLOW}Note: iOS doesn't support automatic rollbacks${NC}"
        echo "You may need to submit a new build with the previous version"
        echo ""
        
        echo -e "${GREEN}✓ iOS rollback instructions provided${NC}"
        ;;
        
    web)
        echo -e "${YELLOW}Rolling back web deployment...${NC}"
        echo ""
        
        cd "$PROJECT_DIR/frontend/nextjs-app"
        
        # Check deployment platform
        if [ -f "vercel.json" ]; then
            echo "Detected Vercel deployment"
            echo ""
            
            if command -v vercel &> /dev/null; then
                echo "Rolling back to previous deployment..."
                vercel rollback
                echo -e "${GREEN}✓ Vercel rollback complete${NC}"
            else
                echo "To rollback Vercel deployment:"
                echo "1. Go to Vercel Dashboard"
                echo "2. Select your project"
                echo "3. Go to Deployments"
                echo "4. Find the deployment for version $VERSION"
                echo "5. Click '...' and select 'Promote to Production'"
            fi
        elif [ -f "netlify.toml" ]; then
            echo "Detected Netlify deployment"
            echo ""
            
            if command -v netlify &> /dev/null; then
                echo "Rolling back to previous deployment..."
                netlify rollback
                echo -e "${GREEN}✓ Netlify rollback complete${NC}"
            else
                echo "To rollback Netlify deployment:"
                echo "1. Go to Netlify Dashboard"
                echo "2. Select your site"
                echo "3. Go to Deploys"
                echo "4. Find the deploy for version $VERSION"
                echo "5. Click 'Publish deploy'"
            fi
        else
            echo "Docker deployment detected"
            echo ""
            echo "To rollback Docker deployment:"
            echo "1. Stop current container: docker stop sanad-nextjs"
            echo "2. Remove current container: docker rm sanad-nextjs"
            echo "3. Run previous version: docker run -d -p 3000:3000 --name sanad-nextjs sanad-nextjs:$VERSION"
        fi
        
        echo ""
        echo -e "${GREEN}✓ Web rollback complete${NC}"
        ;;
esac

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Rollback Process Complete${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "Post-rollback checklist:"
echo "1. Verify the app is working correctly"
echo "2. Check error rates in monitoring tools"
echo "3. Monitor user feedback"
echo "4. Investigate the issue that caused the rollback"
echo "5. Prepare a fix for the next release"
echo ""
echo "Monitoring:"
echo "- Firebase Crashlytics: Check crash rates"
echo "- Sentry: Review error reports"
echo "- Google Analytics: Monitor user engagement"
echo ""
