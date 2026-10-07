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

        let all_cards = [self.drawn.clone(), self.to_draw.clone()].concat();
        let mut position = self.drawn.len() - 1;
        let len = all_cards.len();

        let mut candidate_positions = vec![];

        loop {
            candidate_positions.push(position);
            position += 3;
            if position >= len {
                break;
            }
        }

        position = std::cmp::min(2, len - 1);

        loop {
            if position == candidate_positions[0] {
                break;
            }
            if position >= len {
                break;
            }
            candidate_positions.push(position);
            position += 3;
        }

        candidate_positions
            .into_iter()
            .map(|position| {
                let drawn = all_cards[0..position].to_vec();
                let to_draw = all_cards[position + 1..].to_vec();
                (DrawPile { drawn, to_draw }, all_cards[position])
            })
            .collect::<Vec<_>>()
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

    #[test]
    fn candidate_draws_0_3_9() {
        assert_candidate_draws(
            draw_pile(0, 3, 9),
            vec![
                (
                    DrawPile {
                        drawn: deck_slice(0, 2),
                        to_draw: deck_slice(3, 9),
                    },
                    deck_card(2),
                ),
                (
                    DrawPile {
                        drawn: deck_slice(0, 5),
                        to_draw: deck_slice(6, 9),
                    },
                    deck_card(5),
                ),
                (
                    DrawPile {
                        drawn: deck_slice(0, 8),
                        to_draw: deck_slice(9, 9),
                    },
                    deck_card(8),
                ),
            ],
        );
    }
}
