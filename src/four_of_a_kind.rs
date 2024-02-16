use super::{
    combination::Combination,
    hand::{Hand, HandTrait},
};

pub fn four_of_a_kind(hand: &mut Hand) -> Option<Combination> {
    hand.sort_to_descending();
    let counts = hand.count_card_numbers();

    for (_, &count) in counts.iter() {
        if count == 4 {
            return Some(Combination::FourOfAKind(hand.clone()));
        }
    }

    None
}
