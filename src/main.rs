mod cards;

use cards::collection_setup::Card;
use cards::collection_setup::generate_decks;

use cards::shuffle::Shuffles;
use cards::shuffle::many_shuffles;

mod games;

use games::blackjack::play_blackjack;

use std::io;
use std::io::{stdout, Write};
use std::process::Command;

fn clear_screen() {
    if cfg!(target_os = "windows") {
        // On Windows, run the 'cls' command
        Command::new("clear").status().unwrap();
    } else {
        // On Unix-like systems, use ANSI escape codes
        print!("\x1b[2J\x1b[H");
        stdout().flush().unwrap();
    }
}

fn main() {
    clear_screen();
    let mut player_money: i32 = 100;

    loop {
        println!("-=-=-=-=-=-=-=-=-MAIN MENU-=-=-=-=-=-=-=-=-");
        println!("\n=========You have {}$ to your name=========", player_money);
        println!("\nChoose a game that you would like to play:\n");
        println!("(1) Blackjack\n");

        loop {
            let mut menu_input = String::new();
            io::stdin().read_line(&mut menu_input).expect("Failed to read input");
            match menu_input.trim().to_lowercase().as_str() {
                "1" => {
                    clear_screen();
                    play_blackjack(&mut player_money, 6);
                    clear_screen();
                    break;
                }
                _ => println!("Invalid input: enter a valid number corresponding to an existing game")
            }
        }
    }
}

