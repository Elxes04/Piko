# Piko Makefile
# Simplifies common development and packaging tasks

.PHONY: help build test clean package deb rpm aur copr release install

# Configuration
PROJECT_NAME = piko
VERSION = 0.2.1
TARGETS = x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu i686-unknown-linux-gnu armv7-unknown-linux-gnueabihf

# Default target
help:
	@echo "Piko Development and Packaging Commands"
	@echo "========================================"
	@echo ""
	@echo "Development:"
	@echo "  build     - Build the project in release mode"
	@echo "  test      - Run tests"
	@echo "  clean     - Clean build artifacts"
	@echo "  install   - Install locally"
	@echo ""
	@echo "Packaging:"
	@echo "  package   - Build packages for all architectures"
	@echo "  deb       - Build Debian packages"
	@echo "  rpm       - Build RPM packages"
	@echo "  aur       - Create Arch Linux PKGBUILD"
	@echo "  copr      - Publish to Fedora Copr"
	@echo ""
	@echo "Release:"
	@echo "  release   - Create a new release (requires tag)"
	@echo ""
	@echo "Cross-compilation:"
	@echo "  targets   - Install cross-compilation targets"

# Development targets
build:
	cargo build --release

test:
	cargo test

clean:
	cargo clean
	rm -rf target/package dist

install:
	cargo install --path .

# Cross-compilation
targets:
	@echo "Installing cross-compilation targets..."
	@for target in $(TARGETS); do \
		rustup target add $$target; \
	done

# Packaging targets
package: targets
	@echo "Building packages for all architectures..."
	@chmod +x scripts/build-packages.sh
	@./scripts/build-packages.sh

deb: targets
	@echo "Building Debian packages..."
	@chmod +x scripts/build-packages.sh
	@./scripts/build-packages.sh

rpm:
	@echo "Building RPM packages..."
	@mkdir -p ~/rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
	@cp piko-$(VERSION).tar.gz ~/rpmbuild/SOURCES/ 2>/dev/null || echo "Please create source tarball first"
	@cp piko.spec ~/rpmbuild/SPECS/
	@rpmbuild -ba ~/rpmbuild/SPECS/piko.spec

aur:
	@echo "Creating Arch Linux PKGBUILD..."
	@cp PKGBUILD dist/ 2>/dev/null || mkdir -p dist && cp PKGBUILD dist/

copr:
	@echo "Publishing to Fedora Copr..."
	@chmod +x scripts/publish-copr.sh
	@./scripts/publish-copr.sh

# Release target
release:
	@echo "Creating release..."
	@if [ -z "$(shell git describe --tags --exact-match 2>/dev/null)" ]; then \
		echo "Error: No tag found. Please create a tag first:"; \
		echo "  git tag v$(VERSION)"; \
		echo "  git push origin v$(VERSION)"; \
		exit 1; \
	fi
	@echo "Release will be created automatically via GitHub Actions"

# Utility targets
fmt:
	cargo fmt

clippy:
	cargo clippy

check: fmt clippy test

# Documentation
docs:
	@echo "Building documentation..."
	@cd docs && make html

# Install dependencies for packaging
deps-deb:
	sudo apt-get update
	sudo apt-get install -y dpkg-dev

deps-rpm:
	sudo dnf install -y rpm-build copr-cli

deps-aur:
	@echo "Arch Linux dependencies are typically pre-installed"

# Show current version
version:
	@echo "Current version: $(VERSION)"

# Update version in all files
update-version:
	@echo "Updating version to $(VERSION) in all files..."
	@sed -i 's/version = ".*"/version = "$(VERSION)"/' Cargo.toml
	@sed -i 's/pkgver=.*/pkgver=$(VERSION)/' PKGBUILD
	@sed -i 's/Version:.*/Version: $(VERSION)/' piko.spec
	@echo "Version updated to $(VERSION)"
