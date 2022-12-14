use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use itertools::{Either, Itertools};

type Crane = Box<dyn Fn(&mut Vec<Vec<char>>, usize, usize, usize) -> ()>;

fn move_crates_cm9000(stacks: &mut Vec<Vec<char>>, from: usize, to: usize, count: usize) {
    (0..count).for_each(|_| {
        let removed = stacks[from].pop().unwrap();
        stacks[to].push(removed);
    })
}
fn move_crates_cm9001(stacks: &mut Vec<Vec<char>>, from: usize, to: usize, count: usize) {
    let len_from = stacks[from].len();
    let mut removed = stacks[from]
        .drain(len_from - count..len_from)
        .collect::<Vec<_>>();
    stacks[to].append(&mut removed)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let crane: Crane = match args[2].as_str() {
        "9000" => Box::new(move_crates_cm9000),
        "9001" => Box::new(move_crates_cm9001),
        _ => panic!("Selected unknown CraneMover type"),
    };
    let top_crates = top_crates(&args[1], crane);
    println!("Top crates after moves were: {top_crates}");
}

fn top_crates(path: impl AsRef<Path>, crane_mover: Crane) -> String {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file)
        .lines()
        .into_iter()
        .map(|line| line.unwrap());
    let (arrangement_lines, mutate_lines): (Vec<_>, Vec<_>) = lines
        .filter_map(|line| {
            match line
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .chars()
                .next()
                .unwrap_or_default()
            {
                '[' => Some(Either::Left(line)),
                'm' => Some(Either::Right(line)),
                _ => None,
            }
        })
        .partition_map(|line| line);
    let mut arrangement = parse_starting(arrangement_lines);
    println!("Got starting arrangement of: {arrangement:?}");
    mutate_arrangement(&mut arrangement, mutate_lines, crane_mover);
    println!("Got final arrangement of: {arrangement:?}");
    arrangement
        .iter()
        .map(|stack| *stack.last().unwrap())
        .collect()
}

fn parse_starting<L: IntoIterator<Item = String>>(lines: L) -> Vec<Vec<char>> {
    let mut stacks = Vec::<Vec<char>>::new();
    lines.into_iter().for_each(|line| {
        line.chars()
            .skip(1)
            .step_by(4)
            .enumerate()
            .for_each(|(stack_idx, label)| {
                if stack_idx >= stacks.len() {
                    stacks.extend([Vec::<char>::new()]);
                }
                if label != ' ' {
                    stacks[stack_idx].insert(0, label);
                }
            });
    });
    stacks
}

fn mutate_arrangement(
    arrangement: &mut Vec<Vec<char>>,
    lines: impl IntoIterator<Item = String>,
    crane: Crane,
) -> () {
    lines.into_iter().for_each(|line| {
        let (count, from, to) = line
            .split_whitespace()
            .skip(1)
            .step_by(2)
            .map(|num| num.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();
        crane(arrangement, from - 1, to - 1, count);
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            "CMZ",
            top_crates("example.txt", Box::new(move_crates_cm9000))
        )
    }

    #[test]
    fn example_2() {
        assert_eq!(
            "MCD",
            top_crates("example.txt", Box::new(move_crates_cm9001))
        )
    }
}
