use super::{
    combination::Combination,
    hand::{Hand, HandTrait},
};

pub fn three_of_a_kind(hand: &mut Hand) -> Option<Combination> {
    hand.sort_to_descending();
    let counts = hand.count_card_numbers();
    let mut three_values = Vec::new();

    for (&number, &count) in counts.iter() {
        if count == 3 {
            three_values.push(number);
        }
    }

    match three_values.len() {
        1 => Some(Combination::ThreeOfAKind(hand.clone())),
        _ => None,
    }
}
