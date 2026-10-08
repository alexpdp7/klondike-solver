fn main() {
    let initial_state =
        klondike_solver::state::KlondikeState::new(klondike_solver::deck::shuffled_deck());
    let final_state = klondike_solver::solver::solve(initial_state.clone());
    match final_state {
        Some(final_state) => {
            println!("{}", initial_state.as_text());
            for movement in &final_state.movements {
                println!("{}", movement);
            }
        }
        None => {
            println!("No solution");
        }
    }
}
