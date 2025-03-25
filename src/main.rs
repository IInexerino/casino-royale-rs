mod cards;

use cards::collection_setup::Card;
use cards::collection_setup::generate_decks;

use cards::shuffle::Shuffles;
use cards::shuffle::many_shuffles;

fn main() {
    let mut card_collection: Vec<Card> = generate_decks(1);
    many_shuffles(&mut card_collection, Shuffles::SplitShuffle, 20);
    println!("{:?}", card_collection)

}