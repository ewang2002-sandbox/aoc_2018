use std::time::Instant;
use helpers::io;

mod aoc;
mod helpers;

fn main() {
    let input_file = io::file_read_all_lines("input/day08.txt");

    // Execution begins
    let start = Instant::now();

    use aoc::day08 as aoc_problem;

    // Get both parts
    let (part1_sol, part2_sol) = aoc_problem::execute(&input_file);
    println!("Solution to Part 1: {}", part1_sol);
    println!("Solution to Part 2: {}", part2_sol);

    let duration = start.elapsed();

    // Execution ends
    println!("Time Taken: {} Milliseconds", duration.as_millis());
}
