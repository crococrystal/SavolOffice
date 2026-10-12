//! The interface's symbol characters must not render as empty boxes (#197, #54): the UI font often
//! lacks arrows, check marks and geometric shapes, so a symbol face backs it up.

use egui::{Color32, FontFamily, FontId};
use gridcraft_ui_egui::theme;

/// Symbols the interface draws as text (buttons, menus, dialogs, tooltips, Insert › Symbol).
const UI_SYMBOLS: &str = "▲▼▸▾✓✕←↑→↓↳↘↗⇧⌃⌘★•›≠≤≥∞";

#[test]
fn ui_symbols_have_glyphs_when_a_symbol_face_is_installed() {
    if !theme::symbol_face_installed() {
        return; // a minimal machine without any symbol face; nothing to assert
    }
    let ctx = egui::Context::default();
    ctx.set_fonts(theme::font_definitions());
    let mut out = ctx.run_ui(egui::RawInput::default(), |_ui| {});
    out.textures_delta.clear(); // nothing consumes the atlas in a headless test
    for family in [FontFamily::Proportional, FontFamily::Name(theme::CELL.into())] {
        let font = FontId::new(14.0, family);
        let glyphs = |text: &str| {
            let galley = ctx.fonts_mut(|f| f.layout_no_wrap(text.to_owned(), font.clone(), Color32::BLACK));
            galley.rows.iter().flat_map(|r| r.glyphs.iter().map(|g| (g.chr, g.uv_rect))).collect::<Vec<_>>()
        };
        // A private-use code point no font covers renders as the replacement box.
        let tofu: Vec<_> = glyphs("\u{10FFFD}").into_iter().map(|(_, uv)| uv).collect();
        let missing: String = glyphs(UI_SYMBOLS).into_iter().filter(|(_, uv)| tofu.contains(uv)).map(|(c, _)| c).collect();
        assert!(missing.is_empty(), "these interface symbols render as boxes in {:?}: {missing}", font.family);
    }
}

#[test]
fn workbook_fonts_keep_currency_and_multilingual_fallbacks() {
    use gridcraft_ui_egui::{fonts, i18n::Language};
    fonts::catalog();
    let requested: Vec<String> = ["Arial", "Calibri", "Times New Roman", "Courier New"].into_iter().map(str::to_owned).collect();
    let ctx = egui::Context::default();
    let definitions = theme::font_definitions_with(Language::En, &requested);
    let has_cjk = definitions.font_data.keys().any(|k| k == "bundled-cjk" || k.starts_with("sys-cjk"));
    let families = definitions.families.clone();
    ctx.set_fonts(definitions);
    let mut output = ctx.run_ui(egui::RawInput::default(), |_ui| {});
    output.textures_delta.clear();
    for name in &requested {
        for (bold, italic) in [(false, false), (true, false), (false, true), (true, true)] {
            let installed = FontFamily::Name(fonts::role_key(name, bold, italic).into());
            let family = if families.contains_key(&installed) { installed } else { theme::cell_family(name, bold, italic) };
            let font = FontId::new(18.0, family);
            let glyphs = |text: &str| {
                let galley = ctx.fonts_mut(|f| f.layout_no_wrap(text.to_owned(), font.clone(), Color32::BLACK));
                galley.rows.iter().flat_map(|r| r.glyphs.iter().map(|g| (g.chr, g.uv_rect))).collect::<Vec<_>>()
            };
            let tofu: Vec<_> = glyphs("\u{10FFFD}").into_iter().map(|(_, uv)| uv).collect();
            let missing: String =
                glyphs(if has_cjk { "₽€¥РусскийЁё中文繁體日本語한국어" } else { "₽€¥РусскийЁё" })
                    .into_iter()
                    .filter(|(_, uv)| tofu.contains(uv))
                    .map(|(c, _)| c)
                    .collect();
            assert!(missing.is_empty(), "{name} bold={bold} italic={italic} renders boxes: {missing}");
        }
    }
}

#[test]
fn bundled_cjk_renders_without_os_language_fonts() {
    let mut definitions = theme::font_definitions();
    if !definitions.font_data.contains_key("bundled-cjk") {
        return; // Source-only builds may omit the optional font input; release builds require it.
    }
    definitions.font_data.retain(|key, _| key == "bundled-cjk");
    for chain in definitions.families.values_mut() {
        *chain = vec!["bundled-cjk".into()];
    }
    let ctx = egui::Context::default();
    ctx.set_fonts(definitions);
    let mut output = ctx.run_ui(egui::RawInput::default(), |_ui| {});
    output.textures_delta.clear();
    let glyphs = |text: &str| {
        let galley = ctx.fonts_mut(|f| f.layout_no_wrap(text.to_owned(), FontId::proportional(18.0), Color32::BLACK));
        galley.rows.iter().flat_map(|r| r.glyphs.iter().map(|g| (g.chr, g.uv_rect))).collect::<Vec<_>>()
    };
    let tofu: Vec<_> = glyphs("\u{10FFFD}").into_iter().map(|(_, uv)| uv).collect();
    let missing: String = glyphs("РусскийЁё中文繁體日本語한국어").into_iter().filter(|(_, uv)| tofu.contains(uv)).map(|(c, _)| c).collect();
    assert!(missing.is_empty(), "Bundled font renders boxes without system fonts: {missing}");
}
