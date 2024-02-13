use super::card::{Card, Number, Suit};
use darth_rust::DarthRust;
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

pub type Cards = Vec<Card>;

#[derive(Debug, DarthRust, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Deck {
    pub cards: Cards,
}

pub trait DeckTrait {
    fn poker() -> Self;
    fn poker_with_joker() -> Self;
    fn shuffle(&mut self);
    fn draw(&mut self, num: usize) -> Vec<Card>;
    fn draw_specific(&mut self, card: Card) -> Option<Card>;
}

impl DeckTrait for Deck {
    fn poker() -> Self {
        let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let numbers = [
            Number::Ace,
            Number::Two,
            Number::Three,
            Number::Four,
            Number::Five,
            Number::Six,
            Number::Seven,
            Number::Eight,
            Number::Nine,
            Number::Ten,
            Number::Jack,
            Number::Queen,
            Number::King,
        ];
        let size = suits.len() * numbers.len();
        let mut cards = Vec::with_capacity(size);
        for number in &numbers {
            for suit in &suits {
                cards.push(Card::new(*number, *suit));
            }
        }

        Self { cards }
    }
    fn poker_with_joker() -> Self {
        let suits = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];
        let numbers = [
            Number::Joker,
            Number::Ace,
            Number::Two,
            Number::Three,
            Number::Four,
            Number::Five,
            Number::Six,
            Number::Seven,
            Number::Eight,
            Number::Nine,
            Number::Ten,
            Number::Jack,
            Number::Queen,
            Number::King,
        ];
        let size = suits.len() * numbers.len();
        let mut cards = Vec::with_capacity(size);
        for number in &numbers {
            for suit in &suits {
                cards.push(Card::new(*number, *suit));
            }
        }

        Self { cards }
    }

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
    fn draw(&mut self, num: usize) -> Vec<Card> {
        let mut drawn_cards = Vec::new();
        for _ in 0..num {
            if let Some(card) = self.cards.pop() {
                drawn_cards.push(card);
            } else {
                break;
            }
        }
        drawn_cards
    }
    fn draw_specific(&mut self, card: Card) -> Option<Card> {
        let position = self.cards.iter().position(|c| *c == card);
        position.map(|pos| self.cards.remove(pos))
    }
}
