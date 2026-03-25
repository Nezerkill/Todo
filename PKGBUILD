# Maintainer: nezerkill
pkgname=todo-rs
pkgver=0.1.0
pkgrel=1
pkgdesc="Terminal To-Do Manager на Rust"
arch=('x86_64' 'aarch64')
url="https://github.com/nezerkill/dotfiles/tree/main/todo-rs"
license=('MIT')
depends=('gcc-libs')
makedepends=('rust' 'cargo')
source=("$pkgname-$pkgver::git+file://$PWD/../../.git")
sha256sums=('SKIP')

build() {
    cd "$srcdir/$pkgname-$pkgver/todo-rs"
    cargo build --frozen --release
}

package() {
    cd "$srcdir/$pkgname-$pkgver/todo-rs"
    install -Dm755 target/release/todo "$pkgdir/usr/bin/todo"
    install -Dm644 README.md "$pkgdir/usr/share/doc/$pkgname/README.md"
    install -Dm644 LICENSE "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}
