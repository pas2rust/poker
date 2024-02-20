use crate::{
    deck::{Deck, DeckTrait},
    hand::Hand,
    round::Round,
};
use darth_rust::DarthRust;
use serde::{Deserialize, Serialize};
use wrapper_uuid::wrapper::{UuidTrait, WrapperUuid};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub enum State {
    #[default]
    Waiting,
    Thinking,
    Fold,
    Check,
    Call,
    Raise,
    Allin,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub enum Position {
    #[default]
    Utg,
    UtgPlus1,
    UtgPlus2,
    MiddlePosition1,
    MiddlePosition2,
    MiddlePosition3,
    MiddlePosition,
    CutOff,
    Button,
    SmallBlind,
    BigBlind,
}

#[derive(Debug, DarthRust, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Player {
    pub id: String,
    pub history: Vec<Round>,
    pub position: Position,
    pub stack: u32,
    pub bet: u32,
    pub state: State,
    pub hand: Hand,
}

fn for_players(positions: Vec<Position>, deck: &mut Deck) -> Vec<Player> {
    let mut players = Vec::new();
    for position in positions {
        let state = match position {
            Position::Utg => State::Thinking,
            _ => State::Waiting,
        };
        let player = Player::new(
            WrapperUuid::create(),
            vec![],
            position,
            1000,
            0,
            state,
            Hand::new(deck.draw(2)),
        );
        players.push(player);
    }
    players
}

pub trait PlayerTrait {
    fn new_nine_players(deck: &mut Deck) -> Vec<Player>;
    fn new_six_players(deck: &mut Deck) -> Vec<Player>;
    fn call(&mut self, current_round: &mut Round);
}

impl PlayerTrait for Player {
    fn new_nine_players(deck: &mut Deck) -> Vec<Player> {
        let positions = vec![
            Position::Utg,
            Position::UtgPlus1,
            Position::UtgPlus2,
            Position::MiddlePosition1,
            Position::MiddlePosition2,
            Position::CutOff,
            Position::Button,
            Position::SmallBlind,
            Position::BigBlind,
        ];
        for_players(positions, deck)
    }
    fn new_six_players(deck: &mut Deck) -> Vec<Player> {
        let positions = vec![
            Position::Utg,
            Position::MiddlePosition,
            Position::CutOff,
            Position::Button,
            Position::SmallBlind,
            Position::BigBlind,
        ];
        for_players(positions, deck)
    }
    fn call(&mut self, current_round: &mut Round) {

    }
}
