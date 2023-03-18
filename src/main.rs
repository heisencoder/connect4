// ChatGPT:
// Write a program in rust to play connect 4 using a bitmap for storing the game
// board and using monte carlo simulation for finding the next best move.

// Here's a possible implementation of a Connect Four game in Rust that uses a bitmap
// to store the game board and a Monte Carlo simulation to find the next best move:

use rand::{thread_rng, Rng};

const WIDTH: usize = 7;
const HEIGHT: usize = 6;
const NUM_MOVES: usize = 1000;

#[derive(Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Red,
    Yellow,
}

impl Cell {
    fn from_char(c: char) -> Option<Cell> {
        match c {
            ' ' => Some(Cell::Empty),
            'R' => Some(Cell::Red),
            'Y' => Some(Cell::Yellow),
            _ => None,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Cell::Empty => ' ',
            Cell::Red => 'R',
            Cell::Yellow => 'Y',
        }
    }
}

struct Board {
    bitmap: u64,
}

impl Board {
    fn new() -> Self {
        Self { bitmap: 0 }
    }

    fn get(&self, x: usize, y: usize) -> Cell {
        let pos = y * WIDTH + x;
        let bit = 1 << pos;
        if (self.bitmap & bit) != 0 {
            Cell::Red
        } else if (self.bitmap & (bit << HEIGHT)) != 0 {
            Cell::Yellow
        } else {
            Cell::Empty
        }
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) {
        let pos = y * WIDTH + x;
        let bit = 1 << pos;
        match cell {
            Cell::Empty => {
                self.bitmap &= !(bit | (bit << HEIGHT));
            }
            Cell::Red => {
                self.bitmap |= bit;
                self.bitmap &= !(bit << HEIGHT);
            }
            Cell::Yellow => {
                self.bitmap |= bit << HEIGHT;
                self.bitmap &= !bit;
            }
        }
    }

    fn is_valid_move(&self, x: usize) -> bool {
        let pos = 1 << (HEIGHT - 1) * WIDTH + x;
        (self.bitmap & pos) == 0
    }

    fn make_move(&mut self, x: usize, cell: Cell) -> bool {
        for y in 0..HEIGHT {
            if self.get(x, y) == Cell::Empty {
                self.set(x, y, cell);
                return true;
            }
        }
        false
    }

    fn is_full(&self) -> bool {
        (self.bitmap & ((1 << HEIGHT * WIDTH) - 1)) == ((1 << HEIGHT * WIDTH) - 1)
    }

    fn is_win(&self, cell: Cell) -> bool {
        let mut mask = 0;
        for i in 0..WIDTH {
            mask |= 1 << i * HEIGHT;
        }
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pos = y * WIDTH + x;
                let bit = 1 << pos;
                if (self.bitmap & bit) == (mask & bit) {
                    return true;
                }
                if x + 3 < WIDTH {
                    if (self.bitmap & (bit | (bit << 1) | (bit << 2) | (bit << 3)))
                        == (mask & (bit | (bit << 1) | (bit << 2) | (bit << 3)))
                    {
                        return true;
                    }
                }
                if y + 3 < HEIGHT {
                    if (self.bitmap
                        & (bit | (bit << WIDTH) | (bit << 2 * WIDTH) | (bit << 3 * WIDTH)))
                        == (mask & (bit | (bit << WIDTH) | (bit << 2 * WIDTH) | (bit << 3 * WIDTH)))
                    {
                        return true;
                    }
                }
                if x + 3 < WIDTH && y + 3 < HEIGHT {
                    if (self.bitmap
                        & (bit
                            | (bit << (HEIGHT + 1))
                            | (bit << 2 * (HEIGHT + 1))
                            | (bit << 3 * (HEIGHT + 1))))
                        == (mask
                            & (bit
                                | (bit << (HEIGHT + 1))
                                | (bit << 2 * (HEIGHT + 1))
                                | (bit << 3 * (HEIGHT + 1))))
                    {
                        return true;
                    }
                }
                if x >= 3 && y + 3 < HEIGHT {
                    if (self.bitmap
                        & (bit
                            | (bit << (HEIGHT - 1))
                            | (bit << 2 * (HEIGHT - 1))
                            | (bit << 3 * (HEIGHT - 1))))
                        == (mask
                            & (bit
                                | (bit << (HEIGHT - 1))
                                | (bit << 2 * (HEIGHT - 1))
                                | (bit << 3 * (HEIGHT - 1))))
                    {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn print(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("|{} ", self.get(x, y).to_char());
            }
            println!("|");
        }
        for x in 0..WIDTH {
            print!("+-");
        }
        println!("+");
        for x in 0..WIDTH {
            print!("|{} ", x);
        }
        println!("|");
    }
}

fn monte_carlo(board: &Board, cell: Cell) -> usize {
    let mut rng = thread_rng();
    let mut wins = 0;
    let mut moves = 0;
    for _ in 0..NUM_MOVES {
        let mut sim_board = board.clone();
        let mut sim_cell = cell;
        loop {
            let mut valid_moves = Vec::new();
            for x in 0..WIDTH {
                if sim_board.is_valid_move(x) {
                    valid_moves.push(x);
                }
            }
            if valid_moves.is_empty() {
                break;
            }
            let index = rng.gen_range(0, valid_moves.len());
            let x = valid_moves[index];
            sim_board.make_move(x, sim_cell);
            if sim_board.is_win(sim_cell) {
                wins += 1;
                break;
            }
            sim_cell = match sim_cell {
                Cell::Red => Cell::Yellow,
                Cell::Yellow => Cell::Red,
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
        wins * WIDTH / moves
    }
}

fn main() {
    let mut board = Board::new();
    let mut cell = Cell::Red;
    loop {
        board.print();
        if board.is_win(cell) {
            println!("{} wins!", cell.to_char());
            break;
        }
        if board.is_full() {
            println!("It's a draw!");
            break;
        }
        let mut best_score = 0;
        let mut best_move = None;
        for x in 0..WIDTH {
            if board.is_valid_move(x) {
                let score = monte_carlo(&board, cell);

                if score > best_score {
                    best_score = score;
                    best_move = Some(x);
                }
            }
        }
        match best_move {
            Some(x) => {
                board.make_move(x, cell);
                cell = match cell {
                    Cell::Red => Cell::Yellow,
                    Cell::Yellow => Cell::Red,
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
