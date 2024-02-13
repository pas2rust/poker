use super::{
    combination::Combination,
    hand::{Hand, HandTrait},
};

pub fn pairs(hand: &mut Hand) -> Option<Combination> {
    hand.sort_to_descending();
    let cards = &hand.cards;
    let mut pairs = Vec::new();
    let mut count = 0;
    for i in 0..cards.len() - 1 {
        if cards[i].number == cards[i + 1].number {
            pairs.push(cards[i].clone());
            pairs.push(cards[i + 1].clone());
            count += 1;
            if count == 2 {
                break;
            }
        }
    }
    match pairs.len() {
        2 => Some(Combination::OnePair(hand.clone())),
        4 => {
            if pairs[0].number != pairs[3].number {
                Some(Combination::TwoPair(hand.clone()))
            } else {
                None
            }
        }
        6 => {
            if pairs[0].number != pairs[2].number && pairs[2].number != pairs[4].number {
                Some(Combination::TwoPair(hand.clone()))
            } else {
                None
            }
        }
        _ => None,
    }
}
