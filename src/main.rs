// Play Connect 4 using monte carlo simulation

mod board;
mod play;

use board::{Board, Cell};

fn main() {
    let mut board = Board::new();
    let mut cell = Cell::X;
    loop {
        board.print();
        if board.is_full() {
            println!("It's a draw!");
            break;
        }
        let mut best_score = 0;
        let mut best_move = None;
        for x in 0..board::WIDTH {
            if board.is_valid_move(x) {
                let score = play::monte_carlo(&board, cell);

                if score > best_score {
                    best_score = score;
                    best_move = Some(x);
                }
            }
        }
        match best_move {
            Some(x) => {
                board.make_move(x);
                cell = match cell {
                    Cell::X => Cell::O,
                    Cell::O => Cell::X,
                    Cell::Empty => unreachable!(),
                };
            }
            None => {
                println!("No valid moves left!");
                break;
            }
        }
    }
}
