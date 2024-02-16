use super::{
    combination::Combination,
    hand::{Hand, HandTrait},
};

pub fn flush(hand: &mut Hand) -> Option<Combination> {
    hand.sort_to_descending();
    let counts = hand.count_card_suits();

    for (_, &count) in counts.iter() {
        if count >= 5 {
            return Some(Combination::Flush(hand.clone()));
        }
    }

    None
}
