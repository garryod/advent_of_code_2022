use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let score = evaluate_strategy(&args[1]);
    println!("Strategy resulted in score of {}", score);
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn score(self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

enum Choices {
    Rock,
    Paper,
    Scissors,
}

impl Choices {
    fn from_opponent(character: &char) -> Self {
        match character {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissors,
            _ => panic!("Opponent play '{}' is not understood", character),
        }
    }

    fn from_response(character: &char) -> Self {
        match character {
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            _ => panic!("Reponse '{}' is not understood", character),
        }
    }

    fn value(&self) -> u32 {
        match self {
            Choices::Rock => 1,
            Choices::Paper => 2,
            Choices::Scissors => 3,
        }
    }

    fn round_outcome(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Choices::Rock, Choices::Rock) => Outcome::Draw,
            (Choices::Rock, Choices::Paper) => Outcome::Loss,
            (Choices::Rock, Choices::Scissors) => Outcome::Win,
            (Choices::Paper, Choices::Rock) => Outcome::Win,
            (Choices::Paper, Choices::Paper) => Outcome::Draw,
            (Choices::Paper, Choices::Scissors) => Outcome::Loss,
            (Choices::Scissors, Choices::Rock) => Outcome::Loss,
            (Choices::Scissors, Choices::Paper) => Outcome::Win,
            (Choices::Scissors, Choices::Scissors) => Outcome::Draw,
        }
    }

    fn round_score(self, other: &Self) -> u32 {
        self.round_outcome(other).score() + self.value()
    }
}

fn evaluate_strategy<P: AsRef<Path>>(path: P) -> u32 {
    let file = File::open(path).unwrap();
    let buffered_reader = BufReader::new(file);
    buffered_reader
        .lines()
        .into_iter()
        .map(|line| line.unwrap())
        .map(|line| (line.chars().next().unwrap(), line.chars().last().unwrap()))
        .map(|(opponent, response)| {
            (
                Choices::from_opponent(&opponent),
                Choices::from_response(&response),
            )
        })
        .fold(0_u32, |points, (opponent, resposne)| {
            points + resposne.round_score(&opponent)
        })
}

#[cfg(test)]
mod tests {
    use crate::evaluate_strategy;

    #[test]
    fn example() {
        assert_eq!(15, evaluate_strategy("example.txt"))
    }
}
