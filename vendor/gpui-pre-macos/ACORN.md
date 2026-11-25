# gpui-pre-macos 0.3.14, patched for Acorn apps (first made for Puzil)

GPUI's macOS platform crate from crates.io (Apache-2.0, see `LICENSE-APACHE`), with two changes:

1. **The `CVDisplayLink` path pauses while the window is idle** (`src/display_link.rs`: `CvFrames`;
   `src/window.rs`: `step`, `frame_waker`, `schedule_frame`). Puzil paces frames with
   `CVDisplayLink` (see `vendor/gpui-pre-apple/ACORN.md`), which upstream leaves running while
   a window is visible: an idle Puzil window woke ~29 times a second. Like upstream's Metal link, a
   window now leaves its display's link after 10 ticks in which GPUI asked for no frame, and
   joins it again when GPUI next asks (its frame waker) or schedules a frame, drawing that frame
   at once instead of at the restarted link's first tick. At rest: 0 wakeups.
2. **The keyboard layout is looked up when first asked for** (`src/keyboard.rs`), not when the
   platform starts: the first input-source lookup costs ~11 ms of launch. Puzil asks for it
   right after its first frame.

Applied with `[patch.crates-io]` in the workspace `Cargo.toml`. To update GPUI, copy the new
release over this folder and reapply the changes (search for `Acorn:`).
