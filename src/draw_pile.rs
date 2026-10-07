use crate::deck::Card;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrawPile {
    pub drawn: Vec<Card>,
    pub to_draw: Vec<Card>,
}

impl DrawPile {
    pub fn new(cards: Vec<Card>) -> DrawPile {
        let (drawn, to_draw) = cards.split_at(3);
        DrawPile {
            drawn: drawn.to_vec(),
            to_draw: to_draw.to_vec(),
        }
    }

    pub fn total_size(&self) -> usize {
        self.drawn.len() + self.to_draw.len()
    }

    pub fn is_empty(&self) -> bool {
        self.total_size() == 0
    }

    pub fn candidate_draws(&self) -> Vec<(DrawPile, Card)> {
        let mut result = vec![];
        if self.is_empty() {
            return result;
        }
        if !self.drawn.is_empty() {
            let mut drawn = self.drawn.clone();
            let card = drawn.pop().unwrap();
            result.push((
                DrawPile {
                    drawn,
                    to_draw: self.to_draw.clone(),
                },
                card,
            ));
        }

        /// TODO: likely not correct
        fn candidate_draws(drawn: Vec<Card>, to_draw: Vec<Card>) -> Vec<(DrawPile, Card)> {
            let mut drawn = drawn.clone();
            let mut to_draw = to_draw.clone();
            let mut result = vec![];
            while !to_draw.is_empty() {
                let (draw, remaining) = to_draw.split_at(std::cmp::min(3, to_draw.len()));
                let (card, draw_rest) = draw.split_last().unwrap();
                let card = *card;
                drawn = [drawn, draw_rest.to_vec()].concat();
                to_draw = remaining.to_vec();
                result.push((
                    DrawPile {
                        drawn: drawn.clone(),
                        to_draw: to_draw.clone(),
                    },
                    card,
                ));
            }
            result
        }

        result.append(&mut candidate_draws(
            self.drawn.clone(),
            self.to_draw.clone(),
        ));

        if !self.drawn.len().is_multiple_of(3) {
            let all = [self.drawn.clone(), self.to_draw.clone()].concat();
            let (drawn, to_draw) = all.split_at(3);
            result.append(&mut candidate_draws(drawn.to_vec(), to_draw.to_vec()));
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deck_slice(i: usize, j: usize) -> Vec<Card> {
        crate::deck::deck()[i..j].to_vec()
    }

    fn deck_card(i: usize) -> Card {
        crate::deck::deck()[i]
    }

    fn draw_pile(i: usize, j: usize, k: usize) -> DrawPile {
        DrawPile {
            drawn: deck_slice(i, j),
            to_draw: deck_slice(j, k),
        }
    }

    /// Asserts using dbg! to pretty-print the Debug formatting which is easier to read
    fn assert_candidate_draws(value: Vec<(DrawPile, Card)>, expected: Vec<(DrawPile, Card)>) {
        assert!(dbg!(value) == dbg!(expected));
    }

    #[test]
    fn candidate_draws_0_3_6() {
        assert_candidate_draws(
            draw_pile(0, 3, 6).candidate_draws(),
            vec![
                (
                    DrawPile {
                        drawn: deck_slice(0, 2),
                        to_draw: deck_slice(3, 6),
                    },
                    deck_card(2),
                ),
                (
                    DrawPile {
                        drawn: deck_slice(0, 5),
                        to_draw: deck_slice(6, 6),
                    },
                    deck_card(5),
                ),
            ],
        );
    }
}
