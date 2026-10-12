# Savol Office

One desktop spreadsheet application, based on [GridCraft](https://github.com/storytold/gridcraft) by the ArtCraft team. Local version: **v4** (`0.6.1-savol.4`).

This fork fixes floating pictures overlapping frozen panes and follows formula hyperlinks (including scalar IF/IFERROR/IFNA wrappers) with a click.

Build the macOS app and installer: `RUSTUP_TOOLCHAIN=1.95.0 packaging/macos/package.sh --arch aarch64`. The result is `dist/release/Savol Office.dmg`, containing **Savol Office.app**. Use `--arch universal` with both Rust targets installed to build for Intel too. Signing and optional notarization use the existing packaging environment variables.

The package contains one application. Internal Rust crate names and automation command IDs remain compatible with GridCraft. Preferences use a separate Savol Office directory. Original licences and contributor attribution are preserved.


Upstream architecture, file-format support and known limitations are documented in `docs/`. Copyright and contributor credits are retained in the source and in Help → About.

The v3 app and DMG volume use the supplied Savol Office artwork, with macOS icon sizes 16–1024 px including Retina. Regenerate them with `bash packaging/macos/icons.sh` (rsvg-convert and ImageMagick required).

Windows x64: `pwsh packaging/windows/package.ps1 -Arch x64` produces a standalone `Savol Office.exe` and a ZIP with licence files. The manual Windows workflow builds and checks the native app on a Windows runner. No separate CLI application is shipped.

Release builds embed the OFL-licensed Noto Sans CJK SC fallback. Before a local release, clone `https://github.com/storytold/craft-fonts` into `craft-fonts` and check out `3d9ab3a132ed0eff457e424226a435a901da30d3`. Installed workbook fonts retain the complete fallback chain (currency symbols, Cyrillic and CJK); font names in the document remain unchanged. Exact metrics still require the requested font to be installed.
