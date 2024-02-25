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
    fn update_thinking(&mut self);
    fn get_participants(&mut self) -> Vec<&mut Player>;
    fn get_dropouts(&mut self) -> Vec<&mut Player>;
    fn get_thinking_player(&mut self) -> &mut Player;
    fn get_biggest_bettor(&mut self) -> &mut Player;
    fn thinking_player_call(&mut self);
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
    fn get_participants(&mut self) -> Vec<&mut Player> {
        self.players.iter_mut().filter(|p| p.state != State::Fold).collect::<Vec<_>>()
    }
    fn get_dropouts(&mut self) -> Vec<&mut Player> {
        self.players.iter_mut().filter(|p| p.state == State::Fold).collect::<Vec<_>>()
    }
    fn get_thinking_player(&mut self) -> &mut Player {
        self.get_participants()
            .into_iter()
            .find(|p| p.state == State::Thinking)
            .expect("Thinking player must be provided")
    }
    fn get_biggest_bettor(&mut self) -> &mut Player {
        self.get_participants().into_iter().max_by_key(|p| p.bet).expect("player must be provided")
    }
    fn update_round(&mut self) {
        self.update_blinds();
        self.update_pot();
    }
    fn update_thinking(&mut self) {
        let thinking_player = self.get_thinking_player().clone();
        let mut participants = self.get_participants();
        let mut next_thinking_index = 0;

        for (index, player) in participants.iter().enumerate() {
            if player.id == thinking_player.id {
                next_thinking_index = (index + 1) % participants.len();
                break;
            }
        }

        participants[next_thinking_index].set_state(State::Thinking);
    }
    fn thinking_player_call(&mut self) {
        self.update_thinking();
        let biggest_bettor = self.get_biggest_bettor().clone();
        let thinking_player = self.get_thinking_player();
        let bet = biggest_bettor.bet;
        thinking_player.set_bet(bet);
        thinking_player.subtract_stack(bet);
        thinking_player.set_state(State::Call);
        self.update_round()
    }
}
