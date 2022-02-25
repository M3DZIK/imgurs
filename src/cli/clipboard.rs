#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
use clipboard_ext::prelude::*;
#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
use clipboard_ext::x11_fork::ClipboardContext;

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
pub fn set_clipboard(content: String) {
    let mut ctx = ClipboardContext::new().expect("init clipboard");
    ctx.set_contents(content).expect("set clipboard");
}

#[cfg(not(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
)))]
use arboard::Clipboard;

#[cfg(not(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
)))]
pub fn set_clipboard(content: String) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(content).unwrap();
}
