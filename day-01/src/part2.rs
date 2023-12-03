
pub fn second_star(words: &Vec<String>) -> Option<u32> {
    fn word_to_char(num_string: &str) -> &str {
        match num_string {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            other => other
        }
    }
    let strs_to_find: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "1", "2", "3", "4", "5", "6", "7", "8", "9"
    ];
    
    words.iter()
        .map(|word| -> Option<u32> {
            let first_num = word_to_char(strs_to_find.iter()
                .filter_map(|to_find| {
                        Some((word.find(to_find)?, to_find))
                })
                .min()?.1
            );
            
            let second_num = word_to_char(strs_to_find.iter()
                .filter_map(|to_find| {
                    Some((word.rfind(to_find)?, to_find))
                }).max()?.1
            );
            (first_num.to_owned() + second_num).parse::<u32>().ok()
        })
        .sum()
}
