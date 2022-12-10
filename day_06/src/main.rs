use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let start_of_packet = find_unique_substr::<4>(&args[1]);
    println!("Start of packet found at {start_of_packet}");
    let start_of_message = find_unique_substr::<14>(&args[1]);
    println!("Start of message found at {start_of_message}");
}

fn find_unique_substr<const N: usize>(path: impl AsRef<Path>) -> usize {
    let file = File::open(path).unwrap();
    let mut lines = BufReader::new(file)
        .lines()
        .into_iter()
        .map(|line| line.unwrap());
    let message = lines.next().unwrap();
    message
        .as_str()
        .char_indices()
        .map(|(from, chr)| {
            let (rel_end, _) = message[from..].char_indices().skip(N - 1).next().unwrap();
            &message.as_str()[from..from + rel_end + chr.len_utf8()]
        })
        .position(|sub_message| sub_message.chars().unique().count() == N)
        .unwrap()
        + N
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_11() {
        assert_eq!(7, find_unique_substr::<4>("example_1.txt"))
    }

    #[test]
    fn example_12() {
        assert_eq!(5, find_unique_substr::<4>("example_2.txt"))
    }

    #[test]
    fn example_13() {
        assert_eq!(6, find_unique_substr::<4>("example_3.txt"))
    }

    #[test]
    fn example_14() {
        assert_eq!(10, find_unique_substr::<4>("example_4.txt"))
    }

    #[test]
    fn example_15() {
        assert_eq!(11, find_unique_substr::<4>("example_5.txt"))
    }

    #[test]
    fn example_21() {
        assert_eq!(19, find_unique_substr::<14>("example_1.txt"))
    }

    #[test]
    fn example_22() {
        assert_eq!(23, find_unique_substr::<14>("example_2.txt"))
    }

    #[test]
    fn example_23() {
        assert_eq!(23, find_unique_substr::<14>("example_3.txt"))
    }

    #[test]
    fn example_24() {
        assert_eq!(29, find_unique_substr::<14>("example_4.txt"))
    }

    #[test]
    fn example_25() {
        assert_eq!(26, find_unique_substr::<14>("example_5.txt"))
    }
}
