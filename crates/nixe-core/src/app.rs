//! The running app's identity. Set once at the top of `main`, read by
//! [`crate::paths`] to find the app's own directories.

use std::sync::OnceLock;

#[derive(Debug, Clone, Copy)]
pub struct App {
    /// Directory name under `~/.config`, `~/.cache`, …
    pub slug: &'static str,
    /// The name shown in the window title and messages, e.g. "Music".
    pub name: &'static str,
}

static APP: OnceLock<App> = OnceLock::new();

/// Record which app is running. Later calls are ignored.
pub fn init(app: App) {
    let _ = APP.set(app);
}

pub fn get() -> App {
    *APP.get().expect("nixe_core::app::init must run first")
}

pub fn slug() -> &'static str {
    get().slug
}

pub fn name() -> &'static str {
    get().name
}
