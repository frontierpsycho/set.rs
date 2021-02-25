use std::fmt::{self, Formatter, Display};

pub enum CardNumber {
    One,
    Two,
    Three,
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
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
            /*
        fmt.write_format
        fmt.write_str(&self.number)

        for name in &self.names {
            fmt.write_str(str)?;
            fmt.write_str(name)?;
            str = ", ";
        }
        */
        Ok(())
    }

}
