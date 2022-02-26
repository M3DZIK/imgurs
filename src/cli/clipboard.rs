#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
pub fn set_clipboard(content: String) {
    use std::{process::{Command, Stdio}, io::Write};

    let mut child = Command::new("xsel")
        .arg("--input")
        .arg("--clipboard")
        .stdin(Stdio::piped())
        .spawn()
        .expect("execute command xsel");

    child.stdin.as_mut().unwrap().write_all(content.as_bytes()).expect("execute command");
    child.wait_with_output().unwrap();
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
