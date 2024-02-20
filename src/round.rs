use crate::{
    deck::{Deck, DeckTrait},
    player::State,
};
use darth_rust::DarthRust;
use serde::{Deserialize, Serialize};

use super::{
    card::Card,
    player::{Player, Position},
};

#[derive(Debug, DarthRust, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Round {
    pub id: String,
    pub players: Vec<Player>,
    pub pot: u32,
    pub flop: Vec<Card>,
    pub river: Card,
    pub turn: Card,
    pub big_blind: u32,
    pub small_blind: u32,
}

pub trait RoundTrait {
    fn update_pot(&mut self);
    fn update_blinds(&mut self);
    fn pot(players: Vec<Player>) -> u32;
    fn flop(deck: &mut Deck) -> Vec<Card>;
    fn river(deck: &mut Deck) -> Card;
    fn turn(deck: &mut Deck) -> Card;
    fn update_round(&mut self);
    fn get_participants(&mut self) -> Vec<Player>;
    fn get_dropouts(&mut self) -> Vec<Player>;
    fn get_current_player(&mut self) -> Player;
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
                    if player.bet == 0 {
                        let big_blind = self.big_blind;
                        player.subtract_stack(big_blind);
                        player.set_bet(big_blind);
                    }
                }
                Position::SmallBlind => {
                    if player.bet == 0 {
                        let small_blind = self.small_blind;
                        player.subtract_stack(small_blind);
                        player.set_bet(small_blind);
                    }
                }
                _ => {}
            };
        }
    }
    fn get_participants(&mut self) -> Vec<Player> {
        self.players.iter().filter(|p| p.state != State::Fold).cloned().collect::<Vec<_>>()
    }
    fn get_dropouts(&mut self) -> Vec<Player> {
        self.players.iter().filter(|p| p.state == State::Fold).cloned().collect::<Vec<_>>()
    }
    fn get_current_player(&mut self) -> Player {
        self.players
            .iter()
            .find(|p| p.state == State::Thinking)
            .cloned()
            .expect("current player must be provided")
    }
    fn update_round(&mut self) {
        self.update_blinds();
        self.update_pot();
    }
}
