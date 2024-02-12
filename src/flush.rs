use super::{
    card::Card,
    combination::Combination,
    hand::{Hand, HandTrait},
};

pub fn flush(hand: &mut Hand) -> Option<Combination> {
    hand.sort_to_descending();
    let cards = &hand.cards;
    let mut suited_cards: Vec<Card> = Vec::new();

    for card in cards {
        if suited_cards.is_empty() || card.suit == suited_cards[0].suit {
            suited_cards.push(*card);
        } else if suited_cards.len() < 5 {
            suited_cards.clear();
            suited_cards.push(*card);
        }
    }

    if suited_cards.len() >= 5 {
        Some(Combination::Flush(hand.clone()))
    } else {
        None
    }
}
