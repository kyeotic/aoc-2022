use std::{env, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    // println!("{}", args[1]);

    let cmd = Command::new("cargo")
        .args(["run", "-q", "--color", "always", "--release", "--bin", &day])
        .output()
        .unwrap();

    let errors = String::from_utf8(cmd.stderr).unwrap();
    if !errors.is_empty() {
        print!("errors{}", errors);
        return;
    }
    let output = String::from_utf8(cmd.stdout).unwrap();
    println!("=====Day {}=====\n\n{}", day, output);
    // total_time += extract_microseconds(&output)?;
}
