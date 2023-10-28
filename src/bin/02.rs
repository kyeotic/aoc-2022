use aoc::{get_input, report};

// I went a little nuts on this one writing a "perfect" program
// And when part two fucked up the middle I went for a dirty bolt-on job

#[derive(Debug)]
enum RpsMove {
    Rock,
    Paper,
    Scissors,
}

impl RpsMove {
    fn parse_attack(chr: &str) -> Self {
        match chr {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            _ => panic!("Unknown attack{}", chr),
        }
    }
    fn parse_response(chr: &str) -> Self {
        match chr {
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => panic!("Unknown response{}", chr),
        }
    }
    fn parse_response_predict(chr: &str, attack: &Self) -> Self {
        if chr == "X" && matches!(attack, RpsMove::Rock) {
            Self::Scissors
        } else if chr == "X" && matches!(attack, RpsMove::Scissors) {
            Self::Paper
        } else if chr == "X" && matches!(attack, RpsMove::Paper) {
            Self::Rock
        } else if chr == "Y" && matches!(attack, RpsMove::Rock) {
            Self::Rock
        } else if chr == "Y" && matches!(attack, RpsMove::Scissors) {
            Self::Scissors
        } else if chr == "Y" && matches!(attack, RpsMove::Paper) {
            Self::Paper
        } else if chr == "Z" && matches!(attack, RpsMove::Rock) {
            Self::Paper
        } else if chr == "Z" && matches!(attack, RpsMove::Scissors) {
            Self::Rock
        } else if chr == "Z" && matches!(attack, RpsMove::Paper) {
            Self::Scissors
        } else {
            panic!("unknown predict")
        }
    }
    fn get_score(&self) -> i32 {
        match &self {
            RpsMove::Rock => 1,
            RpsMove::Paper => 2,
            RpsMove::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum RpsResult {
    Win,
    Loss,
    Draw,
}

impl RpsResult {
    fn result(attack: &RpsMove, response: &RpsMove) -> Self {
        match attack {
            RpsMove::Rock if matches!(response, RpsMove::Paper) => Self::Win,
            RpsMove::Rock if matches!(response, RpsMove::Scissors) => Self::Loss,
            RpsMove::Paper if matches!(response, RpsMove::Rock) => Self::Loss,
            RpsMove::Paper if matches!(response, RpsMove::Scissors) => Self::Win,
            RpsMove::Scissors if matches!(response, RpsMove::Rock) => Self::Win,
            RpsMove::Scissors if matches!(response, RpsMove::Paper) => Self::Loss,
            _ => Self::Draw,
        }
    }

    fn get_score(&self) -> i32 {
        match &self {
            RpsResult::Win => 6,
            RpsResult::Loss => 0,
            RpsResult::Draw => 3,
        }
    }
}

struct Round {
    #[allow(dead_code)]
    attack: RpsMove,
    #[allow(dead_code)]
    response: RpsMove,
    #[allow(dead_code)]
    result: RpsResult,
    score: i32,
}

impl Round {
    fn parse(line: &str) -> Self {
        let moves: Vec<&str> = line.split(" ").collect();
        let attack = RpsMove::parse_attack(moves[0]);
        let response = RpsMove::parse_response(moves[1]);
        let result = RpsResult::result(&attack, &response);
        let score = result.get_score() + response.get_score();
        Self {
            attack,
            response,
            result,
            score,
        }
    }

    fn parse_predict(line: &str) -> Self {
        let moves: Vec<&str> = line.split(" ").collect();
        let attack = RpsMove::parse_attack(moves[0]);
        let response = RpsMove::parse_response_predict(moves[1], &attack);
        let result = RpsResult::result(&attack, &response);
        let score = result.get_score() + response.get_score();
        Self {
            attack,
            response,
            result,
            score,
        }
    }
}

fn main() {
    let input = get_input("02");
    let rounds = input.split("\n").filter(|l| !l.is_empty());

    let a: i32 = rounds
        .clone()
        .map(|round| Round::parse(round))
        .map(|r| r.score)
        .sum();

    let b: i32 = rounds
        .map(|round| Round::parse_predict(round))
        .map(|r| r.score)
        .sum();

    report(&a, &b);
}
