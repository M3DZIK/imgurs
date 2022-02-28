#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
fn is_program_in_path(program: &str) -> bool {
    use std::{env, fs};

    if let Ok(path) = env::var("PATH") {
        for p in path.split(":") {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true
            }
        }
    }
    false
}

#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
pub fn set_clipboard(content: String) {
    use std::{
        io::Write,
        process::{Command, Stdio},
    };

    use colored::Colorize;

    let mut child;

    // xsel
    if is_program_in_path("xsel") {
        child = Command::new("xsel")
            .arg("--input")
            .arg("--clipboard")
            .stdin(Stdio::piped())
            .spawn()
            .expect("execute command xsel")
    // xclip
    } else if is_program_in_path("xclip") {
        child = Command::new("xclip")
            .arg("-in")
            .arg("-selection")
            .arg("clipboard")
            .stdin(Stdio::piped())
            .spawn()
            .expect("execute command xclip")
    // termux
    } else if is_program_in_path("termux-clipboard-set") {
        child = Command::new("termux-clipboard-set")
            .stdin(Stdio::piped())
            .spawn()
            .expect("execute command termux-clipboard-set")
    } else {
        println!("{} {}", "WARN".yellow(), "command for clipboard not found".magenta());
        return
    }

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(content.as_bytes())
        .expect("execute command");
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
