use crate::{card::Suit, combination::Combination};

use super::{
    card::{Card, Number},
    flush::flush,
    four_of_a_kind::four_of_a_kind,
    full_house::full_house,
    high_card::high_card,
    pairs::pairs,
    royal_straight_flush::royal_straight_flush,
    straight::straight,
    straight_flush::straight_flush,
    three_of_a_kind::three_of_a_kind,
};
use darth_rust::DarthRust;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, DarthRust, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Hand {
    pub cards: Vec<Card>,
}

pub trait HandTrait {
    fn sort_to_descending(&mut self);
    fn sort_to_ascending(&mut self);
    fn count_card_numbers(&mut self) -> HashMap<Number, i32>;
    fn is_combination(&mut self) -> Option<Combination>;
    fn count_card_suits(&mut self) -> HashMap<Suit, i32>;
}

impl HandTrait for Hand {
    fn sort_to_descending(&mut self) {
        self.cards.sort_by(|a, b| b.number.value().cmp(&a.number.value()));
    }
    fn sort_to_ascending(&mut self) {
        self.cards.sort_by(|a, b| a.number.cmp(&b.number))
    }
    fn count_card_numbers(&mut self) -> HashMap<Number, i32> {
        let mut counts = HashMap::new();
        for &card in &self.cards {
            *counts.entry(card.number).or_insert(0) += 1;
        }
        counts
    }
    fn count_card_suits(&mut self) -> HashMap<Suit, i32> {
        let mut counts = HashMap::new();
        for &card in &self.cards {
            *counts.entry(card.suit).or_insert(0) += 1;
        }
        counts
    }
    fn is_combination(&mut self) -> Option<Combination> {
        [
            royal_straight_flush(self),
            straight_flush(self),
            four_of_a_kind(self),
            full_house(self),
            flush(self),
            straight(self),
            three_of_a_kind(self),
            pairs(self),
            high_card(self),
        ]
        .iter()
        .find_map(|r| r.clone())
    }
}
