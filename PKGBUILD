pkgname="weather-widget"
pkgver="0.2.3"
pkgrel="1"
pkgdesc="Weather widget for Hyprland desktop powered by https://www.weatherapi.com."
url="https://github.com/AndreiLubinets/weather-widget"
arch=("x86_64")
license=(MIT)
depends=(gtk3)
makedepends=(
    cargo
    git
)
source=("git+$url")
b2sums=('SKIP')

prepare() {
    cd $pkgname
    export RUSTUP_TOOLCHAIN=stable
    cargo fetch --locked --target x86_64-unknown-linux-gnu
}

check() {
    cd $pkgname
    export RUSTUP_TOOLCHAIN=stable
    cargo test --frozen --all-features
}

build() {
    cd $pkgname
    export RUSTUP_TOOLCHAIN=stable
    cargo build --release --frozen --all-targets
}

package() {
    cd $pkgname
    install -Dt "$pkgdir/usr/bin" target/release/weather-widget
    install -dm755 "$pkgdir/home/$USER/.config/$pkgname"
}
