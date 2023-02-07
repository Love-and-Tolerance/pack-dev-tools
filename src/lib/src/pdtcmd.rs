use std::process::{Command, Output};

pub fn execute_windows_command(cmd: &str) {
    Command::new("cmd")
        .args(["/C", cmd])
        .output()
        .unwrap_or_else(|_| panic!("failed to execute: {cmd}"));
}

pub fn execute_unix_command(cmd: &str) {
    Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .unwrap_or_else(|_| panic!("failed to execute: {cmd}"));
}

pub fn execute_windows_command_with_return(cmd: &str) -> Output {
    return Command::new("cmd")
        .args(["/C", cmd])
        .output()
        .unwrap_or_else(|_| panic!("failed to execute: {cmd}"));
}

pub fn execute_unix_command_with_return(cmd: &str) -> Output {
    return Command::new("sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .unwrap_or_else(|_| panic!("failed to execute: {cmd}"));
}
