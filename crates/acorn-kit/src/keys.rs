//! Keys a text field would otherwise take, for the views around it: ↑ ↓ Tab Esc, bound in the
//! field's context (`Input`) ahead of its own bindings. A handler that doesn't want one calls
//! `cx.propagate()` and the field gets it (a suggestion list takes ↑↓ while it's open, say).

use gpui_kit::*;

actions!(acorn_kit, [FieldUp, FieldDown, FieldTab, FieldEscape]);

pub(crate) fn bind(cx: &mut App) {
    cx.bind_keys([
        KeyBinding::new("up", FieldUp, Some("Input")),
        KeyBinding::new("down", FieldDown, Some("Input")),
        KeyBinding::new("tab", FieldTab, Some("Input")),
        KeyBinding::new("escape", FieldEscape, Some("Input")),
    ]);
}
