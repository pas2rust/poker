use crate::hand::HandTrait;

use super::{combination::Combination, hand::Hand};

pub fn full_house(hand: &mut Hand) -> Option<Combination> {
    hand.sort_to_descending();
    let counts = hand.count_card_numbers();
    let mut has_three_of_a_kind = false;
    let mut has_pair = false;

    for &count in counts.values() {
        if count == 3 {
            has_three_of_a_kind = true;
        } else if count == 2 {
            has_pair = true;
        }
    }

    if has_three_of_a_kind && has_pair {
        Some(Combination::FullHouse(hand.clone()))
    } else {
        None
    }
}
