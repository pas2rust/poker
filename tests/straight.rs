use poker::{
    card::{Card, Number, Suit},
    combination::Combination,
    hand::{Hand, HandTrait},
};

#[test]
fn straight_ace_low_seven_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Ace, Suit::Hearts),
            Card::new(Number::Two, Suit::Diamonds),
            Card::new(Number::Three, Suit::Clubs),
            Card::new(Number::Four, Suit::Spades),
            Card::new(Number::Five, Suit::Hearts),
            Card::new(Number::Seven, Suit::Spades),
            Card::new(Number::Nine, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let mut hand_expected = Hand::new(
        [
            Card::new(Number::Five, Suit::Hearts),
            Card::new(Number::Four, Suit::Spades),
            Card::new(Number::Three, Suit::Clubs),
            Card::new(Number::Two, Suit::Diamonds),
            Card::new(Number::One, Suit::Hearts),
            Card::new(Number::Nine, Suit::Diamonds),
            Card::new(Number::Seven, Suit::Spades),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    hand_expected.sort_to_descending();
    assert_eq!(combination, Some(Combination::Straight(hand_expected)));
}

#[test]
fn straight_ace_high_seven_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Ace, Suit::Hearts),
            Card::new(Number::King, Suit::Diamonds),
            Card::new(Number::Queen, Suit::Clubs),
            Card::new(Number::Jack, Suit::Spades),
            Card::new(Number::Ten, Suit::Hearts),
            Card::new(Number::Seven, Suit::Spades),
            Card::new(Number::Two, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::Straight(hand)));
}

#[test]
fn straight_middle_seven_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Three, Suit::Hearts),
            Card::new(Number::Four, Suit::Diamonds),
            Card::new(Number::Five, Suit::Clubs),
            Card::new(Number::Six, Suit::Spades),
            Card::new(Number::Seven, Suit::Hearts),
            Card::new(Number::Nine, Suit::Spades),
            Card::new(Number::Jack, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::Straight(hand)));
}

#[test]
fn no_straight_seven_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Ace, Suit::Hearts),
            Card::new(Number::King, Suit::Diamonds),
            Card::new(Number::Queen, Suit::Clubs),
            Card::new(Number::Nine, Suit::Spades),
            Card::new(Number::Ten, Suit::Hearts),
            Card::new(Number::Seven, Suit::Spades),
            Card::new(Number::Two, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::HighCard(hand)));
}

#[test]
fn straight_middle_five_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Three, Suit::Hearts),
            Card::new(Number::Four, Suit::Diamonds),
            Card::new(Number::Five, Suit::Clubs),
            Card::new(Number::Six, Suit::Spades),
            Card::new(Number::Seven, Suit::Hearts),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::Straight(hand)));
}

#[test]
fn straight_ace_low_five_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Ace, Suit::Hearts),
            Card::new(Number::Two, Suit::Diamonds),
            Card::new(Number::Three, Suit::Clubs),
            Card::new(Number::Four, Suit::Spades),
            Card::new(Number::Five, Suit::Hearts),
        ]
        .to_vec(),
    );
    let mut hand_expected = Hand::new(
        [
            Card::new(Number::Five, Suit::Hearts),
            Card::new(Number::Four, Suit::Spades),
            Card::new(Number::Three, Suit::Clubs),
            Card::new(Number::Two, Suit::Diamonds),
            Card::new(Number::One, Suit::Hearts),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    hand_expected.sort_to_descending();
    assert_eq!(combination, Some(Combination::Straight(hand_expected)));
}

#[test]
fn straight_ace_high_five_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Ace, Suit::Hearts),
            Card::new(Number::King, Suit::Diamonds),
            Card::new(Number::Queen, Suit::Clubs),
            Card::new(Number::Jack, Suit::Spades),
            Card::new(Number::Ten, Suit::Hearts),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::Straight(hand)));
}
