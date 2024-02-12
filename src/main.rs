pub mod brain;
pub mod card;
pub mod combination;
pub mod deck;
pub mod flush;
pub mod four_of_a_kind;
pub mod full_house;
pub mod hand;
pub mod high_card;
pub mod pairs;
pub mod royal_straight_flush;
pub mod straight;
pub mod straight_flush;
pub mod table;
pub mod three_of_a_kind;

use card::CardTrait;
use deck::{Deck, DeckTrait};
use table::{Table, TableKind, TableTrait};

fn main() {
    let mut table =
        Table::new(Deck::poker(), (), (), (), TableKind::Cash).create();
    let cards = table.deck.draw(2);
    let hand = (cards[0], cards[1]);
    let draw_card1 = hand.0.draw(&mut table.deck);
    let draw_card2 = hand.1.draw(&mut table.deck);
    match draw_card1 {
        Ok(card) => card.print_success("Draw this card by effect"),
        Err(_) => {}
    };
    match draw_card2 {
        Ok(card) => card.print_success("Draw this card by effect"),
        Err(_) => {}
    }
}
