use std::{env, fs};

pub fn get_input(day: &str) -> String {
    let cwd = env::current_dir().unwrap();
    let filepath = cwd.join("inputs").join(format!("{}.in", day));

    fs::read_to_string(filepath).unwrap()
}

pub fn report<T: std::fmt::Display, K: std::fmt::Display>(a: &T, b: &K) {
    println!("A:\t{}\nB:\t{}", a, b);
}
