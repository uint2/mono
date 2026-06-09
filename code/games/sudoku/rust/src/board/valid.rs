use super::*;

impl Board {
    /// Checks if the current board is valid. Doesn't matter if it's solved or
    /// not.
    pub fn is_valid(&self) -> Result<()> {
        for l1 in 0..81 {
            if self[l1].has_no_candidates() {
                return Err(Error::NoCandidates(l1));
            }
            for l2 in location::related(l1) {
                // l1 sees l2.
                let c1 = self[l1];
                let c2 = self[l2];
                // If neither of the cells are solved, there is no (immediate)
                // validity checking required.
                if c1.is_solved() || c2.is_solved() {
                    if !c1.intersect(c2).has_no_candidates() {
                        return Err(Error::InvalidBoardFromClash(l1, l2));
                    }
                }
            }
        }
        Ok(())
    }

    /// Checks if this sudoku board is solved.
    pub fn is_solved(&self) -> Result<()> {
        if !self.cells.iter().all(|v| v.is_solved()) {
            return Err(Error::BoardNotSolved);
        }
        self.is_valid()
    }
}
