use std::collections::HashSet;

use crate::column::Column;
use crate::deck::Value;
use crate::draw_pile::DrawPile;
use crate::state::KlondikeState;

pub fn solve(state: KlondikeState) -> Option<KlondikeState> {
    let mut states = std::collections::BinaryHeap::new();
    let mut best_state: Option<EvaluableState> = None;
    let mut max_moves = state.movements.len();
    let mut moves = 0;
    let mut seen_states = HashSet::new();
    states.push(EvaluableState(state));
    loop {
        let state = states.pop()?;
        if state.0.is_solved() {
            return Some(state.0);
        }

        let movement_less_klondike_state = MovementLessKlondikeState::from_full_state(&state.0);

        if seen_states.contains(&movement_less_klondike_state) {
            continue;
        }

        seen_states.insert(movement_less_klondike_state);

        moves += 1;
        if state.0.movements.len() > max_moves {
            max_moves = state.0.movements.len();
            println!("seen {max_moves} max_moves in {moves} seen moves");
        }
        if best_state.is_none() || best_state.clone().unwrap() < state {
            best_state = Some(state.clone());
            println!("{}", best_state.clone().unwrap().0.as_text());
        }
        for state in state.0.possible_moves() {
            states.push(EvaluableState(state.clone()))
        }
    }
}

#[derive(Clone)]
struct EvaluableState(KlondikeState);

impl EvaluableState {
    pub fn score(&self) -> (i32, i32, i32) {
        (
            -(self.0.cards_in_columns_and_draw_pile() as i32),
            -(self.0.covered_cards_in_columns() as i32),
            -(self.0.movements.len() as i32),
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
pub struct MovementLessKlondikeState {
    pub collected_clubs: Option<Value>,
    pub collected_spades: Option<Value>,
    pub collected_diamonds: Option<Value>,
    pub collected_hearts: Option<Value>,

    pub columns: [Column; 7],
    pub draw_pile: DrawPile,
}

impl MovementLessKlondikeState {
    pub fn from_full_state(state: &KlondikeState) -> MovementLessKlondikeState {
        MovementLessKlondikeState {
            collected_clubs: state.collected_clubs,
            collected_spades: state.collected_spades,
            collected_diamonds: state.collected_diamonds,
            collected_hearts: state.collected_hearts,
            columns: state.columns.clone(),
            draw_pile: state.draw_pile.clone(),
        }
    }
}
