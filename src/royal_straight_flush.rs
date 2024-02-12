use super::{
    card::Number,
    combination::Combination,
    hand::{Hand, HandTrait},
};

pub fn royal_straight_flush(hand: &mut Hand) -> Option<Combination> {
    hand.sort_to_descending();
    let cards = &hand.cards;
    let royal_flush =
        [Number::Ace, Number::King, Number::Queen, Number::Jack, Number::Ten];
    let hand_suits: Vec<_> = hand.cards.iter().map(|card| card.suit).collect();
    if royal_flush.iter().all(|number| {
        cards.iter().map(|c| c.number).collect::<Vec<_>>().contains(number)
    }) && hand_suits.windows(5).all(|w| w.iter().all(|&s| s == w[0]))
    {
        return Some(Combination::RoyalStraightFlush(hand.clone()));
    }
    None
}
