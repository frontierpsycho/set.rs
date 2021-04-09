use std::fmt::{self, Formatter, Display};
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use crate::card::Card;

#[derive(PartialEq, Eq)]
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

impl Hash for Set {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_string().hash(state);
    }
}

impl Display for Set {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut card_vector: Vec<&Card> = self.cards.iter().collect();
        card_vector.sort();
        write!(f, "{}, {}, {}", card_vector[0], card_vector[1], card_vector[2])
    }
}
