use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let original_score = evaluate_original_strategy(&args[1]);
    let new_score = evaluate_new_strategy(&args[1]);
    println!("Original strategy resulted in score of {original_score}");
    println!("New strategy resulted in score of {new_score}");
}

enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn from_outcome(character: &char) -> Self {
        match character {
            'X' => Self::Loss,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("Outcome '{character}' is not understood"),
        }
    }

    fn from_response_to_opponent(response: &Choices, opponent: &Choices) -> Self {
        match (response, opponent) {
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

    fn score(self) -> u32 {
        match self {
            Self::Win => 6,
            Self::Draw => 3,
            Self::Loss => 0,
        }
    }
}

#[derive(Clone, Copy)]
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
            _ => panic!("Opponent play '{character}' is not understood"),
        }
    }

    fn from_response(character: &char) -> Self {
        match character {
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissors,
            _ => panic!("Reponse '{character}' is not understood"),
        }
    }

    fn from_opponent_and_outcome(opponent: &Self, outcome: &Outcome) -> Self {
        match (opponent, outcome) {
            (_, Outcome::Draw) => opponent.clone(),
            (Choices::Rock, Outcome::Win) => Self::Paper,
            (Choices::Rock, Outcome::Loss) => Self::Scissors,
            (Choices::Paper, Outcome::Win) => Self::Scissors,
            (Choices::Paper, Outcome::Loss) => Self::Rock,
            (Choices::Scissors, Outcome::Win) => Self::Rock,
            (Choices::Scissors, Outcome::Loss) => Self::Paper,
        }
    }

    fn value(&self) -> u32 {
        match self {
            Choices::Rock => 1,
            Choices::Paper => 2,
            Choices::Scissors => 3,
        }
    }
}

fn evaluate_original_strategy<P: AsRef<Path>>(path: P) -> u32 {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    lines.fold(0_u32, |acc, line| {
        let opponent = Choices::from_opponent(&line.chars().next().unwrap());
        let response = Choices::from_response(&line.chars().last().unwrap());
        let outcome = Outcome::from_response_to_opponent(&opponent, &response);
        acc + response.value() + outcome.score()
    })
}

fn evaluate_new_strategy<P: AsRef<Path>>(path: P) -> u32 {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    lines.fold(0_u32, |acc, line| {
        let outcome = Outcome::from_outcome(&line.chars().last().unwrap());
        let opponent = Choices::from_opponent(&line.chars().next().unwrap());
        let response = Choices::from_opponent_and_outcome(&opponent, &outcome);
        acc + response.value() + outcome.score()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(15, evaluate_original_strategy("example.txt"))
    }

    #[test]
    fn example_2() {
        assert_eq!(12, evaluate_new_strategy("example.txt"))
    }
}
