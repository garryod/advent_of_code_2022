use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (best_idx, best_count) = find_best_elf(&args[0]);
    println!(
        "Elf {} carried the most calories, with {}",
        best_idx, best_count
    )
}

fn find_best_elf<P: AsRef<Path>>(path: P) -> (usize, u32) {
    let file = File::open(path).unwrap();
    let buffered_reader = BufReader::new(file);
    let (_, _, best_idx, best_count) = buffered_reader.lines().into_iter().fold(
        (1_usize, 0_u32, 0_usize, 0_u32),
        |(current_idx, current_count, best_idx, best_count), line| match line.unwrap().as_str() {
            "" => {
                if current_count > best_count {
                    println!("New best: {} with {}", current_idx, current_count);
                    (current_idx + 1, 0, current_idx, current_count)
                } else {
                    (current_idx + 1, 0, best_idx, best_count)
                }
            }
            str => (
                current_idx,
                current_count + str.parse::<u32>().unwrap(),
                best_idx,
                best_count,
            ),
        },
    );
    (best_idx, best_count)
}

#[cfg(test)]
mod tests {
    use crate::find_best_elf;

    #[test]
    fn example() {
        assert_eq!((4, 24000), find_best_elf("example.txt"));
    }
}
