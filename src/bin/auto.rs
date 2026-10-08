fn main() {
    let state = klondike_solver::state::KlondikeState::new(klondike_solver::deck::shuffled_deck());
    let state = klondike_solver::solver::solve(state);
    println!("{}", state.as_text());
    println!("{:?}", state.movements);
}
