mod set;

fn main() {
    println!("Hello, world!");

    let number = set::CardNumber::One;
    let shading = set::CardShading::Solid;
    let colour = set::CardColour::Purple;
    let shape = set::CardShape::Diamond;

    let card = set::Card { number , shading, colour, shape };

    println!("Card: {}", card);
}
