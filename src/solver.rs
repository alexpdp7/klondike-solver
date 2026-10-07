use rand::seq::SliceRandom;

use crate::state::KlondikeState;

pub fn solve(state: KlondikeState) -> KlondikeState {
    let mut states = vec![state];
    let mut rng = rand::rng();
    let mut best_state: Option<KlondikeState> = None;
    loop {
        states.shuffle(&mut rng);
        let state = states.pop().unwrap();
        if best_state.is_none()
            || best_state.clone().unwrap().cards_in_columns_and_draw_pile()
                > state.cards_in_columns_and_draw_pile()
        {
            best_state = Some(state.clone());
            println!("{}", best_state.clone().unwrap().as_text());
        }
        states.append(&mut state.possible_moves());
    }
}
