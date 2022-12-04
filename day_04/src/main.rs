use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_overlapping = num_overlapping_assignments(&args[1]);
    println!("Found {num_overlapping} instances of overlapping assignments.")
}

fn num_overlapping_assignments<P: AsRef<Path>>(path: P) -> usize {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .into_iter()
        .map(|line| {
            let line = line.unwrap();
            let bounds: [[u32; 2]; 2] = line
                .split(",")
                .map(|assignment| -> [u32; 2] {
                    assignment
                        .split("-")
                        .map(|section| section.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            (bounds[0][0] <= bounds[1][0] && bounds[0][1] >= bounds[1][1])
                || (bounds[0][0] >= bounds[1][0] && bounds[0][1] <= bounds[1][1])
        })
        .filter(|overlapping| *overlapping)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::num_overlapping_assignments;

    #[test]
    fn example() {
        assert_eq!(2, num_overlapping_assignments("example.txt"))
    }
}
