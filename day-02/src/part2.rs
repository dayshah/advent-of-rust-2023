use std::collections::HashMap;


pub fn second_star(games: &Vec<String>) -> u32 {
    games.iter()
        .map(|game_line| -> Vec<&str> {
            game_line.split(": ").last().unwrap_or_default().split("; ").collect()
        })
        .map(power_of_game)
        .sum()
}

fn power_of_game(game: Vec<&str>) -> u32 {
    let minimum_rgb: (u32,u32,u32) = game.iter()
        .map(game_set_to_rgb)
        .fold(
            (0,0,0),
            |acc, set_rgb| {
                (
                    u32::max(acc.0, set_rgb.0),
                    u32::max(acc.1, set_rgb.1),
                    u32::max(acc.2, set_rgb.2)
                )
            }
        );
    minimum_rgb.0 * minimum_rgb.1 * minimum_rgb.2
}

fn game_set_to_rgb(game_set: &&str) -> (u32, u32, u32) {
    let colors_to_nums: HashMap<&str, u32> = game_set.split(", ").map(
        |num_and_color_str| -> (&str, u32) {
            let num_and_color: Vec<&str> = num_and_color_str.split(" ").collect();
            let num: u32 = num_and_color.first()
                .unwrap_or(&"0")
                .parse().unwrap_or(0);
            let color: &str = num_and_color.last().unwrap_or(&"");
            (color, num)
        }
    ).collect();
    (
        *colors_to_nums.get("red").unwrap_or(&0),
        *colors_to_nums.get("green").unwrap_or(&0),
        *colors_to_nums.get("blue").unwrap_or(&0)
    )
}
