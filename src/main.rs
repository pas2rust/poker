pub mod brain;
pub mod card;
pub mod combination;
pub mod deck;
pub mod flush;
pub mod four_of_a_kind;
pub mod full_house;
pub mod hand;
pub mod high_card;
pub mod pairs;
pub mod royal_straight_flush;
pub mod straight;
pub mod straight_flush;
pub mod table;
pub mod three_of_a_kind;

use darth_rust::DarthRust;
use poker::{
    combination::CombinationTrait,
    deck::{Deck, DeckTrait},
    hand::{Hand, HandTrait},
};
use std::collections::HashMap;

#[derive(Debug, DarthRust, Clone)]
struct Probability {
    percent: f64,
}

fn main() {
    let mut counts = HashMap::new();
    let trials = 1_000_000;

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
    let mut prob = Probability::new(0.00);
    let percent = 100.00;
    prob.set_percent((0.438 - pair_probability).abs() * percent);
    prob.print_rust_percent("Probability of One Pair");
    prob.set_percent((0.235 - two_pair_probability).abs() * percent);
    prob.print_rust_percent("Probability of Two Pair");
    prob.set_percent((0.0483 - three_of_a_kind_probability).abs() * percent);
    prob.print_rust_percent("Probability of Three of a Kind");
    prob.set_percent((0.000979 - straight_probability).abs() * percent);
    prob.print_rust_percent("Probability of Straight");
    prob.set_percent((0.0303 - flush_probability).abs() * percent);
    prob.print_rust_percent("Probability of Flush");
    prob.set_percent((0.0256 - full_house_probability).abs() * percent);
    prob.print_rust_percent("Probability of Full House");
    prob.set_percent((0.001680 - four_of_a_kind_probability).abs() * percent);
    prob.print_rust_percent("Probability of Four of a Kind");
    prob.set_percent((0.000279 - straight_flush_probability).abs() * percent);
    prob.print_rust_percent("Probability of Straight Flush");
    prob.set_percent((0.000032 - royal_straight_flush_probability).abs() * percent);
    prob.print_rust_percent("Probability of Royal Straight Flush");
}
