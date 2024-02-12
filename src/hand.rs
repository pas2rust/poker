use crate::combination::Combination;

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
use std::collections::HashMap;
#[derive(Clone, Debug, DarthRust, Default, PartialEq, Eq, Hash)]
pub struct Hand {
    pub cards: Vec<Card>,
}

pub trait HandTrait {
    fn sort_to_descending(&mut self);
    fn sort_to_ascending(&mut self);
    fn count_card_numbers(&mut self) -> HashMap<Number, i32>;
    fn is_high_card(&mut self) -> Option<Combination>;
    fn is_flush(&mut self) -> Option<Combination>;
    fn is_four_of_a_kind(&mut self) -> Option<Combination>;
    fn is_full_house(&mut self) -> Option<Combination>;
    fn is_pairs(&mut self) -> Option<Combination>;
    fn is_royal_straight_flush(&mut self) -> Option<Combination>;
    fn is_straight(&mut self) -> Option<Combination>;
    fn is_straight_flush(&mut self) -> Option<Combination>;
    fn is_three_of_a_kind(&mut self) -> Option<Combination>;
    fn is_combination(&mut self) -> Option<Combination>;
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
    fn is_high_card(&mut self) -> Option<Combination> {
        high_card(self)
    }
    fn is_flush(&mut self) -> Option<Combination> {
        flush(self)
    }
    fn is_four_of_a_kind(&mut self) -> Option<Combination> {
        four_of_a_kind(self)
    }
    fn is_full_house(&mut self) -> Option<Combination> {
        full_house(self)
    }
    fn is_pairs(&mut self) -> Option<Combination> {
        pairs(self)
    }
    fn is_royal_straight_flush(&mut self) -> Option<Combination> {
        royal_straight_flush(self)
    }
    fn is_straight(&mut self) -> Option<Combination> {
        straight(self)
    }
    fn is_straight_flush(&mut self) -> Option<Combination> {
        straight_flush(self)
    }
    fn is_three_of_a_kind(&mut self) -> Option<Combination> {
        three_of_a_kind(self)
    }
    fn is_combination(&mut self) -> Option<Combination> {
        [
            self.is_royal_straight_flush(),
            self.is_straight_flush(),
            self.is_four_of_a_kind(),
            self.is_full_house(),
            self.is_flush(),
            self.is_straight(),
            self.is_three_of_a_kind(),
            self.is_pairs(),
            self.is_high_card(),
        ]
        .iter()
        .find_map(|r| r.clone())
    }
}
