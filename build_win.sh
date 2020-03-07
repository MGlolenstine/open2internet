export PKG_CONFIG_ALLOW_CROSS=1
export PKG_CONFIG_PATH=/usr/x86_64-w64-mingw32/lib/pkgconfig
cargo build --target=x86_64-pc-windows-gnu --release