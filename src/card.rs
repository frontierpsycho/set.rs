use std::fmt::{self, Formatter, Display};
use std::collections::HashMap;
use lazy_static::lazy_static;
use rand::random;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, PartialOrd, Ord)]
pub enum CardNumber {
    One,
    Two,
    Three,
}

lazy_static! {
    pub static ref CARDNUMBERS: HashMap<CardNumber, &'static str> = {
        let mut m = HashMap::with_capacity(3);
        m.insert(CardNumber::One, "one");
        m.insert(CardNumber::Two, "two");
        m.insert(CardNumber::Three, "three");
        m
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
    pub static ref CARDSHADINGS: HashMap<CardShading, &'static str> = {
        let mut m = HashMap::with_capacity(3);
        m.insert(CardShading::Solid, "solid");
        m.insert(CardShading::Striped, "striped");
        m.insert(CardShading::Outlined, "outlined");
        m
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
    pub static ref CARDCOLOURS: HashMap<CardColour, &'static str> = {
        let mut m = HashMap::with_capacity(3);
        m.insert(CardColour::Red, "red");
        m.insert(CardColour::Purple, "purple");
        m.insert(CardColour::Green, "green");
        m
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
    pub static ref CARDSHAPES: HashMap<CardShape, &'static str> = {
        let mut m = HashMap::with_capacity(3);
        m.insert(CardShape::Oval, "oval");
        m.insert(CardShape::Diamond, "diamond");
        m.insert(CardShape::Squiggle, "squiggle");
        m
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

    match index {
        _i if CardNumber::One as u32 == index => return Ok(CardNumber::One),
        _i if CardNumber::Two as u32 == index => return Ok(CardNumber::Two),
        _i if CardNumber::Three as u32 == index => return Ok(CardNumber::Three),
        _ => (),
    }

    return Err(format!("No valid card number found for {}", index));
}

fn random_card_shading() -> Result<CardShading, String>  {
    let index = random::<u32>() % 3;

    match index {
        _i if CardShading::Solid as u32 == index => return Ok(CardShading::Solid),
        _i if CardShading::Striped as u32 == index => return Ok(CardShading::Striped),
        _i if CardShading::Outlined as u32 == index => return Ok(CardShading::Outlined),
        _ => (),
    }

    return Err(format!("No valid card shading found for {}", index));
}

fn random_card_colour() -> Result<CardColour, String>  {
    let index = random::<u32>() % 3;

    match index {
        _i if CardColour::Red as u32 == index => return Ok(CardColour::Red),
        _i if CardColour::Purple as u32 == index => return Ok(CardColour::Purple),
        _i if CardColour::Green as u32 == index => return Ok(CardColour::Green),
        _ => (),
    }

    return Err(format!("No valid card colour found for {}", index));
}

fn random_card_shape() -> Result<CardShape, String>  {
    let index = random::<u32>() % 3;

    match index {
        _i if CardShape::Oval as u32 == index => return Ok(CardShape::Oval),
        _i if CardShape::Diamond as u32 == index => return Ok(CardShape::Diamond),
        _i if CardShape::Squiggle as u32 == index => return Ok(CardShape::Squiggle),
        _ => (),
    }

    return Err(format!("No valid card shape found for {}", index));
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
