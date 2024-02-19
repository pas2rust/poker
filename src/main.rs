use poker::{deck::{Deck, DeckTrait}, player::{Player, PlayerTrait}, round::{Round, RoundTrait}, table::{Table, TableKind, TableTrait}};
use wrapper_uuid::wrapper::{UuidTrait, WrapperUuid};

fn main() {
    let mut deck = Deck::poker();
    deck.shuffle();
    let players = Player::new_six_players(&mut deck);
    let round = Round::new(
        WrapperUuid::create(),
        players.clone(),
        Round::pot(players.clone()),
        Round::flop(&mut deck),
        Round::river(&mut deck),
        Round::turn(&mut deck),
        2,
        1
    );
    let mut table = Table::new(
        WrapperUuid::create(),
        deck,
        players,
        vec![round],
        TableKind::Cash
    );
    table.get_current_round().update_blinds();
    table.get_current_round().print_info_players("after update_blinds");
    table.get_current_round().update_pot();
    table.get_current_round().print_info_pot("after update_pot");
}
