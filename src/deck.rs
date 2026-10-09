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
    #[must_use]
    pub fn color(&self) -> SuitColor {
        match self {
            Self::Clubs | Suit::Spades => SuitColor::Black,
            Suit::Diamonds | Suit::Hearts => SuitColor::Red,
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
    #[must_use]
    pub fn next(&self) -> Option<Value> {
        match self {
            Value::Ace => Some(Value::Two),
            Value::Two => Some(Value::Three),
            Value::Three => Some(Value::Four),
            Value::Four => Some(Value::Five),
            Value::Five => Some(Value::Six),
            Value::Six => Some(Value::Seven),
            Value::Seven => Some(Value::Eight),
            Value::Eight => Some(Value::Nine),
            Value::Nine => Some(Value::Ten),
            Value::Ten => Some(Value::Jack),
            Value::Jack => Some(Value::Queen),
            Value::Queen => Some(Value::King),
            Value::King => None,
        }
    }

    #[must_use]
    pub fn previous(&self) -> Option<Value> {
        match self {
            Value::Ace => None,
            Value::Two => Some(Value::Ace),
            Value::Three => Some(Value::Two),
            Value::Four => Some(Value::Three),
            Value::Five => Some(Value::Four),
            Value::Six => Some(Value::Five),
            Value::Seven => Some(Value::Six),
            Value::Eight => Some(Value::Seven),
            Value::Nine => Some(Value::Eight),
            Value::Ten => Some(Value::Nine),
            Value::Jack => Some(Value::Ten),
            Value::Queen => Some(Value::Jack),
            Value::King => Some(Value::Queen),
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

#[must_use]
pub fn deck() -> [Card; 52] {
    let mut result = vec![];
    for suit in SUITS {
        for value in VALUES {
            result.push(Card { suit, value });
        }
    }
    // infallible as long as SUITS.len() * VALUES.len() == 52
    #[expect(clippy::missing_panics_doc, reason = "infallible")]
    *result.as_array().unwrap()
}

#[must_use]
pub fn shuffled_deck() -> [Card; 52] {
    let mut rng = rand::rng();
    let mut deck = deck();
    deck.shuffle(&mut rng);
    deck
}
