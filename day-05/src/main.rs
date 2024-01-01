use std::fs;
pub mod part1;

pub struct Almanac {
    pub seeds: Vec<i32>,
    pub mappings: Vec<Vec<DestSourceLen>>
}

pub type DestSourceLen = (i64,i64,i64);

fn main() {
    println!("Day 5!");

    let input = fs::read_to_string("input.txt").unwrap_or_default();
    let mut input_lines = input.lines();
    let seeds: Vec<i32> = input_lines.next().unwrap_or_default()
        .split(':').last().unwrap_or_default()
        .split_ascii_whitespace()
        .filter_map(|num_str| num_str.parse().ok()).collect();

    let mappings: Vec<Vec<DestSourceLen>> = input_lines
        .fold(String::new(), |acc,e| { format!("{acc}\n{e}") })
        .split(":")
        .map(|map: &str| -> Vec<DestSourceLen> {
            map.to_owned().lines().filter_map(
                |mapping_str| -> Option<DestSourceLen> {
                    let dest_source_len: Vec<i64> = mapping_str
                        .split_ascii_whitespace()
                        .filter_map(|num_str| num_str.parse().ok())
                        .collect();
                    if dest_source_len.len() < 3 { return None }
                    else { Some((
                        dest_source_len.get(0).cloned().unwrap_or_default(), 
                        dest_source_len.get(1).cloned().unwrap_or_default(), 
                        dest_source_len.get(2).cloned().unwrap_or_default()
                    ))}
                }
            ).collect()
        }).filter(
            |mapping| mapping.len() > 0
        ).collect();

    let almanac = Almanac { seeds, mappings };

    println!("{}", part1::get_lowest_location(&almanac));

}
