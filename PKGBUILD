# Maintainer: Kartofi <kartofdev@gmail.com>
pkgname=korki
pkgver=1.0.2
pkgrel=1
pkgdesc="Simple emoji picker for hyprland people"
arch=('x86_64')
url="https://github.com/Kartofi/korki"
license=('MIT' 'Apache') # Adjust based on your actual license
depends=('gtk4')
makedepends=('cargo' 'git')
source=("$pkgname-$pkgver.tar.gz::https://github.com/Kartofi/korki/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP') # Replace with actual checksum later

prepare() {
  cd "$pkgname-$pkgver"
  cargo fetch --locked --target "$CARCH-unknown-linux-gnu"
}

build() {
  cd "$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release --all-features
}

check() {
  cd "$pkgname-$pkgver"
  export RUSTUP_TOOLCHAIN=stable
  cargo test --frozen --all-features
}

package() {
  cd "$pkgname-$pkgver"
  install -Dm755 "target/release/korki" "$pkgdir/usr/bin/korki"
}
