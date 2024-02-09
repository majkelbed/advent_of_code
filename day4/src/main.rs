use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_numbers(s: &str) -> Vec<usize> {
    return s
        .replace("| ", "")
        .trim()
        .split_whitespace()
        .map(|n| n.to_string().parse::<usize>().unwrap())
        .collect();
}

fn main() {
    let mut total = 0;
    let mut scratchcards_won = HashMap::new();

    if let Ok(lines) = read_lines("src/input") {
        for line in lines.flatten() {
            let info: Vec<&str> = line.split(':').collect();
            let (id, result) = (info[0], info[1]);

            let mut nums: Vec<_> = parse_numbers(result);
            nums.sort_unstable();

            let winning_numbers: Vec<_> = nums
                .iter()
                .zip(nums.iter().skip(1))
                .filter_map(|(prev_n, n)| if prev_n == n { Some(n) } else { None })
                .collect();
            let mut score = 0;
            let matches = winning_numbers.len();

            if matches > 0 {
                score = 2_u32.pow((matches - 1) as u32);
            }

            println!(
                "{} has {} matches, score: {}",
                id,
                matches,
                score
            );
            total += score;

            // PART TWO
            scratchcards_won.insert(id.to_owned(), matches);
        }
    }

    println!("Total score: {}", total);
}
