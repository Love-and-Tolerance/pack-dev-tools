#[cfg(target_os = "windows")]
pub fn get_os_slash() -> String {
    r"\".to_string()
}

#[cfg(not(target_os = "windows"))]
pub fn get_os_slash() -> String {
    "/".to_string()
}
