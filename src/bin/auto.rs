fn main() {
    let state = klondike_solver::state::KlondikeState::new(klondike_solver::deck::shuffled_deck());
    let state = klondike_solver::solver::solve(state);
    match state {
        Some(state) => {
            println!("{}", state.as_text());
            println!("{:?}", state.movements);
        }
        None => {
            println!("No solution");
        }
    }
}
