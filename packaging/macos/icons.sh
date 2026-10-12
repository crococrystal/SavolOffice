#!/usr/bin/env bash
# Render the user-supplied vector at high resolution, then generate every macOS icon size.
# Needs rsvg-convert, ImageMagick and Apple's iconutil. Outputs are committed.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
DIR="$ROOT/assets/app-icon"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT
rsvg-convert -w 2048 -h 2048 "$DIR/savoloffice.svg" -o "$TMP/master.png"
render() { magick "$TMP/master.png" -filter Lanczos -resize "${1}x${1}" "$2"; }
render 512 "$DIR/savoloffice-512.png"
SET="$TMP/savoloffice.iconset"
mkdir -p "$SET"
for s in 16 32 128 256 512; do
  render "$s" "$SET/icon_${s}x${s}.png"
  render "$((s * 2))" "$SET/icon_${s}x${s}@2x.png"
done
iconutil -c icns -o "$DIR/savoloffice.icns" "$SET"
