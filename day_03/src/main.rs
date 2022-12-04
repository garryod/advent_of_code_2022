use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let total_shared_priorities = total_shared_priorities(&args[1]);
    println!("Together the elves have shared priorities of {total_shared_priorities}");
}

fn total_shared_priorities<P: AsRef<Path>>(path: P) -> u32 {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .into_iter()
        .map(|line| {
            let line = line.unwrap();
            let (front, rear) = line.split_at(line.len() / 2);
            let common = front
                .chars()
                .find(|front_item| rear.contains(*front_item))
                .unwrap();
            ('a'..='z')
                .chain('A'..='Z')
                .position(|possible_item| possible_item == common)
                .unwrap() as u32
                + 1
        })
        .sum()
}

#[cfg(test)]
mod test {
    use crate::total_shared_priorities;

    #[test]
    fn example() {
        assert_eq!(157, total_shared_priorities("example.txt"))
    }
}
