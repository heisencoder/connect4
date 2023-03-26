// Plays connect 4 using Monte Carlo

use crate::board::{self, Board, Cell, MoveResult};
use rand::{thread_rng, Rng};

const NUM_GAMES: usize = 5;

pub fn monte_carlo(board: &Board, cell: Cell) -> f64 {
    let mut rng = thread_rng();
    let mut wins: isize = 0;
    let mut moves = 0;
    for _ in 0..NUM_GAMES {
        let mut sim_board = board.clone();
        let mut sim_cell = cell;
        loop {
            let mut valid_moves = Vec::new();
            for x in 0..board::WIDTH {
                if sim_board.is_valid_move(x) {
                    valid_moves.push(x);
                }
            }
            if valid_moves.is_empty() {
                break;
            }
            let index = rng.gen_range(0..valid_moves.len());
            let x = valid_moves[index];
            let result = sim_board.make_move(x);
            moves += 1;
            if result == MoveResult::WinX || result == MoveResult::WinO {
                //println!("Game Result: {x}: {result:?}");
                //sim_board.print();
                if Cell::from(result) == cell {
                    wins += 1;
                } else {
                    wins -= 1;
                }
                break;
            }
            sim_cell = match sim_cell {
                Cell::X => Cell::O,
                Cell::O => Cell::X,
                Cell::Empty => unreachable!(),
            };
            if sim_board.is_full() {
                //println!("Board is full");
                //sim_board.print();
                break;
            }
        }
    }
    if moves == 0 {
        0.0
    } else {
        wins as f64 / moves as f64
    }
}
