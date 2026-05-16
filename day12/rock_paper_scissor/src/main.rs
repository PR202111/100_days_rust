use rand::Rng;
use std::io::{self, Write};

enum GameResult {
    Win,
    Lose,
    Draw,
}

fn main() {
    println!("Welcome to Rock Paper Scissors!");
    println!("Instructions: Enter rock, paper, or scissors.");
    println!("Type quit to exit.\n");

    loop {
        print!("Make a choice: ");
        io::stdout().flush().unwrap();

        let user_choice = get_user_choice();

        if user_choice == "quit" {
            println!("Thanks for playing!");
            break;
        }

        if !is_valid_choice(&user_choice) {
            println!("Invalid choice. Try again.\n");
            continue;
        }

        let computer_choice = get_computer_choice();

        println!("Computer Choice: {}", computer_choice);

        match determine_winner(&user_choice, &computer_choice) {
            GameResult::Win => println!("You Win!\n"),
            GameResult::Lose => println!("You Lose!\n"),
            GameResult::Draw => println!("It's a Draw!\n"),
        }
    }
}

fn get_user_choice() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_lowercase()
}

fn get_computer_choice() -> String {
    let choices = ["rock", "paper", "scissors"];

    let mut rng = rand::thread_rng();

    let index = rng.gen_range(0..3);

    choices[index].to_string()
}

fn determine_winner(user: &str, computer: &str) -> GameResult {
    if user == computer {
        GameResult::Draw
    } else if (user == "rock" && computer == "scissors")
        || (user == "paper" && computer == "rock")
        || (user == "scissors" && computer == "paper")
    {
        GameResult::Win
    } else {
        GameResult::Lose
    }
}

fn is_valid_choice(choice: &str) -> bool {
    choice == "rock" || choice == "paper" || choice == "scissors" || choice == "quit"
}
