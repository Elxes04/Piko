#!/bin/bash

# Piko Multi-Architecture Package Builder
# This script builds packages for different architectures and distributions

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
BUILD_DIR="target/package"
DIST_DIR="dist"

# Supported architectures
ARCHITECTURES=("x86_64" "aarch64" "i686" "armv7")

# Supported targets for cross-compilation
TARGETS=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "i686-unknown-linux-gnu"
    "armv7-unknown-linux-gnueabihf"
)

echo -e "${BLUE}🚀 Piko Multi-Architecture Package Builder${NC}"
echo -e "${BLUE}==========================================${NC}"

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    if ! command_exists cargo; then
        print_error "Cargo is not installed. Please install Rust first."
        exit 1
    fi
    
    if ! command_exists rustup; then
        print_error "Rustup is not installed. Please install Rust first."
        exit 1
    fi
    
    print_status "Prerequisites check passed!"
}

# Install cross-compilation targets
install_targets() {
    print_status "Installing cross-compilation targets..."
    
    for target in "${TARGETS[@]}"; do
        if ! rustup target list --installed | grep -q "$target"; then
            print_status "Installing target: $target"
            rustup target add "$target"
        else
            print_status "Target $target already installed"
        fi
    done
}

# Build for specific target
build_target() {
    local target=$1
    local arch=$2
    
    print_status "Building for $arch ($target)..."
    
    # Create build directory
    mkdir -p "$BUILD_DIR/$arch"
    
    # Build the project
    cargo build --release --target "$target"
    
    # Copy binary to build directory
    cp "target/$target/release/$PROJECT_NAME" "$BUILD_DIR/$arch/"
    
    # Copy configuration files
    mkdir -p "$BUILD_DIR/$arch/usr/share/$PROJECT_NAME"
    cp config/*.toml "$BUILD_DIR/$arch/usr/share/$PROJECT_NAME/"
    cp config/*.md "$BUILD_DIR/$arch/usr/share/$PROJECT_NAME/"
    
    print_status "Build completed for $arch"
}

# Build all architectures
build_all_architectures() {
    print_status "Building for all architectures..."
    
    # Clean previous builds
    rm -rf "$BUILD_DIR"
    mkdir -p "$BUILD_DIR"
    
    # Build for each architecture
    for i in "${!TARGETS[@]}"; do
        build_target "${TARGETS[$i]}" "${ARCHITECTURES[$i]}"
    done
    
    print_status "All architecture builds completed!"
}

# Create Debian packages
create_deb_packages() {
    print_status "Creating Debian packages..."
    
    mkdir -p "$DIST_DIR"
    
    for arch in "${ARCHITECTURES[@]}"; do
        if [ -f "$BUILD_DIR/$arch/$PROJECT_NAME" ]; then
            print_status "Creating Debian package for $arch..."
            
            # Create package directory structure
            local pkg_dir="$BUILD_DIR/${PROJECT_NAME}_${VERSION}_${arch}"
            mkdir -p "$pkg_dir/DEBIAN"
            mkdir -p "$pkg_dir/usr/bin"
            mkdir -p "$pkg_dir/usr/share/$PROJECT_NAME"
            mkdir -p "$pkg_dir/usr/share/doc/$PROJECT_NAME"
            
            # Copy files
            cp "$BUILD_DIR/$arch/$PROJECT_NAME" "$pkg_dir/usr/bin/"
            cp config/*.toml "$pkg_dir/usr/share/$PROJECT_NAME/"
            cp config/*.md "$pkg_dir/usr/share/doc/$PROJECT_NAME/"
            cp README.md "$pkg_dir/usr/share/doc/$PROJECT_NAME/"
            cp LICENSE "$pkg_dir/usr/share/doc/$PROJECT_NAME/copyright"
            
            # Create control file
            cat > "$pkg_dir/DEBIAN/control" << EOF
Package: $PROJECT_NAME
Version: $VERSION
Architecture: $arch
Maintainer: Elxes04 <elxes04@example.com>
Depends: libc6 (>= 2.31)
Section: utils
Priority: optional
Homepage: https://github.com/Elxes04/$PROJECT_NAME
Description: A minimal, customizable system information tool
 Piko is a lightweight and extensible command-line tool that gathers
 and displays system information in a customizable format.
 Inspired by Neofetch, it offers a clean and flexible way to view
 details about your system — from OS to CPU, memory, and more.
EOF
            
            # Build the package
            dpkg-deb --build "$pkg_dir" "$DIST_DIR/"
            
            print_status "Debian package created: ${PROJECT_NAME}_${VERSION}_${arch}.deb"
        fi
    done
}

# Create RPM packages
create_rpm_packages() {
    print_status "Creating RPM packages..."
    
    # This would require rpmbuild and spec files
    # For now, we'll create a basic structure
    print_warning "RPM package creation requires additional setup with rpmbuild"
    print_status "Creating RPM spec files..."
    
    for arch in "${ARCHITECTURES[@]}"; do
        if [ -f "$BUILD_DIR/$arch/$PROJECT_NAME" ]; then
            local rpm_arch
            case $arch in
                "x86_64") rpm_arch="x86_64" ;;
                "aarch64") rpm_arch="aarch64" ;;
                "i686") rpm_arch="i686" ;;
                "armv7") rpm_arch="armv7hl" ;;
                *) rpm_arch="$arch" ;;
            esac
            
            # Create spec file
            cat > "$BUILD_DIR/${PROJECT_NAME}.spec" << EOF
Name:           $PROJECT_NAME
Version:        $VERSION
Release:        1%{?dist}
Summary:        A minimal, customizable system information tool

License:        MIT
URL:            https://github.com/Elxes04/$PROJECT_NAME
Source0:        %{url}/archive/v%{version}/%{name}-%{version}.tar.gz

BuildArch:      $rpm_arch
BuildRequires:  rust
BuildRequires:  cargo

%description
Piko is a lightweight and extensible command-line tool that gathers
and displays system information in a customizable format.
Inspired by Neofetch, it offers a clean and flexible way to view
details about your system — from OS to CPU, memory, and more.

%files
%license LICENSE
%doc README.md config/*.md
%{_bindir}/%{name}
%{_datadir}/%{name}/*.toml

%changelog
* $(date '+%a %b %d %Y') Elxes04 <elxes04@example.com> - $VERSION-1
- Initial package release
EOF
            
            print_status "RPM spec file created for $arch"
        fi
    done
}

# Create Arch Linux PKGBUILD
create_arch_pkgbuild() {
    print_status "Creating Arch Linux PKGBUILD..."
    
    cat > "$DIST_DIR/PKGBUILD" << EOF
# Maintainer: Elxes04 <elxes04@example.com>
pkgname=$PROJECT_NAME
pkgver=$VERSION
pkgrel=1
pkgdesc="A minimal, customizable system information tool written in Rust"
arch=('x86_64' 'aarch64' 'i686' 'armv7h')
url="https://github.com/Elxes04/$PROJECT_NAME"
license=('MIT')
depends=('glibc')
makedepends=('rust' 'cargo')
source=("\$pkgname-\$pkgver.tar.gz::https://github.com/Elxes04/$PROJECT_NAME/archive/v\$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "\$pkgname-\$pkgver"
    cargo build --release
}

package() {
    cd "\$pkgname-\$pkgver"
    
    install -Dm755 "target/release/\$pkgname" "\$pkgdir/usr/bin/\$pkgname"
    install -Dm644 LICENSE "\$pkgdir/usr/share/licenses/\$pkgname/LICENSE"
    install -Dm644 README.md "\$pkgdir/usr/share/doc/\$pkgname/README.md"
    
    # Install configuration files
    install -dm755 "\$pkgdir/usr/share/\$pkgname"
    install -m644 config/*.toml "\$pkgdir/usr/share/\$pkgname/"
    install -m644 config/*.md "\$pkgdir/usr/share/\$pkgname/"
}
EOF
    
    print_status "Arch Linux PKGBUILD created"
}

# Main execution
main() {
    check_prerequisites
    install_targets
    build_all_architectures
    create_deb_packages
    create_rpm_packages
    create_arch_pkgbuild
    
    print_status "Package building completed!"
    print_status "Check the '$DIST_DIR' directory for generated packages"
    
    echo -e "${GREEN}✅ Build completed successfully!${NC}"
}

# Run main function
main "$@"
