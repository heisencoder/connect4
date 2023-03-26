// Defines the connect 4 board

use std::convert::From;

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

impl From<MoveResult> for Cell {
    fn from(item: MoveResult) -> Self {
        match item {
            MoveResult::WinX => Cell::X,
            MoveResult::WinO => Cell::O,
            _ => Cell::Empty,
        }
    }
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

    pub fn get_move_count(&self) -> usize {
        self.moves
    }

    pub fn get(&self, col: usize, y: usize) -> Cell {
        let pos = col * PADDED_HEIGHT + y;
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

    pub fn is_valid_move(&self, col: usize) -> bool {
        col < WIDTH && self.mask & (1 << col * PADDED_HEIGHT + HEIGHT - 1) == 0
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
        self.check_vertical_win(bit, col)
            || self.check_horizontal_win(bit, col)
            || self.check_upper_diagonal_win(bit, col)
            || self.check_lower_diagonal_win(bit, col)
    }

    fn check_vertical_win(&self, bit: u64, col: usize) -> bool {
        let col0_bit = bit >> (col * PADDED_HEIGHT); // shift to first column

        // Check vertical
        // Only possible to have vertical win in rows 3 or higher
        if col0_bit >= 1 << 3 {
            let mask = ((1 << 4) - 1) * (bit >> 3);
            if self.bitmap & mask == mask {
                return true;
            }
        }
        false
    }

    fn check_horizontal_win(&self, bit: u64, col: usize) -> bool {
        const LEFT_MASK: u64 = 1 << (0 * PADDED_HEIGHT)
            | 1 << (1 * PADDED_HEIGHT)
            | 1 << (2 * PADDED_HEIGHT)
            | 1 << (3 * PADDED_HEIGHT);

        let mut mask = if col > WIDTH - 4 {
            bit >> (PADDED_HEIGHT * (col - (WIDTH - 4)))
        } else {
            bit
        } * LEFT_MASK;

        for _ in 0..=col.min(WIDTH - col - 1) {
            if self.bitmap & mask == mask {
                return true;
            }
            mask >>= PADDED_HEIGHT;
        }
        false
    }

    fn check_upper_diagonal_win(&self, bit: u64, col: usize) -> bool {
        const MASK: u64 = 1 << (0 * PADDED_HEIGHT + 0)
            | 1 << (1 * PADDED_HEIGHT + 1)
            | 1 << (2 * PADDED_HEIGHT + 2)
            | 1 << (3 * PADDED_HEIGHT + 3);

        let mut mask = if col > WIDTH - 4 {
            bit >> ((PADDED_HEIGHT + 1) * (col - (WIDTH - 4)))
        } else {
            bit
        } * MASK;

        for _ in 0..=col.min(WIDTH - col - 1) {
            //println!("upper=> {col} {0:#064b}", self.bitmap);
            //println!("          {mask:#064b}");
            if self.bitmap & mask == mask {
                return true;
            }
            if mask & ((1 << (PADDED_HEIGHT + 1)) - 1) != 0 {
                break;
            }
            mask >>= PADDED_HEIGHT + 1;
        }
        false
    }

    fn check_lower_diagonal_win(&self, bit: u64, col: usize) -> bool {
        const MASK: u64 = 1 << (0 * PADDED_HEIGHT - 0)
            | 1 << (1 * PADDED_HEIGHT - 1)
            | 1 << (2 * PADDED_HEIGHT - 2)
            | 1 << (3 * PADDED_HEIGHT - 3);

        let mut mask = if col > WIDTH - 4 {
            bit >> ((PADDED_HEIGHT - 1) * (col - (WIDTH - 4)))
        } else {
            bit
        } * MASK;

        for _ in 0..=col.min(WIDTH - col - 1) {
            //println!("lower=> {col} {0:#064b}", self.bitmap);
            //println!("          {mask:#064b}");
            if self.bitmap & mask == mask {
                return true;
            }
            mask >>= PADDED_HEIGHT - 1;
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
    fn fill_board_with_horizontal_wins() {
        let mut board = Board::new();

        for col in 0..WIDTH {
            for _row in 0..HEIGHT {
                let result = board.make_move(col);
                if col < 3 {
                    assert_eq!(result, MoveResult::None);
                } else if (board.get_move_count() % 2) == 0 {
                    assert_eq!(result, MoveResult::WinO);
                } else {
                    assert_eq!(result, MoveResult::WinX);
                }
            }
        }
    }

    #[test]
    fn fill_board_with_diagonal_wins() {
        let mut board = Board::new();

        for row in 0..HEIGHT {
            if row & 1 == 0 {
                for col in 0..WIDTH {
                    let result = board.make_move(col);
                    if row < 3 {
                        assert_eq!(result, MoveResult::None);
                    } else if (board.get_move_count() % 2) == 0 {
                        assert_eq!(result, MoveResult::WinO);
                    } else {
                        assert_eq!(result, MoveResult::WinX);
                    }
                }
            } else {
                for col in (0..WIDTH).rev() {
                    let result = board.make_move(col);
                    if row < 3 {
                        assert_eq!(result, MoveResult::None);
                    } else if (board.get_move_count() % 2) == 0 {
                        assert_eq!(result, MoveResult::WinO);
                    } else {
                        assert_eq!(result, MoveResult::WinX);
                    }
                }
            }
        }
    }

    #[test]
    fn fill_row() {
        let mut board = Board::new();
        let mut cell = Cell::X;

        for col in 0..WIDTH {
            board.make_move(col);
            assert!(board.get(col, 0) == cell);
            cell = cell.switch();
        }
    }

    #[test]
    fn fill_col() {
        let mut board = Board::new();
        let mut cell = Cell::X;

        for row in 0..HEIGHT {
            assert!(board.make_move(0) == MoveResult::None);
            assert!(board.get(0, row) == cell);
            cell = cell.switch();
        }
        assert!(board.make_move(0) == MoveResult::Illegal);
    }
}
