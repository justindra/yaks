#!/usr/bin/env bash
#
# Package yx release for testing installer
# Creates a zip file similar to what nix build would produce
#

set -e

echo "Building Rust binary..."
cargo build --release

echo "Creating release bundle..."
rm -rf release-bundle
mkdir -p release-bundle/bin
mkdir -p release-bundle/completions

cp target/release/yx release-bundle/bin/
cp -r completions/* release-bundle/completions/

echo "Creating zip..."
cd release-bundle
zip -r ../yx.zip .
cd ..

echo "✓ Release package created: yx.zip"
ls -lh yx.zip
