/// Ensured to be valid.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

pub struct Grid<T> {
    pub tiles: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    /// Checks the validity of a position.
    pub fn pos(&self, row: usize, col: usize) -> Option<Pos> {
        if row < self.tiles[0].len() && col < self.tiles.len() {
            Some(Pos { row, col })
        } else {
            None
        }
    }

    pub fn get(&self, Pos { row, col }: Pos) -> &T {
        &self.tiles[row][col]
    }

    pub fn get_mut(&mut self, Pos { row, col }: Pos) -> &mut T {
        &mut self.tiles[row][col]
    }

    pub fn shift(&self, Pos { row, col }: Pos, (dr, dc): (isize, isize)) -> Option<Pos> {
        let row = row.checked_add_signed(dr)?;
        let col = col.checked_add_signed(dc)?;

        self.pos(row, col)
    }

    pub fn shift_wraparound(&self, Pos { row, col }: Pos, (dr, dc): (isize, isize)) -> Pos {
        let (rows, cols) = (self.tiles.len(), self.tiles[0].len());

        let row = (row + rows).checked_add_signed(dr).unwrap() % rows;
        let col = (col + cols).checked_add_signed(dc).unwrap() % cols;

        self.pos(row, col).unwrap()
    }
}
