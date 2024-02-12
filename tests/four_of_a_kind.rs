use poker::{
    card::{Card, Number, Suit},
    combination::Combination,
    hand::{Hand, HandTrait},
};

#[test]
fn four_of_a_kind() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Two, Suit::Hearts),
            Card::new(Number::Two, Suit::Diamonds),
            Card::new(Number::Two, Suit::Clubs),
            Card::new(Number::Two, Suit::Spades),
            Card::new(Number::Six, Suit::Hearts),
            Card::new(Number::King, Suit::Spades),
            Card::new(Number::Ace, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::FourOfAKind(hand)));
}

#[test]
fn four_of_a_kind_three_number() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Three, Suit::Hearts),
            Card::new(Number::Three, Suit::Diamonds),
            Card::new(Number::Three, Suit::Clubs),
            Card::new(Number::Three, Suit::Spades),
            Card::new(Number::Six, Suit::Hearts),
            Card::new(Number::King, Suit::Spades),
            Card::new(Number::Ace, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::FourOfAKind(hand)));
}

#[test]
fn four_of_a_kind_three_number_and_pair() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Three, Suit::Hearts),
            Card::new(Number::Three, Suit::Diamonds),
            Card::new(Number::Three, Suit::Clubs),
            Card::new(Number::Three, Suit::Spades),
            Card::new(Number::Six, Suit::Hearts),
            Card::new(Number::Six, Suit::Spades),
            Card::new(Number::Ace, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::FourOfAKind(hand)));
}

#[test]
fn four_of_a_kind_three_number_and_three_of_a_kind() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Three, Suit::Hearts),
            Card::new(Number::Three, Suit::Diamonds),
            Card::new(Number::Three, Suit::Clubs),
            Card::new(Number::Three, Suit::Spades),
            Card::new(Number::Six, Suit::Hearts),
            Card::new(Number::Six, Suit::Spades),
            Card::new(Number::Six, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::FourOfAKind(hand)));
}
