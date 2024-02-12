use super::{
    combination::Combination,
    hand::{Hand, HandTrait},
};

pub fn high_card(hand: &mut Hand) -> Option<Combination> {
    hand.sort_to_descending();
    Some(Combination::HighCard(hand.clone()))
}
