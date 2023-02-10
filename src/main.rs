use std::process::{Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;

fn main() {
    let mut child = Command::new("python")
        .arg("./programs/test.py")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let one_sec = Duration::from_secs(1);
    let status_code = match child.wait_timeout(one_sec).unwrap() {
        Some(status) => status.code(),
        None => {
            println!("Time limit!");
            child.kill().unwrap();
            child.wait().unwrap().code()
        }
    };

    let output = child.wait_with_output().expect("Failed to read stdout");
    let stdout = String::from_utf8_lossy(&output.stdout);

    print!("code: {}\nout: {}", status_code.unwrap(), stdout);
}
