#[cfg(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
))]
// use xclip (or a similar program that is installed) because the kernel deletes the clipboard after the process ends
pub fn set_clipboard(content: String) {
    fn is_program_in_path(program: &str) -> bool {
        if let Ok(path) = std::env::var("PATH") {
            for p in path.split(':') {
                let p_str = format!("{}/{}", p, program);
                if std::fs::metadata(p_str).is_ok() {
                    return true;
                }
            }
        }
        false
    }

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

    // the above programs responsible for the clipboard were not found
    } else {
        println!(
            "{} {}",
            "WARN".yellow(),
            "command for clipboard not found".magenta()
        );

        return;
    }

    // copy the content (send it to stdin command)
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(content.as_bytes())
        .expect("execute command");

    child
        .wait_with_output()
        .expect("wait for clipboard command output");
}

#[cfg(not(all(
    unix,
    not(any(target_os = "macos", target_os = "android", target_os = "emscripten"))
)))]
pub fn set_clipboard(content: String) {
    let mut clipboard = arboard::Clipboard::new().unwrap();
    clipboard.set_text(content).unwrap();
}
