
use std::fs;
pub mod part1;
pub mod part2;

fn main() {
    println!("Day 2!");

    let games: Vec<String> = fs::read_to_string("input.txt").unwrap_or_default()
        .lines()
        .map(String::from)
        .collect();
    
    println!("{}", part1::first_star(&games));
 
    println!("{}", part2::second_star(&games));
}
