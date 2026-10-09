use std::collections::HashSet;

use crate::column::Column;
use crate::deck::Value;
use crate::draw_pile::DrawPile;
use crate::state::KlondikeState;

#[must_use]
pub fn solve(state: KlondikeState) -> Option<KlondikeState> {
    let mut states = std::collections::BinaryHeap::new();
    let mut best_state = EvaluableState(state.clone());
    let mut max_moves = state.movements.len();
    let mut seen_positions = 0;
    let mut dupes = 0;
    let mut seen_states = HashSet::new();
    states.push(EvaluableState(state));
    loop {
        let state = states.pop()?;
        if state.0.is_solved() {
            return Some(state.0);
        }

        let movementless_klondike_state = MovementlessKlondikeState::from_full_state(&state.0);

        if seen_states.contains(&movementless_klondike_state) {
            dupes += 1;
            continue;
        }

        seen_states.insert(movementless_klondike_state);

        seen_positions += 1;
        if state.0.movements.len() > max_moves {
            max_moves = state.0.movements.len();
            println!("seen {max_moves} max_moves in {seen_positions} seen positions with {dupes} duplicate positions");
        }
        if best_state < state {
            best_state = state.clone();
            println!("{}", best_state.0.as_text());
        }
        for state in state.0.possible_moves() {
            states.push(EvaluableState(state.clone()));
        }
    }
}

#[derive(Clone)]
struct EvaluableState(KlondikeState);

impl EvaluableState {
    pub fn score(&self) -> (isize, isize, isize) {
        // Values should be small enough to cast from usize to isizes
        (
            -(self.0.cards_in_columns_and_draw_pile().cast_signed()),
            -(self.0.covered_cards_in_columns().cast_signed()),
            -(self.0.movements.len().cast_signed()),
        )
    }
}

impl Eq for EvaluableState {}

impl PartialEq for EvaluableState {
    fn eq(&self, other: &Self) -> bool {
        self.score().eq(&other.score())
    }
}

impl PartialOrd for EvaluableState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for EvaluableState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score().cmp(&other.score())
    }
}

#[derive(Hash, Eq, PartialEq)]
pub struct MovementlessKlondikeState {
    pub collected_clubs: Option<Value>,
    pub collected_spades: Option<Value>,
    pub collected_diamonds: Option<Value>,
    pub collected_hearts: Option<Value>,

    pub columns: [Column; 7],
    pub draw_pile: DrawPile,
}

impl MovementlessKlondikeState {
    #[must_use]
    pub fn from_full_state(state: &KlondikeState) -> MovementlessKlondikeState {
        MovementlessKlondikeState {
            collected_clubs: state.collected_clubs,
            collected_spades: state.collected_spades,
            collected_diamonds: state.collected_diamonds,
            collected_hearts: state.collected_hearts,
            columns: state.columns.clone(),
            draw_pile: state.draw_pile.clone(),
        }
    }
}
