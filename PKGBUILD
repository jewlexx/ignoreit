pkgname=ignoreit
pkgver=1.0.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="Quickly download .gitignore templates for nearly any project"
license=('MIT')

package() {
    cd $srcdir
    cargo install --root="$pkgdir" --git=https://github.com/jewlexx/ignoreit
}
