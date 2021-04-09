use std::collections::HashSet;
use crate::card::{*};

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
            // blech
            let mut extrapolated_third: CardNumber = card1.number;
            for (&card_number, _) in CARDNUMBERS.iter() {
                if card_number != card1.number && card_number != card2.number {
                    extrapolated_third = card_number;
                }
            }
            extrapolated_third
        }
    };

    let shading = {
        // if both are the same, the third should also have the same
        if card1.shading == card2.shading {
            card1.shading
        } else {
            // blech
            let mut extrapolated_third: CardShading = card1.shading;
            for (&card_shading, _) in CARDSHADINGS.iter() {
                if card_shading != card1.shading && card_shading != card2.shading {
                    extrapolated_third = card_shading;
                }
            }
            extrapolated_third
        }
    };

    let colour = {
        // if both are the same, the third should also have the same
        if card1.colour == card2.colour {
            card1.colour
        } else {
            // blech
            let mut extrapolated_third: CardColour = card1.colour;
            for (&card_colour, _) in CARDCOLOURS.iter() {
                if card_colour != card1.colour && card_colour != card2.colour {
                    extrapolated_third = card_colour;
                }
            }
            extrapolated_third
        }
    };

    let shape = {
        // if both are the same, the third should also have the same
        if card1.shape == card2.shape {
            card1.shape
        } else {
            // blech
            let mut extrapolated_third: CardShape = card1.shape;
            for (&card_shape, _) in CARDSHAPES.iter() {
                if card_shape != card1.shape && card_shape != card2.shape {
                    extrapolated_third = card_shape;
                }
            }
            extrapolated_third
        }
    };

    Card { number: number, shading: shading, colour: colour, shape: shape }
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
