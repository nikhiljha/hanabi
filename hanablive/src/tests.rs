use expect_test::expect;
use futures_util::{SinkExt, TryStreamExt};
use reqwest::Client;
use reqwest_websocket::{Message, RequestBuilderExt};
use crate::types::Game;

#[test]
fn example_game_parses() {
    let data = include_str!("../test_data/games/example_game.json");
    let game: Game = serde_json::from_str(data).unwrap();
    let expected = expect![[r#"
        Game {
            id: Some(
                2906,
            ),
            seed: None,
            players: [
                "Alice",
                "Bob",
                "Cathy",
            ],
            deck: [
                Card {
                    suit_index: 2,
                    rank: 3,
                },
                Card {
                    suit_index: 2,
                    rank: 3,
                },
                Card {
                    suit_index: 3,
                    rank: 1,
                },
                Card {
                    suit_index: 1,
                    rank: 3,
                },
                Card {
                    suit_index: 0,
                    rank: 5,
                },
                Card {
                    suit_index: 4,
                    rank: 4,
                },
                Card {
                    suit_index: 2,
                    rank: 1,
                },
                Card {
                    suit_index: 4,
                    rank: 5,
                },
                Card {
                    suit_index: 0,
                    rank: 4,
                },
                Card {
                    suit_index: 0,
                    rank: 2,
                },
                Card {
                    suit_index: 2,
                    rank: 2,
                },
                Card {
                    suit_index: 1,
                    rank: 4,
                },
                Card {
                    suit_index: 3,
                    rank: 3,
                },
                Card {
                    suit_index: 4,
                    rank: 3,
                },
                Card {
                    suit_index: 4,
                    rank: 1,
                },
                Card {
                    suit_index: 0,
                    rank: 1,
                },
                Card {
                    suit_index: 3,
                    rank: 1,
                },
                Card {
                    suit_index: 1,
                    rank: 1,
                },
                Card {
                    suit_index: 2,
                    rank: 4,
                },
                Card {
                    suit_index: 4,
                    rank: 1,
                },
                Card {
                    suit_index: 3,
                    rank: 2,
                },
                Card {
                    suit_index: 0,
                    rank: 3,
                },
                Card {
                    suit_index: 1,
                    rank: 1,
                },
                Card {
                    suit_index: 4,
                    rank: 2,
                },
                Card {
                    suit_index: 0,
                    rank: 1,
                },
                Card {
                    suit_index: 4,
                    rank: 4,
                },
                Card {
                    suit_index: 0,
                    rank: 3,
                },
                Card {
                    suit_index: 1,
                    rank: 3,
                },
                Card {
                    suit_index: 3,
                    rank: 1,
                },
                Card {
                    suit_index: 1,
                    rank: 1,
                },
                Card {
                    suit_index: 2,
                    rank: 1,
                },
                Card {
                    suit_index: 3,
                    rank: 5,
                },
                Card {
                    suit_index: 4,
                    rank: 2,
                },
                Card {
                    suit_index: 2,
                    rank: 1,
                },
                Card {
                    suit_index: 0,
                    rank: 4,
                },
                Card {
                    suit_index: 3,
                    rank: 4,
                },
                Card {
                    suit_index: 1,
                    rank: 4,
                },
                Card {
                    suit_index: 3,
                    rank: 4,
                },
                Card {
                    suit_index: 0,
                    rank: 1,
                },
                Card {
                    suit_index: 1,
                    rank: 2,
                },
                Card {
                    suit_index: 1,
                    rank: 2,
                },
                Card {
                    suit_index: 3,
                    rank: 2,
                },
                Card {
                    suit_index: 4,
                    rank: 3,
                },
                Card {
                    suit_index: 2,
                    rank: 2,
                },
                Card {
                    suit_index: 3,
                    rank: 3,
                },
                Card {
                    suit_index: 1,
                    rank: 5,
                },
                Card {
                    suit_index: 2,
                    rank: 5,
                },
                Card {
                    suit_index: 0,
                    rank: 2,
                },
                Card {
                    suit_index: 2,
                    rank: 4,
                },
                Card {
                    suit_index: 4,
                    rank: 1,
                },
            ],
            actions: [
                ColorClue {
                    target: 1,
                    value: 2,
                },
                Play {
                    target: 6,
                },
                ColorClue {
                    target: 0,
                    value: 3,
                },
                ColorClue {
                    target: 1,
                    value: 0,
                },
                ColorClue {
                    target: 2,
                    value: 4,
                },
                Play {
                    target: 14,
                },
                ColorClue {
                    target: 2,
                    value: 2,
                },
                Play {
                    target: 15,
                },
                Play {
                    target: 10,
                },
                ColorClue {
                    target: 2,
                    value: 2,
                },
                Play {
                    target: 17,
                },
                RankClue {
                    target: 1,
                    value: 5,
                },
                Discard {
                    target: 2,
                },
                RankClue {
                    target: 2,
                    value: 3,
                },
                Play {
                    target: 16,
                },
                Play {
                    target: 20,
                },
                Discard {
                    target: 5,
                },
                RankClue {
                    target: 1,
                    value: 2,
                },
                RankClue {
                    target: 1,
                    value: 4,
                },
                Play {
                    target: 9,
                },
                Play {
                    target: 12,
                },
                Discard {
                    target: 0,
                },
                Play {
                    target: 23,
                },
                RankClue {
                    target: 0,
                    value: 3,
                },
                Play {
                    target: 1,
                },
                Discard {
                    target: 19,
                },
                Play {
                    target: 18,
                },
                ColorClue {
                    target: 2,
                    value: 4,
                },
                Discard {
                    target: 24,
                },
                ColorClue {
                    target: 0,
                    value: 0,
                },
                Play {
                    target: 26,
                },
                Play {
                    target: 8,
                },
                Play {
                    target: 13,
                },
                Play {
                    target: 4,
                },
                RankClue {
                    target: 0,
                    value: 4,
                },
                Play {
                    target: 25,
                },
                Play {
                    target: 35,
                },
                Discard {
                    target: 27,
                },
                RankClue {
                    target: 1,
                    value: 5,
                },
                Discard {
                    target: 22,
                },
                Play {
                    target: 31,
                },
                ColorClue {
                    target: 0,
                    value: 1,
                },
                Discard {
                    target: 28,
                },
                RankClue {
                    target: 0,
                    value: 2,
                },
                Discard {
                    target: 11,
                },
                Play {
                    target: 39,
                },
                ColorClue {
                    target: 2,
                    value: 1,
                },
                Discard {
                    target: 21,
                },
                Play {
                    target: 3,
                },
                Play {
                    target: 7,
                },
                Play {
                    target: 36,
                },
                RankClue {
                    target: 1,
                    value: 5,
                },
                Play {
                    target: 46,
                },
                RankClue {
                    target: 0,
                    value: 5,
                },
                Play {
                    target: 45,
                },
            ],
            options: Some(
                Options {
                    variant: "No Variant",
                    speedrun: false,
                    card_cycle: false,
                    deck_plays: true,
                    empty_clues: false,
                    one_extra_card: false,
                    one_less_card: false,
                    all_or_nothing: false,
                    detrimental_characters: false,
                },
            ),
            notes: Some(
                [
                    [
                        "y3?",
                        "y4",
                        "r2",
                    ],
                    [],
                    [
                        "not 1",
                        "y4",
                        "b3>",
                        "p4?",
                    ],
                ],
            ),
            characters: None,
        }
    "#]];
    expected.assert_debug_eq(&game);
}
