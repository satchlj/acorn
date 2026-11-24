//! The standard app and window actions (Hide, Hide Others, Show All, Minimize, Zoom), closing
//! the main window hiding the app on macOS, and the window coming back where it was left.

use gpui_kit::*;

actions!(acorn_app, [Hide, HideOthers, ShowAll, Minimize, Zoom]);

/// Handles [`Hide`], [`HideOthers`] and [`ShowAll`], and binds ⌘H, ⌥⌘H and ⌘M on macOS.
/// Minimize and Zoom are window actions: menus send them to the window.
pub fn init(cx: &mut App) {
    cx.on_action(|_: &Hide, cx| cx.hide());
    cx.on_action(|_: &HideOthers, cx| cx.hide_other_apps());
    cx.on_action(|_: &ShowAll, cx| cx.unhide_other_apps());
    #[cfg(target_os = "macos")]
    cx.bind_keys([KeyBinding::new("cmd-h", Hide, None), KeyBinding::new("alt-cmd-h", HideOthers, None), KeyBinding::new("cmd-m", Minimize, None)]);
}

/// On macOS closing the main window hides the app instead (it keeps running, and the Dock
/// icon reopens it). Elsewhere there is no app-level hide, so closing the window quits.
pub fn attach(window: &mut Window, cx: &mut App) {
    #[cfg(target_os = "macos")]
    window.on_window_should_close(cx, |_, cx| {
        cx.hide();
        false
    });
    #[cfg(not(target_os = "macos"))]
    let _ = (window, cx);
}

/// Where the window was (`window.json` in the data folder), so the next launch opens it
/// there.
pub fn saved_bounds() -> Option<Bounds<Pixels>> {
    let j = std::fs::read_to_string(crate::data_dir().join("window.json")).ok()?;
    let v: [f32; 4] = serde_json::from_str(&j).ok()?;
    (v[2] >= 600. && v[3] >= 400.).then(|| Bounds::new(point(px(v[0]), px(v[1])), size(px(v[2]), px(v[3]))))
}

pub fn save_bounds(b: Bounds<Pixels>) {
    let v = [f32::from(b.origin.x), f32::from(b.origin.y), f32::from(b.size.width), f32::from(b.size.height)];
    if let Ok(j) = serde_json::to_string(&v) {
        let _ = std::fs::write(crate::data_dir().join("window.json"), j);
    }
}
