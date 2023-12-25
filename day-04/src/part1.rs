use std::collections::HashSet;

pub fn points_from_deck(cards: &Vec<(HashSet<u32>, Vec<u32>)>) -> u32 {
    cards.into_iter().map(
        |(winning_numbers, your_numbers)| {
            your_numbers.into_iter().fold(0, |acc, e| {
                if winning_numbers.contains(e) { 
                    if acc == 0 { 1 }
                    else { acc * 2 }
                } else {
                    acc
                }
            })
        }
    ).sum()
}
