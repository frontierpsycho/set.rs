use std::collections::HashSet;
use set::card::Card;
use set::solving::find_a_set;
// use set::{CardNumber::*, CardShading::*, CardColour::*, CardShape::*};
//
// TODO
// - write unit tests for solving

fn main() {
    let mut board: HashSet<Card> = HashSet::new();

    let mut count = 0u32;
    loop {
        count += 1;

        board.insert(Card::new());

        if count == 12 {
            break;
        }
    }

    match find_a_set(&board) {
        Some(set) => println!("There's a set! {}", set),
        None => println!("There's no set."),
    }
}
