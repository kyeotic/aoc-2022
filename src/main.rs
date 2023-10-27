use std::{env, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();
    let day = &args[1];
    // println!("{}", args[1]);

    println!("=====Day {}=====\n", day);

    Command::new("cargo")
        .args(["run", "-q", "--color", "always", "--release", "--bin", &day])
        .spawn()
        .unwrap();
}
