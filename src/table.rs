use super::deck::{Deck, DeckTrait};
use darth_rust::DarthRust;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Default)]
pub enum TableKind {
    #[default]
    Cash,
    Tournament,
}

#[derive(
    Debug, DarthRust, Deserialize, Serialize, Clone, PartialEq, Default,
)]
pub struct Table {
    pub deck: Deck,
    pub players: (),
    pub round: (),
    pub history: (),
    pub kind: TableKind,
}

pub trait TableTrait {
    fn create(&mut self) -> Self;
}

impl TableTrait for Table {
    fn create(&mut self) -> Self {
        let deck = &mut self.deck;
        let players = self.players;
        let round = self.round;
        let history = self.history;
        let kind = &self.kind;
        deck.shuffle();
        Self::new(deck.clone(), players, round, history, kind.clone())
    }
}
