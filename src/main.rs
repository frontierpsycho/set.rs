mod set;
use set::{CardNumber::*, CardShading::*, CardColour::*, CardShape::*};

fn main() {
    println!("Hello, world!");

    let card = set::Card { number: One , shading: Solid, colour: Purple, shape: Diamond };

    println!("Card: {}", card);
}
