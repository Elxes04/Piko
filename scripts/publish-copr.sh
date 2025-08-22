#!/bin/bash

# Piko Copr Publisher
# This script publishes Piko to Fedora Copr

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
PROJECT_NAME="piko"
VERSION="0.2.1"
COPR_USERNAME="elxes04"
COPR_PROJECT="piko"

print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    if ! command -v copr-cli >/dev/null 2>&1; then
        print_error "copr-cli is not installed. Please install it first:"
        print_error "sudo dnf install copr-cli"
        exit 1
    fi
    
    if ! command -v rpmbuild >/dev/null 2>&1; then
        print_error "rpmbuild is not installed. Please install it first:"
        print_error "sudo dnf install rpm-build"
        exit 1
    fi
    
    print_status "Prerequisites check passed!"
}

# Create source tarball
create_source_tarball() {
    print_status "Creating source tarball..."
    
    # Create temporary directory
    local temp_dir=$(mktemp -d)
    local tarball_name="${PROJECT_NAME}-${VERSION}.tar.gz"
    
    # Copy source files
    cp -r . "$temp_dir/${PROJECT_NAME}-${VERSION}"
    
    # Remove unnecessary files
    cd "$temp_dir/${PROJECT_NAME}-${VERSION}"
    rm -rf target/ .git/ dist/ scripts/
    
    # Create tarball
    cd "$temp_dir"
    tar -czf "$tarball_name" "${PROJECT_NAME}-${VERSION}"
    
    # Move to current directory
    mv "$tarball_name" ./
    
    # Clean up
    rm -rf "$temp_dir"
    
    print_status "Source tarball created: $tarball_name"
}

# Build RPM package
build_rpm() {
    print_status "Building RPM package..."
    
    # Create RPM build directory
    mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
    
    # Copy source tarball
    cp "${PROJECT_NAME}-${VERSION}.tar.gz" ~/rpmbuild/SOURCES/
    
    # Copy spec file
    cp piko.spec ~/rpmbuild/SPECS/
    
    # Build RPM
    rpmbuild -ba ~/rpmbuild/SPECS/piko.spec
    
    print_status "RPM package built successfully!"
}

# Publish to Copr
publish_to_copr() {
    print_status "Publishing to Copr..."
    
    # Create Copr project if it doesn't exist
    if ! copr-cli list | grep -q "$COPR_PROJECT"; then
        print_status "Creating Copr project: $COPR_PROJECT"
        copr-cli create "$COPR_PROJECT" --description "Piko - A minimal, customizable system information tool"
    fi
    
    # Build in Copr
    copr-cli build "$COPR_PROJECT" ~/rpmbuild/SRPMS/piko-${VERSION}-1.*.src.rpm
    
    print_status "Package submitted to Copr!"
    print_status "You can monitor the build at: https://copr.fedorainfracloud.org/coprs/$COPR_USERNAME/$COPR_PROJECT/"
}

# Main execution
main() {
    echo -e "${BLUE}🚀 Piko Copr Publisher${NC}"
    echo -e "${BLUE}=====================${NC}"
    
    check_prerequisites
    create_source_tarball
    build_rpm
    publish_to_copr
    
    print_status "Publication completed!"
    print_status "Users can install Piko with:"
    print_status "sudo dnf copr enable $COPR_USERNAME/$COPR_PROJECT"
    print_status "sudo dnf install piko"
    
    echo -e "${GREEN}✅ Publication completed successfully!${NC}"
}

# Run main function
main "$@"
