use crate::deck::Card;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Column {
    pub covered: Vec<Card>,
    pub uncovered: Vec<Card>,
}

impl Column {
    pub fn pop_last_uncovered(&self) -> (Column, Card) {
        let (popped_card, except_last) = self.uncovered.split_last().unwrap();
        let popped_column = match except_last.len() {
            0 => match self.covered.len() {
                0 => Column {
                    covered: vec![],
                    uncovered: vec![],
                },
                _ => {
                    let (uncovered, covered) = self.covered.split_last().unwrap();
                    Column {
                        covered: covered.to_vec(),
                        uncovered: vec![*uncovered],
                    }
                }
            },
            _ => Column {
                covered: self.covered.clone(),
                uncovered: except_last.to_vec(),
            },
        };
        (popped_column, *popped_card)
    }

    pub fn is_empty(&self) -> bool {
        self.covered.is_empty() && self.uncovered.is_empty()
    }

    pub fn push(&self, card: Card) -> Column {
        Column {
            covered: self.covered.clone(),
            uncovered: [self.uncovered.clone(), [card].to_vec()].concat(),
        }
    }
}

pub fn shift(chunk: &[Card], from: &Column, to: &Column) -> (Column, Column) {
    let mut covered = from.covered.clone();
    let mut uncovered = from.uncovered[0..from.uncovered.len() - chunk.len()].to_vec();
    if uncovered.is_empty() && !covered.is_empty() {
        let (uncovered_card, new_covered) = covered.split_last().unwrap();
        uncovered = vec![*uncovered_card];
        covered = new_covered.to_vec();
    }
    (
        Column {
            covered,
            uncovered,
        },
        Column {
            covered: to.covered.clone(),
            uncovered: [to.uncovered.clone(), chunk.to_vec()].concat(),
        },
    )
}
