use poker::{
    card::{Card, Number, Suit},
    combination::Combination,
    hand::{Hand, HandTrait},
};

#[test]
fn straight_flush_ace_low() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Ace, Suit::Hearts),
            Card::new(Number::Two, Suit::Hearts),
            Card::new(Number::Three, Suit::Hearts),
            Card::new(Number::Four, Suit::Hearts),
            Card::new(Number::Five, Suit::Hearts),
            Card::new(Number::Seven, Suit::Spades),
            Card::new(Number::Nine, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let mut hand_expected = Hand::new(
        [
            Card::new(Number::Five, Suit::Hearts),
            Card::new(Number::Four, Suit::Hearts),
            Card::new(Number::Three, Suit::Hearts),
            Card::new(Number::Two, Suit::Hearts),
            Card::new(Number::One, Suit::Hearts),
            Card::new(Number::Nine, Suit::Diamonds),
            Card::new(Number::Seven, Suit::Spades),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    hand_expected.sort_to_descending();
    assert_eq!(combination, Some(Combination::StraightFlush(hand_expected)));
}

#[test]
fn straight_flush_ace_high() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Ace, Suit::Hearts),
            Card::new(Number::King, Suit::Hearts),
            Card::new(Number::Queen, Suit::Hearts),
            Card::new(Number::Jack, Suit::Hearts),
            Card::new(Number::Ten, Suit::Hearts),
            Card::new(Number::Seven, Suit::Spades),
            Card::new(Number::Two, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::StraightFlush(hand)));
}

#[test]
fn straight_flush_middle() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Three, Suit::Hearts),
            Card::new(Number::Four, Suit::Hearts),
            Card::new(Number::Five, Suit::Hearts),
            Card::new(Number::Six, Suit::Hearts),
            Card::new(Number::Seven, Suit::Hearts),
            Card::new(Number::Nine, Suit::Spades),
            Card::new(Number::Jack, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::StraightFlush(hand)));
}
