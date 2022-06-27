use pleco::bots::JamboreeSearcher;
use pleco::tools::Searcher;
use pleco::Board;
use std::io::{self, BufRead};

fn main() {
    let mut board = Board::start_pos();

    loop {
        // get move for white
        let mut uci_move = String::new();
        io::stdin().lock().read_line(&mut uci_move).unwrap();

        uci_move.pop();
        if !board.apply_uci_move(&uci_move) {
            println!("invalid move! try again");
        }

        // generate next move for black
        let m = JamboreeSearcher::best_move(board.shallow_clone(), 6);

        println!("{}", m.stringify());
        board.apply_move(m);
    }
}
