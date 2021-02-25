mod set;
use set::{CardNumber::*, CardShading::*, CardColour::*, CardShape::*};

fn main() {
    let card = set::Card { number: Two , shading: Solid, colour: Purple, shape: Diamond };

    println!("Card: {}", card);
}
