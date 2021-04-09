use std::collections::HashSet;
use set::card::Card;
use set::solving::find_all_sets;

// TODO
// - write unit tests for solving

fn main() {
    let mut board: HashSet<Card> = HashSet::new();

    let mut count = 0u32;
    loop {
        count += 1;

        board.insert(Card::random());

        if count == 12 {
            break;
        }
    }

    let all_sets = find_all_sets(&board);
    for set in all_sets {
        println!("{}", set);
    }
}
