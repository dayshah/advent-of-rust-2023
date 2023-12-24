
pub fn first_star(engine_schematic: &Vec<Vec<char>>) -> u32 {
    let possible_part_numbers_and_indices = get_possible_part_num_and_locs(engine_schematic);
    possible_part_numbers_and_indices.iter()
        .filter(|num_and_loc| 
            is_valid_part_num_and_loc(num_and_loc, engine_schematic)
        ).map(|num_and_loc| num_and_loc.number)
        .sum()
}

pub struct NumAndLoc { pub number: u32, pub row: usize, pub start_c: usize, pub end_c: usize }

pub fn get_possible_part_num_and_locs(engine_schematic: &Vec<Vec<char>>) -> Vec<NumAndLoc> {
    let mut result: Vec<NumAndLoc> = Vec::new();
    for (row_i, row) in engine_schematic.iter().enumerate() {
        let mut number_str = String::new();
        let mut start = 0;
        let mut col_i: usize = 0;
        while let Some(c) = row.get(col_i) {
            if c.is_ascii_digit() {
                if number_str.len() == 0 { start = col_i; }
                number_str.push(*c);
            } else if number_str.len() > 0 {
                result.push(NumAndLoc { 
                    number: number_str.parse().unwrap_or_default(), 
                    row: row_i, start_c: start, end_c: col_i-1
                });
                number_str = String::new();
            }
            col_i += 1;
        }
        if number_str.len() > 0 {
            result.push(NumAndLoc { 
                number: number_str.parse().unwrap_or_default(), 
                row: row_i, start_c: start, end_c: col_i-1
            });
        }
    }
    result
}

pub fn is_valid_part_num_and_loc(num_and_loc: &NumAndLoc, engine_schematic: &Vec<Vec<char>>) -> bool {
    let start_row = 
        if num_and_loc.row > 0 { num_and_loc.row - 1 }
        else { 0 };
    let end_row = num_and_loc.row + 1;
    let start_col = 
        if num_and_loc.start_c > 0 { num_and_loc.start_c - 1 }
        else { 0 };
    let end_col = num_and_loc.end_c + 1;

    for row_i in start_row..end_row+1 {
        if let Some(row) = engine_schematic.get(row_i) {
            for col_i in start_col..end_col+1 {
                if let Some(c) = row.get(col_i) {
                    if !c.is_ascii_digit() && *c != '.' {
                        return true;
    }}}}}
    return false;
}
