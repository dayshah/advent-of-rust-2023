
pub fn first_star(games: &Vec<String>) -> u32 {    
    return games.iter()
        .filter_map(get_valid_game_id)
        .sum()
}

fn get_valid_game_id(game: &String) -> Option<u32> {
    let game_colon_split: Vec<&str> = game.split(": ").collect();
    let is_not_valid: bool = game_colon_split.last()?
        .split("; ")
        .any(|game_set| {
            is_game_set_not_valid(game_set.split(", ").collect())
        });
    if is_not_valid { None } 
    else { 
        let game_id: u32 = game_colon_split.first()?
            .split(' ').last()?
            .parse().ok()?;
        Some(game_id)
    }
}

fn is_game_set_not_valid(game_set: Vec<&str>) -> bool {
    game_set.iter().any(|num_and_color_str| {
        let num_and_color: Vec<&str> = num_and_color_str.split(" ").collect();
        let num: u32 = num_and_color.first()
            .unwrap_or(&"0")
            .parse().unwrap_or(0);
        let color: &str = num_and_color.last().unwrap_or(&"");
        match color {
            "red"   => num > 12,
            "green" => num > 13,
            "blue"  => num > 14,
            _       => true
        }
    })
}
