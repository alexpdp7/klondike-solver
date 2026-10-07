use crate::state::KlondikeState;

pub fn solve(state: KlondikeState) -> KlondikeState {
    let mut states = std::collections::BinaryHeap::new();
    let mut best_state: Option<EvaluableState> = None;
    states.push(EvaluableState(state));
    loop {
        let state = states.pop().unwrap();
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
    pub fn score(&self) -> (i32, i32) {
        (
            -(self.0.cards_in_columns_and_draw_pile() as i32),
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
