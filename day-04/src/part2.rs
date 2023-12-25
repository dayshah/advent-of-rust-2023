use std::collections::{HashSet, HashMap};

pub fn number_of_scratchcards(cards: &Vec<(HashSet<u32>, Vec<u32>)>) -> u32 {
    let mut result = 0;
    let mut card_quantity: HashMap<usize, u32> = HashMap::new();
    for (i, (winning_numbers, your_numbers)) in cards.iter().enumerate() {
        card_quantity.entry(i).and_modify(|e| { *e+=1; }).or_insert(1);
        let this_card_quantity = card_quantity.get(&i).copied().unwrap_or_default();
        result += this_card_quantity;
        let mut next_card = i + 1;
        for your_num in your_numbers {
            if winning_numbers.contains(your_num) {
                card_quantity.entry(next_card)
                    .and_modify(|e| { *e+=this_card_quantity; })
                    .or_insert(this_card_quantity);
                next_card += 1;
            }
        }
    }
    result
    //card_quantity.into_iter().fold(0,|acc, (_card, quantity)| { acc + quantity })
}
