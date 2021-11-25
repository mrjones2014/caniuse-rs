#[cfg(target_os = "macos")]
pub fn open<S: AsRef<str>>(url: S) {
    let _ = std::process::Command::new("open")
        .arg(url.as_ref())
        .output();
}

#[cfg(target_os = "linux")]
pub fn open<S: AsRef<str>>(url: S) {
    let _ = std::process::Command::new("xdg-open")
        .arg(url.as_ref())
        .output();
}
