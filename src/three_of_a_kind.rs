use super::{
    combination::Combination,
    hand::{Hand, HandTrait},
};

pub fn three_of_a_kind(hand: &mut Hand) -> Option<Combination> {
    hand.sort_to_descending();
    let cards = &hand.cards;
    let mut triples = Vec::new();
    for i in 0..cards.len() - 2 {
        if cards[i].number == cards[i + 1].number && cards[i + 1].number == cards[i + 2].number {
            triples.push(cards[i]);
            triples.push(cards[i + 1]);
            triples.push(cards[i + 2]);
            break;
        }
    }
    match triples.len() {
        3 => Some(Combination::ThreeOfAKind(hand.clone())),
        6 => Some(Combination::ThreeOfAKind(hand.clone())),
        _ => None,
    }
}
