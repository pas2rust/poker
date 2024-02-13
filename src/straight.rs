use super::{
    card::{Card, Number},
    combination::Combination,
    hand::{Hand, HandTrait},
};

pub fn straight(hand: &mut Hand) -> Option<Combination> {
    hand.sort_to_descending();
    let ace_low = vec![Number::Five, Number::Four, Number::Three, Number::Two, Number::Ace];
    let hand_numbers: Vec<_> = hand.cards.iter().map(|card| card.number).collect();
    if ace_low.iter().all(|n| hand_numbers.contains(n)) {
        let mut ace = *hand
            .cards
            .to_vec()
            .iter()
            .find(|&&card| card.number == Number::Ace)
            .expect("Ace must be provided");
        ace.set_number(Number::One);
        let mut cards_vec: Vec<Card> = hand.cards.iter().cloned().collect();
        cards_vec.retain(|&card| card.number != Number::Ace);
        cards_vec.push(ace);
        hand.cards = cards_vec.try_into().expect("Failed to convert Vec to array");
        hand.sort_to_descending();
        return Some(Combination::Straight(hand.clone()));
    }
    for window in hand_numbers.windows(5) {
        if is_consecutive(window) {
            return Some(Combination::Straight(hand.clone()));
        }
    }
    None
}

pub fn is_consecutive(window: &[Number]) -> bool {
    for i in 0..(window.len() - 1) {
        if window[i] as u8 != window[i + 1] as u8 + 1 {
            return false;
        }
    }
    true
}
