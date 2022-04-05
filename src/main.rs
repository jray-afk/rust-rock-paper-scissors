use text_io::{read};
use rand::seq::SliceRandom; // 0.7.2


struct GameScore {
    comp_wins: i32,
    user_wins: i32,
}


fn main() {
    println!("Starting game...");

    // let best_out_of: i32 = 3;

    let choices: Vec<String> = vec!["rock".to_string(),"paper".to_string(),"scissors".to_string()];
    let num_games = 5;
    let mut game_score = GameScore {comp_wins: 0, user_wins: 0};

    for game_count in 0..num_games {
        println!("Starting game #{}!", game_count);
        let comp_choice: String = computer_choose(&choices);
        
        println!("Choose rock, paper, or scissors: ");
        let user_choice: String = read!("{}");
        
        if choices.iter().any(|i| i == &user_choice) {
            println!("Computer chose: {}", comp_choice);
            let outcome = game(comp_choice, user_choice);
            if outcome == -1 {
                game_score.comp_wins += 1;
            } else if outcome == 1 {
                game_score.user_wins += 1;
            }
            println!("Current score: Computer {}, User {}", game_score.comp_wins, game_score.user_wins);
        } else {
            println!("What are you talking about?!");
        }
        println!("\n");
    }
    println!("Game over!");
}


fn computer_choose(choices: &Vec<String>) -> String {
    choices.choose(&mut rand::thread_rng()).unwrap().to_string()
}

fn game(comp_choice: String, user_choice: String) -> i32 {
    // println!("Debug: {}, {}", comp_choice, user_choice);

    match user_choice.as_str() {
        "rock" => {
            // println!("Debug: Matched for rock!");
            if comp_choice.as_str() == "rock" {
                println!("It's a tie!");
                0
            } else if comp_choice == "paper" {
                println!("You lose!");
                -1
            } else if comp_choice == "scissors" {
                println!("You win!");
                1
            } else {
                0
            }
        },
        "paper" => {
            // println!("Debug: Matched for paper!");
            if comp_choice.as_str() == "paper" {
                println!("It's a tie!");
                0
            } else if comp_choice == "scissors" {
                println!("You lose!");
                -1
            } else if comp_choice == "rock" {
                println!("You win!");
                1
            } else {
                0
            }
        },
        "scissors" => {
            // println!("Debug: Matched for scissors!");
            if comp_choice.as_str() == "scissors" {
                println!("It's a tie!");
                0
            } else if comp_choice == "rock" {
                println!("You lose!");
                -1
            } else if comp_choice == "paper" {
                println!("You win!");
                1
            } else {
                0
            }
        },
        _ => {
            println!("Uhh, not sure what happened here...");
            0
        }
    }
}
