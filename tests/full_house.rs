use poker::{
    card::{Card, Number, Suit},
    combination::Combination,
    hand::{Hand, HandTrait},
};

#[test]
fn full_house_seven_cards_two_and_ace() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Two, Suit::Hearts),
            Card::new(Number::Two, Suit::Diamonds),
            Card::new(Number::Two, Suit::Clubs),
            Card::new(Number::Three, Suit::Spades),
            Card::new(Number::Six, Suit::Hearts),
            Card::new(Number::Ace, Suit::Spades),
            Card::new(Number::Ace, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::FullHouse(hand)));
}

#[test]
fn full_house_seven_cards_three_and_king() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Three, Suit::Hearts),
            Card::new(Number::Three, Suit::Diamonds),
            Card::new(Number::Three, Suit::Clubs),
            Card::new(Number::King, Suit::Spades),
            Card::new(Number::King, Suit::Hearts),
            Card::new(Number::Six, Suit::Spades),
            Card::new(Number::Nine, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::FullHouse(hand)));
}

#[test]
fn full_house_seven_cards_five_and_queen() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Five, Suit::Hearts),
            Card::new(Number::Five, Suit::Diamonds),
            Card::new(Number::Five, Suit::Clubs),
            Card::new(Number::Queen, Suit::Spades),
            Card::new(Number::Queen, Suit::Hearts),
            Card::new(Number::Two, Suit::Spades),
            Card::new(Number::Eight, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::FullHouse(hand)));
}

#[test]
fn full_house_five_cards_two_and_ace() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Two, Suit::Hearts),
            Card::new(Number::Two, Suit::Diamonds),
            Card::new(Number::Two, Suit::Clubs),
            Card::new(Number::Ace, Suit::Spades),
            Card::new(Number::Ace, Suit::Hearts),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::FullHouse(hand)));
}

#[test]
fn full_house_five_cards_three_and_king() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Three, Suit::Hearts),
            Card::new(Number::Three, Suit::Diamonds),
            Card::new(Number::Three, Suit::Clubs),
            Card::new(Number::King, Suit::Spades),
            Card::new(Number::King, Suit::Hearts),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::FullHouse(hand)));
}

#[test]
fn full_house_five_cards_five_and_queen() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Five, Suit::Hearts),
            Card::new(Number::Five, Suit::Diamonds),
            Card::new(Number::Five, Suit::Clubs),
            Card::new(Number::Queen, Suit::Spades),
            Card::new(Number::Queen, Suit::Hearts),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::FullHouse(hand)));
}
