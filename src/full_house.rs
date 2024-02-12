use crate::{pairs::pairs, three_of_a_kind::three_of_a_kind};

use super::{combination::Combination, hand::Hand};

pub fn full_house(hand: &mut Hand) -> Option<Combination> {
    if let Some(_) = three_of_a_kind(hand) {
        if let Some(_) = pairs(hand) {
            return Some(Combination::FullHouse(hand.clone()));
        }
    }
    None
}
