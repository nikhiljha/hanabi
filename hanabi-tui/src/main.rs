use hanabi::{Clue, HanabiGame, Player, Value};

fn main() {
    let mut game = HanabiGame::new(vec![
        Player::new("Alice".to_string()),
        Player::new("Bob".to_string()),
    ]);

    // get user input in a loop
    loop {
        // print the game state
        println!("score: {}", game.score());
        for (i, player) in game.players().iter().enumerate() {
            println!("player {}: {:?}", i, player.hand);
        }

        // get user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input: Vec<&str> = input.trim().split(" ").collect();

        // parse the user input
        let action = match input[0] {
            "play" => hanabi::Action::Play(input[1].parse().unwrap()),
            "discard" => hanabi::Action::Discard(input[1].parse().unwrap()),
            "clue" => hanabi::Action::Clue {
                clue: Clue::Value(Value::try_from(input[1]).unwrap()),
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
