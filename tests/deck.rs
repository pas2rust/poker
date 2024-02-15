use poker::deck::{Deck, DeckTrait};

#[test]
fn poker_deck_size() {
    let deck = Deck::poker();
    assert_eq!(deck.cards.len(), 52)
}
