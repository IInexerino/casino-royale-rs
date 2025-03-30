use crate::cards::collection_setup::Card;
use crate::cards::collection_setup::generate_decks;

use crate::cards::shuffle::Shuffles;
use crate::cards::shuffle::many_shuffles;

use std::io;

fn calculate_hand(hand_to_calculate: &mut Vec<(Card, u8)>) -> u8 {
    let mut total = 0;
    let mut aces = 0;
    for &mut(_, value) in hand_to_calculate {
        if value == 1 {
            aces += 1;
        } else {
            total += value
        }
    }
    for _ in 0..aces {
        if total + 11 <= 21 {
            total += 11;
        } else {
            total += 1;
        }
    }
    total
}

pub fn play_blackjack(player_money: &mut i32, num_of_decks: u8) {

    println!("==========================================");
    println!("------WELCOME TO BLACKJACK SIMULATOR------");
    println!("==========================================\n");

    let mut card_collection: Vec<Card> = generate_decks(num_of_decks);
    println!("Generating {} standard deck/s of 52 cards...", num_of_decks);

    many_shuffles(&mut card_collection, Shuffles::SplitShuffle, 5);
    many_shuffles(&mut card_collection, Shuffles::RandShuffle, 100);
    println!("Shuffling cards...");

    //give each card its blackjack value and put in a tuple with this value, immersed in a vector
    let mut shoe: Vec<(Card, u8)> = Vec::new();

    for card in &card_collection {
        match card.number {
            1..=10 => shoe.push((*card, card.number)),
            11..=13 => shoe.push((*card, 10)),
            _ => panic!("Invalid cards present"),
        }
    }
    //numbered card collection is now what we work with
    'menu: loop {
        play_round(player_money, &mut shoe, num_of_decks);
        
        println!("\nWould you like to play another round (y) or exit to the main menu (n)?\n");
        loop {
            
            let mut menu_input = String::new();
            io::stdin().read_line(&mut menu_input).expect("Failed to read input");
            match menu_input.trim().to_lowercase().as_str() {
                "y" => continue 'menu,
                "n" => return,
                _ => {
                    println!("Invalid input: please type either (y) or (n)");
                    continue;
                }
            }
        }
    }
}

fn play_round(player_money: &mut i32, shoe: &mut Vec<(Card, u8)>, num_of_decks: u8) {

    println!("\n=========You have {}$ to your name=========\n\nHow much would you like to bet on this round?\n", player_money);
    let mut bet_amount = 0; // Variable to store the valid bet
    loop {
        let mut player_input = String::new(); // Fresh string for each iteration
        io::stdin().read_line(&mut player_input).expect("Failed to read input"); // Handle input reading failure
        // Try to parse the input as an i32
        match player_input.trim().parse::<i32>() {
            Ok(amount) => {
                // Check if the amount is positive and within the player's money
                if amount > 0 && amount <= *player_money {
                    bet_amount = amount; // Store the valid bet
                    break; // Exit the loop
                } else if amount == 0 {
                    println!("\nYou have chosen not to bet on this round.\nThe next round will begin...\n");
                    return;
                } else {
                    println!("Invalid bet amount. You can bet between 1 and {}", player_money);
                }
            }
            Err(_) => {
                println!("Please enter a valid number."); // Handle non-numeric input
            }
        }
    }
    
    fn reshuffle_deck(deck: &mut Vec<(Card, u8)>, num_of_decks: u8) {
        deck.clear();
        let mut card_collection: Vec<Card> = generate_decks(num_of_decks);
        many_shuffles(&mut card_collection, Shuffles::SplitShuffle, 5);
        many_shuffles(&mut card_collection, Shuffles::RandShuffle, 100);
        for card in card_collection {
            match card.number {
                1..=10 => deck.push((card, card.number)),
                11..=13 => deck.push((card, 10u8)),
                _ => panic!("Invalid cards present"),
            }
        }
        println!("Deck ran out of cards. Reshuffling {} deck(s)...", num_of_decks);
    }


    // Now bet_amount contains the valid bet amount
    println!("\n--You have bet {}$--", bet_amount);
    *player_money = *player_money - &bet_amount;
    println!("--You now have {0} - {bet_amount} = {player_money}$--", *player_money + bet_amount);

    if shoe.len() < 4 {
        reshuffle_deck(shoe, num_of_decks);
    }

    let mut player_cards: Vec<(Card, u8)> = Vec::new();
    player_cards.push(shoe.pop().unwrap());
    player_cards.push(shoe.pop().unwrap());

    let mut dealer_cards: Vec<(Card, u8)> = Vec::new();
    dealer_cards.push(shoe.pop().unwrap());
    dealer_cards.push(shoe.pop().unwrap());

    println!("\nCards drawn by the dealer:");
    dealer_cards[0].0.display_card();
    println!("(-*hidden card*-)");

    println!("\nCards drawn by player:");
    player_cards[0].0.display_card();
    player_cards[1].0.display_card();

    loop {
        println!("\nDraw another one (d) or let the dealer draw (e)?\n");
        let mut menu_input = String::new();
        io::stdin().read_line(&mut menu_input).expect("Failed to read input");
        match menu_input.trim().to_lowercase().as_str() {
            "d" => {
                if shoe.is_empty() {
                    reshuffle_deck(shoe, num_of_decks);
                }
                player_cards.push(shoe.pop().unwrap());
                
                println!("\nCards drawn by the dealer:");
                dealer_cards[0].0.display_card();
                println!("(-*hidden card*-)");
                
                println!("\nCards drawn by player:");
                for card in &player_cards {
                    card.0.display_card();
                }

                if calculate_hand(&mut player_cards) > 21 {
                    println!("\nOver 21, you go bust");
                    return;
                } else {
                    continue;
                }
            }
            "e" => break,
            _ => {
                println!("Invalid input: please type either (d) or (e)");
                continue;
            }
        }
    }   

    while calculate_hand(&mut dealer_cards) < 16 {
        if shoe.is_empty() {
            reshuffle_deck(shoe, num_of_decks);
        }
        dealer_cards.push(shoe.pop().unwrap());
    }
    if calculate_hand(&mut dealer_cards) > 21 {
        println!("\nFinal cards drawn by the dealer:");
        for card in &dealer_cards {
            card.0.display_card();
        }
        println!("\nThe dealer has gone bust, you win this round!");
        *player_money = *player_money + (&bet_amount * 2);
        println!("\n--You now have {0} + {1} = {player_money}$--", *player_money - (bet_amount*2), bet_amount * 2);
        return;
    } else if calculate_hand(&mut player_cards) > calculate_hand(&mut dealer_cards) {
        println!("\nFinal cards drawn by the dealer:");
        for card in &dealer_cards {
            card.0.display_card();
        }
        println!("\nYou have more points than the dealer, you win this round!");
        *player_money = *player_money + (&bet_amount * 2);
        println!("\n--You now have {0} + {1} = {player_money}$--", *player_money - (bet_amount*2), bet_amount * 2);
        return;
    } else if calculate_hand(&mut player_cards) < calculate_hand(&mut dealer_cards) {
        println!("\nFinal cards drawn by the dealer:");
        for card in &dealer_cards {
            card.0.display_card();
        }
        println!("\nYou have less points than the dealer, you lose this round");
        return;
    } else if calculate_hand(&mut player_cards) == calculate_hand(&mut dealer_cards) {
        println!("\nFinal cards drawn by the dealer:");
        for card in &dealer_cards {
            card.0.display_card();
        }
        println!("\nYou have an equal amount of points as the dealer, the result of this round is a push.");
        *player_money = *player_money + &bet_amount;
        println!("\n--You now have {0} + {bet_amount} = {player_money}$--", *player_money - bet_amount);
        return;
    }
}