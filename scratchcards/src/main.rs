use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // Set up points vairable
    let mut points = 0;
    // Open input file
    if let Ok(lines) = read_lines("./input.txt") {
        // Process lines in file
        for line in lines.flatten() {
            // Remove "Card #" from string
            let Some((_, card)) = line.split_once(":") else {
                todo!()
            };
            // Split number in two groups
            let Some((allwinning, allnumbers)) = card.split_once("|") else {
                todo!()
            };
            // Break up winning and listed numbers
            let winning = allwinning.split_whitespace().collect::<Vec<&str>>();
            let numbers = allnumbers.split_whitespace().collect::<Vec<&str>>();
            // Prepare mutable variable to track wining numbers
            let mut wins: u32 = 0;
            // Count winning numbers
            for number in numbers {
                if winning.contains(&number) {
                    wins += 1;
                }
            }
            // Calculate points
            if wins > 0 {
                points += 2_u32.pow(wins - 1);
            }
        }
        println!("Total Points: {}", points);
    }
}

// File line iterator from rust docs
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
