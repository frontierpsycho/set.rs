use std::fmt::{self, Formatter, Display};

#[derive(PartialEq, Eq)]
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

// TODO there must be an easier way
impl fmt::Display for CardShading {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           CardShading::Solid => write!(f, "solid"),
           CardShading::Striped => write!(f, "striped"),
           CardShading::Outlined => write!(f, "outlined"),
       }
    }
}

pub enum CardColour {
    Red,
    Purple,
    Green,
}

// TODO there must be an easier way
impl fmt::Display for CardColour {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           CardColour::Red => write!(f, "red"),
           CardColour::Purple => write!(f, "purple"),
           CardColour::Green => write!(f, "green"),
       }
    }
}

pub enum CardShape {
    Oval,
    Diamond,
    Squiggle,
}

// TODO there must be an easier way
impl fmt::Display for CardShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           CardShape::Oval => write!(f, "oval"),
           CardShape::Diamond => write!(f, "diamond"),
           CardShape::Squiggle => write!(f, "squiggle"),
       }
    }
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
        write!(f, "{} {} {} {}", self.number, self.shading, self.colour, self.shape);

        if self.number != CardNumber::One {
            write!(f, "s");
        }

        Ok(())
    }

}
