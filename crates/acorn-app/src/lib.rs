//! App plumbing for GPUI apps: who the app is ([`AppInfo`], set once at startup), where it
//! keeps its data and logs, a tokio runtime bridged to GPUI ([`runtime`]), wording and keys
//! that differ by platform ([`platform`], [`chord!`], [`key!`]), single-instance handoff on
//! Linux ([`instance`]), the Dock icon ([`dock`]), and the standard window behaviour and
//! actions ([`window`]).

use std::{path::PathBuf, sync::OnceLock};

pub mod dock;
pub mod instance;
pub mod platform;
pub mod runtime;
pub mod window;

/// Who the app is. Set once, first thing in `main`, with [`init`].
#[derive(Debug, Clone)]
pub struct AppInfo {
    /// The name people see: "Puzil".
    pub name: &'static str,
    /// For files, sockets and threads: "puzil".
    pub slug: &'static str,
    /// Prefix of the app's environment variables: "PUZIL" reads `PUZIL_DATA_DIR`.
    pub env_prefix: &'static str,
    /// `CARGO_PKG_VERSION` of the app.
    pub version: &'static str,
    /// Where debug builds keep their data (a gitignored folder in the checkout), if not the
    /// usual place.
    pub dev_data_dir: Option<&'static str>,
}

static INFO: OnceLock<AppInfo> = OnceLock::new();

/// Set the app's identity. Call before anything else in this crate.
pub fn init(info: AppInfo) {
    let _ = INFO.set(info);
}

/// The app's identity. Panics if [`init`] wasn't called.
pub fn info() -> &'static AppInfo {
    INFO.get().expect("acorn_app::init must be called first")
}

/// An environment variable of the app's: `env("DATA_DIR")` reads `PUZIL_DATA_DIR`.
pub fn env(name: &str) -> Option<std::ffi::OsString> {
    std::env::var_os(format!("{}_{name}", info().env_prefix))
}

/// `<PREFIX>_DATA_DIR`, else [`AppInfo::dev_data_dir`] in debug builds, else the platform's
/// per-user application data folder (`~/Library/Application Support/<Name>`, or
/// `$XDG_DATA_HOME/<slug>`).
pub fn data_dir() -> PathBuf {
    if let Some(d) = env("DATA_DIR") {
        return d.into();
    }
    if cfg!(debug_assertions)
        && let Some(d) = info().dev_data_dir
    {
        return PathBuf::from(d);
    }
    let home = PathBuf::from(std::env::var_os("HOME").unwrap_or_default());
    if cfg!(target_os = "macos") {
        home.join("Library/Application Support").join(info().name)
    } else {
        std::env::var_os("XDG_DATA_HOME").map(PathBuf::from).unwrap_or_else(|| home.join(".local/share")).join(info().slug)
    }
}

/// Where logs go: `~/Library/Logs/<Name>` on macOS, `<data dir>/logs` elsewhere and in debug
/// builds (or with `<PREFIX>_DATA_DIR` set).
pub fn log_dir() -> PathBuf {
    if cfg!(all(target_os = "macos", not(debug_assertions))) && env("DATA_DIR").is_none() {
        let home = PathBuf::from(std::env::var_os("HOME").unwrap_or_default());
        return home.join("Library/Logs").join(info().name);
    }
    data_dir().join("logs")
}
