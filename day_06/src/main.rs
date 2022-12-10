use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let start_of_packet = find_start_marker(&args[1]);
    println!("Start of packet found at {start_of_packet}");
}

fn find_start_marker(path: impl AsRef<Path>) -> usize {
    let file = File::open(path).unwrap();
    let mut lines = BufReader::new(file)
        .lines()
        .into_iter()
        .map(|line| line.unwrap());
    let message = lines.next().unwrap();
    message
        .chars()
        .into_iter()
        .tuple_windows::<(_, _, _, _)>()
        .position(|sub_message| {
            [sub_message.0, sub_message.1, sub_message.2, sub_message.3]
                .iter()
                .unique()
                .count()
                == 4
        })
        .unwrap()
        + 4
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_11() {
        assert_eq!(7, find_start_marker("example_1.txt"))
    }

    #[test]
    fn example_12() {
        assert_eq!(5, find_start_marker("example_2.txt"))
    }

    #[test]
    fn example_13() {
        assert_eq!(6, find_start_marker("example_3.txt"))
    }

    #[test]
    fn example_14() {
        assert_eq!(10, find_start_marker("example_4.txt"))
    }

    #[test]
    fn example_15() {
        assert_eq!(11, find_start_marker("example_5.txt"))
    }
}
