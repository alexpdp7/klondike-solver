use rand::seq::SliceRandom;

fn main() {
    for card in shuffled_deck() {
        println!("{}", card);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Suit {
    CLUBS,
    SPADES,
    DIAMONDS,
    HEARTS,
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::CLUBS => "♣",
                Suit::SPADES => "♠",
                Suit::DIAMONDS => "♦",
                Suit::HEARTS => "♥",
            }
        )
    }
}

pub const SUITS: [Suit; 4] = [Suit::CLUBS, Suit::SPADES, Suit::DIAMONDS, Suit::HEARTS];

#[derive(Debug, Clone, Copy)]
pub enum Value {
    ACE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::ACE => " A",
                Value::TWO => " 2",
                Value::THREE => " 3",
                Value::FOUR => " 4",
                Value::FIVE => " 5",
                Value::SIX => " 6",
                Value::SEVEN => " 7",
                Value::EIGHT => " 8",
                Value::NINE => " 9",
                Value::TEN => "10",
                Value::JACK => " J",
                Value::QUEEN => " Q",
                Value::KING => " K",
            }
        )
    }
}

pub const VALUES: [Value; 13] = [
    Value::ACE,
    Value::TWO,
    Value::THREE,
    Value::FOUR,
    Value::FIVE,
    Value::SIX,
    Value::SEVEN,
    Value::EIGHT,
    Value::NINE,
    Value::TEN,
    Value::JACK,
    Value::QUEEN,
    Value::KING,
];

#[derive(Debug, Clone, Copy)]
pub struct Card {
    suit: Suit,
    value: Value,
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
