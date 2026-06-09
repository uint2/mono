use super::*;

impl Index<u8> for Board {
    type Output = Cell;
    fn index(&self, index: u8) -> &Self::Output {
        unsafe { self.cells.get_unchecked(index as usize) }
    }
}

impl IndexMut<u8> for Board {
    fn index_mut(&mut self, index: u8) -> &mut Self::Output {
        unsafe { self.cells.get_unchecked_mut(index as usize) }
    }
}
