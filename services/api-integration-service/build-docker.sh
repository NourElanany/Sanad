#!/bin/bash
# Build script for API Integration Service Docker image

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
IMAGE_NAME="sanad/api-integration-service"
VERSION="latest"
DOCKERFILE="services/api-integration-service/Dockerfile"
BUILD_CONTEXT="."

# Parse command line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -v|--version)
            VERSION="$2"
            shift 2
            ;;
        -n|--name)
            IMAGE_NAME="$2"
            shift 2
            ;;
        --no-cache)
            NO_CACHE="--no-cache"
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -v, --version VERSION    Set image version tag (default: latest)"
            echo "  -n, --name NAME          Set image name (default: sanad/api-integration-service)"
            echo "  --no-cache               Build without using cache"
            echo "  -h, --help               Show this help message"
            echo ""
            echo "Examples:"
            echo "  $0                       # Build with default settings"
            echo "  $0 -v v1.0.0             # Build with version tag"
            echo "  $0 --no-cache            # Build without cache"
            exit 0
            ;;
        *)
            echo -e "${RED}Error: Unknown option $1${NC}"
            exit 1
            ;;
    esac
done

# Check if we're in the project root
if [ ! -f "Cargo.toml" ]; then
    echo -e "${RED}Error: Must be run from project root directory${NC}"
    exit 1
fi

# Check if Dockerfile exists
if [ ! -f "$DOCKERFILE" ]; then
    echo -e "${RED}Error: Dockerfile not found at $DOCKERFILE${NC}"
    exit 1
fi

echo -e "${GREEN}Building Docker image...${NC}"
echo -e "Image name: ${YELLOW}${IMAGE_NAME}:${VERSION}${NC}"
echo -e "Dockerfile: ${YELLOW}${DOCKERFILE}${NC}"
echo ""

# Build the image
docker build \
    $NO_CACHE \
    -t "${IMAGE_NAME}:${VERSION}" \
    -f "$DOCKERFILE" \
    "$BUILD_CONTEXT"

if [ $? -eq 0 ]; then
    echo ""
    echo -e "${GREEN}✓ Build successful!${NC}"
    echo ""
    echo "Image: ${IMAGE_NAME}:${VERSION}"
    
    # Show image size
    IMAGE_SIZE=$(docker images "${IMAGE_NAME}:${VERSION}" --format "{{.Size}}")
    echo "Size: ${IMAGE_SIZE}"
    
    echo ""
    echo "To run the container:"
    echo "  docker run -d -p 8080:8080 ${IMAGE_NAME}:${VERSION}"
    echo ""
    echo "To push to registry:"
    echo "  docker push ${IMAGE_NAME}:${VERSION}"
else
    echo -e "${RED}✗ Build failed${NC}"
    exit 1
fi
