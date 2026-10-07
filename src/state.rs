use std::collections::HashMap;
use std::fmt::Write;

use crate::column::Column;
use crate::deck::{Card, Suit, Value};
use crate::draw_pile::DrawPile;

#[derive(Debug, Clone)]
pub enum Movement {
    CollectFromColumn(usize),
    CollectFromDrawPile(Card),
    MoveFromDrawPileToColumn(Card, usize),
}

impl std::fmt::Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CollectFromColumn(column) => write!(f, "collect from column {column}"),
            Self::CollectFromDrawPile(card) => write!(f, "collect from draw pile {card}"),
            Self::MoveFromDrawPileToColumn(card, column) => {
                write!(f, "move from draw pile {card} to column {column}")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct KlondikeState {
    pub collected_clubs: Option<Value>,
    pub collected_spades: Option<Value>,
    pub collected_diamonds: Option<Value>,
    pub collected_hearts: Option<Value>,

    pub columns: [Column; 7],
    pub draw_pile: DrawPile,
    pub movements: Vec<Movement>,
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
        let draw_pile = DrawPile::new(cards.collect::<Vec<_>>());
        assert_eq!(draw_pile.total_size(), 24);
        KlondikeState {
            collected_clubs: None,
            collected_spades: None,
            collected_diamonds: None,
            collected_hearts: None,

            columns,
            draw_pile,
            movements: vec![],
        }
    }

    pub fn as_text(&self) -> String {
        let mut result = String::new();

        fn to_draw(c: &[Card]) -> String {
            let (chunks, remainder) = c.as_chunks::<3>();

            fn chunk(c: Vec<Card>) -> String {
                c.iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .concat()
            }

            let mut chunks = chunks.iter().map(|c| c.to_vec()).collect::<Vec<_>>();
            chunks.push(remainder.to_vec());

            chunks
                .iter()
                .map(|c| c.to_vec())
                .map(chunk)
                .collect::<Vec<_>>()
                .join(", ")
        }

        writeln!(&mut result, "drawn:   {}", to_draw(&self.draw_pile.drawn),).unwrap();
        writeln!(&mut result, "to_draw: {}", to_draw(&self.draw_pile.to_draw),).unwrap();

        fn collected(collected: Option<Value>) -> String {
            match collected {
                None => "---".into(),
                Some(value) => format!("{}", value),
            }
        }

        write!(&mut result, "{}♣ ", collected(self.collected_clubs)).unwrap();
        write!(&mut result, "{}♠ ", collected(self.collected_spades)).unwrap();
        write!(&mut result, "{}♦ ", collected(self.collected_diamonds)).unwrap();
        writeln!(&mut result, "{}♥ ", collected(self.collected_hearts)).unwrap();

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
        writeln!(&mut result, "  0    1    2    3    4    5    6").unwrap();
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

    pub fn cards_in_columns(&self) -> usize {
        self.columns
            .iter()
            .map(|c| c.uncovered.len() + c.covered.len())
            .sum()
    }

    pub fn cards_in_columns_and_draw_pile(&self) -> usize {
        self.cards_in_columns() + self.draw_pile.total_size()
    }

    pub fn possible_moves(&self) -> Vec<KlondikeState> {
        let mut result = vec![];
        result.extend(self.possible_collect_from_columns());
        result.extend(self.possible_collect_from_draw_pile());
        result.extend(self.possible_move_from_draw_pile_to_column());
        // TODO possible_move_between_columns
        // TODO possible_return_collected
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
            Suit::Clubs => self.collected_clubs,
            Suit::Spades => self.collected_spades,
            Suit::Diamonds => self.collected_diamonds,
            Suit::Hearts => self.collected_hearts,
        }
    }

    fn next_to_collect_by_suit(&self, suit: Suit) -> Option<Value> {
        match self.collected_by_suit(suit) {
            None => Some(Value::Ace),
            Some(value) => value.next(),
        }
    }

    fn collect_from_column(&self, column: &Column) -> KlondikeState {
        let (popped_column, card) = column.pop_last_uncovered();
        let mut collected_by_suit = self.collected_by_suit_hashmap();
        collected_by_suit.insert(card.suit, Some(card.value));
        let column_index = self.columns.iter().position(|c| c == column).unwrap();
        let mut columns = self.columns.clone();
        columns[column_index] = popped_column;
        KlondikeState {
            collected_clubs: *collected_by_suit.get(&Suit::Clubs).unwrap(),
            collected_spades: *collected_by_suit.get(&Suit::Spades).unwrap(),
            collected_diamonds: *collected_by_suit.get(&Suit::Diamonds).unwrap(),
            collected_hearts: *collected_by_suit.get(&Suit::Hearts).unwrap(),
            columns,
            draw_pile: self.draw_pile.clone(),
            movements: new_movements(
                self.movements.clone(),
                Movement::CollectFromColumn(column_index),
            ),
        }
    }

    fn collected_by_suit_hashmap(&self) -> HashMap<Suit, Option<Value>> {
        HashMap::from([
            (Suit::Clubs, self.collected_clubs),
            (Suit::Spades, self.collected_spades),
            (Suit::Diamonds, self.collected_diamonds),
            (Suit::Hearts, self.collected_hearts),
        ])
    }

    fn possible_collect_from_draw_pile(&self) -> Vec<KlondikeState> {
        let mut result = vec![];

        if self.draw_pile.is_empty() {
            return result;
        }

        for (candidate_draw_pile, candidate_card) in self.draw_pile.candidate_draws() {
            if Some(candidate_card.value) == self.next_to_collect_by_suit(candidate_card.suit) {
                let mut collected_by_suit = self.collected_by_suit_hashmap();
                collected_by_suit.insert(candidate_card.suit, Some(candidate_card.value));

                result.push(KlondikeState {
                    collected_clubs: *collected_by_suit.get(&Suit::Clubs).unwrap(),
                    collected_spades: *collected_by_suit.get(&Suit::Spades).unwrap(),
                    collected_diamonds: *collected_by_suit.get(&Suit::Diamonds).unwrap(),
                    collected_hearts: *collected_by_suit.get(&Suit::Hearts).unwrap(),
                    columns: self.columns.clone(),
                    draw_pile: candidate_draw_pile,
                    movements: new_movements(
                        self.movements.clone(),
                        Movement::CollectFromDrawPile(candidate_card),
                    ),
                });
            }
        }
        result
    }

    fn possible_move_from_draw_pile_to_column(&self) -> Vec<KlondikeState> {
        let mut result = vec![];

        if self.draw_pile.is_empty() {
            return result;
        }

        for (candidate_draw_pile, candidate_card) in self.draw_pile.candidate_draws() {
            for (column_index, column) in self.columns.iter().enumerate() {
                match column.uncovered.last() {
                    None => {}
                    Some(last) => {
                        // TODO: unverified king move
                        if (candidate_card.value.next() != Some(last.value)
                            || candidate_card.suit.color() == last.suit.color())
                            && (candidate_card.value != Value::King || !column.is_empty())
                        {
                            continue;
                        }
                        let mut columns = self.columns.clone();
                        columns[column_index] = columns[column_index].push(candidate_card);
                        result.push(KlondikeState {
                            collected_clubs: self.collected_clubs,
                            collected_spades: self.collected_spades,
                            collected_diamonds: self.collected_diamonds,
                            collected_hearts: self.collected_hearts,
                            columns,
                            draw_pile: candidate_draw_pile.clone(),
                            movements: new_movements(
                                self.movements.clone(),
                                Movement::MoveFromDrawPileToColumn(candidate_card, column_index),
                            ),
                        });
                    }
                }
            }
        }
        result
    }
}

fn new_movements(old_movements: Vec<Movement>, movement: Movement) -> Vec<Movement> {
    let mut movements = old_movements.into_iter().collect::<Vec<_>>();
    movements.push(movement);
    movements
}
