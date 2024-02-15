use darth_rust::DarthRust;
use itertools::Itertools;
use poker::{
    combination::{Combination, CombinationTrait},
    deck::{Deck, DeckTrait},
    hand::{Hand, HandTrait},
};
use std::collections::HashMap;

#[derive(Debug, DarthRust, Clone)]
struct Probability {
    percent: f64,
}

#[test]
#[ignore = "time"]
fn poker_probabilities_seven_draws() {
    let mut counts = HashMap::new();
    let trials = 3_000_000;

    for _ in 0..trials {
        let mut deck = Deck::poker();
        deck.shuffle();
        let draws = deck.draw(7);
        let mut hand = Hand::new(draws);
        let combination = hand.is_combination().expect("also combination must be some");
        *counts.entry(combination.to_str().to_string()).or_insert(0) += 1;
    }

    let mut probabilities = HashMap::new();
    for (combination, count) in counts {
        probabilities.insert(combination, count as f64 / trials as f64);
    }
    let high_card_probability = probabilities.get("High Card").unwrap_or(&0.0);
    let pair_probability = probabilities.get("One Pair").unwrap_or(&0.0);
    let two_pair_probability = probabilities.get("Two Pair").unwrap_or(&0.0);
    let three_of_a_kind_probability = probabilities.get("Three of a Kind").unwrap_or(&0.0);
    let straight_probability = probabilities.get("Straight").unwrap_or(&0.0);
    let flush_probability = probabilities.get("Flush").unwrap_or(&0.0);
    let full_house_probability = probabilities.get("Full House").unwrap_or(&0.0);
    let four_of_a_kind_probability = probabilities.get("Four of a Kind").unwrap_or(&0.0);
    let straight_flush_probability = probabilities.get("Straight Flush").unwrap_or(&0.0);
    let royal_straight_flush_probability =
        probabilities.get("Royal Straight Flush").unwrap_or(&0.0);
    let high_card_percentage = 0.501177;
    let one_pair_percentage = 0.422569;
    let two_pair_percentage = 0.047539;
    let three_of_a_kind_percentage = 0.021128;
    let straight_percentage = 0.003925;
    let flush_percentage = 0.001965;
    let full_house_percentage = 0.001441;
    let four_of_a_kind_percentage = 0.000240;
    let straight_flush_percentage = 0.000014;
    let royal_straight_flush_percentage = 0.00000154;
    let high_card_error_margin = 0.04 * high_card_probability;
    let one_pair_error_margin = 0.04 * pair_probability;
    let two_pair_error_margin = 0.04 * two_pair_probability;
    let three_of_a_kind_error_margin = 0.04 * three_of_a_kind_probability;
    let straight_error_margin = 0.04 * straight_probability;
    let flush_error_margin = 0.04 * flush_probability;
    let full_house_error_margin = 0.04 * full_house_probability;
    let four_of_a_kind_error_margin = 0.04 * four_of_a_kind_probability;
    let straight_flush_error_margin = 0.04 * straight_flush_probability;
    let royal_straight_flush_error_margin = 0.04 * royal_straight_flush_probability;
    let mut prob = Probability::new(0.00);
    let percent = 100.00;
    prob.set_percent((high_card_percentage - high_card_probability).abs() * percent);
    prob.print_err_percent("Probability of High Card");
    prob.set_percent((one_pair_percentage - pair_probability).abs() * percent);
    prob.print_err_percent("Probability of One Pair");
    prob.set_percent((two_pair_percentage - two_pair_probability).abs() * percent);
    prob.print_err_percent("Probability of Two Pair");
    prob.set_percent((three_of_a_kind_percentage - three_of_a_kind_probability).abs() * percent);
    prob.print_err_percent("Probability of Three of a Kind");
    prob.set_percent((straight_percentage - straight_probability).abs() * percent);
    prob.print_err_percent("Probability of Straight");
    prob.set_percent((flush_percentage - flush_probability).abs() * percent);
    prob.print_err_percent("Probability of Flush");
    prob.set_percent((full_house_percentage - full_house_probability).abs() * percent);
    prob.print_err_percent("Probability of Full House");
    prob.set_percent((four_of_a_kind_percentage - four_of_a_kind_probability).abs() * percent);
    prob.print_err_percent("Probability of Four of a Kind");
    prob.set_percent((straight_flush_percentage - straight_flush_probability).abs() * percent);
    prob.print_err_percent("Probability of Straight Flush");
    prob.set_percent(
        (royal_straight_flush_percentage - royal_straight_flush_probability).abs() * percent,
    );
    prob.print_err_percent("Probability of Royal Straight Flush");
    assert!((high_card_percentage - high_card_probability).abs() < high_card_error_margin);
    assert!((one_pair_percentage - pair_probability).abs() < one_pair_error_margin);
    assert!((two_pair_percentage - two_pair_probability).abs() < two_pair_error_margin);
    assert!(
        (three_of_a_kind_percentage - three_of_a_kind_probability).abs()
            < three_of_a_kind_error_margin
    );
    assert!((straight_percentage - straight_probability).abs() < straight_error_margin);
    assert!((flush_percentage - flush_probability).abs() < flush_error_margin);
    assert!((full_house_percentage - full_house_probability).abs() < full_house_error_margin);
    assert!(
        (four_of_a_kind_percentage - four_of_a_kind_probability).abs()
            < four_of_a_kind_error_margin
    );
    assert!(
        (straight_flush_percentage - straight_flush_probability).abs()
            < straight_flush_error_margin
    );
    assert!(
        (royal_straight_flush_percentage - royal_straight_flush_probability).abs()
            < royal_straight_flush_error_margin
    );
}

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

#[test]
#[ignore = "time"]
fn test_poker_hand_seven_cards_combinations() {
    let mut deck = Deck::poker();
    deck.shuffle();
    let combinations = deck.cards.iter().combinations(7);
    let mut is = Is::default();
    let is_expected = Is::new(
        32,          // Royal Straight Flush
        5_448,       // Straight Flush (excluindo Royal Straight Flush)
        224_848,     // Four of a Kind
        3_473_184,   // Full House
        4_047_644,   // Flush (excluindo Straight Flush)
        6_180_020,   // Straight (excluindo Straight Flush)
        6_461_620,   // Three of a Kind
        31_433_400,  // Two Pair
        58_627_800,  // One Pair
        23_294_460,  // High Card
        133_784_560, // Total Hand Count
    );
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
fn test_poker_hand_five_cards_combinations() {
    let mut deck = Deck::poker();
    deck.shuffle();
    let combinations = deck.cards.iter().combinations(5);
    let mut is = Is::default();
    let is_expected =
        Is::new(4, 40, 624, 3_744, 5_108, 10_200, 54_912, 123_552, 1_098_240, 1_302_540, 2_598_960);
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
