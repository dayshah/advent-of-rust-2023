
pub fn first_star(words: &Vec<String>) -> u32 {
    words.iter()
        .filter_map(|word| -> Option<u32> {
            (
                word.chars()
                    .find(|c| c.is_numeric())?
                    .to_string()
                +
                &word.chars()
                    .rfind(|c| c.is_numeric())?
                    .to_string()
            ).parse::<u32>().ok()
        })
        .sum()
}
