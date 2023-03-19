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

#[derive(Clone)]
pub struct Board {
    // Pieces are stored in columns
    bitmap: u64,
    mask: u64,
}

impl Board {
    pub fn new() -> Self {
        Self { bitmap: 0, mask: 0 }
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        let pos = x * PADDED_HEIGHT + y;
        let bit = 1 << pos;
        if bit & self.mask == 0 {
            Cell::Empty
        } else if (self.bitmap & bit) != 0 {
            Cell::X
        } else {
            Cell::O
        }
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        let pos = x * PADDED_HEIGHT + y;
        let bit = 1 << pos;
        match cell {
            Cell::Empty => {
                self.bitmap &= !bit;
                self.mask &= !bit;
            }
            Cell::X => {
                self.bitmap |= bit;
                self.mask |= bit;
            }
            Cell::O => {
                self.bitmap &= !bit;
                self.mask |= bit;
            }
        }
    }

    pub fn is_valid_move(&self, x: usize) -> bool {
        self.mask & (1 << x * PADDED_HEIGHT + HEIGHT - 1) == 0
    }

    pub fn make_move(&mut self, x: usize, cell: Cell) -> bool {
        if cell == Cell::Empty {
            unimplemented!("Removing cells not implemented!");
        }
        if !self.is_valid_move(x) {
            panic!("Invalid Move: {}!", x);
        }

        let bit = ((self.mask + (1 << x * PADDED_HEIGHT)) | self.mask) ^ self.mask;
        self.mask |= bit;

        if cell == Cell::X {
            self.bitmap |= bit;
        }
        true
    }

    pub fn is_full(&self) -> bool {
        (self.bitmap & ((1 << HEIGHT * WIDTH) - 1)) == ((1 << HEIGHT * WIDTH) - 1)
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
    use super::{Board, Cell, HEIGHT, WIDTH};

    #[test]
    fn move_get() {
        let mut board = Board::new();
        let mut cell = Cell::X;

        for x in 0..WIDTH {
            board.make_move(x, cell);
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
            assert!(board.make_move(0, cell));
            board.print();
            assert!(board.get(0, y) == cell);
            cell = cell.switch();
        }
        assert!(!board.make_move(0, cell));
        board.print();
    }
}
