use std::fs;
use aoc::year2024::day01::solution;

fn main() {
    let input = fs::read_to_string("./inputs/year2024/day01/input.txt")
        .expect("Failed to load input file");

    let result = solution::run(&input);
    println!("Solution Result: {}", result);
}
