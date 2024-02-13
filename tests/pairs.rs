use poker::{
    card::{Card, Number, Suit},
    combination::Combination,
    hand::{Hand, HandTrait},
};

#[test]
fn two_pair_seven_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Two, Suit::Hearts),
            Card::new(Number::Two, Suit::Diamonds),
            Card::new(Number::Three, Suit::Clubs),
            Card::new(Number::Four, Suit::Spades),
            Card::new(Number::Four, Suit::Hearts),
            Card::new(Number::King, Suit::Spades),
            Card::new(Number::Ace, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::TwoPair(hand)));
}

#[test]
fn three_pair_seven_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Two, Suit::Hearts),
            Card::new(Number::Two, Suit::Diamonds),
            Card::new(Number::Three, Suit::Clubs),
            Card::new(Number::Three, Suit::Spades),
            Card::new(Number::Four, Suit::Spades),
            Card::new(Number::Four, Suit::Hearts),
            Card::new(Number::Ace, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::TwoPair(hand)));
}

#[test]
fn one_pair_seven_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Two, Suit::Hearts),
            Card::new(Number::Two, Suit::Diamonds),
            Card::new(Number::Three, Suit::Clubs),
            Card::new(Number::Five, Suit::Spades),
            Card::new(Number::Six, Suit::Hearts),
            Card::new(Number::King, Suit::Spades),
            Card::new(Number::Ace, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::OnePair(hand)));
}

#[test]
fn two_pair_five_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Two, Suit::Hearts),
            Card::new(Number::Two, Suit::Diamonds),
            Card::new(Number::Three, Suit::Clubs),
            Card::new(Number::Four, Suit::Spades),
            Card::new(Number::Four, Suit::Hearts),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::TwoPair(hand)));
}

#[test]
fn one_pair_five_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Two, Suit::Hearts),
            Card::new(Number::Two, Suit::Diamonds),
            Card::new(Number::Three, Suit::Clubs),
            Card::new(Number::Five, Suit::Spades),
            Card::new(Number::Six, Suit::Hearts),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::OnePair(hand)));
}
