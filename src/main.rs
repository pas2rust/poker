use poker::{
    deck::{Deck, DeckTrait},
    player::{Player, PlayerTrait},
    round::{Round, RoundTrait},
    table::{Table, TableKind, TableTrait},
};
use wrapper_uuid::wrapper::{UuidTrait, WrapperUuid};

fn main() {
    let mut deck = Deck::poker();
    deck.shuffle();
    let players = Player::new_six_players(&mut deck);
    let round = Round::new(
        WrapperUuid::create(),
        players.clone(),
        Round::pot(players),
        Round::flop(&mut deck),
        Round::river(&mut deck),
        Round::turn(&mut deck),
        2,
        1,
    );
    let mut table = Table::new(WrapperUuid::create(), deck, vec![round], TableKind::Cash);
    let current_round = table.get_current_round();
    current_round.thinking_player_fold();
    current_round.thinking_player_fold();
    current_round.thinking_player_fold();
    current_round.thinking_player_fold();
    current_round.thinking_player_fold();
    current_round.print_info("after actions")
}
