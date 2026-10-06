use rand::seq::SliceRandom;
use std::fmt::Write;

fn main() {
    println!("{}", KlondikeState::new(shuffled_deck()).as_text());
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

#[derive(Debug)]
pub struct Column {
    pub covered: Vec<Card>,
    pub uncovered: Vec<Card>,
}

#[derive(Debug)]
pub struct KlondikeState {
    pub collected_clubs: Option<Value>,
    pub collected_spades: Option<Value>,
    pub collected_diamonds: Option<Value>,
    pub collected_hearts: Option<Value>,

    pub columns: [Column; 7],
    pub draw_pile: Vec<Card>,
    pub draw_pile_position: usize,
}

impl KlondikeState {
    pub fn new(cards: [Card; 52]) -> KlondikeState {
        let mut cards = cards.into_iter();
        let mut columns = vec![];
        for column in 0..7 {
            columns.push(Column {
                covered: cards.by_ref().take(column).collect(),
                uncovered: vec![cards.next().unwrap()],
            });
        }
        let columns: [Column; 7] = columns.try_into().unwrap();
        let draw_pile = cards.collect::<Vec<_>>();
        assert_eq!(draw_pile.len(), 24);
        KlondikeState {
            collected_clubs: None,
            collected_spades: None,
            collected_diamonds: None,
            collected_hearts: None,

            columns,
            draw_pile,
            draw_pile_position: 2,
        }
    }

    pub fn as_text(&self) -> String {
        let mut result = String::new();
        let draw_pile = self
            .draw_pile
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(&mut result, "draw pile: {}", draw_pile).unwrap();
        writeln!(
            &mut result,
            "           {}^^^^",
            "    ".repeat(self.draw_pile_position)
        )
        .unwrap();

        fn collected(collected: Option<Value>) -> String {
            match collected {
                None => "none".into(),
                Some(value) => format!("{:?}", value),
            }
        }

        write!(&mut result, "♣: {} ", collected(self.collected_clubs)).unwrap();
        write!(&mut result, "♠: {} ", collected(self.collected_spades)).unwrap();
        write!(&mut result, "♦: {} ", collected(self.collected_diamonds)).unwrap();
        writeln!(&mut result, "♥: {}", collected(self.collected_hearts)).unwrap();

        fn column_to_strings(column: &Column) -> Vec<String> {
            let mut result: Vec<_> = column
                .covered
                .iter()
                .map(|card| format!("({})", card))
                .collect();
            result.extend(
                column
                    .uncovered
                    .iter()
                    .map(|card| format!(" {} ", card))
                    .collect::<Vec<_>>(),
            );
            result
        }

        let column_strings = self.columns.iter().map(column_to_strings);
        let tallest_column = column_strings.clone().map(|cs| cs.len()).max().unwrap();

        writeln!(&mut result).unwrap();
        for i in 0..tallest_column {
            for column in column_strings.clone() {
                write!(
                    &mut result,
                    "{}",
                    match column.get(i) {
                        None => "     ",
                        Some(s) => s,
                    }
                )
                .unwrap();
            }
            writeln!(&mut result).unwrap();
        }

        result
    }
}
