use std::collections::HashSet;
use crate::card::{*};
use crate::set::Set;

fn test_attribute(
    value1: &u32,
    value2: &u32,
    value3: &u32,
) -> bool {
    let value_set: HashSet<&u32> = [ value1, value2, value3 ].iter().cloned().collect();
    let len = value_set.len();
    len == 1 || len == 3
}

pub fn test_number( card1: &Card, card2: &Card, card3: &Card) -> bool {
    // apparently array maps are experimental (!)
    test_attribute(&(card1.number as u32), &(card2.number as u32), &(card3.number as u32))
}

pub fn test_shading( card1: &Card, card2: &Card, card3: &Card) -> bool {
    // apparently array maps are experimental (!)
    test_attribute(&(card1.shading as u32), &(card2.shading as u32), &(card3.shading as u32))
}

pub fn test_colour( card1: &Card, card2: &Card, card3: &Card) -> bool {
    // apparently array maps are experimental (!)
    test_attribute(&(card1.colour as u32), &(card2.colour as u32), &(card3.colour as u32))
}

pub fn test_shape( card1: &Card, card2: &Card, card3: &Card) -> bool {
    // apparently array maps are experimental (!)
    test_attribute(&(card1.shape as u32), &(card2.shape as u32), &(card3.shape as u32))
}

pub fn extrapolate_third(card1: &Card, card2: &Card) -> Card {
    let number = {
        if card1.number == card2.number {
            // if both are the same, the third should also have the same
            card1.number
        } else {
            // otherwise, get the third value

            // create a set of the input card numbers
            let value_set: HashSet<CardNumber> = [card1.number, card2.number].iter().cloned().collect();

            let extrapolated_third = CARDNUMBERS.difference(&value_set) // difference between all card numbers and input ones
                .into_iter().next() // get first (as an option)
                .unwrap(); // unwrap option, no way this is None

            *extrapolated_third
        }
    };

    let shading = {
        if card1.shading == card2.shading {
            // if both are the same, the third should also have the same
            card1.shading
        } else {
            // otherwise, get the third value
            let value_set: HashSet<CardShading> = [card1.shading, card2.shading].iter().cloned().collect();

            let extrapolated_third = CARDSHADINGS.difference(&value_set)
                .into_iter().next()
                .unwrap();

            *extrapolated_third
        }
    };

    let colour = {
        if card1.colour == card2.colour {
            // if both are the same, the third should also have the same
            card1.colour
        } else {
            // otherwise, get the third value
            let value_set: HashSet<CardColour> = [card1.colour, card2.colour].iter().cloned().collect();

            let extrapolated_third = CARDCOLOURS.difference(&value_set)
                .into_iter().next()
                .unwrap();

            *extrapolated_third
        }
    };

    let shape = {
        if card1.shape == card2.shape {
            // if both are the same, the third should also have the same
            card1.shape
        } else {
            // otherwise, get the third value
            let value_set: HashSet<CardShape> = [card1.shape, card2.shape].iter().cloned().collect();

            let extrapolated_third = CARDSHAPES.difference(&value_set)
                .into_iter().next()
                .unwrap();

            *extrapolated_third
        }
    };

    Card { number, shading, colour, shape }
}

pub fn find_all_sets(board: &HashSet<Card>) -> HashSet<Set> {
    let board_clone = board.clone();

    let mut sets_set = HashSet::new();

    for card1 in &board_clone {
        let mut card_for_difference: HashSet<Card> = HashSet::new();
        card_for_difference.insert(*card1);

        let board_without_card1 = board_clone.difference(&card_for_difference).collect::<HashSet<&Card>>();

        for card2 in &board_without_card1 {
            let extrapolated_card = extrapolate_third(&card1, &card2);
            // println!("Extrapolated third: {}", extrapolated_card);
            if board.contains(&extrapolated_card) {
                // println!("Found set: \n{}\n{}\n{}", card1, card2, extrapolated_card);
                let found_set = Set::new(card1.clone(), *card2.clone(), extrapolated_card.clone());
                sets_set.insert(found_set);
            }
        }
    }

    sets_set
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{CardNumber::*, CardShading::*, CardColour::*, CardShape::*};

    #[test]
    fn test_test_attributes() {
        let card1 = Card { number: One, shading: Solid, colour: Red, shape: Squiggle };
        let card2 = Card { number: One, shading: Striped, colour: Purple, shape: Squiggle };
        let card3 = Card { number: One, shading: Outlined, colour: Purple, shape: Diamond };

        // same on all cards should be positive
        assert_eq!(test_number(&card1, &card2, &card3), true);

        // different on all cards should be positive
        assert_eq!(test_shading(&card1, &card2, &card3), true);

        // 2 and 1 should be negative
        assert_eq!(test_colour(&card1, &card2, &card3), false);
        assert_eq!(test_shape(&card1, &card2, &card3), false);
    }

    #[test]
    fn test_extrapolate_third() {
        let card1 = Card { number: One, shading: Solid, colour: Red, shape: Squiggle };
        let card2 = Card { number: Two, shading: Striped, colour: Purple, shape: Squiggle };

        let mut extrapolated_card = extrapolate_third(&card1, &card2);

        assert_eq!(extrapolated_card.number, Three);
        assert_eq!(extrapolated_card.shading, Outlined);
        assert_eq!(extrapolated_card.colour, Green);
        assert_eq!(extrapolated_card.shape, Squiggle);

        let card4 = Card { number: One, shading: Outlined, colour: Purple, shape: Squiggle };
        let card5 = Card { number: One, shading: Striped, colour: Purple, shape: Squiggle };

        extrapolated_card = extrapolate_third(&card4, &card5);

        assert_eq!(extrapolated_card.number, One);
        assert_eq!(extrapolated_card.shading, Solid);
        assert_eq!(extrapolated_card.colour, Purple);
        assert_eq!(extrapolated_card.shape, Squiggle);
    }
}
