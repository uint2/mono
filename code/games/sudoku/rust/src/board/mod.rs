mod display;
mod index;
mod valid;

use crate::prelude::*;

/// A Sudoku Board.
pub struct Board {
    cells: [Cell; 81],

    /// These store locations of the cells that are solved, and not dealt with
    /// yet. Every time we solve a cell, we would like to update the candidates
    /// of all the related cells. This is a sort of queue for that.
    sets: RingBuf<72>,
}

impl Clone for Board {
    fn clone(&self) -> Self {
        Self { cells: self.cells, sets: RingBuf::new() }
    }
}

impl Board {
    pub fn new(data: [u8; 81]) -> Self {
        let mut board = Self { cells: [Cell::new(); 81], sets: RingBuf::new() };
        for loc in 0..81 {
            let value = data[loc as usize];
            if let 1..=9 = value {
                board.set_solved_cell(loc, Cell::from_u8(value));
            }
        }
        board
    }

    pub fn set_solved_cell(&mut self, loc: u8, value_mask: Cell) {
        debug_assert!(value_mask.unique().is_some());
        self[loc] = value_mask;
        self.sets.push_back(loc);
    }

    /// Get, amongst the unsolved cells, the popularity of each candidate.
    pub fn popularity(&self) -> [u8; 10] {
        let mut popularity = [0; 10];
        for j in 0..81 {
            if self.cells[j].is_solved() {
                continue;
            }
            for i in 1..=9 {
                if self.cells[j].contains(Cell::from_u8(i)) {
                    popularity[i as usize] += 1;
                }
            }
        }
        popularity
    }

    /// Get the first unsolved cell. Favors cells with the least remaining options.
    pub fn first_unsolved_cell_location(&self) -> Option<(u8, [u8; 10])> {
        let popularity = self.popularity();
        let rank = |cell: &Cell| {
            (1..=9)
                .filter(|i| cell.contains(Cell::from_u8(*i)))
                .map(|i| popularity[i as usize])
                .sum::<u8>()
        };
        self.cells
            .iter()
            .enumerate()
            .map(|(i, cell)| (i, cell.count_ones(), rank(cell)))
            // Keep only the unsolved ones.
            .filter(|v| v.1 > 1)
            // Sort by least candidates remaining first, and then tie-break
            // by greatest popularity first.
            .min_by(|a, b| a.1.cmp(&b.1).then(a.2.cmp(&b.2).reverse()))
            .map(|v| (v.0 as u8, popularity))
    }

    /// Fast-forwards this board using heuristic rules as far as we can go.
    pub fn fast_forward(&mut self) -> Result<()> {
        self.clear_set_queue();
        let mut unsolved_bits = self.count_ones();
        loop {
            self.apply_rules()?;
            debug_assert!(self.sets.is_empty());
            unsolved_bits = match self.count_ones() {
                // If no changes were made, we end this fast-forward.
                v if v == unsolved_bits => return Ok(()),
                v => v,
            };
        }
    }

    pub fn clear_set_queue(&mut self) -> bool {
        let mut dirty = false;
        while let Some(anchor) = self.sets.pop_front() {
            let anchor_cell = self[anchor];
            debug_assert!(anchor_cell.is_solved(), "Cell[{anchor}] is not solved.");
            for loc in location::related(anchor) {
                if !self[loc].is_solved() {
                    dirty |= !(self[loc].intersect(anchor_cell)).has_no_candidates();
                    self[loc].eliminate(anchor_cell);
                    if self[loc].is_solved() {
                        self.sets.push_back(loc);
                    }
                }
            }
        }
        dirty
    }

    pub fn apply_rules(&mut self) -> Result<()> {
        for mask in NINE_CELLS {
            location::ALL_GROUPS
                .into_iter()
                .map(|group| rules::hidden_groups(self, group, mask, 1))
                .collect::<Result<()>>()?;
        }

        for i in 1..=9 {
            for j in i + 1..=9 {
                let mask = Cell::from_u8(i) | Cell::from_u8(j);
                location::ALL_GROUPS
                    .into_iter()
                    .map(|group| rules::obvious_groups(self, group, mask, 2))
                    .collect::<Result<()>>()?;

                // This algorithm takes longer, but reduces the number of backtracks.
                location::ALL_GROUPS
                    .into_iter()
                    .map(|group| rules::hidden_groups(self, group, mask, 2))
                    .collect::<Result<()>>()?;
            }
        }

        for i in 1..=9 {
            for j in i + 1..=9 {
                let submask = Cell::from_u8(i) | Cell::from_u8(j);
                for k in j + 1..=9 {
                    let mask = submask | Cell::from_u8(k);
                    location::ALL_GROUPS
                        .into_iter()
                        .map(|group| rules::obvious_groups(self, group, mask, 3))
                        .collect::<Result<()>>()?;
                }
            }
        }

        for box_group in location::BOX_GROUPS {
            for mask in NINE_CELLS {
                rules::pointing_pairs(self, box_group, mask);
            }
        }

        self.is_valid()?;

        Ok(())
    }

    /// Mark a cell as solved, so we can update its surroundings later.
    pub fn mark_as_solved(&mut self, loc: u8) {
        self.sets.push_back(loc);
        self.clear_set_queue();
    }
}
