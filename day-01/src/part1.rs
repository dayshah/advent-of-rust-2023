
pub fn first_star(words: &Vec<String>) -> Option<u32> {
    words.iter()
        .map(|word| -> Option<u32> {
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
