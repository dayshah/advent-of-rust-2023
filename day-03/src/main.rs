use std::fs;
pub mod part1;
pub mod part2;

fn main() {
    println!("Day 3!");

    let engine_schematic: Vec<Vec<char>> = fs::read_to_string("input.txt").unwrap_or_default()
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    println!("{}", part1::first_star(&engine_schematic));

    println!("{}", part2::second_star(&engine_schematic));
}
