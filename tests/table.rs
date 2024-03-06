use poker::{
    deck::{Deck, DeckTrait},
    player::{Player, PlayerTrait, Position, State},
    round::{Round, RoundTrait},
    table::{Table, TableKind, TableTrait},
};
use wrapper_uuid::wrapper::{UuidTrait, WrapperUuid};

#[test]
fn thinking_player_call() {
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
    current_round.thinking_player_call();
    let utg = Player::new(
        current_round.players[0].id.clone(),
        current_round.players[0].history.clone(),
        Position::Utg,
        998,
        2,
        State::Call,
        current_round.players[0].hand.clone()
    );
    let mp = Player::new(
        current_round.players[1].id.clone(),
        current_round.players[1].history.clone(),
        Position::MiddlePosition,
        1000,
        0,
        State::Thinking,
        current_round.players[1].hand.clone()
    );
    assert_eq!(current_round.players[0], utg);
    assert_eq!(current_round.players[1], mp)
}
