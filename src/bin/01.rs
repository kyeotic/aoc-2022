use aoc::{get_input, report};
use itertools::Itertools;

fn main() {
    let calorie_per_elf: Vec<i32> = get_input("01")
        .split("\n\n")
        // .collect()
        .map(|elf| elf.lines().map(|l| l.parse::<i32>().unwrap()).sum())
        .sorted_by(|a, b| Ord::cmp(&a, &b))
        .collect();

    let a = calorie_per_elf.last().unwrap();
    let b: i32 = calorie_per_elf.iter().rev().take(3).sum();

    report(a, b);
}
