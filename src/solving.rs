use std::collections::HashSet;
use crate::card::Card;

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
        // if both are the same, the third should also have the same
        if card1.number == card2.number {
            card1.number
        } else {
            // TODO no, extrapolate third
            card1.number
        }
    };

    println!("Extrapolated number (should be three): {}", number);

    Card::new()
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

        extrapolate_third(&card1, &card2);
    }
}
