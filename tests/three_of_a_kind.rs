use poker::{
    card::{Card, Number, Suit},
    combination::Combination,
    hand::{Hand, HandTrait},
};

#[test]
fn three_of_a_kind() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Two, Suit::Hearts),
            Card::new(Number::Two, Suit::Diamonds),
            Card::new(Number::Two, Suit::Clubs),
            Card::new(Number::Five, Suit::Spades),
            Card::new(Number::Six, Suit::Hearts),
            Card::new(Number::King, Suit::Spades),
            Card::new(Number::Ace, Suit::Diamonds),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::ThreeOfAKind(hand)));
}
