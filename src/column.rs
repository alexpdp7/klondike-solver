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
