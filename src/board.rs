// Defines the connect 4 board

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 6;
const PADDED_HEIGHT: usize = 8; // align to a byte boundary

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Empty,
    X, // First Player
    O, // Second Player
}

impl Cell {
    pub fn to_char(&self) -> char {
        match self {
            Cell::Empty => '.',
            Cell::X => 'X',
            Cell::O => 'O',
        }
    }

    pub fn switch(&self) -> Cell {
        match self {
            Cell::Empty => Cell::Empty,
            Cell::X => Cell::O,
            Cell::O => Cell::X,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MoveResult {
    None,    // No win, loss, or draw
    WinX,    // The move wins the game for X (i.e., first player)
    WinO,    // The move wins the game for O (i.e., second player)
    Draw,    // The game is filled up and is a draw
    Illegal, // The move is illegal
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Board {
    // Pieces are stored in columns. The player to move next is '1'
    bitmap: u64,

    // bitmap of all board positions that contain a piece (as marked with '1')
    mask: u64,

    // Total number of moves made on this board.
    moves: usize,
}

impl Board {
    pub fn new() -> Self {
        Self {
            bitmap: 0,
            mask: 0,
            moves: 0,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        let pos = x * PADDED_HEIGHT + y;
        let bit = 1 << pos;
        if bit & self.mask == 0 {
            Cell::Empty
        } else if (self.bitmap & bit) == 0 {
            if self.moves & 1 == 0 {
                Cell::X
            } else {
                Cell::O
            }
        } else {
            if self.moves & 1 != 0 {
                Cell::X
            } else {
                Cell::O
            }
        }
    }

    pub fn is_valid_move(&self, x: usize) -> bool {
        x < WIDTH && self.mask & (1 << x * PADDED_HEIGHT + HEIGHT - 1) == 0
    }

    pub fn get_current_player(&self) -> Cell {
        match self.moves & 1 {
            0 => Cell::X,
            1 => Cell::O,
            _ => unreachable!(),
        }
    }

    #[cfg(test)]
    pub fn make_moves(&mut self, moves: &Vec<usize>) -> MoveResult {
        let mut last_result = MoveResult::None;
        for m in moves {
            last_result = self.make_move(*m);
        }
        last_result
    }

    /// Adds the current player's piece to the given column.
    /// Returns whether this move results in a win.
    pub fn make_move(&mut self, col: usize) -> MoveResult {
        if !self.is_valid_move(col) {
            return MoveResult::Illegal;
        }

        let bit = ((self.mask + (1 << col * PADDED_HEIGHT)) | self.mask) ^ self.mask;
        self.mask |= bit;
        self.bitmap ^= self.mask; // Flip all Cells
        self.moves += 1;
        self.print();
        if self.check_win(bit, col) {
            match self.get_current_player() {
                // reverse because this is now the next player.
                Cell::O => MoveResult::WinX,
                Cell::X => MoveResult::WinO,
                _ => unreachable!(),
            }
        } else if self.is_full() {
            MoveResult::Draw
        } else {
            MoveResult::None
        }
    }

    fn check_win(&self, bit: u64, col: usize) -> bool {
        let col0_bit = bit >> (col * PADDED_HEIGHT); // shift to first column

        // Check vertical
        // Only possible to have vertical win in rows 3 or higher
        if col0_bit >= 1 << 3 {
            let pattern = (((1 << 4) - 1) * bit) >> 3;
            if self.bitmap & pattern == pattern {
                return true;
            }
        }

        false
    }

    pub fn is_full(&self) -> bool {
        self.moves == HEIGHT * WIDTH
    }

    pub fn print(&self) {
        for row in (0..HEIGHT).rev() {
            for col in 0..WIDTH {
                print!("|{} ", self.get(col, row).to_char());
            }
            println!("|");
        }
        for _col in 0..WIDTH {
            print!("+--");
        }
        println!("+");
        for col in 0..WIDTH {
            print!("|{} ", col);
        }
        println!("|");
        println!();
    }
}

#[cfg(test)]
mod board_tests {
    use std::vec;

    use crate::board::MoveResult;

    use super::{Board, Cell, HEIGHT, WIDTH};

    #[test]
    fn check_vertical_win() {
        let mut board = Board::new();

        // fill first two columns up to 3 rows high, each with the same Cell.
        assert_eq!(board.make_moves(&vec![0, 1, 0, 1, 0, 1]), MoveResult::None);
        board.print();
        assert_eq!(board.make_move(0), MoveResult::WinX);

        // Fill last two columns, but make O win in the upper-right corner.
        board = Board::new();

        assert_eq!(
            board.make_moves(&vec![6, 5, 6, 5, 5, 6, 5, 6, 5, 6, 4]),
            MoveResult::None
        );
        assert_eq!(board.make_move(6), MoveResult::WinO);
    }

    #[test]
    fn fill_board() {
        let mut board = Board::new();
        let mut moves = 0;

        for x in 0..WIDTH {
            for _row in 0..HEIGHT {
                let result = board.make_move(x);
                moves += 1;
                board.print();
                if moves == WIDTH * HEIGHT {
                    assert!(result == MoveResult::Draw);
                } else {
                    assert!(result != MoveResult::Draw);
                }
            }
        }
    }

    #[test]
    fn fill_row() {
        let mut board = Board::new();
        let mut cell = Cell::X;

        for x in 0..WIDTH {
            board.make_move(x);
            board.print();
            assert!(board.get(x, 0) == cell);
            cell = cell.switch();
        }
    }

    #[test]
    fn fill_col() {
        let mut board = Board::new();
        let mut cell = Cell::X;

        for row in 0..HEIGHT {
            assert!(board.make_move(0) == MoveResult::None);
            board.print();
            assert!(board.get(0, row) == cell);
            cell = cell.switch();
        }
        assert!(board.make_move(0) == MoveResult::Illegal);
        board.print();
    }
}
