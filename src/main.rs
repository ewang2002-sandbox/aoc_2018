use std::time::Instant;
use helpers::io;

mod aoc;
mod helpers;

fn main() {
    let input_file = io::file_read_all_lines("input/day03.txt");

    // Execution begins
    let start = Instant::now();

    use aoc::day03 as aoc_problem;

    // Part 1
    let first_part_sol = aoc_problem::part1(&input_file);
    println!("Solution to Part 1: {}", first_part_sol);

    let second_part_sol = aoc_problem::part2(&input_file);
    println!("Solution to Part 2: {}", second_part_sol);

    let duration = start.elapsed();

    // Execution ends
    println!("Time Taken: {} Milliseconds", duration.as_millis());
}
