const ROWS: usize = 8;
const COLS: usize = 8;

enum Color {
    White,
    Black,
}

trait Piece {

}

struct Pawn {}

impl Pawn {
    fn new() -> Pawn {
        Pawn { }
    }
}

impl Piece for Pawn {}

struct Rock {}

impl Rock {
    fn new() -> Rock {
        Rock {  }
    }
}

impl Piece for Rock {}

struct Horse {}

impl Horse {
    fn new() -> Horse {
        Horse {  }
    }
}

impl Piece for Horse {}

struct Bishop {}

impl Bishop {
    fn new() -> Bishop {
        Bishop {  }
    }
}

impl Piece for Bishop {}

struct Queen {}

impl Queen {
    fn new() -> Queen {
        Queen {  }
    }
}

impl Piece for Queen {}

struct King {}

impl King {
    fn new() -> King {
        King {  }
    }
}

impl Piece for King {}

struct Cell {
    piece: Option<Box<dyn Piece>>,
    color: Color,
}

impl Cell {
    fn new(piece: Option<Box<dyn Piece>>, color: Color) -> Cell {
        Cell { piece, color }
    }

    fn set_piece(&mut self, piece: Box<dyn Piece>) {
        self.piece = Some(piece);
    }
}

pub struct Board {
    rows: usize,
    cols: usize,
    cells: Vec<Cell>,
}

impl Board {
    pub fn new() -> Board {
        let rows = ROWS;
        let cols = COLS;

        let mut cells = Vec::with_capacity(rows * cols);
        
        for i in 0..rows {
            for j in 0..cols {
                let color = if i + j % 2 == 0 { 
                    Color::White 
                } else { 
                    Color::Black 
                }; 

                cells.push(Cell::new(None, color));
            }
        }
        
        let mut board = Board { rows, cols, cells };
        board.init_table();

        board    
    }

    fn get_position(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    fn init_table(&mut self) {
        let pawns_position = [
            (1, 0), (1, 1), (1, 2), (1, 3), (1, 4), (1, 5), (1, 6), (1, 7), 
            (6, 0), (6, 1), (6, 2), (6, 3), (6, 4), (6, 5), (6, 6), (6, 7), 
        ];

        for (row, col) in pawns_position {
            let position = self.get_position(row, col);
            self.cells[position].set_piece(Box::new(Pawn::new()));
        }

        let rocks_position = [(0, 0), (0, 7), (7, 0), (7, 7)];

        for (row, col) in rocks_position {
            let position = self.get_position(row, col);
            self.cells[position].set_piece(Box::new(Rock::new()));
        }

        let horses_position = [(0, 1), (0, 6), (7, 1), (7, 6)];

        for (row, col) in horses_position {
            let position = self.get_position(row, col);
            self.cells[position].set_piece(Box::new(Horse::new()));
        }

        let bishops_position = [(0, 2), (0, 5), (7, 2), (7, 5)];

        for (row, col) in bishops_position {
            let position = self.get_position(row, col);
            self.cells[position].set_piece(Box::new(Bishop::new()));
        }

        let queens_position = [(0, 3), (7, 3)];

        for (row, col) in queens_position {
            let position = self.get_position(row, col);
            self.cells[position].set_piece(Box::new(Queen::new()));
        }

        let kings_position = [(0, 4), (7, 4)];

        for (row, col) in kings_position {
            let position = self.get_position(row, col);
            self.cells[position].set_piece(Box::new(King::new()));
        }
    }
}



