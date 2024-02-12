use crate::{flush::flush, straight::straight};

use super::{combination::Combination, hand::Hand};

pub fn straight_flush(hand: &mut Hand) -> Option<Combination> {
    if let Some(Combination::Straight(straight)) = straight(hand) {
        if let Some(Combination::Flush(flush)) = flush(hand) {
            if straight == flush {
                return Some(Combination::StraightFlush(straight));
            }
        }
    }
    None
}
