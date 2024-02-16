use poker::{
    card::{Card, Number, Suit},
    deck::{Deck, DeckTrait},
};

#[test]
fn poker_deck_size() {
    let deck = Deck::poker();
    assert_eq!(deck.cards.len(), 52)
}

#[test]
fn test_poker_deck_contents() {
    let deck = Deck::poker();
    let static_deck = vec![
        Card::new(Number::Ace, Suit::Clubs),
        Card::new(Number::Two, Suit::Clubs),
        Card::new(Number::Three, Suit::Clubs),
        Card::new(Number::Four, Suit::Clubs),
        Card::new(Number::Five, Suit::Clubs),
        Card::new(Number::Six, Suit::Clubs),
        Card::new(Number::Seven, Suit::Clubs),
        Card::new(Number::Eight, Suit::Clubs),
        Card::new(Number::Nine, Suit::Clubs),
        Card::new(Number::Ten, Suit::Clubs),
        Card::new(Number::Jack, Suit::Clubs),
        Card::new(Number::Queen, Suit::Clubs),
        Card::new(Number::King, Suit::Clubs),
        Card::new(Number::Ace, Suit::Diamonds),
        Card::new(Number::Two, Suit::Diamonds),
        Card::new(Number::Three, Suit::Diamonds),
        Card::new(Number::Four, Suit::Diamonds),
        Card::new(Number::Five, Suit::Diamonds),
        Card::new(Number::Six, Suit::Diamonds),
        Card::new(Number::Seven, Suit::Diamonds),
        Card::new(Number::Eight, Suit::Diamonds),
        Card::new(Number::Nine, Suit::Diamonds),
        Card::new(Number::Ten, Suit::Diamonds),
        Card::new(Number::Jack, Suit::Diamonds),
        Card::new(Number::Queen, Suit::Diamonds),
        Card::new(Number::King, Suit::Diamonds),
        Card::new(Number::Ace, Suit::Hearts),
        Card::new(Number::Two, Suit::Hearts),
        Card::new(Number::Three, Suit::Hearts),
        Card::new(Number::Four, Suit::Hearts),
        Card::new(Number::Five, Suit::Hearts),
        Card::new(Number::Six, Suit::Hearts),
        Card::new(Number::Seven, Suit::Hearts),
        Card::new(Number::Eight, Suit::Hearts),
        Card::new(Number::Nine, Suit::Hearts),
        Card::new(Number::Ten, Suit::Hearts),
        Card::new(Number::Jack, Suit::Hearts),
        Card::new(Number::Queen, Suit::Hearts),
        Card::new(Number::King, Suit::Hearts),
        Card::new(Number::Ace, Suit::Spades),
        Card::new(Number::Two, Suit::Spades),
        Card::new(Number::Three, Suit::Spades),
        Card::new(Number::Four, Suit::Spades),
        Card::new(Number::Five, Suit::Spades),
        Card::new(Number::Six, Suit::Spades),
        Card::new(Number::Seven, Suit::Spades),
        Card::new(Number::Eight, Suit::Spades),
        Card::new(Number::Nine, Suit::Spades),
        Card::new(Number::Ten, Suit::Spades),
        Card::new(Number::Jack, Suit::Spades),
        Card::new(Number::Queen, Suit::Spades),
        Card::new(Number::King, Suit::Spades),
    ];

    for card in static_deck {
        assert!(deck.cards.contains(&card));
    }
}
