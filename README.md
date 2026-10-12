# SavolOffice

One desktop spreadsheet application, based on [GridCraft](https://github.com/storytold/gridcraft) by the ArtCraft team. Local version: **v2** (`0.6.1-savol.2`).

This fork fixes floating pictures overlapping frozen panes and follows formula hyperlinks (including scalar IF/IFERROR/IFNA wrappers) with a click.

Build the macOS app and installer: `RUSTUP_TOOLCHAIN=1.95.0 packaging/macos/package.sh --arch aarch64`. The result is `dist/release/SavolOffice.dmg`, containing **SavolOffice.app**. Use `--arch universal` with both Rust targets installed to build for Intel too. Signing and optional notarization use the existing packaging environment variables.

The package contains one application. Internal Rust crate names and automation command IDs remain compatible with GridCraft. Preferences use a separate SavolOffice directory. Original licences and contributor attribution are preserved.


Upstream architecture, file-format support and known limitations are documented in `docs/`. Copyright and contributor credits are retained in the source and in Help → About.
