#!/bin/sh
set -eu

cd "$(dirname "$0")"

APP=rusty_palette
BINS="rusty_palette"

arch=$(uname -m)
case "$arch" in
    x86_64|amd64)  target=x86_64-unknown-linux-gnu ;;
    aarch64|arm64) target=aarch64-unknown-linux-gnu ;;
    *)             echo "Unsupported architecture: $arch" >&2; exit 1 ;;
esac

echo "Building $target..."
(cd bin && cargo build --release --target "$target" --bins)

pkg="$APP-$target"
mkdir -p release
rm -rf "release/$pkg"
mkdir "release/$pkg"
for bin in $BINS; do
    cp "bin/target/$target/release/$bin" "release/$pkg/"
done
tar -czf "release/$pkg.tar.gz" -C release "$pkg"
rm -rf "release/$pkg"

echo ""
echo "Created release/$pkg.tar.gz"
echo "Upload it on github.com: Releases -> Draft a new release -> attach the tarball"
