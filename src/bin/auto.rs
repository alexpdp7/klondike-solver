fn main() {
    let state = klondike_solver::state::KlondikeState::new(klondike_solver::deck::shuffled_deck());
    klondike_solver::solver::solve(state);
}
