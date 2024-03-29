use crate::{
    combination::CombinationTrait, deck::{Deck, DeckTrait}, hand::{Hand, HandTrait}, player::State
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
    fn get_winner(&mut self) -> &mut Player;
    fn thinking_player_call(&mut self);
    fn thinking_player_fold(&mut self);
    fn thinking_player_all_in(&mut self);
    fn thinking_player_raise(&mut self, bet: u32);
    fn thinking_player_raise_multiply(&mut self, mult: u32);
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
        participants.iter().enumerate().for_each(|(index, player)| {
            if player.id == thinking_player.id  {
                next_thinking_index = (index + 1) % participants.len();
            }
        });
        participants[next_thinking_index].set_state(State::Thinking);
    }
    fn thinking_player_call(&mut self) {
        self.update_thinking();
        let biggest_bettor = self.get_biggest_bettor().clone();
        let thinking_player = self.get_thinking_player();
        let bet = thinking_player.bet - biggest_bettor.bet;
        thinking_player.set_bet(bet);
        thinking_player.subtract_stack(bet);
        thinking_player.set_state(State::Call);
        self.update_round()
    }
    fn thinking_player_raise(&mut self, bet: u32) {
        self.update_thinking();
        let thinking_player = self.get_thinking_player();
        let bet = thinking_player.bet - bet;
        thinking_player.set_bet(bet);
        thinking_player.subtract_stack(bet);
        thinking_player.set_state(State::Raise);
        self.update_round()
    }
    fn thinking_player_raise_multiply(&mut self, mult: u32) {
        self.update_thinking();
        let mut biggest_bettor = self.get_biggest_bettor().clone();
        let thinking_player = self.get_thinking_player();
        biggest_bettor.multiply_bet(mult);
        let bet = thinking_player.bet - biggest_bettor.bet;
        thinking_player.set_bet(bet);
        thinking_player.subtract_stack(bet);
        thinking_player.set_state(State::Raise);
        self.update_round()
    }
    fn thinking_player_all_in(&mut self) {
        self.update_thinking();
        let thinking_player = self.get_thinking_player();
        let bet = thinking_player.stack + thinking_player.bet;
        thinking_player.set_bet(bet);
        thinking_player.subtract_stack(bet);
        thinking_player.set_state(State::Allin);
        self.update_round()
    }
    fn thinking_player_fold(&mut self) {
        self.update_thinking();
        let thinking_player = self.get_thinking_player();
        thinking_player.set_state(State::Fold);
        self.update_round()
    }
    fn get_winner(&mut self) -> &mut Player {
        let mut cards = Vec::new();
        let flop = self.flop.clone();
        let river = self.river.clone();
        let turn = self.turn.clone();
        let participants = self.get_participants();
        if participants.len() == 1 {
            self.get_participants()[0]
        } else {
            cards.extend(flop.iter());
            cards.push(river.clone());
            cards.push(turn.clone());
            let winner = participants.iter().max_by_key(|p| {
                let hand = &p.hand.cards;
                cards.extend(hand.iter());
                let mut hand = Hand::new(hand.clone());
                hand.is_combination().expect("combination must be provided").power().value
            }).expect("Winner must be provided");
            let winner = &mut **winner;
            winner
        }
    }
}
