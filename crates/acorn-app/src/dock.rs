//! The Dock icon. A bundled `.app` gets it from its `.icns`; [`set_icon`] sets it when the app
//! runs as a bare binary (as in development). Call it after the first frame.

/// `png`: the icon, 512 px or so.
#[cfg(target_os = "macos")]
pub fn set_icon(png: &'static [u8]) {
    use objc2::{AllocAnyThread, MainThreadMarker};
    use objc2_app_kit::{NSApplication, NSImage};
    use objc2_foundation::NSData;

    // A bundle's Dock icon comes from its Info.plist; decoding the PNG costs ~18 ms.
    if std::env::current_exe().is_ok_and(|p| p.to_string_lossy().contains(".app/Contents/MacOS/")) {
        return;
    }
    let Some(mtm) = MainThreadMarker::new() else { return };
    let data = NSData::with_bytes(png);
    let image = NSImage::initWithData(NSImage::alloc(), &data);
    let app = NSApplication::sharedApplication(mtm);
    unsafe { app.setApplicationIconImage(image.as_deref()) };
}

#[cfg(not(target_os = "macos"))]
pub fn set_icon(_: &'static [u8]) {}
