#[derive(Debug, Copy, Clone)]
pub struct Card {
    pub number: u8,
    suite: CSuite,
}

#[derive(Debug, Copy, Clone)]
pub enum CSuite {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl Card {
    pub fn display_card(&self) {
        let card_type: String;
        match self.number {
            2u8..=10u8 => card_type = self.number.to_string(),
            11 => card_type = String::from("Jack"),
            12 => card_type = String::from("Queen"), 
            13 => card_type = String::from("King"),
            1 => card_type = String::from("Ace"),
            _ => card_type = String::from("/////INVALID/////"),
        }
        
        let sui: String;
        match self.suite {
            CSuite::Hearts => sui = String::from("Hearts"),
            CSuite::Diamonds => sui = String::from("Diamonds"),
            CSuite::Clubs => sui = String::from("Clubs"),
            CSuite::Spades => sui = String::from("Spades"),
        }
        
        println!("({card_type} of {sui})");
    }
}

pub fn generate_decks(num_of_decks: u8) -> Vec<Card> {
    (0..num_of_decks)
        .flat_map(|_| {
            [CSuite::Hearts, CSuite::Diamonds, CSuite::Clubs, CSuite::Spades]
                .iter()
                .flat_map(|&suite| (1..=13).map(move |n| Card { number: n, suite }))
        })
        .collect()
}