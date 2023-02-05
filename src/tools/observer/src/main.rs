use pdtlib::observer::observe;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let old_release = args[1].to_string();
    let new_release = args[2].to_string();
    observe(old_release, new_release);
}

#[test]
fn test_observer() {
    let old_release = "/home/velvetremedy/Stuff/previous-release/".to_string();
    let new_release = "/home/velvetremedy/Stuff/new-release/".to_string();
    observe(old_release, new_release);
}
