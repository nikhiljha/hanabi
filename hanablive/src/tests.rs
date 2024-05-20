use expect_test::expect;
use serde_json::Value;
use tracing::Level;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use crate::messages;
use crate::messages::WebsocketParseError;
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

#[tokio::test]
async fn websocket_trace_parses() -> Result<(), WebsocketParseError> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::filter::LevelFilter::from_level(
            Level::INFO,
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let data = include_str!("../test_data/ws_traces/simple_terminated_game.json");
    let trace: Vec<Value> = serde_json::from_str(data).unwrap();
    let trace = trace.into_iter().map(|obj| {
        let obj = obj.as_object().unwrap();
        // let kind = obj.get("type").unwrap().as_str().unwrap().to_string();
        let msg = obj.get("data").unwrap().as_str().unwrap();
        let msg: messages::Message = messages::Message::from_websocket_message(msg)?;
        Ok::<messages::Message, WebsocketParseError>(msg)
    }).collect::<Result<Vec<_>, WebsocketParseError>>()?;

    let expected = expect![[r#"
        [
            Welcome(
                WelcomeMessage {
                    user_id: 88454,
                    username: "njha",
                    total_games: 143,
                    muted: false,
                    first_time_user: false,
                    settings: Settings {
                        desktop_notification: false,
                        sound_move: true,
                        sound_timer: true,
                        keldon_mode: false,
                        colorblind_mode: false,
                        real_life_mode: false,
                        reverse_hands: false,
                        style_numbers: false,
                        show_timer_in_untimed: true,
                        volume: 100,
                        speedrun_preplay: false,
                        speedrun_mode: true,
                        hyphenated_conventions: true,
                        create_table_variant: "Matryoshka (5 Suits)",
                        create_table_timed: false,
                        create_table_time_base_minutes: 2,
                        create_table_time_per_turn_seconds: 20,
                        create_table_speedrun: false,
                        create_table_card_cycle: false,
                        create_table_deck_plays: false,
                        create_table_empty_clues: false,
                        create_table_one_extra_card: false,
                        create_table_one_less_card: false,
                        create_table_all_or_nothing: false,
                        create_table_detrimental_characters: false,
                        create_table_max_players: 6,
                    },
                    friends: [
                        "adym",
                        "etw",
                        "girantinas",
                        "oliver",
                        "tiffnami",
                        "trnin",
                        "wet",
                    ],
                    playing_at_tables: [],
                    discon_spectating_table: Some(
                        0,
                    ),
                    discon_shadowing_seat: Some(
                        0,
                    ),
                    random_table_name: "Shauna anxiously daffier",
                    shutting_down: false,
                    datetime_shutdown_init: Some(
                        "0001-01-01T00:00:00Z",
                    ),
                    maintenance_mode: false,
                },
            ),
            User(
                UserData {
                    user_id: 88454,
                    name: "njha",
                    status: 0,
                    table_id: Some(
                        0,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            UserList(
                [
                    UserData {
                        user_id: 75671,
                        name: "maddy",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: true,
                        inactive: true,
                    },
                    UserData {
                        user_id: 68050,
                        name: "Magesrook",
                        status: 2,
                        table_id: Some(
                            15027,
                        ),
                        hyphenated: false,
                        inactive: false,
                    },
                    UserData {
                        user_id: 22353,
                        name: "Reiman",
                        status: 2,
                        table_id: Some(
                            15028,
                        ),
                        hyphenated: false,
                        inactive: false,
                    },
                    UserData {
                        user_id: 74724,
                        name: "yagami_black",
                        status: 2,
                        table_id: Some(
                            15020,
                        ),
                        hyphenated: true,
                        inactive: false,
                    },
                    UserData {
                        user_id: 70237,
                        name: "Cocorifle",
                        status: 2,
                        table_id: Some(
                            15020,
                        ),
                        hyphenated: true,
                        inactive: false,
                    },
                    UserData {
                        user_id: 10507,
                        name: "MKQ",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: true,
                        inactive: false,
                    },
                    UserData {
                        user_id: 86934,
                        name: "morganz_",
                        status: 2,
                        table_id: Some(
                            15027,
                        ),
                        hyphenated: false,
                        inactive: false,
                    },
                    UserData {
                        user_id: 91520,
                        name: "Sulugy",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: false,
                        inactive: false,
                    },
                    UserData {
                        user_id: 91466,
                        name: "ashishla",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: false,
                        inactive: false,
                    },
                    UserData {
                        user_id: 86969,
                        name: "Qwertygamer",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: true,
                        inactive: true,
                    },
                    UserData {
                        user_id: 24679,
                        name: "Simon",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: true,
                        inactive: false,
                    },
                    UserData {
                        user_id: 76885,
                        name: "RandomGuyJCI",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: true,
                        inactive: true,
                    },
                    UserData {
                        user_id: 90522,
                        name: "yui_yukihira",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: true,
                        inactive: false,
                    },
                    UserData {
                        user_id: 90466,
                        name: "mets14",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: true,
                        inactive: false,
                    },
                    UserData {
                        user_id: 33580,
                        name: "Bjuuti",
                        status: 2,
                        table_id: Some(
                            15028,
                        ),
                        hyphenated: false,
                        inactive: false,
                    },
                    UserData {
                        user_id: 88089,
                        name: "darr",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: true,
                        inactive: false,
                    },
                    UserData {
                        user_id: 24923,
                        name: "Kyeudo",
                        status: 2,
                        table_id: Some(
                            15027,
                        ),
                        hyphenated: false,
                        inactive: false,
                    },
                    UserData {
                        user_id: 91398,
                        name: "WarsawPact",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: false,
                        inactive: true,
                    },
                    UserData {
                        user_id: 43784,
                        name: "newsun",
                        status: 2,
                        table_id: Some(
                            14583,
                        ),
                        hyphenated: true,
                        inactive: false,
                    },
                    UserData {
                        user_id: 68193,
                        name: "Tailz",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: true,
                        inactive: true,
                    },
                    UserData {
                        user_id: 29740,
                        name: "newduke",
                        status: 2,
                        table_id: Some(
                            14583,
                        ),
                        hyphenated: true,
                        inactive: false,
                    },
                    UserData {
                        user_id: 75662,
                        name: "gsymon",
                        status: 2,
                        table_id: Some(
                            15024,
                        ),
                        hyphenated: true,
                        inactive: false,
                    },
                    UserData {
                        user_id: 55989,
                        name: "miksu",
                        status: 2,
                        table_id: Some(
                            15028,
                        ),
                        hyphenated: false,
                        inactive: false,
                    },
                    UserData {
                        user_id: 85421,
                        name: "ElenaDhynho",
                        status: 2,
                        table_id: Some(
                            15020,
                        ),
                        hyphenated: true,
                        inactive: false,
                    },
                    UserData {
                        user_id: 88219,
                        name: "morefriends",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: true,
                        inactive: true,
                    },
                    UserData {
                        user_id: 80261,
                        name: "str8tsknacker",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: true,
                        inactive: true,
                    },
                    UserData {
                        user_id: 79457,
                        name: "Cadenza",
                        status: 2,
                        table_id: Some(
                            14583,
                        ),
                        hyphenated: true,
                        inactive: false,
                    },
                    UserData {
                        user_id: 2000,
                        name: "HuHu",
                        status: 2,
                        table_id: Some(
                            15024,
                        ),
                        hyphenated: false,
                        inactive: false,
                    },
                    UserData {
                        user_id: 89959,
                        name: "Jakub123",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: true,
                        inactive: false,
                    },
                    UserData {
                        user_id: 78442,
                        name: "will-bot2",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: false,
                        inactive: false,
                    },
                    UserData {
                        user_id: 78102,
                        name: "will-bot1",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: false,
                        inactive: false,
                    },
                    UserData {
                        user_id: 27131,
                        name: "arataya87",
                        status: 2,
                        table_id: Some(
                            15027,
                        ),
                        hyphenated: false,
                        inactive: false,
                    },
                    UserData {
                        user_id: 32037,
                        name: "HelanaAshryvr",
                        status: 3,
                        table_id: Some(
                            15020,
                        ),
                        hyphenated: false,
                        inactive: false,
                    },
                    UserData {
                        user_id: 4737,
                        name: "TimeHoodie",
                        status: 2,
                        table_id: Some(
                            15020,
                        ),
                        hyphenated: true,
                        inactive: false,
                    },
                    UserData {
                        user_id: 88454,
                        name: "njha",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: true,
                        inactive: false,
                    },
                    UserData {
                        user_id: 24919,
                        name: "cooper",
                        status: 2,
                        table_id: Some(
                            15027,
                        ),
                        hyphenated: false,
                        inactive: false,
                    },
                    UserData {
                        user_id: 85500,
                        name: "will-bot4",
                        status: 0,
                        table_id: Some(
                            0,
                        ),
                        hyphenated: false,
                        inactive: false,
                    },
                    UserData {
                        user_id: 90039,
                        name: "submachine",
                        status: 3,
                        table_id: Some(
                            14583,
                        ),
                        hyphenated: true,
                        inactive: false,
                    },
                ],
            ),
            TableList(
                [
                    TableData {
                        id: 15027,
                        joined: false,
                        max_players: 6,
                        name: "Lucid (#2)",
                        num_players: 5,
                        options: GameOptions {
                            num_players: Some(
                                5,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "Dark Rainbow (6 Suits)",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: false,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: true,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        owned: false,
                        password_protected: true,
                        players: [
                            "morganz_",
                            "Magesrook",
                            "cooper",
                            "Kyeudo",
                            "arataya87",
                        ],
                        progress: 7,
                        running: true,
                        shared_replay: false,
                        spectators: [],
                        time_base: 0,
                        time_per_turn: 0,
                        timed: false,
                        variant: "Dark Rainbow (6 Suits)",
                    },
                    TableData {
                        id: 15028,
                        joined: false,
                        max_players: 5,
                        name: "Actaeon innersole Federal (#13)",
                        num_players: 3,
                        options: GameOptions {
                            num_players: Some(
                                3,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "Gray Pink (5 Suits)",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: false,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        owned: false,
                        password_protected: true,
                        players: [
                            "Bjuuti",
                            "Reiman",
                            "miksu",
                        ],
                        progress: 36,
                        running: true,
                        shared_replay: false,
                        spectators: [],
                        time_base: 0,
                        time_per_turn: 0,
                        timed: false,
                        variant: "Gray Pink (5 Suits)",
                    },
                    TableData {
                        id: 14583,
                        joined: false,
                        max_players: 5,
                        name: "lobsters coffin logout",
                        num_players: 3,
                        options: GameOptions {
                            num_players: Some(
                                3,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "Light Pink & Cocoa Rainbow (6 Suits)",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: false,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        owned: false,
                        password_protected: false,
                        players: [
                            "newsun",
                            "newduke",
                            "Cadenza",
                        ],
                        progress: 67,
                        running: true,
                        shared_replay: false,
                        spectators: [
                            Spectator {
                                name: "submachine",
                                shadowing_player_index: Some(
                                    -1,
                                ),
                                shadowing_player_username: Some(
                                    "",
                                ),
                            },
                        ],
                        time_base: 0,
                        time_per_turn: 0,
                        timed: false,
                        variant: "Light Pink & Cocoa Rainbow (6 Suits)",
                    },
                    TableData {
                        id: 15020,
                        joined: false,
                        max_players: 5,
                        name: "unsettles photosynthesized preemptive (#2)",
                        num_players: 4,
                        options: GameOptions {
                            num_players: Some(
                                4,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "Brown-Fives & Brown (6 Suits)",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: false,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        owned: false,
                        password_protected: false,
                        players: [
                            "Cocorifle",
                            "TimeHoodie",
                            "ElenaDhynho",
                            "yagami_black",
                        ],
                        progress: 27,
                        running: true,
                        shared_replay: false,
                        spectators: [
                            Spectator {
                                name: "HelanaAshryvr",
                                shadowing_player_index: Some(
                                    1,
                                ),
                                shadowing_player_username: Some(
                                    "TimeHoodie",
                                ),
                            },
                        ],
                        time_base: 0,
                        time_per_turn: 0,
                        timed: false,
                        variant: "Brown-Fives & Brown (6 Suits)",
                    },
                    TableData {
                        id: 15024,
                        joined: false,
                        max_players: 6,
                        name: "veterinarians Bantu gentries (#10)",
                        num_players: 2,
                        options: GameOptions {
                            num_players: Some(
                                2,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "Rainbow-Fives & Dark Pink (6 Suits)",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: false,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        owned: false,
                        password_protected: true,
                        players: [
                            "HuHu",
                            "gsymon",
                        ],
                        progress: 77,
                        running: true,
                        shared_replay: false,
                        spectators: [],
                        time_base: 0,
                        time_per_turn: 0,
                        timed: false,
                        variant: "Rainbow-Fives & Dark Pink (6 Suits)",
                    },
                ],
            ),
            ChatList(
                ChatListData {
                    list: [
                        ChatData {
                            msg: "The level summary for the H-Group: https://hanabi.github.io/learning-path/#level-summary",
                            who: Some(
                                "__server",
                            ),
                            discord: false,
                            server: true,
                            datetime: "2024-04-02T08:30:43.368411Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "LFG lvl 3..10",
                            who: Some(
                                "ape3000",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-02T17:24:52.276187Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "+2 no var any level",
                            who: Some(
                                "Fireheart",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-02T17:29:08.015839Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "and any can join",
                            who: Some(
                                "Fireheart",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-02T17:29:21.827279Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "anyone up 4 a no variant gme?",
                            who: Some(
                                "Agent12",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-03T05:49:57.007215Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "HANAB-CON 2024 in Houston",
                            who: Some(
                                "blortjr.",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-04T01:47:20.642075Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "anyone up for a 3-5 person game?",
                            who: Some(
                                "pepe_s",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T05:46:35.811556Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "sadfasdfsa",
                            who: Some(
                                "sadfasd",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-04T11:09:44.795619Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "asdfasd",
                            who: Some(
                                "sadfasd",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-04T11:09:46.162296Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "asdfgdasrdfwe4zadxfsd",
                            who: Some(
                                "sadfasd",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-04T11:09:48.484118Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "aafdsadfasdf",
                            who: Some(
                                "sadfasd",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-04T11:09:50.146804Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "asdf",
                            who: Some(
                                "sadfasd",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-04T11:09:50.489678Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "asd",
                            who: Some(
                                "sadfasd",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-04T11:09:50.757999Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "f",
                            who: Some(
                                "sadfasd",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-04T11:09:51.063676Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "df",
                            who: Some(
                                "sadfasd",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-04T11:09:51.362995Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "asd",
                            who: Some(
                                "sadfasd",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-04T11:09:51.645402Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "f",
                            who: Some(
                                "sadfasd",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-04T11:09:51.9172Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "1 needed for L1 game if anyone can :)",
                            who: Some(
                                "NNewQuark",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-04T12:37:14.548457Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: ":( been kicked out",
                            who: Some(
                                "Ussel",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-04T12:46:05.553774Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "https://www.hanabi-con.com/ huh that&#39;s a real thing, wow",
                            who: Some(
                                "bluenovember",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T18:22:05.953341Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "wait a second...",
                            who: Some(
                                "bluenovember",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T18:22:19.048955Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "&#34;Cosplay Chess&#34; Now that is to my liking",
                            who: Some(
                                "stuurm",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T18:23:43.816887Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "I can&#39;t. My dad said if I ever dressed for pawn he&#39;d disown me.",
                            who: Some(
                                "bluenovember",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T18:24:20.019627Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "Do not let your dreams be memes!",
                            who: Some(
                                "stuurm",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T18:24:38.159828Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "But but all my dreams are memes",
                            who: Some(
                                "bluenovember",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T18:24:49.680899Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "aahhh....checkmate I guess",
                            who: Some(
                                "stuurm",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T18:25:12.039802Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "https://i.imgflip.com/8lkqx2.jpg",
                            who: Some(
                                "bluenovember",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T18:25:29.648141Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "https://tenor.com/view/ill-be-a-knight-knight-brave-confident-lets-do-this-gif-15183538",
                            who: Some(
                                "springiest",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T18:26:49.928078Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "It looks like we have multiple participiants!",
                            who: Some(
                                "stuurm",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T18:29:38.796873Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "I was legit excited for cards, but all I got was fake fireworks :/",
                            who: Some(
                                "cynas",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T18:33:21.327437Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "I&#39;m meeting up with @nitrate6262 in person on the weekend. Does that count as a convention?",
                            who: Some(
                                "bluenovember",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T18:56:16.325199Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "Also if I stop turning up in tables it&#39;s because he&#39;s murdered me and sold my organs, in pay back for all the times I gave clues that were lies",
                            who: Some(
                                "bluenovember",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T18:56:42.297555Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "Lies or bluffs?  I, too, enjoy my organs INSIDE my body.",
                            who: Some(
                                "cynas",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T19:00:19.975282Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "it was nice knowing you blue",
                            who: Some(
                                "darr",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T19:25:16.121553Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "Nice! Yes, it does. It&#39;s called 2p conv 😄",
                            who: Some(
                                "elo_moni",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T19:27:14.670115Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "/missing",
                            who: Some(
                                "jieship",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-04T20:14:24.396676Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "You can only perform this command while in a game.",
                            who: Some(
                                "__server",
                            ),
                            discord: false,
                            server: true,
                            datetime: "2024-04-04T20:14:24.82693Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "Hi, I would like to play a game. How can I do that?",
                            who: Some(
                                "ashishla",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-04T20:30:46.48032Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "@stuurm @springiest @.mangopie @ilikeeelst Thanks for the game, sorry if I made some mistakes. Would love to play again",
                            who: Some(
                                "ashishla",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T20:58:54.023067Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "Mistakes are part of the game. You are welcome to join again!",
                            who: Some(
                                "stuurm",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T21:07:58.232917Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "That&#39;s so sweet.  Most people quit Hanabi in disgust long before they ever get close to the number of mistakes I make",
                            who: Some(
                                "cynas",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-04T21:47:58.76076Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "/help",
                            who: Some(
                                "M1n3c4rt",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-05T16:12:31.423385Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "lvl4 game anybody? 👀",
                            who: Some(
                                "chengmoniu",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-06T04:36:09.56501Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "/path",
                            who: Some(
                                "Begun",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-06T13:45:26.661794Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "The level summary for the H-Group: https://hanabi.github.io/learning-path/#level-summary",
                            who: Some(
                                "__server",
                            ),
                            discord: false,
                            server: true,
                            datetime: "2024-04-06T13:45:26.937382Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "One more for a beginner game?",
                            who: Some(
                                "Kyle",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-07T21:16:04.721874Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "just started playing Stardew Valley with my wife and stumbled across a forum post where somebody tagged Zamiel said he was putting together some kind of guide and i looked at the profile pic and it&#39;s the same person 😮",
                            who: Some(
                                "russ75",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-08T05:39:03.123687Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "@loveontop, do you want to play?",
                            who: Some(
                                "Gingy120",
                            ),
                            discord: false,
                            server: false,
                            datetime: "2024-04-08T08:08:11.286044Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "Zam did some stardew speed runs I believe",
                            who: Some(
                                "adrone",
                            ),
                            discord: true,
                            server: false,
                            datetime: "2024-04-08T12:28:50.193915Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "The server has successfully started at: Mon Apr 08 14:12:50 UTC 2024 (c936808df2b78aa4a24be7b0d622fceb75393f17)",
                            who: Some(
                                "__server",
                            ),
                            discord: false,
                            server: true,
                            datetime: "2024-04-08T14:12:50.282482Z",
                            room: Some(
                                "lobby",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                    ],
                    unread: 0,
                },
            ),
            Chat(
                ChatData {
                    msg: "Find teammates and discuss strategy in the <a href=\"https://discord.gg/FADvkJp\" target=\"_blank\" rel=\"noopener noreferrer\">Discord chat</a>.",
                    who: Some(
                        "",
                    ),
                    discord: false,
                    server: true,
                    datetime: "2024-04-08T18:53:51.082662958Z",
                    room: Some(
                        "lobby",
                    ),
                    recipient: Some(
                        "",
                    ),
                },
            ),
            Chat(
                ChatData {
                    msg: "[Server Notice] January 27th, 2024 - There are new Critical Fours variant combinations. (Thanks Ramanujan)",
                    who: Some(
                        "",
                    ),
                    discord: false,
                    server: true,
                    datetime: "2024-04-08T18:53:51.082749731Z",
                    room: Some(
                        "lobby",
                    ),
                    recipient: Some(
                        "",
                    ),
                },
            ),
            GameHistory(
                [
                    GameHistory {
                        id: 1141196,
                        options: GameOptions {
                            num_players: Some(
                                3,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "Matryoshka & Prism (6 Suits)",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: false,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p3v2047s2",
                        score: 30,
                        num_turns: 72,
                        end_condition: Normal,
                        datetime_started: 2024-04-08T06:20:50.058635Z,
                        datetime_finished: 2024-04-08T06:43:18.427194Z,
                        num_games_on_this_seed: 2,
                        player_names: [
                            "etw",
                            "njha",
                            "oliver",
                        ],
                        increment_num_games: false,
                        tags: "bottom deck win",
                    },
                    GameHistory {
                        id: 1141193,
                        options: GameOptions {
                            num_players: Some(
                                3,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "Matryoshka & Prism (6 Suits)",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: false,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p3v2047s1",
                        score: 0,
                        num_turns: 2,
                        end_condition: TerminatedByPlayer,
                        datetime_started: 2024-04-08T06:19:44.677869Z,
                        datetime_finished: 2024-04-08T06:20:46.750333Z,
                        num_games_on_this_seed: 4,
                        player_names: [
                            "etw",
                            "njha",
                            "oliver",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                    GameHistory {
                        id: 1141191,
                        options: GameOptions {
                            num_players: Some(
                                4,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "Matryoshka (5 Suits)",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: false,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p4v193s8",
                        score: 25,
                        num_turns: 52,
                        end_condition: Normal,
                        datetime_started: 2024-04-08T05:55:14.350466Z,
                        datetime_finished: 2024-04-08T06:13:02.394274Z,
                        num_games_on_this_seed: 2,
                        player_names: [
                            "etw",
                            "girantinas",
                            "njha",
                            "oliver",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                    GameHistory {
                        id: 1141187,
                        options: GameOptions {
                            num_players: Some(
                                4,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "Matryoshka (5 Suits)",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: false,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p4v193s7",
                        score: 0,
                        num_turns: 9,
                        end_condition: TerminatedByPlayer,
                        datetime_started: 2024-04-08T05:50:00.135704Z,
                        datetime_finished: 2024-04-08T05:54:24.618478Z,
                        num_games_on_this_seed: 2,
                        player_names: [
                            "etw",
                            "girantinas",
                            "njha",
                            "oliver",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                    GameHistory {
                        id: 1141184,
                        options: GameOptions {
                            num_players: Some(
                                4,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "Matryoshka (5 Suits)",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: false,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p4v193s6",
                        score: 25,
                        num_turns: 52,
                        end_condition: Normal,
                        datetime_started: 2024-04-08T05:26:27.098440Z,
                        datetime_finished: 2024-04-08T05:46:30.393649Z,
                        num_games_on_this_seed: 3,
                        player_names: [
                            "etw",
                            "girantinas",
                            "njha",
                            "oliver",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                    GameHistory {
                        id: 1141180,
                        options: GameOptions {
                            num_players: Some(
                                4,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "Matryoshka (5 Suits)",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: false,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p4v193s5",
                        score: 24,
                        num_turns: 56,
                        end_condition: Normal,
                        datetime_started: 2024-04-08T05:08:03.631922Z,
                        datetime_finished: 2024-04-08T05:25:00.806462Z,
                        num_games_on_this_seed: 3,
                        player_names: [
                            "etw",
                            "girantinas",
                            "njha",
                            "oliver",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                    GameHistory {
                        id: 1141177,
                        options: GameOptions {
                            num_players: Some(
                                4,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "Matryoshka (5 Suits)",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: false,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p4v193s4",
                        score: 0,
                        num_turns: 7,
                        end_condition: TerminatedByPlayer,
                        datetime_started: 2024-04-08T05:04:57.224439Z,
                        datetime_finished: 2024-04-08T05:07:58.918974Z,
                        num_games_on_this_seed: 6,
                        player_names: [
                            "etw",
                            "girantinas",
                            "njha",
                            "oliver",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                    GameHistory {
                        id: 1141175,
                        options: GameOptions {
                            num_players: Some(
                                4,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "No Variant",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: false,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p4v0s81",
                        score: 25,
                        num_turns: 55,
                        end_condition: Normal,
                        datetime_started: 2024-04-08T04:43:03.656711Z,
                        datetime_finished: 2024-04-08T05:03:10.588732Z,
                        num_games_on_this_seed: 98,
                        player_names: [
                            "etw",
                            "girantinas",
                            "njha",
                            "oliver",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                    GameHistory {
                        id: 1141169,
                        options: GameOptions {
                            num_players: Some(
                                4,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "No Variant",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: false,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p4v0s80",
                        score: 25,
                        num_turns: 48,
                        end_condition: Normal,
                        datetime_started: 2024-04-08T04:27:57.272295Z,
                        datetime_finished: 2024-04-08T04:38:45.335379Z,
                        num_games_on_this_seed: 97,
                        player_names: [
                            "etw",
                            "girantinas",
                            "njha",
                            "oliver",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                    GameHistory {
                        id: 1141167,
                        options: GameOptions {
                            num_players: Some(
                                3,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "No Variant",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: false,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p3v0s178",
                        score: 0,
                        num_turns: 1,
                        end_condition: TerminatedByPlayer,
                        datetime_started: 2024-04-08T04:27:32.576888Z,
                        datetime_finished: 2024-04-08T04:27:47.834132Z,
                        num_games_on_this_seed: 78,
                        player_names: [
                            "etw",
                            "girantinas",
                            "njha",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                ],
            ),
            GameHistoryFriends(
                [
                    GameHistory {
                        id: 1141231,
                        options: GameOptions {
                            num_players: Some(
                                2,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "No Variant",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: true,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p2v0s551",
                        score: 25,
                        num_turns: 64,
                        end_condition: Normal,
                        datetime_started: 2024-04-08T09:51:07.359230Z,
                        datetime_finished: 2024-04-08T09:52:48.914552Z,
                        num_games_on_this_seed: 61,
                        player_names: [
                            "etw",
                            "will-bot4",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                    GameHistory {
                        id: 1141230,
                        options: GameOptions {
                            num_players: Some(
                                2,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "No Variant",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: true,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p2v0s550",
                        score: 0,
                        num_turns: 12,
                        end_condition: TerminatedByPlayer,
                        datetime_started: 2024-04-08T09:50:46.789267Z,
                        datetime_finished: 2024-04-08T09:51:05.199777Z,
                        num_games_on_this_seed: 62,
                        player_names: [
                            "etw",
                            "will-bot4",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                    GameHistory {
                        id: 1141229,
                        options: GameOptions {
                            num_players: Some(
                                2,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "No Variant",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: true,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p2v0s549",
                        score: 0,
                        num_turns: 34,
                        end_condition: TerminatedByPlayer,
                        datetime_started: 2024-04-08T09:49:45.392698Z,
                        datetime_finished: 2024-04-08T09:50:44.455814Z,
                        num_games_on_this_seed: 58,
                        player_names: [
                            "etw",
                            "will-bot4",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                    GameHistory {
                        id: 1141228,
                        options: GameOptions {
                            num_players: Some(
                                2,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "No Variant",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: true,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p2v0s548",
                        score: 0,
                        num_turns: 22,
                        end_condition: SpeedrunFail,
                        datetime_started: 2024-04-08T09:49:01.739978Z,
                        datetime_finished: 2024-04-08T09:49:41.847660Z,
                        num_games_on_this_seed: 60,
                        player_names: [
                            "etw",
                            "will-bot4",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                    GameHistory {
                        id: 1141227,
                        options: GameOptions {
                            num_players: Some(
                                2,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "No Variant",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: true,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p2v0s547",
                        score: 23,
                        num_turns: 63,
                        end_condition: Normal,
                        datetime_started: 2024-04-08T09:47:09.128935Z,
                        datetime_finished: 2024-04-08T09:48:55.403798Z,
                        num_games_on_this_seed: 58,
                        player_names: [
                            "etw",
                            "will-bot4",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                    GameHistory {
                        id: 1141226,
                        options: GameOptions {
                            num_players: Some(
                                2,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "No Variant",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: true,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p2v0s546",
                        score: 25,
                        num_turns: 60,
                        end_condition: Normal,
                        datetime_started: 2024-04-08T09:38:11.720675Z,
                        datetime_finished: 2024-04-08T09:40:11.821460Z,
                        num_games_on_this_seed: 58,
                        player_names: [
                            "etw",
                            "will-bot4",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                    GameHistory {
                        id: 1141225,
                        options: GameOptions {
                            num_players: Some(
                                2,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "No Variant",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: true,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p2v0s545",
                        score: 0,
                        num_turns: 36,
                        end_condition: SpeedrunFail,
                        datetime_started: 2024-04-08T09:37:14.141530Z,
                        datetime_finished: 2024-04-08T09:38:08.591732Z,
                        num_games_on_this_seed: 62,
                        player_names: [
                            "etw",
                            "will-bot4",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                    GameHistory {
                        id: 1141224,
                        options: GameOptions {
                            num_players: Some(
                                2,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "No Variant",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: true,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p2v0s544",
                        score: 24,
                        num_turns: 66,
                        end_condition: Normal,
                        datetime_started: 2024-04-08T09:35:32.542377Z,
                        datetime_finished: 2024-04-08T09:37:07.230843Z,
                        num_games_on_this_seed: 58,
                        player_names: [
                            "etw",
                            "will-bot4",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                    GameHistory {
                        id: 1141221,
                        options: GameOptions {
                            num_players: Some(
                                2,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "No Variant",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: true,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p2v0s543",
                        score: 0,
                        num_turns: 8,
                        end_condition: TerminatedByPlayer,
                        datetime_started: 2024-04-08T09:35:08.282597Z,
                        datetime_finished: 2024-04-08T09:35:30.183835Z,
                        num_games_on_this_seed: 58,
                        player_names: [
                            "etw",
                            "will-bot4",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                    GameHistory {
                        id: 1141220,
                        options: GameOptions {
                            num_players: Some(
                                2,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "No Variant",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: true,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p2v0s542",
                        score: 0,
                        num_turns: 33,
                        end_condition: SpeedrunFail,
                        datetime_started: 2024-04-08T09:33:44.943531Z,
                        datetime_finished: 2024-04-08T09:34:56.283415Z,
                        num_games_on_this_seed: 62,
                        player_names: [
                            "etw",
                            "will-bot4",
                        ],
                        increment_num_games: false,
                        tags: "",
                    },
                ],
            ),
            UserLeft(
                UserId {
                    user_id: 91466,
                },
            ),
            GetName(
                CommandData {
                    table_id: None,
                    database_id: None,
                },
            ),
            Setting(
                CommandSettingData {
                    setting: "true",
                },
            ),
            TableCreate(
                CommandTableCreateData {
                    name: "Shauna anxiously daffier",
                    options: GameOptions {
                        num_players: None,
                        starting_player: None,
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    password: "boba",
                    max_players: 6,
                },
            ),
            Table(
                TableData {
                    id: 15029,
                    joined: true,
                    max_players: 6,
                    name: "Shauna anxiously daffier",
                    num_players: 1,
                    options: GameOptions {
                        num_players: Some(
                            0,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: true,
                    password_protected: true,
                    players: [
                        "njha",
                    ],
                    progress: 0,
                    running: false,
                    shared_replay: false,
                    spectators: [],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Matryoshka (5 Suits)",
                },
            ),
            Game(
                GameMessage {
                    table_id: 15029,
                    name: "Shauna anxiously daffier",
                    owner: 88454,
                    players: [
                        GamePlayerMessage {
                            index: 0,
                            name: "njha",
                            you: true,
                            present: true,
                            stats: Some(
                                PregameStats {
                                    num_games: 105,
                                    variant: UserStatsRow {
                                        num_games: 10,
                                        best_scores: [
                                            BestScore {
                                                num_players: 2,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 3,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 4,
                                                score: 25,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 5,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 6,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                        ],
                                        average_score: 24.666666666666668,
                                        num_strikeouts: 7,
                                    },
                                },
                            ),
                        },
                    ],
                    options: GameOptions {
                        num_players: Some(
                            0,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    password_protected: true,
                    max_players: 6,
                },
            ),
            User(
                UserData {
                    user_id: 88454,
                    name: "njha",
                    status: 1,
                    table_id: Some(
                        15029,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            Joined(
                CommandData {
                    table_id: Some(
                        15029,
                    ),
                    database_id: None,
                },
            ),
            ChatList(
                ChatListData {
                    list: [
                        ChatData {
                            msg: "<strong>njha</strong> created the table.",
                            who: Some(
                                "",
                            ),
                            discord: false,
                            server: true,
                            datetime: "2024-04-08T18:54:18.639825053Z",
                            room: Some(
                                "table15029",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                    ],
                    unread: 1,
                },
            ),
            PregameSpectators(
                SpectatorsMessage {
                    table_id: 15029,
                    spectators: [],
                },
            ),
            ChatTyping(
                CommandData {
                    table_id: Some(
                        15029,
                    ),
                    database_id: None,
                },
            ),
            ChatTyping(
                CommandData {
                    table_id: Some(
                        15029,
                    ),
                    database_id: None,
                },
            ),
            ChatTyping(
                CommandData {
                    table_id: Some(
                        15029,
                    ),
                    database_id: None,
                },
            ),
            ChatTyping(
                CommandData {
                    table_id: Some(
                        15029,
                    ),
                    database_id: None,
                },
            ),
            ChatPM(
                ChatPMData {
                    msg: "/join boba",
                    recipient: "will-bot1",
                    room: "table15029",
                },
            ),
            Chat(
                ChatData {
                    msg: "/join boba",
                    who: Some(
                        "njha",
                    ),
                    discord: false,
                    server: false,
                    datetime: "2024-04-08T18:54:32.600957576Z",
                    room: Some(
                        "",
                    ),
                    recipient: Some(
                        "will-bot1",
                    ),
                },
            ),
            Table(
                TableData {
                    id: 15029,
                    joined: true,
                    max_players: 6,
                    name: "Shauna anxiously daffier",
                    num_players: 2,
                    options: GameOptions {
                        num_players: Some(
                            0,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: true,
                    password_protected: true,
                    players: [
                        "njha",
                        "will-bot1",
                    ],
                    progress: 0,
                    running: false,
                    shared_replay: false,
                    spectators: [],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Matryoshka (5 Suits)",
                },
            ),
            Game(
                GameMessage {
                    table_id: 15029,
                    name: "Shauna anxiously daffier",
                    owner: 88454,
                    players: [
                        GamePlayerMessage {
                            index: 0,
                            name: "njha",
                            you: true,
                            present: true,
                            stats: Some(
                                PregameStats {
                                    num_games: 105,
                                    variant: UserStatsRow {
                                        num_games: 10,
                                        best_scores: [
                                            BestScore {
                                                num_players: 2,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 3,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 4,
                                                score: 25,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 5,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 6,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                        ],
                                        average_score: 24.666666666666668,
                                        num_strikeouts: 7,
                                    },
                                },
                            ),
                        },
                        GamePlayerMessage {
                            index: 1,
                            name: "will-bot1",
                            you: false,
                            present: true,
                            stats: Some(
                                PregameStats {
                                    num_games: 1685,
                                    variant: UserStatsRow {
                                        num_games: 0,
                                        best_scores: [
                                            BestScore {
                                                num_players: 2,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 3,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 4,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 5,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 6,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                        ],
                                        average_score: 0.0,
                                        num_strikeouts: 0,
                                    },
                                },
                            ),
                        },
                    ],
                    options: GameOptions {
                        num_players: Some(
                            0,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    password_protected: true,
                    max_players: 6,
                },
            ),
            User(
                UserData {
                    user_id: 78102,
                    name: "will-bot1",
                    status: 1,
                    table_id: Some(
                        15029,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            Chat(
                ChatData {
                    msg: "will-bot1 joined the game.",
                    who: Some(
                        "",
                    ),
                    discord: false,
                    server: true,
                    datetime: "2024-04-08T18:54:33.289466101Z",
                    room: Some(
                        "table15029",
                    ),
                    recipient: Some(
                        "",
                    ),
                },
            ),
            ChatRead(
                CommandData {
                    table_id: Some(
                        15029,
                    ),
                    database_id: None,
                },
            ),
            PregameSpectators(
                SpectatorsMessage {
                    table_id: 15029,
                    spectators: [],
                },
            ),
            TableStart(
                CommandTableStartData {
                    intended_players: [
                        "njha",
                        "will-bot1",
                    ],
                },
            ),
            TableStart(
                CommandTableStartData {
                    intended_players: [],
                },
            ),
            GetGameInfo1(
                CommandData {
                    table_id: Some(
                        15029,
                    ),
                    database_id: None,
                },
            ),
            Table(
                TableData {
                    id: 15029,
                    joined: true,
                    max_players: 6,
                    name: "Shauna anxiously daffier",
                    num_players: 2,
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: true,
                    password_protected: true,
                    players: [
                        "will-bot1",
                        "njha",
                    ],
                    progress: 0,
                    running: true,
                    shared_replay: false,
                    spectators: [],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Matryoshka (5 Suits)",
                },
            ),
            User(
                UserData {
                    user_id: 78102,
                    name: "will-bot1",
                    status: 2,
                    table_id: Some(
                        15029,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            User(
                UserData {
                    user_id: 88454,
                    name: "njha",
                    status: 2,
                    table_id: Some(
                        15029,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            Init(
                InitMessage {
                    table_id: 15029,
                    player_names: [
                        "will-bot1",
                        "njha",
                    ],
                    our_player_index: 1,
                    spectating: false,
                    shadowing: false,
                    replay: false,
                    database_id: -1,
                    has_custom_seed: false,
                    seed: "p2v193s1",
                    datetime_started: 2024-04-08T18:54:36.999797998Z,
                    datetime_finished: 0001-01-01T00:00:00Z,
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    character_assignments: [],
                    character_metadata: [],
                    shared_replay: false,
                    shared_replay_leader: "njha",
                    shared_replay_segment: 0,
                    shared_replay_eff_mod: 0,
                    paused: false,
                    pause_player_index: -1,
                    pause_queued: false,
                },
            ),
            GetGameInfo2(
                CommandData {
                    table_id: Some(
                        15029,
                    ),
                    database_id: None,
                },
            ),
            GameActionList(
                GameActionListMessage {
                    table_id: 15029,
                    list: Array [
                        Object {
                            "order": Number(0),
                            "playerIndex": Number(0),
                            "rank": Number(1),
                            "suitIndex": Number(3),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(1),
                            "playerIndex": Number(0),
                            "rank": Number(3),
                            "suitIndex": Number(0),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(2),
                            "playerIndex": Number(0),
                            "rank": Number(2),
                            "suitIndex": Number(2),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(3),
                            "playerIndex": Number(0),
                            "rank": Number(4),
                            "suitIndex": Number(3),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(4),
                            "playerIndex": Number(0),
                            "rank": Number(1),
                            "suitIndex": Number(1),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(5),
                            "playerIndex": Number(1),
                            "rank": Number(-1),
                            "suitIndex": Number(-1),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(6),
                            "playerIndex": Number(1),
                            "rank": Number(-1),
                            "suitIndex": Number(-1),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(7),
                            "playerIndex": Number(1),
                            "rank": Number(-1),
                            "suitIndex": Number(-1),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(8),
                            "playerIndex": Number(1),
                            "rank": Number(-1),
                            "suitIndex": Number(-1),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(9),
                            "playerIndex": Number(1),
                            "rank": Number(-1),
                            "suitIndex": Number(-1),
                            "type": String("draw"),
                        },
                    ],
                },
            ),
            Loaded(
                CommandData {
                    table_id: Some(
                        15029,
                    ),
                    database_id: None,
                },
            ),
            Connected(
                ConnectedMessage {
                    table_id: 15029,
                    list: [
                        false,
                        false,
                    ],
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15029,
                    times: [
                        -1021,
                        0,
                    ],
                    active_player_index: 0,
                    time_taken: 1021,
                },
            ),
            NoteListPlayer(
                NoteListPlayerMessage {
                    table_id: 15029,
                    notes: [
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                    ],
                },
            ),
            VoteChange(
                VoteMessage {
                    vote: false,
                },
            ),
            Spectators(
                SpectatorsMessage {
                    table_id: 15029,
                    spectators: [],
                },
            ),
            ChatList(
                ChatListData {
                    list: [
                        ChatData {
                            msg: "<strong>njha</strong> created the table.",
                            who: Some(
                                "",
                            ),
                            discord: false,
                            server: true,
                            datetime: "2024-04-08T18:54:18.639825053Z",
                            room: Some(
                                "table15029",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "will-bot1 joined the game.",
                            who: Some(
                                "",
                            ),
                            discord: false,
                            server: true,
                            datetime: "2024-04-08T18:54:33.289466101Z",
                            room: Some(
                                "table15029",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                    ],
                    unread: 0,
                },
            ),
            Connected(
                ConnectedMessage {
                    table_id: 15029,
                    list: [
                        false,
                        true,
                    ],
                },
            ),
            Connected(
                ConnectedMessage {
                    table_id: 15029,
                    list: [
                        true,
                        true,
                    ],
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15029,
                    times: [
                        0,
                        0,
                    ],
                    active_player_index: 0,
                    time_taken: 0,
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Clue {
                        clue: Clue {
                            clue_type: 1,
                            value: 5,
                        },
                        giver: 0,
                        list: [
                            8,
                            9,
                        ],
                        target: 1,
                        turn: 0,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Status {
                        clues: 7,
                        score: 0,
                        max_score: 25,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Turn {
                        num: 1,
                        current_player_index: 1,
                    },
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15029,
                    times: [
                        -2686,
                        0,
                    ],
                    active_player_index: 1,
                    time_taken: 0,
                },
            ),
            Action(
                ActionWithTableID {
                    table_id: 15029,
                    action: RankClue {
                        target: 0,
                        value: 1,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Clue {
                        clue: Clue {
                            clue_type: 1,
                            value: 1,
                        },
                        giver: 1,
                        list: [
                            0,
                            4,
                        ],
                        target: 0,
                        turn: 1,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Status {
                        clues: 6,
                        score: 0,
                        max_score: 25,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Turn {
                        num: 2,
                        current_player_index: 0,
                    },
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15029,
                    times: [
                        -2687,
                        -2436,
                    ],
                    active_player_index: 0,
                    time_taken: 0,
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Play {
                        player_index: 0,
                        order: 4,
                        suit_index: 1,
                        rank: 1,
                    },
                },
            ),
            TableProgress(
                TableProgressMessage {
                    table_id: 15029,
                    progress: 4,
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Draw {
                        player_index: 0,
                        order: 10,
                        suit_index: 1,
                        rank: 2,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Status {
                        clues: 6,
                        score: 1,
                        max_score: 25,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Turn {
                        num: 3,
                        current_player_index: 1,
                    },
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15029,
                    times: [
                        -2871,
                        -2436,
                    ],
                    active_player_index: 1,
                    time_taken: 0,
                },
            ),
            Note(
                CommandNoteData {
                    note: " [cm]",
                    order: 5,
                },
            ),
            Table(
                TableData {
                    id: 15020,
                    joined: false,
                    max_players: 5,
                    name: "unsettles photosynthesized preemptive (#2)",
                    num_players: 4,
                    options: GameOptions {
                        num_players: Some(
                            4,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Brown-Fives & Brown (6 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: false,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: false,
                    password_protected: false,
                    players: [
                        "Cocorifle",
                        "TimeHoodie",
                        "ElenaDhynho",
                        "yagami_black",
                    ],
                    progress: 27,
                    running: true,
                    shared_replay: false,
                    spectators: [
                        Spectator {
                            name: "HelanaAshryvr",
                            shadowing_player_index: Some(
                                1,
                            ),
                            shadowing_player_username: Some(
                                "TimeHoodie",
                            ),
                        },
                        Spectator {
                            name: "Simon",
                            shadowing_player_index: Some(
                                1,
                            ),
                            shadowing_player_username: Some(
                                "TimeHoodie",
                            ),
                        },
                    ],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Brown-Fives & Brown (6 Suits)",
                },
            ),
            User(
                UserData {
                    user_id: 24679,
                    name: "Simon",
                    status: 3,
                    table_id: Some(
                        15020,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            Action(
                ActionWithTableID {
                    table_id: 15029,
                    action: ColorClue {
                        target: 0,
                        value: 1,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Clue {
                        clue: Clue {
                            clue_type: 0,
                            value: 1,
                        },
                        giver: 1,
                        list: [
                            0,
                            2,
                            3,
                            10,
                        ],
                        target: 0,
                        turn: 3,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Status {
                        clues: 5,
                        score: 1,
                        max_score: 25,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Turn {
                        num: 4,
                        current_player_index: 0,
                    },
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15029,
                    times: [
                        -2872,
                        -11998,
                    ],
                    active_player_index: 0,
                    time_taken: 0,
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Clue {
                        clue: Clue {
                            clue_type: 1,
                            value: 3,
                        },
                        giver: 0,
                        list: [
                            5,
                        ],
                        target: 1,
                        turn: 4,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Status {
                        clues: 4,
                        score: 1,
                        max_score: 25,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Turn {
                        num: 5,
                        current_player_index: 1,
                    },
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15029,
                    times: [
                        -3240,
                        -11998,
                    ],
                    active_player_index: 1,
                    time_taken: 0,
                },
            ),
            User(
                UserData {
                    user_id: 91466,
                    name: "ashishla",
                    status: 0,
                    table_id: Some(
                        0,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            TableGone(
                CommandData {
                    table_id: Some(
                        15024,
                    ),
                    database_id: None,
                },
            ),
            User(
                UserData {
                    user_id: 2000,
                    name: "HuHu",
                    status: 0,
                    table_id: Some(
                        0,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            User(
                UserData {
                    user_id: 75662,
                    name: "gsymon",
                    status: 0,
                    table_id: Some(
                        0,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            User(
                UserData {
                    user_id: 2000,
                    name: "HuHu",
                    status: 5,
                    table_id: Some(
                        15024,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            User(
                UserData {
                    user_id: 75662,
                    name: "gsymon",
                    status: 5,
                    table_id: Some(
                        15024,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            Table(
                TableData {
                    id: 15024,
                    joined: false,
                    max_players: 6,
                    name: "veterinarians Bantu gentries (#10) (Game #1141356)",
                    num_players: 2,
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Rainbow-Fives & Dark Pink (6 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: false,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: false,
                    password_protected: true,
                    players: [
                        "HuHu",
                        "gsymon",
                    ],
                    progress: 100,
                    running: true,
                    shared_replay: true,
                    spectators: [
                        Spectator {
                            name: "HuHu",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                        Spectator {
                            name: "gsymon",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                    ],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Rainbow-Fives & Dark Pink (6 Suits)",
                },
            ),
            User(
                UserData {
                    user_id: 24679,
                    name: "Simon",
                    status: 0,
                    table_id: Some(
                        0,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            Table(
                TableData {
                    id: 15020,
                    joined: false,
                    max_players: 5,
                    name: "unsettles photosynthesized preemptive (#2)",
                    num_players: 4,
                    options: GameOptions {
                        num_players: Some(
                            4,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Brown-Fives & Brown (6 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: false,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: false,
                    password_protected: false,
                    players: [
                        "Cocorifle",
                        "TimeHoodie",
                        "ElenaDhynho",
                        "yagami_black",
                    ],
                    progress: 27,
                    running: true,
                    shared_replay: false,
                    spectators: [
                        Spectator {
                            name: "HelanaAshryvr",
                            shadowing_player_index: Some(
                                1,
                            ),
                            shadowing_player_username: Some(
                                "TimeHoodie",
                            ),
                        },
                    ],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Brown-Fives & Brown (6 Suits)",
                },
            ),
            Table(
                TableData {
                    id: 15020,
                    joined: false,
                    max_players: 5,
                    name: "unsettles photosynthesized preemptive (#2)",
                    num_players: 4,
                    options: GameOptions {
                        num_players: Some(
                            4,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Brown-Fives & Brown (6 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: false,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: false,
                    password_protected: false,
                    players: [
                        "Cocorifle",
                        "TimeHoodie",
                        "ElenaDhynho",
                        "yagami_black",
                    ],
                    progress: 27,
                    running: true,
                    shared_replay: false,
                    spectators: [
                        Spectator {
                            name: "HelanaAshryvr",
                            shadowing_player_index: Some(
                                1,
                            ),
                            shadowing_player_username: Some(
                                "TimeHoodie",
                            ),
                        },
                        Spectator {
                            name: "Simon",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                    ],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Brown-Fives & Brown (6 Suits)",
                },
            ),
            User(
                UserData {
                    user_id: 24679,
                    name: "Simon",
                    status: 3,
                    table_id: Some(
                        15020,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            Action(
                ActionWithTableID {
                    table_id: 15029,
                    action: Discard {
                        target: 6,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Discard {
                        player_index: 1,
                        order: 6,
                        suit_index: 1,
                        rank: 4,
                        failed: false,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Draw {
                        player_index: 1,
                        order: 11,
                        suit_index: -1,
                        rank: -1,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Status {
                        clues: 5,
                        score: 1,
                        max_score: 25,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Turn {
                        num: 6,
                        current_player_index: 0,
                    },
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15029,
                    times: [
                        -3241,
                        -25268,
                    ],
                    active_player_index: 0,
                    time_taken: 0,
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Play {
                        player_index: 0,
                        order: 10,
                        suit_index: 1,
                        rank: 2,
                    },
                },
            ),
            TableProgress(
                TableProgressMessage {
                    table_id: 15029,
                    progress: 8,
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Draw {
                        player_index: 0,
                        order: 12,
                        suit_index: 2,
                        rank: 2,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Status {
                        clues: 5,
                        score: 2,
                        max_score: 25,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Turn {
                        num: 7,
                        current_player_index: 1,
                    },
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15029,
                    times: [
                        -3565,
                        -25269,
                    ],
                    active_player_index: 1,
                    time_taken: 0,
                },
            ),
            User(
                UserData {
                    user_id: 24679,
                    name: "Simon",
                    status: 0,
                    table_id: Some(
                        0,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            Table(
                TableData {
                    id: 15020,
                    joined: false,
                    max_players: 5,
                    name: "unsettles photosynthesized preemptive (#2)",
                    num_players: 4,
                    options: GameOptions {
                        num_players: Some(
                            4,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Brown-Fives & Brown (6 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: false,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: false,
                    password_protected: false,
                    players: [
                        "Cocorifle",
                        "TimeHoodie",
                        "ElenaDhynho",
                        "yagami_black",
                    ],
                    progress: 27,
                    running: true,
                    shared_replay: false,
                    spectators: [
                        Spectator {
                            name: "HelanaAshryvr",
                            shadowing_player_index: Some(
                                1,
                            ),
                            shadowing_player_username: Some(
                                "TimeHoodie",
                            ),
                        },
                    ],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Brown-Fives & Brown (6 Suits)",
                },
            ),
            Action(
                ActionWithTableID {
                    table_id: 15029,
                    action: Play {
                        target: 5,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Play {
                        player_index: 1,
                        order: 5,
                        suit_index: 1,
                        rank: 3,
                    },
                },
            ),
            TableProgress(
                TableProgressMessage {
                    table_id: 15029,
                    progress: 12,
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Draw {
                        player_index: 1,
                        order: 13,
                        suit_index: -1,
                        rank: -1,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Status {
                        clues: 5,
                        score: 3,
                        max_score: 25,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Turn {
                        num: 8,
                        current_player_index: 0,
                    },
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15029,
                    times: [
                        -3565,
                        -27420,
                    ],
                    active_player_index: 0,
                    time_taken: 0,
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Clue {
                        clue: Clue {
                            clue_type: 1,
                            value: 4,
                        },
                        giver: 0,
                        list: [
                            7,
                            11,
                        ],
                        target: 1,
                        turn: 8,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Status {
                        clues: 4,
                        score: 3,
                        max_score: 25,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Turn {
                        num: 9,
                        current_player_index: 1,
                    },
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15029,
                    times: [
                        -3897,
                        -27420,
                    ],
                    active_player_index: 1,
                    time_taken: 0,
                },
            ),
            Action(
                ActionWithTableID {
                    table_id: 15029,
                    action: Discard {
                        target: 13,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Discard {
                        player_index: 1,
                        order: 13,
                        suit_index: 2,
                        rank: 1,
                        failed: false,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Draw {
                        player_index: 1,
                        order: 14,
                        suit_index: -1,
                        rank: -1,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Status {
                        clues: 5,
                        score: 3,
                        max_score: 25,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Turn {
                        num: 10,
                        current_player_index: 0,
                    },
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15029,
                    times: [
                        -3897,
                        -35862,
                    ],
                    active_player_index: 0,
                    time_taken: 0,
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Discard {
                        player_index: 0,
                        order: 0,
                        suit_index: 3,
                        rank: 1,
                        failed: false,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Draw {
                        player_index: 0,
                        order: 15,
                        suit_index: 1,
                        rank: 2,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Status {
                        clues: 6,
                        score: 3,
                        max_score: 25,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Turn {
                        num: 11,
                        current_player_index: 1,
                    },
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15029,
                    times: [
                        -4015,
                        -35863,
                    ],
                    active_player_index: 1,
                    time_taken: 0,
                },
            ),
            Table(
                TableData {
                    id: 15029,
                    joined: true,
                    max_players: 6,
                    name: "Shauna anxiously daffier",
                    num_players: 2,
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: true,
                    password_protected: true,
                    players: [
                        "will-bot1",
                        "njha",
                    ],
                    progress: 12,
                    running: true,
                    shared_replay: false,
                    spectators: [
                        Spectator {
                            name: "ashishla",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                    ],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Matryoshka (5 Suits)",
                },
            ),
            Spectators(
                SpectatorsMessage {
                    table_id: 15029,
                    spectators: [
                        Spectator {
                            name: "ashishla",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                    ],
                },
            ),
            User(
                UserData {
                    user_id: 91466,
                    name: "ashishla",
                    status: 3,
                    table_id: Some(
                        15029,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            Action(
                ActionWithTableID {
                    table_id: 15029,
                    action: Play {
                        target: 14,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Strike {
                        num: 1,
                        turn: 11,
                        order: 14,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Discard {
                        player_index: 1,
                        order: 14,
                        suit_index: 2,
                        rank: 4,
                        failed: true,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Draw {
                        player_index: 1,
                        order: 16,
                        suit_index: -1,
                        rank: -1,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Status {
                        clues: 6,
                        score: 3,
                        max_score: 25,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Turn {
                        num: 12,
                        current_player_index: 0,
                    },
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15029,
                    times: [
                        -4015,
                        -40406,
                    ],
                    active_player_index: 0,
                    time_taken: 0,
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Discard {
                        player_index: 0,
                        order: 1,
                        suit_index: 0,
                        rank: 3,
                        failed: false,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Draw {
                        player_index: 0,
                        order: 17,
                        suit_index: 1,
                        rank: 1,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Status {
                        clues: 7,
                        score: 3,
                        max_score: 25,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Turn {
                        num: 13,
                        current_player_index: 1,
                    },
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15029,
                    times: [
                        -4188,
                        -40406,
                    ],
                    active_player_index: 1,
                    time_taken: 0,
                },
            ),
            TableVoteForTermination(
                CommandVotesData {
                    votes: [],
                },
            ),
            VoteChange(
                VoteMessage {
                    vote: true,
                },
            ),
            TableVoteForTermination(
                CommandVotesData {
                    votes: [],
                },
            ),
            VoteChange(
                VoteMessage {
                    vote: false,
                },
            ),
            TableTerminate(
                CommandData {
                    table_id: Some(
                        15029,
                    ),
                    database_id: None,
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Status {
                        clues: 7,
                        score: 3,
                        max_score: 25,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: GameOver {
                        end_condition: 4,
                        player_index: 1,
                        votes: None,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: Turn {
                        num: 14,
                        current_player_index: -1,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15029,
                    action: PlayerTimes {
                        player_times: [
                            -4188,
                            -40406,
                        ],
                        duration: 53660,
                    },
                },
            ),
            TableGone(
                CommandData {
                    table_id: Some(
                        15029,
                    ),
                    database_id: None,
                },
            ),
            User(
                UserData {
                    user_id: 78102,
                    name: "will-bot1",
                    status: 0,
                    table_id: Some(
                        0,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            User(
                UserData {
                    user_id: 88454,
                    name: "njha",
                    status: 0,
                    table_id: Some(
                        0,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            GameHistory(
                [
                    GameHistory {
                        id: 1141357,
                        options: GameOptions {
                            num_players: Some(
                                2,
                            ),
                            starting_player: Some(
                                0,
                            ),
                            variant_name: "Matryoshka (5 Suits)",
                            timed: false,
                            time_base: 0,
                            time_per_turn: 0,
                            speedrun: true,
                            card_cycle: false,
                            deck_plays: false,
                            empty_clues: false,
                            one_extra_card: false,
                            one_less_card: false,
                            all_or_nothing: false,
                            detrimental_characters: false,
                            table_name: None,
                            max_players: None,
                        },
                        seed: "p2v193s1",
                        score: 0,
                        num_turns: 14,
                        end_condition: TerminatedByPlayer,
                        datetime_started: 2024-04-08T18:54:36.999797998Z,
                        datetime_finished: 2024-04-08T18:55:30.660383880Z,
                        num_games_on_this_seed: 62,
                        player_names: [
                            "njha",
                            "will-bot1",
                        ],
                        increment_num_games: true,
                        tags: "",
                    },
                ],
            ),
            Connected(
                ConnectedMessage {
                    table_id: 15029,
                    list: [
                        true,
                        true,
                    ],
                },
            ),
            CardIdentities(
                CardIdentitiesMessage {
                    table_id: 15029,
                    card_identities: [
                        CardIdentity {
                            suit_index: 3,
                            rank: 1,
                        },
                        CardIdentity {
                            suit_index: 0,
                            rank: 3,
                        },
                        CardIdentity {
                            suit_index: 2,
                            rank: 2,
                        },
                        CardIdentity {
                            suit_index: 3,
                            rank: 4,
                        },
                        CardIdentity {
                            suit_index: 1,
                            rank: 1,
                        },
                        CardIdentity {
                            suit_index: 1,
                            rank: 3,
                        },
                        CardIdentity {
                            suit_index: 1,
                            rank: 4,
                        },
                        CardIdentity {
                            suit_index: 1,
                            rank: 4,
                        },
                        CardIdentity {
                            suit_index: 4,
                            rank: 5,
                        },
                        CardIdentity {
                            suit_index: 2,
                            rank: 5,
                        },
                        CardIdentity {
                            suit_index: 1,
                            rank: 2,
                        },
                        CardIdentity {
                            suit_index: 4,
                            rank: 4,
                        },
                        CardIdentity {
                            suit_index: 2,
                            rank: 2,
                        },
                        CardIdentity {
                            suit_index: 2,
                            rank: 1,
                        },
                        CardIdentity {
                            suit_index: 2,
                            rank: 4,
                        },
                        CardIdentity {
                            suit_index: 1,
                            rank: 2,
                        },
                        CardIdentity {
                            suit_index: 1,
                            rank: 1,
                        },
                        CardIdentity {
                            suit_index: 1,
                            rank: 1,
                        },
                        CardIdentity {
                            suit_index: 4,
                            rank: 3,
                        },
                        CardIdentity {
                            suit_index: 4,
                            rank: 4,
                        },
                        CardIdentity {
                            suit_index: 0,
                            rank: 1,
                        },
                        CardIdentity {
                            suit_index: 4,
                            rank: 2,
                        },
                        CardIdentity {
                            suit_index: 1,
                            rank: 3,
                        },
                        CardIdentity {
                            suit_index: 4,
                            rank: 2,
                        },
                        CardIdentity {
                            suit_index: 4,
                            rank: 1,
                        },
                        CardIdentity {
                            suit_index: 0,
                            rank: 1,
                        },
                        CardIdentity {
                            suit_index: 3,
                            rank: 2,
                        },
                        CardIdentity {
                            suit_index: 0,
                            rank: 5,
                        },
                        CardIdentity {
                            suit_index: 3,
                            rank: 2,
                        },
                        CardIdentity {
                            suit_index: 3,
                            rank: 3,
                        },
                        CardIdentity {
                            suit_index: 0,
                            rank: 4,
                        },
                        CardIdentity {
                            suit_index: 0,
                            rank: 2,
                        },
                        CardIdentity {
                            suit_index: 2,
                            rank: 1,
                        },
                        CardIdentity {
                            suit_index: 4,
                            rank: 3,
                        },
                        CardIdentity {
                            suit_index: 2,
                            rank: 1,
                        },
                        CardIdentity {
                            suit_index: 2,
                            rank: 3,
                        },
                        CardIdentity {
                            suit_index: 1,
                            rank: 5,
                        },
                        CardIdentity {
                            suit_index: 0,
                            rank: 4,
                        },
                        CardIdentity {
                            suit_index: 0,
                            rank: 1,
                        },
                        CardIdentity {
                            suit_index: 4,
                            rank: 1,
                        },
                        CardIdentity {
                            suit_index: 3,
                            rank: 5,
                        },
                        CardIdentity {
                            suit_index: 3,
                            rank: 1,
                        },
                        CardIdentity {
                            suit_index: 3,
                            rank: 1,
                        },
                        CardIdentity {
                            suit_index: 4,
                            rank: 1,
                        },
                        CardIdentity {
                            suit_index: 3,
                            rank: 4,
                        },
                        CardIdentity {
                            suit_index: 0,
                            rank: 2,
                        },
                        CardIdentity {
                            suit_index: 2,
                            rank: 4,
                        },
                        CardIdentity {
                            suit_index: 2,
                            rank: 3,
                        },
                        CardIdentity {
                            suit_index: 0,
                            rank: 3,
                        },
                        CardIdentity {
                            suit_index: 3,
                            rank: 3,
                        },
                    ],
                },
            ),
            FinishOngoingGame(
                FinishOngoingGameMessage {
                    table_id: 15029,
                    database_id: 1141357,
                    shared_replay_leader: "njha",
                },
            ),
            ReplayAction(
                CommandData {
                    table_id: Some(
                        15029,
                    ),
                    database_id: None,
                },
            ),
            User(
                UserData {
                    user_id: 91466,
                    name: "ashishla",
                    status: 5,
                    table_id: Some(
                        15029,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            User(
                UserData {
                    user_id: 78102,
                    name: "will-bot1",
                    status: 5,
                    table_id: Some(
                        15029,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            User(
                UserData {
                    user_id: 88454,
                    name: "njha",
                    status: 5,
                    table_id: Some(
                        15029,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            NoteList(
                NoteListMessage {
                    table_id: 15029,
                    notes: [
                        NoteList {
                            name: "will-bot1",
                            notes: [
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                            ],
                            is_spectator: false,
                        },
                        NoteList {
                            name: "njha",
                            notes: [
                                "",
                                "",
                                "",
                                "",
                                "",
                                "[cm]",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                            ],
                            is_spectator: false,
                        },
                        NoteList {
                            name: "ashishla",
                            notes: [
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                            ],
                            is_spectator: true,
                        },
                        NoteList {
                            name: "will-bot1",
                            notes: [
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                            ],
                            is_spectator: true,
                        },
                        NoteList {
                            name: "njha",
                            notes: [
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                                "",
                            ],
                            is_spectator: true,
                        },
                    ],
                },
            ),
            Table(
                TableData {
                    id: 15029,
                    joined: true,
                    max_players: 6,
                    name: "Shauna anxiously daffier (Game #1141357)",
                    num_players: 2,
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: true,
                    password_protected: true,
                    players: [
                        "will-bot1",
                        "njha",
                    ],
                    progress: 100,
                    running: true,
                    shared_replay: true,
                    spectators: [
                        Spectator {
                            name: "ashishla",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                        Spectator {
                            name: "will-bot1",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                        Spectator {
                            name: "njha",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                    ],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Matryoshka (5 Suits)",
                },
            ),
            Spectators(
                SpectatorsMessage {
                    table_id: 15029,
                    spectators: [
                        Spectator {
                            name: "ashishla",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                        Spectator {
                            name: "will-bot1",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                        Spectator {
                            name: "njha",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                    ],
                },
            ),
            ReplaySegment(
                ReplaySegmentMessage {
                    table_id: 15029,
                    segment: 14,
                },
            ),
            TableRestart(
                CommandRestartData {
                    hide_pregame: true,
                },
            ),
            Boot(
                BootMessage {
                    table_id: 15029,
                },
            ),
            User(
                UserData {
                    user_id: 78102,
                    name: "will-bot1",
                    status: 0,
                    table_id: Some(
                        0,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            ChatTyping(
                CommandData {
                    table_id: Some(
                        15029,
                    ),
                    database_id: None,
                },
            ),
            Table(
                TableData {
                    id: 15029,
                    joined: true,
                    max_players: 6,
                    name: "Shauna anxiously daffier (Game #1141357)",
                    num_players: 2,
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: true,
                    password_protected: true,
                    players: [
                        "will-bot1",
                        "njha",
                    ],
                    progress: 100,
                    running: true,
                    shared_replay: true,
                    spectators: [
                        Spectator {
                            name: "ashishla",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                        Spectator {
                            name: "njha",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                    ],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Matryoshka (5 Suits)",
                },
            ),
            Spectators(
                SpectatorsMessage {
                    table_id: 15029,
                    spectators: [
                        Spectator {
                            name: "ashishla",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                        Spectator {
                            name: "njha",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                    ],
                },
            ),
            User(
                UserData {
                    user_id: 88454,
                    name: "njha",
                    status: 0,
                    table_id: Some(
                        0,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            Table(
                TableData {
                    id: 15029,
                    joined: true,
                    max_players: 6,
                    name: "Shauna anxiously daffier (Game #1141357)",
                    num_players: 2,
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: true,
                    password_protected: true,
                    players: [
                        "will-bot1",
                        "njha",
                    ],
                    progress: 100,
                    running: true,
                    shared_replay: true,
                    spectators: [
                        Spectator {
                            name: "ashishla",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                    ],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Matryoshka (5 Suits)",
                },
            ),
            User(
                UserData {
                    user_id: 91466,
                    name: "ashishla",
                    status: 0,
                    table_id: Some(
                        0,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            TableGone(
                CommandData {
                    table_id: Some(
                        15029,
                    ),
                    database_id: None,
                },
            ),
            Game(
                GameMessage {
                    table_id: 15030,
                    name: "Shauna anxiously daffier (#2)",
                    owner: 88454,
                    players: [
                        GamePlayerMessage {
                            index: 0,
                            name: "njha",
                            you: true,
                            present: true,
                            stats: Some(
                                PregameStats {
                                    num_games: 105,
                                    variant: UserStatsRow {
                                        num_games: 10,
                                        best_scores: [
                                            BestScore {
                                                num_players: 2,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 3,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 4,
                                                score: 25,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 5,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 6,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                        ],
                                        average_score: 24.666666666666668,
                                        num_strikeouts: 7,
                                    },
                                },
                            ),
                        },
                    ],
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    password_protected: true,
                    max_players: 6,
                },
            ),
            User(
                UserData {
                    user_id: 88454,
                    name: "njha",
                    status: 1,
                    table_id: Some(
                        15030,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            Joined(
                CommandData {
                    table_id: Some(
                        15030,
                    ),
                    database_id: None,
                },
            ),
            ChatList(
                ChatListData {
                    list: [
                        ChatData {
                            msg: "<strong>njha</strong> created the table.",
                            who: Some(
                                "",
                            ),
                            discord: false,
                            server: true,
                            datetime: "2024-04-08T18:55:34.466621992Z",
                            room: Some(
                                "table15030",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                    ],
                    unread: 1,
                },
            ),
            Game(
                GameMessage {
                    table_id: 15030,
                    name: "Shauna anxiously daffier (#2)",
                    owner: 88454,
                    players: [
                        GamePlayerMessage {
                            index: 0,
                            name: "njha",
                            you: true,
                            present: true,
                            stats: Some(
                                PregameStats {
                                    num_games: 105,
                                    variant: UserStatsRow {
                                        num_games: 10,
                                        best_scores: [
                                            BestScore {
                                                num_players: 2,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 3,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 4,
                                                score: 25,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 5,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 6,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                        ],
                                        average_score: 24.666666666666668,
                                        num_strikeouts: 7,
                                    },
                                },
                            ),
                        },
                        GamePlayerMessage {
                            index: 1,
                            name: "will-bot1",
                            you: false,
                            present: true,
                            stats: Some(
                                PregameStats {
                                    num_games: 1685,
                                    variant: UserStatsRow {
                                        num_games: 0,
                                        best_scores: [
                                            BestScore {
                                                num_players: 2,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 3,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 4,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 5,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                            BestScore {
                                                num_players: 6,
                                                score: 0,
                                                modifier: 0,
                                                deck_plays: false,
                                                empty_clues: false,
                                                one_extra_card: false,
                                                one_less_card: false,
                                                all_or_nothing: false,
                                            },
                                        ],
                                        average_score: 0.0,
                                        num_strikeouts: 0,
                                    },
                                },
                            ),
                        },
                    ],
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    password_protected: true,
                    max_players: 6,
                },
            ),
            User(
                UserData {
                    user_id: 78102,
                    name: "will-bot1",
                    status: 1,
                    table_id: Some(
                        15030,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            Chat(
                ChatData {
                    msg: "will-bot1 joined the game.",
                    who: Some(
                        "",
                    ),
                    discord: false,
                    server: true,
                    datetime: "2024-04-08T18:55:35.205383309Z",
                    room: Some(
                        "table15030",
                    ),
                    recipient: Some(
                        "",
                    ),
                },
            ),
            ChatRead(
                CommandData {
                    table_id: Some(
                        15030,
                    ),
                    database_id: None,
                },
            ),
            TableStart(
                CommandTableStartData {
                    intended_players: [],
                },
            ),
            GetGameInfo1(
                CommandData {
                    table_id: Some(
                        15030,
                    ),
                    database_id: None,
                },
            ),
            Table(
                TableData {
                    id: 15030,
                    joined: true,
                    max_players: 6,
                    name: "Shauna anxiously daffier (#2)",
                    num_players: 2,
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: true,
                    password_protected: true,
                    players: [
                        "will-bot1",
                        "njha",
                    ],
                    progress: 0,
                    running: true,
                    shared_replay: false,
                    spectators: [],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Matryoshka (5 Suits)",
                },
            ),
            User(
                UserData {
                    user_id: 78102,
                    name: "will-bot1",
                    status: 2,
                    table_id: Some(
                        15030,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            User(
                UserData {
                    user_id: 88454,
                    name: "njha",
                    status: 2,
                    table_id: Some(
                        15030,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            Table(
                TableData {
                    id: 15030,
                    joined: true,
                    max_players: 6,
                    name: "Shauna anxiously daffier (#2)",
                    num_players: 2,
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: true,
                    password_protected: true,
                    players: [
                        "will-bot1",
                        "njha",
                    ],
                    progress: 0,
                    running: true,
                    shared_replay: false,
                    spectators: [
                        Spectator {
                            name: "ashishla",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                    ],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Matryoshka (5 Suits)",
                },
            ),
            User(
                UserData {
                    user_id: 91466,
                    name: "ashishla",
                    status: 3,
                    table_id: Some(
                        15030,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            Init(
                InitMessage {
                    table_id: 15030,
                    player_names: [
                        "will-bot1",
                        "njha",
                    ],
                    our_player_index: 1,
                    spectating: false,
                    shadowing: false,
                    replay: false,
                    database_id: -1,
                    has_custom_seed: false,
                    seed: "p2v193s2",
                    datetime_started: 2024-04-08T18:55:35.723813739Z,
                    datetime_finished: 0001-01-01T00:00:00Z,
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    character_assignments: [],
                    character_metadata: [],
                    shared_replay: false,
                    shared_replay_leader: "njha",
                    shared_replay_segment: 0,
                    shared_replay_eff_mod: 0,
                    paused: false,
                    pause_player_index: -1,
                    pause_queued: false,
                },
            ),
            GetGameInfo2(
                CommandData {
                    table_id: Some(
                        15030,
                    ),
                    database_id: None,
                },
            ),
            GameActionList(
                GameActionListMessage {
                    table_id: 15030,
                    list: Array [
                        Object {
                            "order": Number(0),
                            "playerIndex": Number(0),
                            "rank": Number(5),
                            "suitIndex": Number(2),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(1),
                            "playerIndex": Number(0),
                            "rank": Number(1),
                            "suitIndex": Number(3),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(2),
                            "playerIndex": Number(0),
                            "rank": Number(4),
                            "suitIndex": Number(0),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(3),
                            "playerIndex": Number(0),
                            "rank": Number(3),
                            "suitIndex": Number(4),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(4),
                            "playerIndex": Number(0),
                            "rank": Number(3),
                            "suitIndex": Number(1),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(5),
                            "playerIndex": Number(1),
                            "rank": Number(-1),
                            "suitIndex": Number(-1),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(6),
                            "playerIndex": Number(1),
                            "rank": Number(-1),
                            "suitIndex": Number(-1),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(7),
                            "playerIndex": Number(1),
                            "rank": Number(-1),
                            "suitIndex": Number(-1),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(8),
                            "playerIndex": Number(1),
                            "rank": Number(-1),
                            "suitIndex": Number(-1),
                            "type": String("draw"),
                        },
                        Object {
                            "order": Number(9),
                            "playerIndex": Number(1),
                            "rank": Number(-1),
                            "suitIndex": Number(-1),
                            "type": String("draw"),
                        },
                    ],
                },
            ),
            Loaded(
                CommandData {
                    table_id: Some(
                        15030,
                    ),
                    database_id: None,
                },
            ),
            Connected(
                ConnectedMessage {
                    table_id: 15030,
                    list: [
                        false,
                        false,
                    ],
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15030,
                    times: [
                        -858,
                        0,
                    ],
                    active_player_index: 0,
                    time_taken: 858,
                },
            ),
            NoteListPlayer(
                NoteListPlayerMessage {
                    table_id: 15030,
                    notes: [
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                        "",
                    ],
                },
            ),
            VoteChange(
                VoteMessage {
                    vote: false,
                },
            ),
            Spectators(
                SpectatorsMessage {
                    table_id: 15030,
                    spectators: [
                        Spectator {
                            name: "ashishla",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                    ],
                },
            ),
            ChatList(
                ChatListData {
                    list: [
                        ChatData {
                            msg: "<strong>njha</strong> created the table.",
                            who: Some(
                                "",
                            ),
                            discord: false,
                            server: true,
                            datetime: "2024-04-08T18:54:18.639825053Z",
                            room: Some(
                                "table15030",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "will-bot1 joined the game.",
                            who: Some(
                                "",
                            ),
                            discord: false,
                            server: true,
                            datetime: "2024-04-08T18:54:33.289466101Z",
                            room: Some(
                                "table15030",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                        ChatData {
                            msg: "The game has been restarted (from game <a href=\"https://hanab.live/replay/1141357\" target=\"_blank\" rel=\"noopener noreferrer\">#1141357</a>).",
                            who: Some(
                                "",
                            ),
                            discord: false,
                            server: true,
                            datetime: "2024-04-08T18:55:35.735134737Z",
                            room: Some(
                                "table15030",
                            ),
                            recipient: Some(
                                "",
                            ),
                        },
                    ],
                    unread: 0,
                },
            ),
            Connected(
                ConnectedMessage {
                    table_id: 15030,
                    list: [
                        false,
                        true,
                    ],
                },
            ),
            Connected(
                ConnectedMessage {
                    table_id: 15030,
                    list: [
                        true,
                        true,
                    ],
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15030,
                    times: [
                        0,
                        0,
                    ],
                    active_player_index: 0,
                    time_taken: 0,
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15030,
                    action: Clue {
                        clue: Clue {
                            clue_type: 1,
                            value: 2,
                        },
                        giver: 0,
                        list: [
                            5,
                            7,
                            9,
                        ],
                        target: 1,
                        turn: 0,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15030,
                    action: Status {
                        clues: 7,
                        score: 0,
                        max_score: 25,
                    },
                },
            ),
            GameAction(
                GameActionMessage {
                    table_id: 15030,
                    action: Turn {
                        num: 1,
                        current_player_index: 1,
                    },
                },
            ),
            Clock(
                ClockMessage {
                    table_id: 15030,
                    times: [
                        -2635,
                        0,
                    ],
                    active_player_index: 1,
                    time_taken: 0,
                },
            ),
            User(
                UserData {
                    user_id: 91466,
                    name: "ashishla",
                    status: 0,
                    table_id: Some(
                        0,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            ChatTyping(
                CommandData {
                    table_id: Some(
                        15030,
                    ),
                    database_id: None,
                },
            ),
            Table(
                TableData {
                    id: 15030,
                    joined: true,
                    max_players: 6,
                    name: "Shauna anxiously daffier (#2)",
                    num_players: 2,
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: true,
                    password_protected: true,
                    players: [
                        "will-bot1",
                        "njha",
                    ],
                    progress: 0,
                    running: true,
                    shared_replay: false,
                    spectators: [],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Matryoshka (5 Suits)",
                },
            ),
            Spectators(
                SpectatorsMessage {
                    table_id: 15030,
                    spectators: [],
                },
            ),
            UserInactive(
                UserInactiveMessage {
                    user_id: 10507,
                    inactive: true,
                },
            ),
            User(
                UserData {
                    user_id: 90917,
                    name: "Gromzadira",
                    status: 0,
                    table_id: Some(
                        0,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            Table(
                TableData {
                    id: 14583,
                    joined: false,
                    max_players: 5,
                    name: "lobsters coffin logout",
                    num_players: 3,
                    options: GameOptions {
                        num_players: Some(
                            3,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Light Pink & Cocoa Rainbow (6 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: false,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: false,
                    password_protected: false,
                    players: [
                        "newsun",
                        "newduke",
                        "Cadenza",
                    ],
                    progress: 83,
                    running: true,
                    shared_replay: false,
                    spectators: [
                        Spectator {
                            name: "submachine",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                        Spectator {
                            name: "Gromzadira",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                    ],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Light Pink & Cocoa Rainbow (6 Suits)",
                },
            ),
            User(
                UserData {
                    user_id: 90917,
                    name: "Gromzadira",
                    status: 3,
                    table_id: Some(
                        14583,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            Table(
                TableData {
                    id: 15030,
                    joined: true,
                    max_players: 6,
                    name: "Shauna anxiously daffier (#2)",
                    num_players: 2,
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: true,
                    password_protected: true,
                    players: [
                        "will-bot1",
                        "njha",
                    ],
                    progress: 0,
                    running: true,
                    shared_replay: false,
                    spectators: [
                        Spectator {
                            name: "ashishla",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                    ],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Matryoshka (5 Suits)",
                },
            ),
            Spectators(
                SpectatorsMessage {
                    table_id: 15030,
                    spectators: [
                        Spectator {
                            name: "ashishla",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                    ],
                },
            ),
            User(
                UserData {
                    user_id: 91466,
                    name: "ashishla",
                    status: 3,
                    table_id: Some(
                        15030,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            ChatTyping(
                CommandData {
                    table_id: Some(
                        15030,
                    ),
                    database_id: None,
                },
            ),
            User(
                UserData {
                    user_id: 2000,
                    name: "HuHu",
                    status: 0,
                    table_id: Some(
                        0,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            Table(
                TableData {
                    id: 15024,
                    joined: false,
                    max_players: 6,
                    name: "veterinarians Bantu gentries (#10) (Game #1141356)",
                    num_players: 2,
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Rainbow-Fives & Dark Pink (6 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: false,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: false,
                    password_protected: true,
                    players: [
                        "HuHu",
                        "gsymon",
                    ],
                    progress: 11,
                    running: true,
                    shared_replay: true,
                    spectators: [
                        Spectator {
                            name: "gsymon",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                    ],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Rainbow-Fives & Dark Pink (6 Suits)",
                },
            ),
            User(
                UserData {
                    user_id: 75662,
                    name: "gsymon",
                    status: 0,
                    table_id: Some(
                        0,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            TableGone(
                CommandData {
                    table_id: Some(
                        15024,
                    ),
                    database_id: None,
                },
            ),
            User(
                UserData {
                    user_id: 75662,
                    name: "gsymon",
                    status: 1,
                    table_id: Some(
                        15031,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            User(
                UserData {
                    user_id: 2000,
                    name: "HuHu",
                    status: 1,
                    table_id: Some(
                        15031,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            Table(
                TableData {
                    id: 15031,
                    joined: false,
                    max_players: 6,
                    name: "veterinarians Bantu gentries (#11)",
                    num_players: 2,
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Rainbow-Fives & Dark Pink (6 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: false,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: false,
                    password_protected: true,
                    players: [
                        "gsymon",
                        "HuHu",
                    ],
                    progress: 0,
                    running: true,
                    shared_replay: false,
                    spectators: [],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Rainbow-Fives & Dark Pink (6 Suits)",
                },
            ),
            User(
                UserData {
                    user_id: 75662,
                    name: "gsymon",
                    status: 2,
                    table_id: Some(
                        15031,
                    ),
                    hyphenated: true,
                    inactive: false,
                },
            ),
            User(
                UserData {
                    user_id: 2000,
                    name: "HuHu",
                    status: 2,
                    table_id: Some(
                        15031,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            Chat(
                ChatData {
                    msg: "Hi, how do I play with bots?",
                    who: Some(
                        "ashishla",
                    ),
                    discord: false,
                    server: false,
                    datetime: "2024-04-08T18:56:47.579765711Z",
                    room: Some(
                        "table15030",
                    ),
                    recipient: Some(
                        "",
                    ),
                },
            ),
            User(
                UserData {
                    user_id: 91466,
                    name: "ashishla",
                    status: 0,
                    table_id: Some(
                        0,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
            ChatTyping(
                CommandData {
                    table_id: Some(
                        15030,
                    ),
                    database_id: None,
                },
            ),
            Table(
                TableData {
                    id: 15030,
                    joined: true,
                    max_players: 6,
                    name: "Shauna anxiously daffier (#2)",
                    num_players: 2,
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: true,
                    password_protected: true,
                    players: [
                        "will-bot1",
                        "njha",
                    ],
                    progress: 0,
                    running: true,
                    shared_replay: false,
                    spectators: [],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Matryoshka (5 Suits)",
                },
            ),
            Spectators(
                SpectatorsMessage {
                    table_id: 15030,
                    spectators: [],
                },
            ),
            Table(
                TableData {
                    id: 15030,
                    joined: true,
                    max_players: 6,
                    name: "Shauna anxiously daffier (#2)",
                    num_players: 2,
                    options: GameOptions {
                        num_players: Some(
                            2,
                        ),
                        starting_player: Some(
                            0,
                        ),
                        variant_name: "Matryoshka (5 Suits)",
                        timed: false,
                        time_base: 0,
                        time_per_turn: 0,
                        speedrun: true,
                        card_cycle: false,
                        deck_plays: false,
                        empty_clues: false,
                        one_extra_card: false,
                        one_less_card: false,
                        all_or_nothing: false,
                        detrimental_characters: false,
                        table_name: None,
                        max_players: None,
                    },
                    owned: true,
                    password_protected: true,
                    players: [
                        "will-bot1",
                        "njha",
                    ],
                    progress: 0,
                    running: true,
                    shared_replay: false,
                    spectators: [
                        Spectator {
                            name: "ashishla",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                    ],
                    time_base: 0,
                    time_per_turn: 0,
                    timed: false,
                    variant: "Matryoshka (5 Suits)",
                },
            ),
            Spectators(
                SpectatorsMessage {
                    table_id: 15030,
                    spectators: [
                        Spectator {
                            name: "ashishla",
                            shadowing_player_index: Some(
                                -1,
                            ),
                            shadowing_player_username: Some(
                                "",
                            ),
                        },
                    ],
                },
            ),
            User(
                UserData {
                    user_id: 91466,
                    name: "ashishla",
                    status: 3,
                    table_id: Some(
                        15030,
                    ),
                    hyphenated: false,
                    inactive: false,
                },
            ),
        ]
    "#]];
    expected.assert_debug_eq(&trace);
    Ok(())
}