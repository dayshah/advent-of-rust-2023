
use std::fs;
pub mod part1;
pub mod part2;

fn main() {
    println!("Day 1!");

    let words: Vec<String> = fs::read_to_string("input.txt").unwrap_or_default()
        .lines()
        .map(String::from)
        .collect();

    match part1::first_star(&words) {
        Some(result) => println!("{}", result),
        None => println!("We have a problem (on first star)")
    };

    match part2::second_star(&words) {
        Some(result) => println!("{}", result),
        None => println!("We have a problem (on second star)")
    };
}
