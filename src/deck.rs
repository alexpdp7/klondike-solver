use rand::seq::SliceRandom;

#[derive(PartialEq)]
pub enum SuitColor {
    Black,
    Red,
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum Suit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

impl Suit {
    pub fn color(&self) -> SuitColor {
        match self {
            Self::Clubs => SuitColor::Black,
            Suit::Spades => SuitColor::Black,
            Suit::Diamonds => SuitColor::Red,
            Suit::Hearts => SuitColor::Red,
        }
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Clubs => "♣",
                Suit::Spades => "♠",
                Suit::Diamonds => "♦",
                Suit::Hearts => "♥",
            }
        )
    }
}

pub const SUITS: [Suit; 4] = [Suit::Clubs, Suit::Spades, Suit::Diamonds, Suit::Hearts];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Value {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

impl Value {
    pub fn next(&self) -> Option<Value> {
        match self {
            Value::King => None,
            value => Some(VALUES[VALUES.iter().position(|v| v == value).unwrap() + 1]),
        }
    }

    pub fn previous(&self) -> Option<Value> {
        match self {
            Value::Ace => None,
            value => Some(VALUES[VALUES.iter().position(|v| v == value).unwrap() - 1]),
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Ace => " A",
                Value::Two => " 2",
                Value::Three => " 3",
                Value::Four => " 4",
                Value::Five => " 5",
                Value::Six => " 6",
                Value::Seven => " 7",
                Value::Eight => " 8",
                Value::Nine => " 9",
                Value::Ten => "10",
                Value::Jack => " J",
                Value::Queen => " Q",
                Value::King => " K",
            }
        )
    }
}

pub const VALUES: [Value; 13] = [
    Value::Ace,
    Value::Two,
    Value::Three,
    Value::Four,
    Value::Five,
    Value::Six,
    Value::Seven,
    Value::Eight,
    Value::Nine,
    Value::Ten,
    Value::Jack,
    Value::Queen,
    Value::King,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub value: Value,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.suit)
    }
}

pub fn deck() -> [Card; 52] {
    let mut result = vec![];
    for suit in SUITS {
        for value in VALUES {
            result.push(Card { suit, value });
        }
    }
    *result.as_array().unwrap()
}

pub fn shuffled_deck() -> [Card; 52] {
    let mut rng = rand::rng();
    let mut deck = deck();
    deck.shuffle(&mut rng);
    deck
}
