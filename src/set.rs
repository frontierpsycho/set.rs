use std::fmt::{self, Formatter, Display};

pub enum CardNumber {
    One,
    Two,
    Three,
}

// TODO there must be an easier way
impl fmt::Display for CardNumber {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           CardNumber::One => write!(f, "one"),
           CardNumber::Two => write!(f, "two"),
           CardNumber::Three => write!(f, "three"),
       }
    }
}

pub enum CardShading {
    Solid,
    Striped,
    Outlined,
}

pub enum CardColour {
    Red,
    Purple,
    Green,
}

pub enum CardShape {
    Oval,
    Diamond,
    Squiggle,
}

// TODO don't use pub fields, make a constructor
pub struct Card {
    pub number: CardNumber,
    pub shading: CardShading,
    pub colour: CardColour,
    pub shape: CardShape,
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // TODO why should this result be used? Error checking?
        write!(f, "{}", self.number);

        Ok(())
    }

}
