use super::*;

fn two_player_game() -> HanabiGame {
    let players = vec![
        Player::new("Alice".to_string()),
        Player::new("Bob".to_string()),
    ];
    HanabiGame::new(players)
}

fn any_legal_clue(game: &mut HanabiGame) -> AnnotatedAction {
    let target = (game.current_player + 1) % 2;
    AnnotatedAction {
        player: game.current_player,
        action: Action::Clue {
            clue: Clue::Color(game.players[target].hand[0].card.color),
            target,
        },
    }
}

#[test]
fn no_more_clues() {
    let mut game = two_player_game();
    for times_clued in 0..(game.config.max_clues) {
        let clue = any_legal_clue(&mut game);
        let result = game.act(clue);
        assert_eq!(game.clues(), game.config.max_clues - times_clued - 1);
        assert_eq!(result, Ok(()));
    }

    let clue = any_legal_clue(&mut game);
    let result = game.act(clue);
    assert_eq!(result, Err(ActError::NotEnoughClues));
}
