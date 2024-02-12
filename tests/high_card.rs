use poker::{
    card::{Card, Number, Suit},
    combination::Combination,
    hand::{Hand, HandTrait},
};

#[test]
fn high_card_ace() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Jack, Suit::Hearts),
            Card::new(Number::Three, Suit::Diamonds),
            Card::new(Number::Four, Suit::Clubs),
            Card::new(Number::Five, Suit::Spades),
            Card::new(Number::Seven, Suit::Hearts),
            Card::new(Number::King, Suit::Spades),
            Card::new(Number::Ace, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::HighCard(hand)));
}

#[test]
fn high_card_king() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Two, Suit::Hearts),
            Card::new(Number::Three, Suit::Diamonds),
            Card::new(Number::Four, Suit::Clubs),
            Card::new(Number::Five, Suit::Spades),
            Card::new(Number::Seven, Suit::Hearts),
            Card::new(Number::King, Suit::Spades),
            Card::new(Number::Eight, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::HighCard(hand)));
}

#[test]
fn high_card_queen() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Two, Suit::Hearts),
            Card::new(Number::Three, Suit::Diamonds),
            Card::new(Number::Four, Suit::Clubs),
            Card::new(Number::Five, Suit::Spades),
            Card::new(Number::Seven, Suit::Hearts),
            Card::new(Number::Queen, Suit::Spades),
            Card::new(Number::Eight, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::HighCard(hand)));
}

#[test]
fn high_card_jack() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Two, Suit::Hearts),
            Card::new(Number::Three, Suit::Diamonds),
            Card::new(Number::Four, Suit::Clubs),
            Card::new(Number::Five, Suit::Spades),
            Card::new(Number::Eight, Suit::Hearts),
            Card::new(Number::Seven, Suit::Spades),
            Card::new(Number::Jack, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::HighCard(hand)));
}

#[test]
fn high_card_ten() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Ten, Suit::Hearts),
            Card::new(Number::Three, Suit::Diamonds),
            Card::new(Number::Four, Suit::Clubs),
            Card::new(Number::Two, Suit::Spades),
            Card::new(Number::Six, Suit::Hearts),
            Card::new(Number::Seven, Suit::Spades),
            Card::new(Number::Nine, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::HighCard(hand)));
}
