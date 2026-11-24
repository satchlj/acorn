//! Spellchecking for GPUI text fields. [`Spell`] gives a field wavy underlines under
//! misspelled words, ⌘; (Ctrl+; off macOS) to jump to one and pick a fix, the same fixes on
//! right-click (with Cut, Copy and Paste), and autocorrect that Backspace undoes. The words
//! come from [`check`]: the macOS system spellchecker, or Hunspell elsewhere (the locale's
//! dictionary, else a bundled en_US one from SCOWL). First made for Puzil.

pub mod check;
mod field;

pub use check::set_autocorrect;
pub use field::{Getter, Spell, SpellNext};

/// Binds ⌘; / Ctrl+;. Call once at startup.
pub fn init(cx: &mut gpui_kit::App) {
    field::bind_keys(cx);
}
