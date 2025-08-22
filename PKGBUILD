# Maintainer: Elxes04 <elxes04@example.com>
pkgname=piko
pkgver=0.2.1
pkgrel=1
pkgdesc="A minimal, customizable system information tool written in Rust"
arch=('x86_64' 'aarch64' 'i686' 'armv7h')
url="https://github.com/Elxes04/piko"
license=('MIT')
depends=('glibc')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/Elxes04/$pkgname/archive/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$pkgname-$pkgver"
    cargo build --release
}

package() {
    cd "$pkgname-$pkgver"
    
    install -Dm755 "target/release/$pkgname" "$pkgdir/usr/bin/$pkgname"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    
    # Install configuration files
    install -dm755 "$pkgdir/usr/share/$pkgname"
    install -m644 config/*.toml "$pkgdir/usr/share/$pkgname/"
    install -m644 config/*.md "$pkgdir/usr/share/$pkgname/"
}
