#![allow(non_snake_case)]

use anyhow::Result;
use dioxus::prelude::*;
use hanabi::{Action, AnnotatedAction, Clue, HanabiGame, Player};
use hanabi::variants::NoVariant;
use std::sync::{Arc, RwLock};

#[cfg(all(feature = "server", not(target_arch = "wasm32")))]
use axum::{Router, ServiceExt};


#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/player/:name")]
    Player { name: String },
}

fn main() {
    let game = HanabiGame::new(vec![
        Player::new("njha".to_string()),
        Player::new("etw".to_string()),
    ], NoVariant::new());
    let game_ref = Arc::new(RwLock::new(game));

    let _vdom_factory = move || {
        let mut vdom = VirtualDom::new(App);
        let game_ref = game_ref.clone();
        vdom.insert_any_root_context(Box::new(move || Box::new(game_ref)));
        vdom
    };

    #[cfg(all(feature = "server", not(target_arch = "wasm32")))]
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async move {
            let router = Router::new()
                .serve_dioxus_application(ServeConfig::builder().build(), vdom_factory)
                .await;

            let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
            axum::serve(listener, router.into_make_service())
                .await.unwrap();
        });

    #[cfg(not(feature = "server"))]
    {
        #[cfg(feature = "web")]
        {
            let cfg = dioxus::web::Config::new().hydrate(true);
            dioxus::web::launch::launch_virtual_dom(vdom_factory(), cfg);
        }
    }
}

fn App() -> Element {
    rsx! { Router::<Route> {} }
}

#[component]
fn Player(name: String) -> Element {
    let mut count = use_signal(|| 0);
    let game = use_context::<Arc<RwLock<HanabiGame>>>();
    let my_id = game
        .read()
        .unwrap()
        .players()
        .iter()
        .position(|p| p.name == name)
        .unwrap_or_default();
    let players: Vec<Player> = game
        .read()
        .unwrap()
        .players()
        .iter()
        .map(|p| p.clone())
        .collect();

    rsx! {
        h1 { "Hanabi {count}" }
        p { "Your name is {name}" }
        p { "Your score is {game.read().unwrap().score()}" }
        h2 { "Stacks" }
        ul {
            for stack in game.read().unwrap().stacks().iter() {
                li { "{stack:?}" }
            }
        }
        h2 { "Players" }
        for (i , player) in players.iter().enumerate() {
            div {
                "Player: {player.name}"
                ul {
                    for card in player.hand.iter() {
                        if player.name == name {
                            li {
                                "???"
                                button {
                                    onclick: {
                                        let game = game.clone();
                                        move |_| {
                                            count += 1;
                                            game.write()
                                                .unwrap()
                                                .act(AnnotatedAction {
                                                    player: my_id,
                                                    action: Action::Play(i),
                                                })
                                                .unwrap();
                                        }
                                    },
                                    "Play"
                                }
                                button {
                                    onclick: {
                                        let game = game.clone();
                                        move |_| {
                                            count += 1;
                                            game.write()
                                                .unwrap()
                                                .act(AnnotatedAction {
                                                    player: my_id,
                                                    action: Action::Discard(i),
                                                })
                                                .unwrap();
                                        }
                                    },
                                    "Discard"
                                }
                            }
                        } else {
                            li {
                                "{card.card().to_string()}"
                                button {
                                    onclick: {
                                        let game = game.clone();
                                        let suit = card.suit();
                                        move |_| {
                                            count += 1;
                                            game.write()
                                                .unwrap()
                                                .act(AnnotatedAction {
                                                    player: my_id,
                                                    action: Action::Clue {
                                                        clue: Clue::Suit(suit),
                                                        target: i,
                                                    },
                                                })
                                                .unwrap();
                                        }
                                    },
                                    "Clue Suit"
                                }
                                button {
                                    onclick: {
                                        let game = game.clone();
                                        let rank = card.rank();
                                        move |_| {
                                            count += 1;
                                            game.write()
                                                .unwrap()
                                                .act(AnnotatedAction {
                                                    player: my_id,
                                                    action: Action::Clue {
                                                        clue: Clue::Rank(rank),
                                                        target: i,
                                                    },
                                                })
                                                .unwrap();
                                        }
                                    },
                                    "Clue Rank"
                                }
                            }
                        }
                    }
                }
            }
        }

        h2 { "Discards" }
        ul {
            for card in game.read().unwrap().discard_pile().iter() {
                li { "{card.to_string()}" }
            }
        }

        h2 { "Game Actions" }
        button {
            onclick: move |_| {
                count += 1;
            },
            "Refresh"
        }
        ul {
            for action in game.read().unwrap().history().iter() {
                li { "{action:?}" }
            }
        }
    }
}

#[component]
fn Home() -> Element {
    let mut name = use_signal(|| "bob".to_string());

    rsx! {
        input {
            value: "{name}",
            oninput: move |evt| name.set(evt.value().clone())
        }
        Link {
            to: Route::Player {
                name: name.to_string(),
            },
            "Join Game"
        }
    }
}
