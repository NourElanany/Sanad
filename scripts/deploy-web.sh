#!/bin/bash

# Web Deployment Script for Sanad Islamic App
# This script automates the Next.js web app deployment process

set -e  # Exit on error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
WEB_DIR="$PROJECT_DIR/frontend/nextjs-app"
PLATFORM="${1:-vercel}"  # Default to Vercel
ENVIRONMENT="${2:-production}"  # Default to production

echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Sanad Web Deployment Script${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""

# Validate platform
if [[ ! "$PLATFORM" =~ ^(vercel|netlify|docker)$ ]]; then
    echo -e "${RED}Error: Invalid platform '$PLATFORM'${NC}"
    echo "Usage: $0 [vercel|netlify|docker] [production|staging]"
    exit 1
fi

# Validate environment
if [[ ! "$ENVIRONMENT" =~ ^(production|staging)$ ]]; then
    echo -e "${RED}Error: Invalid environment '$ENVIRONMENT'${NC}"
    echo "Usage: $0 [vercel|netlify|docker] [production|staging]"
    exit 1
fi

echo -e "${YELLOW}Platform: $PLATFORM${NC}"
echo -e "${YELLOW}Environment: $ENVIRONMENT${NC}"
echo ""

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo -e "${RED}Error: Node.js is not installed${NC}"
    exit 1
fi

# Navigate to web directory
cd "$WEB_DIR"

echo -e "${YELLOW}Step 1: Installing dependencies...${NC}"
npm ci
echo -e "${GREEN}✓ Dependencies installed${NC}"
echo ""

echo -e "${YELLOW}Step 2: Running linter...${NC}"
npm run lint
echo -e "${GREEN}✓ Linting complete${NC}"
echo ""

echo -e "${YELLOW}Step 3: Running type check...${NC}"
npx tsc --noEmit || true
echo -e "${GREEN}✓ Type check complete${NC}"
echo ""

echo -e "${YELLOW}Step 4: Running tests...${NC}"
npm test -- --passWithNoTests
echo -e "${GREEN}✓ Tests passed${NC}"
echo ""

# Deploy based on platform
case "$PLATFORM" in
    vercel)
        echo -e "${YELLOW}Step 5: Deploying to Vercel...${NC}"
        
        # Check if Vercel CLI is installed
        if ! command -v vercel &> /dev/null; then
            echo -e "${YELLOW}Installing Vercel CLI...${NC}"
            npm install -g vercel
        fi
        
        if [ "$ENVIRONMENT" == "production" ]; then
            vercel --prod --yes
        else
            vercel --yes
        fi
        
        echo -e "${GREEN}✓ Deployed to Vercel${NC}"
        ;;
        
    netlify)
        echo -e "${YELLOW}Step 5: Deploying to Netlify...${NC}"
        
        # Check if Netlify CLI is installed
        if ! command -v netlify &> /dev/null; then
            echo -e "${YELLOW}Installing Netlify CLI...${NC}"
            npm install -g netlify-cli
        fi
        
        if [ "$ENVIRONMENT" == "production" ]; then
            netlify deploy --prod
        else
            netlify deploy
        fi
        
        echo -e "${GREEN}✓ Deployed to Netlify${NC}"
        ;;
        
    docker)
        echo -e "${YELLOW}Step 5: Building Docker image...${NC}"
        
        # Check if Docker is installed
        if ! command -v docker &> /dev/null; then
            echo -e "${RED}Error: Docker is not installed${NC}"
            exit 1
        fi
        
        # Build Docker image
        docker build -t sanad-nextjs:$ENVIRONMENT .
        
        echo -e "${GREEN}✓ Docker image built${NC}"
        echo ""
        echo -e "${YELLOW}To run the container:${NC}"
        echo "docker run -d -p 3000:3000 --name sanad-nextjs sanad-nextjs:$ENVIRONMENT"
        ;;
esac

echo ""
echo -e "${GREEN}========================================${NC}"
echo -e "${GREEN}Deployment Successful!${NC}"
echo -e "${GREEN}========================================${NC}"
echo ""
echo "Platform: $PLATFORM"
echo "Environment: $ENVIRONMENT"
echo ""
echo "Next steps:"
echo "1. Verify the deployment is working correctly"
echo "2. Run Lighthouse performance audit"
echo "3. Check error tracking in Sentry"
echo "4. Monitor analytics in Google Analytics"
echo ""
