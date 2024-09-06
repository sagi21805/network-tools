use std::process::Command;

#[test]
fn test(){

    let mut command = Command::new("bash");
    command.arg("../ports.sh");
    let out = command.output().expect("Failed to execute command");
    println!("Done {}", std::str::from_utf8(&out.stdout).unwrap())
}