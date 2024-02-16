use super::{
    combination::Combination,
    hand::{Hand, HandTrait},
};

pub fn pairs(hand: &mut Hand) -> Option<Combination> {
    hand.sort_to_descending();
    let counts = hand.count_card_numbers();
    let mut pair_values = Vec::new();

    for (&number, &count) in counts.iter() {
        if count == 2 {
            pair_values.push(number);
        }
    }

    match pair_values.len() {
        1 => Some(Combination::OnePair(hand.clone())),
        2 => Some(Combination::TwoPair(hand.clone())),
        3 => {
            if pair_values[0] != pair_values[1] && pair_values[1] != pair_values[2] {
                Some(Combination::TwoPair(hand.clone()))
            } else {
                None
            }
        }
        _ => None,
    }
}
