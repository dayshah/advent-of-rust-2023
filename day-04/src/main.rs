
use std::{fs, collections::HashSet};
pub mod part1;
pub mod part2;

fn main() {
    println!("Day 4!");

    let cards: Vec<(HashSet<u32>,Vec<u32>)> = fs::read_to_string("input.txt")
        .unwrap_or_default()
        .lines()
        .map(|str| {
            let right_of_colon: &str = str.split(':').last().unwrap_or_default();
            let mut bar_split = right_of_colon.split('|');
            let winning_numbers: HashSet<u32> = bar_split.next().unwrap_or_default()
                .split_ascii_whitespace().filter_map(
                    |number_str| number_str.parse::<u32>().ok()
                ).collect();
            let your_numbers: Vec<u32> = bar_split.next().unwrap_or_default()
                .split_ascii_whitespace().filter_map(
                    |number_str| number_str.parse::<u32>().ok()
                ).collect();
            (winning_numbers, your_numbers)
        }).collect();

    println!("{}", part1::points_from_deck(&cards));

    println!("{}", part2::number_of_scratchcards(&cards));

}
