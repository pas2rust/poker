use super::deck::{Deck, DeckTrait};
use darth_rust::DarthRust;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize, Default,
)]
pub enum Suit {
    #[default]
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    None,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize, Serialize, Default,
)]
pub enum Number {
    None = -2,
    Joker = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    #[default]
    Ace = 14,
}

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Default,
    DarthRust,
    Deserialize,
    Serialize,
)]
pub struct Card {
    pub number: Number,
    pub suit: Suit,
}

pub trait CardTrait {
    fn transform(&mut self, suit: Suit, number: Number) -> Result<(), &str>;
    fn change_suit(&mut self, new_suit: Suit) -> Result<(), &str>;
    fn draw(&self, deck: &mut Deck) -> Result<Card, &str>;
    fn draw_search(&self, deck: &mut Deck) -> Result<Option<Card>, &str>;
}

impl CardTrait for Card {
    fn transform(&mut self, new_suit: Suit, new_number: Number) -> Result<(), &str> {
        if let Self { suit: _, number: Number::Joker } = self {
            self.set_number(new_number);
            self.set_suit(new_suit);
            Ok(())
        } else {
            let msg = "Can only transform a Joker card";
            self.print_err(msg);
            Err(msg)
        }
    }
    fn change_suit(&mut self, new_suit: Suit) -> Result<(), &str> {
        let numbers = [Number::Queen, Number::Two];
        match numbers.contains(&self.number) {
            true => {
                self.set_suit(new_suit);
                Ok(())
            }
            false => {
                let msg = "This card does not have a specific change suit rule";
                self.print_err(msg);
                Err(msg)
            }
        }
    }
    fn draw_search(&self, deck: &mut Deck) -> Result<Option<Card>, &str> {
        if let Self { suit: _, number: Number::King } = self {
            return Ok(deck.draw_specific(Self::new(Number::Queen, self.suit)));
        }
        if let Self { suit: _, number: Number::Jack } = self {
            return Ok(deck.draw_specific(Self::new(Number::Seven, self.suit)));
        }
        if let Self { suit: _, number: Number::Five } = self {
            return Ok(deck.draw_specific(Self::new(Number::Two, self.suit)));
        } else {
            let msg = "This card does not have a specific draw rule";
            self.print_err(msg);
            Err(msg)
        }
    }
    fn draw(&self, deck: &mut Deck) -> Result<Card, &str> {
        let numbers = [Number::Seven, Number::Three, Number::Four, Number::Ace];
        match numbers.contains(&self.number) {
            true => Ok(deck.draw(1)[0]),
            false => {
                let msg = "This card does not have a draw rule";
                self.print_err(msg);
                Err(msg)
            }
        }
    }
}

impl Number {
    pub fn value(&self) -> usize {
        *self as usize
    }
}
