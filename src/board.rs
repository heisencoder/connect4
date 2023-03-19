// Defines the connect 4 board

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 6;
const PADDED_HEIGHT: usize = 8; // align to a byte boundary

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    X,
    O,
}

impl Cell {
    pub fn from_char(c: char) -> Option<Cell> {
        match c {
            '.' => Some(Cell::Empty),
            'X' => Some(Cell::X),
            'O' => Some(Cell::O),
            _ => None,
        }
    }

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

#[derive(Clone, Copy, PartialEq)]
pub enum MoveResult {
    None,    // No win, loss, or draw
    Win,     // The move wins the game
    Draw,    // The game is filled up and is a draw
    Illegal, // The move is illegal
}

#[derive(Clone, Copy, PartialEq)]
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
        } else if (self.bitmap & bit) != 0 {
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
            _ => Cell::Empty,
        }
    }

    /// Adds the current player's piece to the given column.
    /// Returns whether this move results in a win.
    pub fn make_move(&mut self, x: usize) -> MoveResult {
        if !self.is_valid_move(x) {
            return MoveResult::Illegal;
        }

        let bit = ((self.mask + (1 << x * PADDED_HEIGHT)) | self.mask) ^ self.mask;
        self.mask |= bit;
        self.bitmap |= bit;
        self.bitmap ^= self.mask; // Flip all colors
        self.moves += 1;
        MoveResult::None
    }

    pub fn is_full(&self) -> bool {
        self.moves == HEIGHT * WIDTH
    }

    pub fn is_win(&self, cell: Cell) -> bool {
        unimplemented!("not implemented!");
    }

    pub fn print(&self) {
        for y in (0..HEIGHT).rev() {
            for x in 0..WIDTH {
                print!("|{} ", self.get(x, y).to_char());
            }
            println!("|");
        }
        for _x in 0..WIDTH {
            print!("+--");
        }
        println!("+");
        for x in 0..WIDTH {
            print!("|{} ", x);
        }
        println!("|");
        println!();
    }
}

#[cfg(test)]
mod board_tests {
    use crate::board::MoveResult;

    use super::{Board, Cell, HEIGHT, WIDTH};

    #[test]
    fn move_get() {
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

        for y in 0..HEIGHT {
            assert!(board.make_move(0) == MoveResult::None);
            board.print();
            assert!(board.get(0, y) == cell);
            cell = cell.switch();
        }
        assert!(board.make_move(0) == MoveResult::Illegal);
        board.print();
    }
}
