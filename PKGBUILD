# Maintainer: nezerkill
pkgname=todo-rs
pkgver=0.3.0
pkgrel=1
pkgdesc="Terminal To-Do Manager на Rust"
arch=('x86_64' 'aarch64')
url="https://github.com/Nezerkill/Todo"
license=('MIT')
depends=('gcc-libs')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver.tar.gz::https://github.com/Nezerkill/Todo/archive/refs/tags/v$pkgver.tar.gz")
sha256sums=('SKIP')

build() {
    cd "$srcdir/Todo-$pkgver"
    cargo build --release
}

package() {
    cd "$srcdir/Todo-$pkgver"
    install -Dm755 target/release/todo "$pkgdir/usr/bin/todo"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
