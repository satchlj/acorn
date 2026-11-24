# acorn

Ingredients for native desktop apps written in Rust with [GPUI](https://www.gpui.rs/)
(through [gpui-kit](https://crates.io/crates/gpui-kit)), for macOS and Linux. First grown
inside [Puzil](https://github.com/satchlj/puzil), a Zulip client.

| Crate | What it gives an app |
| --- | --- |
| `acorn-app` | Who the app is (`AppInfo`), its data and log folders, a tokio runtime bridged to GPUI, platform wording and the `chord!`/`key!` macros, single-instance URL handoff on Linux, the Dock icon, standard window actions and remembered window placement |
| `acorn-theme` | The house style: colour tokens, six palettes (Neutral, Earth, Aubergine, Ocean, Sunset, Ultraviolet) each light and dark, the fonts (Figtree, Unbounded, JetBrains Mono), and the chrome of a window without a title bar |
| `acorn-kit` | Controls: buttons, fields, check boxes, segmented choices, toggles, menus, tooltips; round avatars; short entrances; Lucide icons; fuzzy matching; text-field keys (↑ ↓ Tab Esc for the views around a field, and ↑/↓ jumping to the start or end of the text) |
| `acorn-spell` | Spellchecking for text fields: wavy underlines, ⌘; to jump to a mistake and pick a fix, right-click suggestions, autocorrect that Backspace undoes; the macOS system checker, or Hunspell elsewhere |
| `acorn-update` | Self-update from GitHub releases: a signed `.app` swapped in place on macOS, an ed25519-signed AppImage on Linux |
| `acorn-math` | LaTeX and Typst math typeset to SVG (Typst and MiTeX) |
| `acorn-avatar` | Generated gradient avatars and app-icon art |

`vendor/` holds GPUI's macOS renderer and platform crates with patches that cut memory and
idle wakeups (see `vendor/*/ACORN.md`); an app applies them with a `[patch.crates-io]` in its
workspace. `tools/` holds the build and release scripts (`.app` bundle and zip, AppImage,
Linux install, public-source release), configured per app by an `acorn-app.env`.

## Using it

```toml
[dependencies]
acorn-app = { git = "https://github.com/satchlj/acorn", rev = "…" }
acorn-kit = { git = "https://github.com/satchlj/acorn", rev = "…" }

[patch.crates-io]
gpui-pre-apple = { git = "https://github.com/satchlj/acorn", rev = "…" }
gpui-pre-macos = { git = "https://github.com/satchlj/acorn", rev = "…" }
```

At startup:

```rust
acorn_app::init(acorn_app::AppInfo { name: "Puzil", slug: "puzil", env_prefix: "PUZIL",
    version: env!("CARGO_PKG_VERSION"), dev_data_dir: None });
// in Application::run, after gpui_kit::init:
cx.text_system().add_fonts(acorn_theme::fonts())?;
acorn_kit::init(cx);
acorn_spell::init(cx);
acorn_app::window::init(cx);
```

and on the window's root element, for the ↑/↓ text-field keys:

```rust
.on_action(|_: &acorn_kit::edge_keys::EdgeUp, w, cx| acorn_kit::edge_keys::handle(false, w, cx))
.on_action(|_: &acorn_kit::edge_keys::EdgeDown, w, cx| acorn_kit::edge_keys::handle(true, w, cx))
```

## License

MIT OR Apache-2.0. The fonts in `crates/acorn-theme/fonts` are under the SIL Open Font
License; the dictionary in `crates/acorn-spell/dictionaries` is SCOWL's (MIT/BSD-like, see
its LICENSE); `vendor/` is GPUI's (Apache-2.0).
