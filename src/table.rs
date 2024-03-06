use super::{deck::Deck, round::Round};
use crate::round::RoundTrait;
use darth_rust::DarthRust;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default, Copy)]
pub enum TableKind {
    #[default]
    Cash,
    Tournament,
}

#[derive(Debug, DarthRust, Deserialize, Serialize, Clone, PartialEq, Default)]
pub struct Table {
    pub id: String,
    pub deck: Deck,
    pub rounds: Vec<Round>,
    pub kind: TableKind,
}

pub trait TableTrait {
    fn get_current_round(&mut self) -> &mut Round;
}

impl TableTrait for Table {
    fn get_current_round(&mut self) -> &mut Round {
        let round = self.rounds.last_mut().expect("vec rounds must be provided");
        round.update_round();
        round
    }
}
