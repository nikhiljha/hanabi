use hanabi::{Clue, HanabiGame, Player};
use hanabi::cards::{Rank, Suit};

fn main() {
    let variant = hanabi::variants::NoVariant;
    let mut game = HanabiGame::new(vec![
        Player::new("Alice".to_string()),
        Player::new("Bob".to_string()),
    ], variant);

    // get user input in a loop
    loop {
        // print the game state
        println!("score: {}", game.score());
        println!("stacks: {:#?}", game.stacks());
        for (i, player) in game.players().iter().enumerate() {
            println!("player {}: {:?}", i, player.hand.iter().map(|c| c.card().to_string()).collect::<Vec<_>>());
        }

        // get user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: Vec<&str> = input.trim().split(" ").collect();

        // parse the user input
        let action = match input[0] {
            "play" => hanabi::Action::Play(input[1].parse().unwrap()),
            "discard" => hanabi::Action::Discard(input[1].parse().unwrap()),
            "rank" => hanabi::Action::Clue {
                clue: Clue::Rank(Rank::try_from(input[1]).unwrap()),
                target: input[2].parse().unwrap(),
            },
            "suit" => hanabi::Action::Clue {
                clue: Clue::Suit(Suit::try_from(input[1]).unwrap()),
                target: input[2].parse().unwrap(),
            },
            _ => {
                println!("Invalid input");
                continue;
            }
        };

        // take the action
        let action = hanabi::AnnotatedAction {
            player: game.current_player(),
            action,
        };
        let result = game.act(action);

        // print the result
        match result {
            Ok(()) => println!("Success"),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
