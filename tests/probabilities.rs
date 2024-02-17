use darth_rust::DarthRust;
use itertools::Itertools;
use poker::{
    combination::Combination,
    deck::{Deck, DeckTrait},
    hand::{Hand, HandTrait},
};

#[derive(Debug, DarthRust, Clone, PartialEq)]
pub struct Is {
    royal_straight_flush_count: i32,
    straight_flush_count: i32,
    four_of_a_kind_count: i32,
    full_house_count: i32,
    flush_count: i32,
    straight_count: i32,
    three_of_a_kind_count: i32,
    two_pair_count: i32,
    one_pair_count: i32,
    high_card_count: i32,
    hand_count: i32,
}

fn combinations(range: usize, is_expected: Is) {
    let deck = Deck::poker();
    let combinations = deck.cards.iter().combinations(range);
    let mut is = Is::default();
    for cards in combinations {
        let mut hand = Hand::new(cards.iter().map(|card| (*card).clone()).collect());
        is.sum_hand_count(1);
        match hand.is_combination() {
            Some(combination) => match combination {
                Combination::HighCard(_) => is.sum_high_card_count(1),
                Combination::OnePair(_) => is.sum_one_pair_count(1),
                Combination::TwoPair(_) => is.sum_two_pair_count(1),
                Combination::ThreeOfAKind(_) => is.sum_three_of_a_kind_count(1),
                Combination::Straight(_) => is.sum_straight_count(1),
                Combination::Flush(_) => is.sum_flush_count(1),
                Combination::FullHouse(_) => is.sum_full_house_count(1),
                Combination::FourOfAKind(_) => is.sum_four_of_a_kind_count(1),
                Combination::StraightFlush(_) => is.sum_straight_flush_count(1),
                Combination::RoyalStraightFlush(_) => is.sum_royal_straight_flush_count(1),
            },
            None => {
                hand.print_err_cards(
                    "These cards resulted in an unexpected error in 'is_combination'",
                );
                panic!("Unexpected 'None' result encountered");
            }
        }
    }
    is.print_err("Results");
    assert_eq!(is, is_expected);
}

#[test]
#[ignore = "time"]
fn test_poker_hand_seven_cards_combinations() {
    let is_expected = Is::new(
        4_324,
        37_260,
        224_848,
        3_473_184,
        4_047_644,
        6_180_020,
        6_461_620,
        31_433_400,
        58_627_800,
        23_294_460,
        133_784_560,
    );
    combinations(7, is_expected)
}

#[test]
#[ignore = "time"]
fn test_poker_hand_six_cards_combinations() {
    let is_expected = Is::new(
        620, 5010, 14664, 164736, 202000, 315906, 732160, 2532816, 9776460, 6614148, 20358520,
    );
    combinations(6, is_expected)
}

#[test]
fn test_poker_hand_five_cards_combinations() {
    let is_expected =
        Is::new(4, 36, 624, 3_744, 5_108, 10_200, 54_912, 123_552, 1_098_240, 1_302_540, 2_598_960);
    combinations(5, is_expected)
}

#[test]
fn test_poker_hand_four_cards_combinations() {
    let is_expected = Is::new(0, 0, 13, 0, 0, 0, 2_496, 2_808, 82_368, 183_040, 270_725);
    combinations(4, is_expected)
}

#[test]
fn test_poker_hand_three_cards_combinations() {
    let is_expected = Is::new(0, 0, 0, 0, 0, 0, 52, 0, 3_744, 18_304, 22_100);
    combinations(3, is_expected)
}

#[test]
fn test_poker_hand_two_cards_combinations() {
    let is_expected = Is::new(0, 0, 0, 0, 0, 0, 0, 0, 78, 1_248, 1_326);
    combinations(2, is_expected)
}
