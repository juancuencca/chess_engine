const ROWS: usize = 8;
const COLS: usize = 8;

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

struct Cell {
    piece: Option<Box<dyn Piece>>,
}

impl Cell {
    fn new(piece: Option<Box<dyn Piece>>) -> Cell {
        Cell { piece }
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

        Self::init_pawns(&mut cells, cols);
        Self::init_rocks(&mut cells, cols);
        
        Board { rows, cols, cells }    
    }

    fn init_rocks(cells: &mut [Cell], cols: usize) {
        cells[0 * cols + 0] = Cell::new(Some(Box::new(Rock::new())));
        cells[0 * cols + 7] = Cell::new(Some(Box::new(Rock::new())));
        cells[7 * cols + 0] = Cell::new(Some(Box::new(Rock::new())));
        cells[7 * cols + 7] = Cell::new(Some(Box::new(Rock::new())));
    }

    fn init_pawns(cells: &mut [Cell], cols: usize) {
        for col in 0..cols {
            cells[1 * cols + col] = Cell::new(Some(Box::new(Pawn::new())));
            cells[6 * cols + col] = Cell::new(Some(Box::new(Pawn::new())));
        }
    }
}



