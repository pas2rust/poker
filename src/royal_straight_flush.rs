use crate::{flush::flush, straight::straight};

use super::{card::Number, combination::Combination, hand::Hand};

pub fn royal_straight_flush(hand: &mut Hand) -> Option<Combination> {
    if let Some(Combination::Straight(straight)) = straight(hand) {
        if let Some(Combination::Flush(flush)) = flush(hand) {
            if straight == flush {
                if straight.cards[0].number == Number::Ace
                    && straight.cards[1].number == Number::King
                {
                    return Some(Combination::RoyalStraightFlush(straight));
                }
            }
        }
    }
    None
}
