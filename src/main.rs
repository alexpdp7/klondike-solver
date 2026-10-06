use rand::seq::SliceRandom;
use std::{collections::HashMap, fmt::Write};

fn main() {
    let state = KlondikeState::new(shuffled_deck());
    println!("{}", state.as_text());
    println!("Possible moves:");
    for possible_move in state.possible_moves() {
        println!("{}", possible_move.as_text());
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
impl Value {
    fn next(&self) -> Option<Value> {
        match self {
            Value::KING => None,
            value => Some(VALUES[VALUES.iter().position(|v| v == value).unwrap() + 1]),
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Column {
    pub covered: Vec<Card>,
    pub uncovered: Vec<Card>,
}

fn split_last<T: Copy>(v: &[T]) -> (&[T], T) {
    let (except_last, last) = v.split_at(v.len() - 1);
    (except_last, *last.first().unwrap())
}

impl Column {
    fn pop_last_uncovered(&self) -> (Column, Card) {
        let (except_last, popped_card) = split_last(&self.uncovered);
        let popped_column = match except_last.len() {
            0 => match self.covered.len() {
                0 => Column {
                    covered: vec![],
                    uncovered: vec![],
                },
                _ => {
                    let (covered, uncovered) = split_last(&self.covered);
                    Column {
                        covered: covered.to_vec(),
                        uncovered: vec![uncovered],
                    }
                }
            },
            _ => Column {
                covered: self.covered.clone(),
                uncovered: except_last.to_vec(),
            },
        };
        (popped_column, popped_card)
    }
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

    pub fn possible_moves(&self) -> Vec<KlondikeState> {
        let mut result = vec![];
        result.extend(self.possible_collect_from_columns());
        // possible_move_between_columns
        // possible_collect_from_draw_pile
        // possible_move_from_draw_pile_to_column
        result
    }

    fn possible_collect_from_columns(&self) -> Vec<KlondikeState> {
        let mut result = vec![];

        for column in &self.columns {
            match column.uncovered.last() {
                None => {}
                Some(last) => match self.next_to_collect_by_suit(last.suit) {
                    None => {}
                    Some(next_to_collect) if next_to_collect == last.value => {
                        result.push(self.collect_from_column(column));
                    }
                    Some(_) => {}
                },
            }
        }

        result
    }

    fn collected_by_suit(&self, suit: Suit) -> Option<Value> {
        match suit {
            Suit::CLUBS => self.collected_clubs,
            Suit::SPADES => self.collected_spades,
            Suit::DIAMONDS => self.collected_diamonds,
            Suit::HEARTS => self.collected_hearts,
        }
    }

    fn next_to_collect_by_suit(&self, suit: Suit) -> Option<Value> {
        match self.collected_by_suit(suit) {
            None => Some(Value::ACE),
            Some(value) => value.next(),
        }
    }

    fn collect_from_column(&self, column: &Column) -> KlondikeState {
        let (popped_column, card) = column.pop_last_uncovered();
        let mut collected_by_suit = self.collected_by_suit_hashmap();
        collected_by_suit.insert(card.suit, Some(card.value));
        let columns = self.columns.clone().map(|c| {
            if c == *column {
                popped_column.clone()
            } else {
                c
            }
        });
        KlondikeState {
            collected_clubs: *collected_by_suit.get(&Suit::CLUBS).unwrap(),
            collected_spades: *collected_by_suit.get(&Suit::SPADES).unwrap(),
            collected_diamonds: *collected_by_suit.get(&Suit::DIAMONDS).unwrap(),
            collected_hearts: *collected_by_suit.get(&Suit::HEARTS).unwrap(),
            columns,
            draw_pile: self.draw_pile.clone(),
            draw_pile_position: self.draw_pile_position,
        }
    }

    fn collected_by_suit_hashmap(&self) -> HashMap<Suit, Option<Value>> {
        HashMap::from([
            (Suit::CLUBS, self.collected_clubs),
            (Suit::SPADES, self.collected_spades),
            (Suit::DIAMONDS, self.collected_diamonds),
            (Suit::HEARTS, self.collected_hearts),
        ])
    }
}
