use poker::{
    card::{Card, Number, Suit},
    combination::Combination,
    hand::{Hand, HandTrait},
};

#[test]
fn royal_straight_flush_hearts_seven_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Ten, Suit::Hearts),
            Card::new(Number::Jack, Suit::Hearts),
            Card::new(Number::Queen, Suit::Hearts),
            Card::new(Number::King, Suit::Hearts),
            Card::new(Number::Ace, Suit::Hearts),
            Card::new(Number::Two, Suit::Spades),
            Card::new(Number::Three, Suit::Spades),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::RoyalStraightFlush(hand)));
}

#[test]
fn royal_straight_flush_hearts_five_cards() {
    let mut hand = Hand::new(
        [
            Card::new(Number::Ten, Suit::Hearts),
            Card::new(Number::Jack, Suit::Hearts),
            Card::new(Number::Queen, Suit::Hearts),
            Card::new(Number::King, Suit::Hearts),
            Card::new(Number::Ace, Suit::Hearts),
        ]
        .to_vec(),
    );
    let combination = hand.is_combination();
    hand.sort_to_descending();
    assert_eq!(combination, Some(Combination::RoyalStraightFlush(hand)));
}
