pkgname=info-thing
pkgver=1
pkgrel=1
arch=('x86_64')
makedepends=('rust')
source=("$pkgname::git+https://github.com/elyalon/info_thing")
sha1sums=('SKIP')

build() {
    cd "$pkgname"
    cargo build --release
}

package() {
    cd "$pkgname"
    install -Dm755 "target/release/info_thing" "$pkgdir/usr/bin/info_thing"
}
