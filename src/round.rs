use darth_rust::DarthRust;
use serde::{Deserialize, Serialize};

use crate::deck::Deck;
use crate::deck::DeckTrait;

use super::player::Position;
use super::player::Player;
use super::card::Card;

#[derive(Debug, DarthRust, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Round {
    pub id: String,
    pub players: Vec<Player>,
    pub pot: u32,
    pub flop: Vec<Card>,
    pub river: Card,
    pub turn: Card,
    pub big_blind: u32,
    pub small_blind: u32
}

pub trait RoundTrait {
    fn update_pot(&mut self);
    fn update_blinds(&mut self);
    fn pot(players: Vec<Player>) -> u32;
    fn flop(deck: &mut Deck) -> Vec<Card>;
    fn river(deck: &mut Deck) -> Card;
    fn turn(deck: &mut Deck) -> Card;
}

impl RoundTrait for Round {
    fn flop(deck: &mut Deck) -> Vec<Card> {
        deck.draw(3)
    }
    fn river(deck: &mut Deck) -> Card {
        deck.draw(1)[0]
    }
    fn turn(deck: &mut Deck) -> Card {
        deck.draw(1)[0]
    }
    fn pot(players: Vec<Player>) -> u32 {
        players.iter().map(|p| p.bet).sum::<u32>()
    }
    fn update_pot(&mut self) {
        let size = &self.players.iter().map(|p| p.bet).sum::<u32>();
        self.set_pot(*size)
    }
    fn update_blinds(&mut self) {
        for player in &mut self.players {
            match player.position {
                Position::BigBlind => {
                    let big_blind = self.big_blind;
                    player.subtract_stack(big_blind);
                    player.set_bet(big_blind);
                },
                Position::SmallBlind => {
                    let small_blind = self.small_blind;
                    player.subtract_stack(small_blind);
                    player.set_bet(small_blind);
                },
                _ => {}
            };
        }
    }
}