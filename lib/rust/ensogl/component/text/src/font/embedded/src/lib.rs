//! Collection of embedded fonts generated by the build.rs script.

// === Standard Linter Configuration ===
#![deny(non_ascii_idents)]
#![warn(unsafe_code)]
#![allow(clippy::let_and_return)]
// === Non-Standard Linter Configuration ===
#![allow(clippy::option_map_unit_fn)]
#![allow(clippy::precedence)]
#![allow(dead_code)]
#![deny(unconditional_recursion)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

use enso_prelude::*;

use ensogl_text_font_family as family;



// ==============
// === Export ===
// ==============

include!(concat!(env!("OUT_DIR"), "/embedded_fonts_data.rs"));



// ================
// === Embedded ===
// ================

/// A base of built-in fonts in application.
///
/// The structure keeps a map from a font name to its binary ttf representation. The binary data can
/// be further interpreted by such libs as the MSDF-gen one.
///
/// For list of embedded fonts, see FONTS_TO_EXTRACT constant in `build.rs`.
#[allow(missing_docs)]
#[derive(Clone)]
pub struct Embedded {
    pub definitions: HashMap<family::Name, family::Definition>,
    pub data:        HashMap<&'static str, &'static [u8]>,
}

impl Embedded {
    /// Construct and load all the embedded fonts to memory.
    pub fn init_and_load_embedded_fonts() -> Self {
        let data = embedded_fonts_data();
        let definitions = embedded_family_definitions_ext();
        Self { data, definitions }
    }
}

impl Debug for Embedded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("<Embedded fonts>")
    }
}



// =============
// === Tests ===
// =============

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn loading_embedded_fonts() {
        let fonts = Embedded::init_and_load_embedded_fonts();
        let example_font = fonts.data.get("DejaVuSans.ttf").unwrap();

        assert_eq!(0x00, example_font[0]);
        assert_eq!(0x01, example_font[1]);
        assert_eq!(0x1d, example_font[example_font.len() - 1]);
    }
}


// ======================
// === Embedded Fonts ===
// ======================

/// List of embedded fonts. The list is extended with hardcoded "DejaVuSans" font. It should be
/// generated from the build.rs script in the future.
pub fn embedded_family_definitions_ext() -> HashMap<family::Name, family::Definition> {
    let mut map = embedded_family_definitions();
    let dejavusans = family::Definition::NonVariable(family::NonVariableDefinition::from_iter([
        (
            family::NonVariableFaceHeader::new(
                family::Width::Normal,
                family::Weight::Normal,
                family::Style::Normal,
            ),
            "DejaVuSans.ttf".to_string(),
        ),
        (
            family::NonVariableFaceHeader::new(
                family::Width::Normal,
                family::Weight::Bold,
                family::Style::Normal,
            ),
            "DejaVuSans-Bold.ttf".to_string(),
        ),
    ]));
    let dejavusansmono =
        family::Definition::NonVariable(family::NonVariableDefinition::from_iter([
            (
                family::NonVariableFaceHeader::new(
                    family::Width::Normal,
                    family::Weight::Normal,
                    family::Style::Normal,
                ),
                "DejaVuSansMono.ttf".to_string(),
            ),
            (
                family::NonVariableFaceHeader::new(
                    family::Width::Normal,
                    family::Weight::Bold,
                    family::Style::Normal,
                ),
                "DejaVuSansMono-Bold.ttf".to_string(),
            ),
        ]));
    map.insert("dejavusans".into(), dejavusans.clone());
    map.insert("dejavusansmono".into(), dejavusansmono.clone());
    map.insert("default".into(), dejavusans);
    map.insert("default-mono".into(), dejavusansmono);
    map
}