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
        if self.is_empty() {
            return vec![];
        }

        let mut result = vec![];

        fn draw_up_to_three(cards: &[Card]) -> Option<(Vec<Card>, Vec<Card>, Card)> {
            if cards.is_empty() {
                return None;
            }

            let (drawn, rest) = cards.split_at_checked(std::cmp::min(3, cards.len()))?;
            let (card, drawn) = drawn.split_last().unwrap();
            Some((drawn.to_vec(), rest.to_vec(), *card))
        }

        let drawn = self.drawn.clone();
        let mut to_draw = self.to_draw.clone();

        let binding = drawn.clone();
        let draw = binding.split_last().unwrap();
        let first_card = *draw.0;
        let this_drawn = draw.1.to_vec();

        result.push((
            DrawPile {
                drawn: this_drawn.clone(),
                to_draw: to_draw.clone(),
            },
            first_card,
        ));

        loop {
            let card;
            let this_drawn;
            match draw_up_to_three(&to_draw) {
                None => {
                    if drawn.is_empty() {
                        break;
                    }
                    let draw = draw_up_to_three(&self.drawn).unwrap();
                    this_drawn = draw.0;
                    to_draw = draw.1;
                    card = draw.2;
                }
                Some(draw) => {
                    this_drawn = [drawn.clone(), draw.0].concat();
                    to_draw = draw.1;
                    card = draw.2;
                }
            }
            if card == first_card {
                break;
            }
            result.push((
                DrawPile {
                    drawn: this_drawn.clone(),
                    to_draw: to_draw.clone(),
                },
                card,
            ));
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
    fn assert_candidate_draws(draw_pile: DrawPile, expected: Vec<(DrawPile, Card)>) {
        assert!(dbg!(dbg!(draw_pile).candidate_draws()) == dbg!(expected));
    }

    #[test]
    fn candidate_draws_0_3_6() {
        assert_candidate_draws(
            draw_pile(0, 3, 6),
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

    #[test]
    fn candidate_draws_0_3_3() {
        assert_candidate_draws(
            draw_pile(0, 3, 3),
            vec![(
                DrawPile {
                    drawn: deck_slice(0, 2),
                    to_draw: deck_slice(3, 3),
                },
                deck_card(2),
            )],
        );
    }

    #[test]
    fn candidate_draws_0_2_2() {
        assert_candidate_draws(
            draw_pile(0, 2, 2),
            vec![(
                DrawPile {
                    drawn: deck_slice(0, 1),
                    to_draw: deck_slice(2, 2),
                },
                deck_card(1),
            )],
        );
    }

    #[test]
    fn candidate_draws_0_1_1() {
        assert_candidate_draws(
            draw_pile(0, 1, 1),
            vec![(
                DrawPile {
                    drawn: deck_slice(0, 0),
                    to_draw: deck_slice(1, 1),
                },
                deck_card(0),
            )],
        );
    }

    #[test]
    fn candidate_draws_0_6_6() {
        assert_candidate_draws(
            draw_pile(0, 6, 6),
            vec![
                (
                    DrawPile {
                        drawn: deck_slice(0, 5),
                        to_draw: deck_slice(6, 6),
                    },
                    deck_card(5),
                ),
                (
                    DrawPile {
                        drawn: deck_slice(0, 2),
                        to_draw: deck_slice(3, 6),
                    },
                    deck_card(2),
                ),
            ],
        );
    }

    #[test]
    fn candidate_draws_0_5_5() {
        assert_candidate_draws(
            draw_pile(0, 5, 5),
            vec![
                (
                    DrawPile {
                        drawn: deck_slice(0, 4),
                        to_draw: deck_slice(5, 5),
                    },
                    deck_card(4),
                ),
                (
                    DrawPile {
                        drawn: deck_slice(0, 2),
                        to_draw: deck_slice(3, 5),
                    },
                    deck_card(2),
                ),
            ],
        );
    }

    #[test]
    fn candidate_draws_0_0_0() {
        assert_candidate_draws(draw_pile(0, 0, 0), vec![]);
    }
}
