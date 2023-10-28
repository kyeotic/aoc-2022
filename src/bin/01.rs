use aoc::{get_input, report};
use itertools::Itertools;

fn main() {
    let calorie_per_elf = get_input("01")
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|l| l.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .sorted()
        .rev()
        .collect::<Vec<_>>();

    let a = calorie_per_elf[0];
    let b = calorie_per_elf[0..3].iter().sum::<usize>();

    report(&a, &b);

    assert_eq!(a, 71471);
    assert_eq!(b, 211189);
}
