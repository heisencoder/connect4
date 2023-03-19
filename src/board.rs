// Defines the connect 4 board

pub const WIDTH: usize = 7;
pub const HEIGHT: usize = 6;

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    X,
    O,
}

impl Cell {
    pub fn from_char(c: char) -> Option<Cell> {
        match c {
            ' ' => Some(Cell::Empty),
            'R' => Some(Cell::X),
            'Y' => Some(Cell::O),
            _ => None,
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Cell::Empty => ' ',
            Cell::X => 'R',
            Cell::O => 'Y',
        }
    }
}

#[derive(Clone)]
pub struct Board {
    bitmap: u64,
}

impl Board {
    pub fn new() -> Self {
        Self { bitmap: 0 }
    }

    pub fn get(&self, x: usize, y: usize) -> Cell {
        let pos = y * WIDTH + x;
        let bit = 1 << pos;
        if (self.bitmap & bit) != 0 {
            Cell::X
        } else if (self.bitmap & (bit << HEIGHT)) != 0 {
            Cell::O
        } else {
            Cell::Empty
        }
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        let pos = y * WIDTH + x;
        let bit = 1 << pos;
        match cell {
            Cell::Empty => {
                self.bitmap &= !(bit | (bit << HEIGHT));
            }
            Cell::X => {
                self.bitmap |= bit;
                self.bitmap &= !(bit << HEIGHT);
            }
            Cell::O => {
                self.bitmap |= bit << HEIGHT;
                self.bitmap &= !bit;
            }
        }
    }

    pub fn is_valid_move(&self, x: usize) -> bool {
        let pos = 1 << (HEIGHT - 1) * WIDTH + x;
        (self.bitmap & pos) == 0
    }

    pub fn make_move(&mut self, x: usize, cell: Cell) -> bool {
        for y in 0..HEIGHT {
            if self.get(x, y) == Cell::Empty {
                self.set(x, y, cell);
                return true;
            }
        }
        false
    }

    pub fn is_full(&self) -> bool {
        (self.bitmap & ((1 << HEIGHT * WIDTH) - 1)) == ((1 << HEIGHT * WIDTH) - 1)
    }

    pub fn is_win(&self, cell: Cell) -> bool {
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

    pub fn print(&self) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("|{} ", self.get(x, y).to_char());
            }
            println!("|");
        }
        for x in 0..WIDTH {
            print!("+--");
        }
        println!("+");
        for x in 0..WIDTH {
            print!("|{} ", x);
        }
        println!("|");
    }
}
