use crate::Card;
pub enum Shuffles {
    SplitShuffle,
}

pub fn split_shuffle(card_collection: &mut Vec<Card>) {
    
    let half_len = &card_collection.len()/2;
    
    let mut a_stack: Vec<Card> = Vec::new();
    for card in card_collection[half_len..].iter() {
        a_stack.push(*card);
    }
    let mut b_stack: Vec<Card> = Vec::new();
    for card in card_collection[..half_len].iter() {
        b_stack.push(*card);
    }

    let mut shuffled_card_collection: Vec<Card> = Vec::new();
    for i in 0..a_stack.len() {
        shuffled_card_collection.push(a_stack[i]);
        shuffled_card_collection.push(b_stack[i]);
    }

    *card_collection = shuffled_card_collection;
}

pub fn many_shuffles(card_collection: &mut Vec<Card>, shuffle_type: Shuffles, times: u8) {
    for _ in 1..=times {
        match shuffle_type {
            Shuffles::SplitShuffle => split_shuffle(card_collection)
        }
    }
}