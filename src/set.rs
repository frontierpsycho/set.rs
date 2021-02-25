use std::fmt::{self, Formatter, Display};
use rand::random;

#[derive(PartialEq, Eq, Copy, Clone)]
pub enum CardNumber {
    One = 0,
    Two,
    Three,
}

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

impl fmt::Display for CardShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       match *self {
           CardShape::Oval => write!(f, "oval"),
           CardShape::Diamond => write!(f, "diamond"),
           CardShape::Squiggle => write!(f, "squiggle"),
       }
    }
}

// ================ RANDOM CARD GENERATION ================

// this is ugly but there's probably no better way
fn random_card_number() -> Result<CardNumber, String>  {
    let index = random::<u32>() % 3;

    if CardNumber::One as u32 == index {
        return Ok(CardNumber::One);
    } else if CardNumber::Two as u32 == index {
        return Ok(CardNumber::Two);
    } else if CardNumber::Three as u32 == index {
        return Ok(CardNumber::Three);
    } else {
        return Err(format!("No valid card number found for {}", index));
    }
}

fn random_card_shading() -> Result<CardShading, String>  {
    let index = random::<u32>() % 3;

    if CardShading::Solid as u32 == index {
        return Ok(CardShading::Solid);
    } else if CardShading::Striped as u32 == index {
        return Ok(CardShading::Striped);
    } else if CardShading::Outlined as u32 == index {
        return Ok(CardShading::Outlined);
    } else {
        return Err(format!("No valid card shading found for {}", index));
    }
}

fn random_card_colour() -> Result<CardColour, String>  {
    let index = random::<u32>() % 3;

    if CardColour::Red as u32 == index {
        return Ok(CardColour::Red);
    } else if CardColour::Purple as u32 == index {
        return Ok(CardColour::Purple);
    } else if CardColour::Green as u32 == index {
        return Ok(CardColour::Green);
    } else {
        return Err(format!("No valid card colour found for {}", index));
    }
}

fn random_card_shape() -> Result<CardShape, String>  {
    let index = random::<u32>() % 3;

    if CardShape::Oval as u32 == index {
        return Ok(CardShape::Oval);
    } else if CardShape::Diamond as u32 == index {
        return Ok(CardShape::Diamond);
    } else if CardShape::Squiggle as u32 == index {
        return Ok(CardShape::Squiggle);
    } else {
        return Err(format!("No valid card shape found for {}", index));
    }
}

pub struct Card {
    pub number: CardNumber,
    pub shading: CardShading,
    pub colour: CardColour,
    pub shape: CardShape,
}

// TODO random card constructor

impl Card {
    pub fn new() -> Self {
        let number = random_card_number().unwrap(); // yeah, this panics, it's alright
        let shading = random_card_shading().unwrap(); // yeah, this panics, it's alright
        let colour = random_card_colour().unwrap(); // yeah, this panics, it's alright
        let shape = random_card_shape().unwrap(); // yeah, this panics, it's alright

        Self {
            number,
            shading,
            colour,
            shape,
        }
    }
}

// TODO make cards sortable - actually might not be needed, depending on implementation of Set

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.number, self.shading, self.colour, self.shape)?;

        if self.number != CardNumber::One {
            write!(f, "s")?;
        }

        Ok(())
    }

}
