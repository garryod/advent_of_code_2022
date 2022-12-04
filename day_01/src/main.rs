use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let elf_calories = aggregate_elf_calories(&args[1]);
    let most_calories = max_calories(&elf_calories);
    let max3_calories: u32 = max3_calories(&elf_calories);
    println!("Most calories: {most_calories}");
    println!("Max3 calories: {max3_calories}")
}

fn aggregate_elf_calories<P: AsRef<Path>>(path: P) -> Vec<u32> {
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    lines
        .group_by(|line| line == "")
        .into_iter()
        .filter(|(empty, _)| !*empty)
        .map(|(_, group)| group.fold(0_u32, |acc, line| acc + line.parse::<u32>().unwrap()))
        .collect()
}

fn max_calories(elf_calories: &Vec<u32>) -> u32 {
    *elf_calories.iter().max().unwrap()
}

fn max3_calories(elf_calories: &Vec<u32>) -> u32 {
    elf_calories.iter().sorted().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(24000, max_calories(&aggregate_elf_calories("example.txt")));
    }

    #[test]
    fn example_2() {
        assert_eq!(45000, max3_calories(&aggregate_elf_calories("example.txt")));
    }
}
