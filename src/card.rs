use std::fmt::{self, Formatter, Display};
use std::collections::HashSet;
use lazy_static::lazy_static;
use rand::random;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub enum CardNumber {
    One,
    Two,
    Three,
}

lazy_static! {
    pub static ref CARDNUMBERS: HashSet<CardNumber> = {
        let card_number_array: [CardNumber; 3] = [CardNumber::One, CardNumber::Two, CardNumber::Three];
        card_number_array.iter().cloned().collect()
    };
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

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub enum CardShading {
    Solid,
    Striped,
    Outlined,
}

lazy_static! {
    pub static ref CARDSHADINGS: HashSet<CardShading> = {
        let card_shading_array: [CardShading; 3] = [CardShading::Solid, CardShading::Striped, CardShading::Outlined];
        card_shading_array.iter().cloned().collect()
    };
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

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub enum CardColour {
    Red,
    Purple,
    Green,
}

lazy_static! {
    pub static ref CARDCOLOURS: HashSet<CardColour> = {
        let card_colour_array: [CardColour; 3] = [CardColour::Red, CardColour::Purple, CardColour::Green];
        card_colour_array.iter().cloned().collect()
    };
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

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub enum CardShape {
    Oval,
    Diamond,
    Squiggle,
}

lazy_static! {
    pub static ref CARDSHAPES: HashSet<CardShape> = {
        let card_shape_array: [CardShape; 3] = [CardShape::Oval, CardShape::Diamond, CardShape::Squiggle];
        card_shape_array.iter().cloned().collect()
    };
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

    return match index {
        _i if CardNumber::One as u32 == index => Ok(CardNumber::One),
        _i if CardNumber::Two as u32 == index => Ok(CardNumber::Two),
        _i if CardNumber::Three as u32 == index => Ok(CardNumber::Three),
        _ => Err(format!("No valid card number found for {}", index)),
    }
}

fn random_card_shading() -> Result<CardShading, String>  {
    let index = random::<u32>() % 3;


    return match index {
        _i if CardShading::Solid as u32 == index => Ok(CardShading::Solid),
        _i if CardShading::Striped as u32 == index => Ok(CardShading::Striped),
        _i if CardShading::Outlined as u32 == index => Ok(CardShading::Outlined),
        _ => Err(format!("No valid card shading found for {}", index)),
    }
}

fn random_card_colour() -> Result<CardColour, String>  {
    let index = random::<u32>() % 3;

    return match index {
        _i if CardColour::Red as u32 == index => Ok(CardColour::Red),
        _i if CardColour::Purple as u32 == index => Ok(CardColour::Purple),
        _i if CardColour::Green as u32 == index => Ok(CardColour::Green),
        _ => Err(format!("No valid card colour found for {}", index)),
    }
}

fn random_card_shape() -> Result<CardShape, String>  {
    let index = random::<u32>() % 3;

    return match index {
        _i if CardShape::Oval as u32 == index => Ok(CardShape::Oval),
        _i if CardShape::Diamond as u32 == index => Ok(CardShape::Diamond),
        _i if CardShape::Squiggle as u32 == index => Ok(CardShape::Squiggle),
        _ => Err(format!("No valid card shape found for {}", index)),
    }
}

#[derive(Clone, Copy, Hash, PartialOrd, Ord)]
pub struct Card {
    pub number: CardNumber,
    pub shading: CardShading,
    pub colour: CardColour,
    pub shape: CardShape,
}

impl Card {
    pub fn random() -> Self {
        // TODO OK, perhaps don't panic. Perhaps return Result?
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

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number &&
        self.shading == other.shading &&
        self.colour == other.colour &&
        self.shape == other.shape
    }
}
impl Eq for Card {}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{} {} {} {}", self.number, self.shading, self.colour, self.shape)?;

        if self.number != CardNumber::One {
            write!(f, "s")?;
        }

        Ok(())
    }
}
