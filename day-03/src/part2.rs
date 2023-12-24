use crate::part1;
use std::collections::HashSet;

pub fn second_star(engine_schematic: &Vec<Vec<char>>) -> u32 {
    let part_numbers: Vec<part1::NumAndLoc> = 
        part1::get_possible_part_num_and_locs(engine_schematic).into_iter()
            .filter(|num_and_loc| 
                part1::is_valid_part_num_and_loc(&num_and_loc, engine_schematic)
            )
            .collect();
    get_gears(engine_schematic).into_iter().filter_map(
        |gear_loc| {
            get_gear_ratio(&gear_loc, &part_numbers)
        }
    ).sum()
}

fn get_gears(engine_schematic: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    engine_schematic.iter()
        .enumerate()
        .flat_map(|(row_i, row)| {
            row.iter().enumerate().filter_map(
                move |(col_i, c)| {
                    if *c == '*' { Some((row_i, col_i)) }
                    else { None }
                }
            )
        }).collect()
    
    // let mut result = Vec::new();
    // for (row_i, row) in engine_schematic.iter().enumerate() {
    //     for (col_i, c) in row.iter().enumerate() {
    //         if *c == '*' { result.push((row_i, col_i)) }
    //     }
    // }
    // result
}

fn get_gear_ratio(gear_loc: &(usize, usize), part_numbers: &Vec<part1::NumAndLoc>) -> Option<u32> {
    let mut locs_to_check: HashSet<(usize, usize)> = vec![
        (gear_loc.0+1, gear_loc.1+1),
        (gear_loc.0,   gear_loc.1+1),
        (gear_loc.0+1, gear_loc.1)
    ].into_iter().collect();
    if gear_loc.0 > 0 { 
        locs_to_check.insert((gear_loc.0-1, gear_loc.1));
        locs_to_check.insert((gear_loc.0-1, gear_loc.1+1));
    }
    if gear_loc.1 > 0 { 
        locs_to_check.insert((gear_loc.0,   gear_loc.1-1));
        locs_to_check.insert((gear_loc.0+1, gear_loc.1-1));
    }
    if gear_loc.0 > 0 && gear_loc.1 > 0 {
        locs_to_check.insert((gear_loc.0-1, gear_loc.1-1));
    }

    let locs_to_check_ref = &locs_to_check;
    let ratio_nums: Vec<u32> = part_numbers.iter()
        .filter_map(
            |num_and_loc| {
                if (num_and_loc.start_c..num_and_loc.end_c+1).any(
                    move |col| {
                        locs_to_check_ref.contains(&(num_and_loc.row, col))
                    }
                ) { Some(num_and_loc.number) }
                else { None }
            }
        ).collect();

    // let mut ratio_nums: Vec<u32> = Vec::new();
    // for num_and_loc in part_numbers {
    //     for col in num_and_loc.start_c..num_and_loc.end_c+1 {
    //         if locs_to_check.contains(&(num_and_loc.row, col)) {
    //             ratio_nums.push(num_and_loc.number);
    //             break;
    //         }
    //     }
    // }

    if ratio_nums.len() != 2 { None }
    else { Some(ratio_nums.get(0).unwrap() * ratio_nums.get(1).unwrap()) }
}
