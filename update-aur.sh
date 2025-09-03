#!/bin/bash

# Script to update the Piko AUR package
# Usage: ./update-aur.sh [new_version]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored messages
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in the correct directory
if [[ ! -f "PKGBUILD" ]]; then
    print_error "PKGBUILD not found. Run this script from the AUR repository directory."
    exit 1
fi

# Get current version from PKGBUILD
CURRENT_VERSION=$(grep '^pkgver=' PKGBUILD | cut -d'=' -f2)
print_status "Current version: $CURRENT_VERSION"

# Check if a new version was provided
if [[ -n "$1" ]]; then
    NEW_VERSION="$1"
    print_status "Updating to version: $NEW_VERSION"
else
    # Get latest version from main repository
    print_status "Checking latest version..."
    NEW_VERSION=$(curl -s https://api.github.com/repos/Elxes04/piko/releases/latest | grep '"tag_name"' | cut -d'"' -f4 | sed 's/v//')
    
    if [[ -z "$NEW_VERSION" ]]; then
        print_error "Unable to get latest version from GitHub"
        exit 1
    fi
    
    print_status "Latest version found: $NEW_VERSION"
fi

# Check if version is different
if [[ "$CURRENT_VERSION" == "$NEW_VERSION" ]]; then
    print_warning "Version is already up to date ($CURRENT_VERSION)"
    read -p "Do you want to proceed anyway? (y/N): " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 0
    fi
fi

# Update PKGBUILD
print_status "Updating PKGBUILD..."
sed -i "s/^pkgver=.*/pkgver=$NEW_VERSION/" PKGBUILD

# Increment pkgrel
CURRENT_PKGREL=$(grep '^pkgrel=' PKGBUILD | cut -d'=' -f2)
NEW_PKGREL=$((CURRENT_PKGREL + 1))
sed -i "s/^pkgrel=.*/pkgrel=$NEW_PKGREL/" PKGBUILD

print_status "pkgrel updated: $CURRENT_PKGREL -> $NEW_PKGREL"

# Update .SRCINFO
print_status "Updating .SRCINFO..."
makepkg --printsrcinfo > .SRCINFO

# Check if package builds
print_status "Build test..."
if makepkg --nobuild --nodeps; then
    print_status "Build test completed successfully!"
else
    print_error "Build test failed!"
    exit 1
fi

# Commit and push changes
print_status "Committing changes..."
git add PKGBUILD .SRCINFO
git commit -m "Update to version $NEW_VERSION"

print_status "Pushing changes..."
git push origin main

print_status "Update completed!"
print_status "Version: $CURRENT_VERSION -> $NEW_VERSION"
print_status "pkgrel: $CURRENT_PKGREL -> $NEW_PKGREL"

echo
print_warning "Remember to also update the main repository with the new version!"
print_warning "Run: git tag v$NEW_VERSION && git push origin v$NEW_VERSION"
