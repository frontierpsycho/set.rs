use std::fmt::{self, Formatter, Display};
use std::collections::HashSet;
use crate::card::Card;

pub struct Set {
    pub cards: HashSet<Card>,
}

impl Set {
    pub fn new(card1: Card, card2: Card, card3: Card) -> Self {
        let mut cards = HashSet::with_capacity(3);
        cards.insert(card1);
        cards.insert(card2);
        cards.insert(card3);

        Self { cards }
    }
}

// TODO do we need sorting? Or does the set take care of it?

impl fmt::Display for Set {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let card_vector: Vec<&Card> = self.cards.iter().collect();
        write!(f, "{}, {}, {}", card_vector[0], card_vector[1], card_vector[2])
    }
}
