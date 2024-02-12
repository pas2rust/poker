use super::{
    combination::Combination,
    hand::{Hand, HandTrait},
};

pub fn four_of_a_kind(hand: &mut Hand) -> Option<Combination> {
    hand.sort_to_descending();
    let cards = &hand.cards;
    let mut quatrain = Vec::new();
    for i in 0..cards.len() - 3 {
        if cards[i].number == cards[i + 1].number
            && cards[i + 1].number == cards[i + 2].number
            && cards[i + 2].number == cards[i + 3].number
        {
            quatrain.push(cards[i]);
            quatrain.push(cards[i + 1]);
            quatrain.push(cards[i + 2]);
            quatrain.push(cards[i + 3]);
            break;
        }
    }
    match quatrain.len() {
        4 => Some(Combination::FourOfAKind(hand.clone())),
        _ => None,
    }
}
