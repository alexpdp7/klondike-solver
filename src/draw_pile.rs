use crate::deck::Card;

#[derive(Debug, Clone)]
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
