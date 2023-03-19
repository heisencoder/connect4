// Plays connect 4 using Monte Carlo

use crate::board::{self, Board, Cell};
use rand::{thread_rng, Rng};

const NUM_MOVES: usize = 1000;

pub fn monte_carlo(board: &Board, cell: Cell) -> usize {
    let mut rng = thread_rng();
    let mut wins = 0;
    let mut moves = 0;
    for _ in 0..NUM_MOVES {
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
            sim_board.make_move(x, sim_cell);
            if sim_board.is_win(sim_cell) {
                wins += 1;
                break;
            }
            sim_cell = match sim_cell {
                Cell::X => Cell::O,
                Cell::O => Cell::X,
                Cell::Empty => unreachable!(),
            };
            moves += 1;
            if sim_board.is_full() {
                break;
            }
        }
    }
    if moves == 0 {
        0
    } else {
        wins * board::WIDTH / moves
    }
}
