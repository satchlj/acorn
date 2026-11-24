//! Controls for GPUI apps in the acorn house style ([`acorn_theme`]): buttons, fields, check
//! boxes, segmented choices, toggles, menus and tooltips ([`controls`]), round avatars
//! ([`avatar`]), short entrances ([`motion`]), icons ([`icons`]), fuzzy matching ([`fuzzy`]),
//! and text-field keys ([`keys`], [`edge_keys`]). First made for Puzil.

pub mod avatar;
pub mod controls;
pub mod edge_keys;
pub mod fuzzy;
pub mod icons;
pub mod keys;
pub mod motion;

pub use controls::*;

/// Binds the text-field keys. Call once at startup, after gpui-kit's `init` (so these are
/// tried before the fields' own ↑ ↓ Tab Esc).
pub fn init(cx: &mut gpui_kit::App) {
    edge_keys::bind_keys(cx);
    keys::bind(cx);
}
