use poker::{
    card::{Card, Number, Suit},
    combination::Combination,
    hand::{Hand, HandTrait},
};

#[test]
fn flush_hearts_five_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Ace, Suit::Hearts),
            Card::new(Number::Two, Suit::Hearts),
            Card::new(Number::Nine, Suit::Hearts),
            Card::new(Number::Jack, Suit::Hearts),
            Card::new(Number::King, Suit::Hearts),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    assert_eq!(combination, Some(Combination::Flush(hand)));
}

#[test]
fn flush_spades_five_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Three, Suit::Spades),
            Card::new(Number::Five, Suit::Spades),
            Card::new(Number::Seven, Suit::Spades),
            Card::new(Number::Nine, Suit::Spades),
            Card::new(Number::Queen, Suit::Spades),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    assert_eq!(combination, Some(Combination::Flush(hand)));
}

#[test]
fn flush_clubs_five_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Three, Suit::Clubs),
            Card::new(Number::Five, Suit::Clubs),
            Card::new(Number::Seven, Suit::Clubs),
            Card::new(Number::Nine, Suit::Clubs),
            Card::new(Number::Queen, Suit::Clubs),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    assert_eq!(combination, Some(Combination::Flush(hand)));
}

#[test]
fn flush_diamonds_five_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Three, Suit::Diamonds),
            Card::new(Number::Five, Suit::Diamonds),
            Card::new(Number::Seven, Suit::Diamonds),
            Card::new(Number::Nine, Suit::Diamonds),
            Card::new(Number::Queen, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    assert_eq!(combination, Some(Combination::Flush(hand)));
}

#[test]
fn flush_spades_seven_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Three, Suit::Spades),
            Card::new(Number::Five, Suit::Spades),
            Card::new(Number::Seven, Suit::Spades),
            Card::new(Number::Nine, Suit::Spades),
            Card::new(Number::Queen, Suit::Spades),
            Card::new(Number::Three, Suit::Diamonds),
            Card::new(Number::Three, Suit::Hearts),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    assert_eq!(combination, Some(Combination::Flush(hand)));
}

#[test]
fn flush_clubs_seven_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Three, Suit::Clubs),
            Card::new(Number::Five, Suit::Clubs),
            Card::new(Number::Seven, Suit::Clubs),
            Card::new(Number::Nine, Suit::Clubs),
            Card::new(Number::Queen, Suit::Clubs),
            Card::new(Number::Three, Suit::Diamonds),
            Card::new(Number::Three, Suit::Hearts),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    assert_eq!(combination, Some(Combination::Flush(hand)));
}

#[test]
fn flush_diamonds_seven_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Three, Suit::Diamonds),
            Card::new(Number::Five, Suit::Diamonds),
            Card::new(Number::Seven, Suit::Diamonds),
            Card::new(Number::Nine, Suit::Diamonds),
            Card::new(Number::Queen, Suit::Diamonds),
            Card::new(Number::Three, Suit::Diamonds),
            Card::new(Number::Three, Suit::Hearts),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    assert_eq!(combination, Some(Combination::Flush(hand)));
}
