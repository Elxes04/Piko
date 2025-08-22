# Piko Packaging Guide

This guide explains how to build and distribute Piko packages for different Linux distributions and architectures.

## 🚀 Quick Start

### Using Makefile (Recommended)

```bash
# Build packages for all architectures
make package

# Build specific package types
make deb    # Debian packages
make rpm    # RPM packages
make aur    # Arch Linux PKGBUILD
make copr   # Publish to Fedora Copr
```

### Manual Build

```bash
# Install cross-compilation targets
make targets

# Build for specific architecture
cargo build --release --target x86_64-unknown-linux-gnu

# Create packages
./scripts/build-packages.sh
```

## 📦 Supported Package Formats

### Debian/Ubuntu (.deb)
- **Architectures**: x86_64, aarch64, i686, armv7
- **Dependencies**: libc6 (>= 2.31)
- **Installation**: `sudo dpkg -i piko_0.2.1_*.deb`

### RPM (Fedora/RHEL/CentOS)
- **Architectures**: x86_64, aarch64, i686, armv7
- **Dependencies**: libc >= 2.31
- **Installation**: `sudo dnf install piko-0.2.1-1.*.rpm`

### Arch Linux (PKGBUILD)
- **Architectures**: x86_64, aarch64, i686, armv7h
- **Dependencies**: glibc
- **Installation**: `makepkg -si`

## 🏗️ Build Process

### Prerequisites

1. **Rust Toolchain**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Cross-compilation Targets**
   ```bash
   rustup target add x86_64-unknown-linux-gnu
   rustup target add aarch64-unknown-linux-gnu
   rustup target add i686-unknown-linux-gnu
   rustup target add armv7-unknown-linux-gnueabihf
   ```

3. **Package-specific Dependencies**
   ```bash
   # Debian/Ubuntu
   sudo apt-get install dpkg-dev

   # Fedora/RHEL
   sudo dnf install rpm-build copr-cli

   # Arch Linux
   # No additional dependencies required
   ```

### Build Commands

#### Debian Packages
```bash
# Build all architectures
make deb

# Or manually
./scripts/build-packages.sh
```

#### RPM Packages
```bash
# Build RPM
make rpm

# Or manually
mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
cp piko-0.2.1.tar.gz ~/rpmbuild/SOURCES/
cp piko.spec ~/rpmbuild/SPECS/
rpmbuild -ba ~/rpmbuild/SPECS/piko.spec
```

#### Arch Linux PKGBUILD
```bash
# Create PKGBUILD
make aur

# Or manually
cp PKGBUILD dist/
```

## 🌐 Distribution Channels

### GitHub Releases
- **Automatic**: Triggered by git tags
- **Files**: Debian packages for all architectures
- **URL**: https://github.com/Elxes04/piko/releases

### Fedora Copr
- **Repository**: https://copr.fedorainfracloud.org/coprs/elxes04/piko/
- **Installation**:
  ```bash
  sudo dnf copr enable elxes04/piko
  sudo dnf install piko
  ```

### Arch User Repository (AUR)
- **Package**: piko
- **Installation**: `yay -S piko` or `paru -S piko`

### Ubuntu PPA (Future)
- **Status**: Planned
- **Installation**: TBD

## 🔧 Configuration Files

### Package Structure
```
piko/
├── usr/bin/piko                    # Binary
├── usr/share/piko/                 # Configuration files
│   ├── default_config.toml
│   ├── pastel_config.toml
│   └── dark_config.toml
└── usr/share/doc/piko/             # Documentation
    ├── README.md
    ├── COLOR_SCHEMES.md
    └── copyright
```

### Configuration Locations
- **Package installation**: `/usr/share/piko/`
- **User configuration**: `~/.config/piko/`
- **Custom path**: `--config /path/to/config.toml`

## 🚀 CI/CD Pipeline

### GitHub Actions
The project includes automated CI/CD via GitHub Actions:

1. **Build Matrix**: Builds for all supported architectures
2. **Package Creation**: Creates Debian packages automatically
3. **Release Management**: Creates GitHub releases with packages
4. **AUR Updates**: Updates Arch User Repository automatically

### Triggering a Release
```bash
# Create and push a tag
git tag v0.2.1
git push origin v0.2.1

# GitHub Actions will automatically:
# 1. Build for all architectures
# 2. Create packages
# 3. Create GitHub release
# 4. Update AUR package
```

## 📋 Package Metadata

### Debian Control File
```
Package: piko
Version: 0.2.1
Architecture: amd64
Maintainer: Elxes04 <elxes04@example.com>
Depends: libc6 (>= 2.31)
Section: utils
Priority: optional
Homepage: https://github.com/Elxes04/piko
Description: A minimal, customizable system information tool
 Piko is a lightweight and extensible command-line tool that gathers
 and displays system information in a customizable format.
```

### RPM Spec File
```
Name:           piko
Version:        0.2.1
Release:        1%{?dist}
Summary:        A minimal, customizable system information tool
License:        MIT
URL:            https://github.com/Elxes04/piko
BuildRequires:  rust
BuildRequires:  cargo
```

### PKGBUILD
```
pkgname=piko
pkgver=0.2.1
pkgrel=1
pkgdesc="A minimal, customizable system information tool written in Rust"
arch=('x86_64' 'aarch64' 'i686' 'armv7h')
url="https://github.com/Elxes04/piko"
license=('MIT')
depends=('glibc')
makedepends=('rust' 'cargo')
```

## 🛠️ Troubleshooting

### Common Issues

1. **Cross-compilation fails**
   - Ensure all targets are installed: `make targets`
   - Check Rust toolchain: `rustup show`

2. **Package build fails**
   - Verify dependencies are installed
   - Check file permissions
   - Ensure source files are present

3. **GitHub Actions fails**
   - Check workflow syntax
   - Verify secrets are configured
   - Review build logs

### Debug Commands
```bash
# Check installed targets
rustup target list --installed

# Verify package structure
dpkg -c piko_0.2.1_*.deb

# Check RPM package
rpm -qlp piko-0.2.1-1.*.rpm

# Test PKGBUILD
makepkg --nobuild
```

## 📚 Additional Resources

- [Rust Cross-compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)
- [Debian Packaging Guide](https://www.debian.org/doc/manuals/packaging-tutorial/packaging-tutorial.en.html)
- [RPM Packaging Guide](https://rpm-packaging-guide.github.io/)
- [Arch Linux Packaging Standards](https://wiki.archlinux.org/title/PKGBUILD)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
