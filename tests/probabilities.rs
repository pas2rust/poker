use std::collections::HashMap;

use poker::{
    combination::CombinationTrait,
    deck::{Deck, DeckTrait},
    hand::{Hand, HandTrait},
};

#[test]
fn poker_probabilities_seven_draws() {
    let mut counts = HashMap::new();
    let trials = 100_000;

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
    assert!((0.438 - pair_probability).abs() < 0.04);
    assert!((0.235 - two_pair_probability).abs() < 0.04);
    assert!((0.0483 - three_of_a_kind_probability).abs() < 0.04);
    assert!((0.000979 - straight_probability).abs() < 0.04);
    assert!((0.0303 - flush_probability).abs() < 0.04);
    assert!((0.0256 - full_house_probability).abs() < 0.04);
    assert!((0.001680 - four_of_a_kind_probability).abs() < 0.04);
    assert!((0.000279 - straight_flush_probability).abs() < 0.04);
    assert!((0.000032 - royal_straight_flush_probability).abs() < 0.04);
}
