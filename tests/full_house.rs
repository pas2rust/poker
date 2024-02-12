use poker::{
    card::{Card, Number, Suit},
    combination::Combination,
    hand::{Hand, HandTrait},
};

#[test]
#[ignore = "bug"]
fn full_house_two_and_ace() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Two, Suit::Hearts),
            Card::new(Number::Two, Suit::Diamonds),
            Card::new(Number::Two, Suit::Clubs),
            Card::new(Number::Two, Suit::Spades),
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
