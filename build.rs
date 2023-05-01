use std::process::Command;

fn main() {
    if Command::new("npm").arg("install").output().is_ok() {}
}
