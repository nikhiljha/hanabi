pub use leptos::prelude::*;
pub use leptos_router::{components::*, hooks::*, path};
use hanabi::{Action as HanabiAction, AnnotatedAction, Clue, HanabiGame, Player};
use hanabi::variants::NoVariant;
use std::sync::{Arc, RwLock};

#[component]
pub fn App() -> impl IntoView {
    let game = HanabiGame::new(vec![
        Player::new("njha".to_string()),
        Player::new("etw".to_string()),
    ], NoVariant::new());
    let game_ref = Arc::new(RwLock::new(game));

    provide_context(game_ref);

    view! {
        <Router>
            <div class="min-h-screen bg-gray-50">
                <Routes fallback=|| "Page not found.">
                    <Route path=path!("/") view=Home/>
                    <Route path=path!("/player/:name") view=Player/>
                </Routes>
            </div>
        </Router>
    }
}

#[component]
fn Player() -> impl IntoView {
    let params = use_params_map();
    let name = move || params.with(|params| params.get("name").unwrap_or_default());
    
    let (count, set_count) = signal(0);
    let game = use_context::<Arc<RwLock<HanabiGame>>>().expect("Game context not found");
    
    let game_clone1 = game.clone();
    let my_id = Memo::new(move |_| {
        game_clone1.read()
            .unwrap()
            .players()
            .iter()
            .position(|p| p.name == name())
            .unwrap_or_default()
    });
    
    let game_clone2 = game.clone();
    let players = move || game_clone2.read().unwrap().players().to_vec();

    let game_clone3 = game.clone();
    let score = Memo::new(move |_| {
        game_clone3.read().unwrap().score()
    });

    let game_clone4 = game.clone();
    let stacks = Memo::new(move |_| {
        game_clone4.read().unwrap().stacks().clone()
    });

    let game_clone5 = game.clone();
    let discard_pile = Memo::new(move |_| {
        game_clone5.read().unwrap().discard_pile().to_vec()
    });

    let game_clone6 = game.clone();
    let history = Memo::new(move |_| {
        game_clone6.read().unwrap().history().to_vec()
    });

    view! {
        <div class="container mx-auto px-4 py-8">
            <div class="bg-white rounded-lg shadow-lg p-6 mb-6">
                <h1 class="text-3xl font-bold text-gray-800 mb-4">"Hanabi " {count}</h1>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
                    <div class="bg-blue-50 p-4 rounded-lg">
                        <p class="text-lg font-semibold text-blue-800">"Player: " {name}</p>
                    </div>
                    <div class="bg-green-50 p-4 rounded-lg">
                        <p class="text-lg font-semibold text-green-800">"Score: " {score}</p>
                    </div>
                </div>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div class="bg-white rounded-lg shadow-lg p-6">
                    <h2 class="text-2xl font-bold text-gray-800 mb-4">"Stacks"</h2>
                    <div class="grid grid-cols-5 gap-2">
                        {move || stacks.get().iter().enumerate().map(|(_i, (suit, stack))| {
                            view! {
                                <div class="bg-gray-100 p-3 rounded-lg text-center">
                                    <div class="text-sm font-medium text-gray-600">{format!("{:?}", suit)}</div>
                                    <div class="text-lg font-bold text-gray-800">{format!("{:?}", stack)}</div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                <div class="bg-white rounded-lg shadow-lg p-6">
                    <h2 class="text-2xl font-bold text-gray-800 mb-4">"Players"</h2>
                    <div class="space-y-4">
                        {move || players().into_iter().enumerate().map(|(player_idx, player)| {
                            let current_name = name();
                            view! {
                                <div class="border border-gray-200 rounded-lg p-4">
                                    <h3 class="text-lg font-semibold text-gray-700 mb-3">"Player: " {player.name.clone()}</h3>
                                    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-2">
                                        {player.hand.iter().enumerate().map(|(card_idx, card)| {
                                            {
                                                if player.name == current_name {
                                                    let game_clone = game.clone();
                                                    let my_id_val = my_id.get();
                                                    view! {
                                                        <div class="bg-gray-200 p-3 rounded-lg text-center">
                                                            <div class="text-lg font-bold text-gray-600 mb-2">"???"</div>
                                                            <div class="space-y-1">
                                                                <button
                                                                    class="w-full bg-green-500 hover:bg-green-600 text-white text-xs py-1 px-2 rounded transition-colors"
                                                                    on:click={
                                                                        let game = game_clone.clone();
                                                                        move |_| {
                                                                            set_count.update(|c| *c += 1);
                                                                            let _ = game.write()
                                                                                .unwrap()
                                                                                .act(AnnotatedAction {
                                                                                    player: my_id_val,
                                                                                    action: HanabiAction::Play(card_idx),
                                                                                });
                                                                        }
                                                                    }
                                                                >
                                                                    "Play"
                                                                </button>
                                                                <button
                                                                    class="w-full bg-red-500 hover:bg-red-600 text-white text-xs py-1 px-2 rounded transition-colors"
                                                                    on:click={
                                                                        let game = game_clone.clone();
                                                                        move |_| {
                                                                            set_count.update(|c| *c += 1);
                                                                            let _ = game.write()
                                                                                .unwrap()
                                                                                .act(AnnotatedAction {
                                                                                    player: my_id_val,
                                                                                    action: HanabiAction::Discard(card_idx),
                                                                                });
                                                                        }
                                                                    }
                                                                >
                                                                    "Discard"
                                                                </button>
                                                            </div>
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    let game_clone = game.clone();
                                                    let my_id_val = my_id.get();
                                                    let suit = card.suit();
                                                    let rank = card.rank();
                                                    view! {
                                                        <div class="bg-white border-2 border-gray-300 p-3 rounded-lg text-center">
                                                            <div class="text-sm font-bold text-gray-800 mb-2">{card.card().to_string()}</div>
                                                            <div class="space-y-1">
                                                                <button
                                                                    class="w-full bg-blue-500 hover:bg-blue-600 text-white text-xs py-1 px-2 rounded transition-colors"
                                                                    on:click={
                                                                        let game = game_clone.clone();
                                                                        move |_| {
                                                                            set_count.update(|c| *c += 1);
                                                                            let _ = game.write()
                                                                                .unwrap()
                                                                                .act(AnnotatedAction {
                                                                                    player: my_id_val,
                                                                                    action: HanabiAction::Clue {
                                                                                        clue: Clue::Suit(suit),
                                                                                        target: player_idx,
                                                                                    },
                                                                                });
                                                                        }
                                                                    }
                                                                >
                                                                    "Clue Suit"
                                                                </button>
                                                                <button
                                                                    class="w-full bg-purple-500 hover:bg-purple-600 text-white text-xs py-1 px-2 rounded transition-colors"
                                                                    on:click={
                                                                        let game = game_clone.clone();
                                                                        move |_| {
                                                                            set_count.update(|c| *c += 1);
                                                                            let _ = game.write()
                                                                                .unwrap()
                                                                                .act(AnnotatedAction {
                                                                                    player: my_id_val,
                                                                                    action: HanabiAction::Clue {
                                                                                        clue: Clue::Rank(rank),
                                                                                        target: player_idx,
                                                                                    },
                                                                                });
                                                                        }
                                                                    }
                                                                >
                                                                    "Clue Rank"
                                                                </button>
                                                            </div>
                                                        </div>
                                                    }.into_any()
                                                }
                                            }
                                        }).collect::<Vec<_>>()}
                                    </div>
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </div>

            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6 mt-6">
                <div class="bg-white rounded-lg shadow-lg p-6">
                    <h2 class="text-2xl font-bold text-gray-800 mb-4">"Discards"</h2>
                    <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2 max-h-64 overflow-y-auto">
                        {move || discard_pile.get().into_iter().map(|card| {
                            view! {
                                <div class="bg-gray-100 p-2 rounded text-center text-sm">
                                    {card.to_string()}
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>

                <div class="bg-white rounded-lg shadow-lg p-6">
                    <div class="flex justify-between items-center mb-4">
                        <h2 class="text-2xl font-bold text-gray-800">"Game Actions"</h2>
                        <button
                            class="bg-gray-500 hover:bg-gray-600 text-white px-4 py-2 rounded transition-colors"
                            on:click=move |_| set_count.update(|c| *c += 1)
                        >
                            "Refresh"
                        </button>
                    </div>
                    <div class="max-h-64 overflow-y-auto space-y-2">
                        {move || history.get().into_iter().map(|action| {
                            view! {
                                <div class="bg-gray-50 p-2 rounded text-sm">
                                    {format!("{:?}", action)}
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Home() -> impl IntoView {
    let (name, set_name) = signal("bob".to_string());

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 to-indigo-100">
            <div class="bg-white rounded-lg shadow-xl p-8 w-full max-w-md">
                <h1 class="text-3xl font-bold text-center text-gray-800 mb-8">"Hanabi Game"</h1>
                <div class="space-y-6">
                    <div>
                        <label class="block text-sm font-medium text-gray-700 mb-2">"Enter your name:"</label>
                        <input
                            type="text"
                            class="w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
                            prop:value=name
                            on:input=move |ev| {
                                set_name.set(event_target_value(&ev));
                            }
                            placeholder="Your name"
                        />
                    </div>
                    <A
                        href=move || format!("/player/{}", name.get())
                        attr:class="w-full bg-blue-600 hover:bg-blue-700 text-white font-semibold py-3 px-4 rounded-md transition-colors duration-200 text-center block"
                    >
                        "Join Game"
                    </A>
                </div>
            </div>
        </div>
    }
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use wasm_bindgen::prelude::wasm_bindgen;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
