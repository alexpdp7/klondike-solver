mod column;
mod deck;
mod draw_pile;
mod state;

fn main() {
    let mut state = state::KlondikeState::new(deck::shuffled_deck());
    loop {
        println!("{}", state.as_text());
        let possible_moves = state.possible_moves();
        if possible_moves.is_empty() {
            std::process::exit(1);
        }
        for (i, possible_move) in possible_moves.iter().enumerate() {
            println!("{} {:?}", i, possible_move.movements.last().unwrap());
        }
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        state = possible_moves
            .get(buffer.trim().parse::<usize>().unwrap())
            .unwrap()
            .clone();
    }
}
