use crate::prelude::*;

/// Check `locs` for groups that are contained in `value_mask`, and eliminate
/// the entire `value_mask` from the rest.
///
/// [https://sudoku.com/sudoku-rules/obvious-pairs/]
/// [https://sudoku.com/sudoku-rules/obvious-triples/]
pub fn obvious_groups(
    b: &mut Board,
    mut locs: [u8; 9],
    value_mask: Cell,
    group_size: u8,
) -> Result<()> {
    let mut grouped = 0;
    for loc in &mut locs {
        if value_mask.contains(b[*loc]) {
            if b[*loc].is_solved() {
                return Ok(());
            }
            grouped += 1;
            *loc = location::INVALID;
        }
    }

    // Not enough for this algorithm to help.
    if grouped != group_size {
        return Ok(());
    }

    for i in 0..9 {
        let loc = *unsafe { locs.get_unchecked(i) };
        if loc == location::INVALID {
            continue;
        }
        // Eliminate this entire group of candidates.
        b[loc].eliminate(value_mask);

        #[cfg(debug_assertions)]
        if b[loc].has_no_candidates() {
            // Invalid because the cell at `anchor` has no possible candidates.
            return Err(Error::NoCandidates(loc));
        }

        if b[loc].is_solved() {
            b.mark_as_solved(loc);
        }
    }

    Ok(())
}

/// [https://sudoku.com/sudoku-rules/hidden-pairs/]
/// [https://sudoku.com/sudoku-rules/hidden-triples/]
pub fn hidden_groups(
    b: &mut Board,
    mut locs: [u8; 9],
    value_mask: Cell,
    group_size: u8,
) -> Result<()> {
    let mut grouped = 9;
    for loc in &mut locs {
        if b[*loc].is_solved() {
            if value_mask.contains(b[*loc]) {
                return Ok(());
            }
            *loc = location::INVALID;
            grouped -= 1;
        } else if (value_mask.intersect(b[*loc])).has_no_candidates() {
            *loc = location::INVALID;
            grouped -= 1;
        }
    }

    // Not enough for this algorithm to help.
    if grouped != group_size {
        return Ok(());
    }

    for i in 0..9 {
        let loc = *unsafe { locs.get_unchecked(i) };
        if loc == location::INVALID {
            continue;
        }
        // Keep only those in the value mask.
        b[loc].retain(value_mask);

        #[cfg(debug_assertions)]
        if b[loc].has_no_candidates() {
            // Invalid because the cell at `anchor` has no possible candidates.
            return Err(Error::NoCandidates(loc));
        }

        if b[loc].is_solved() {
            b.mark_as_solved(loc);
        }
    }

    Ok(())
}

/// Checks if within a box, the only pairs remaining forms a row. `value` should
/// contain a cell with only one candidate.
///
/// [https://sudoku.com/sudoku-rules/pointing-pairs/]
pub fn pointing_pairs(b: &mut Board, box_group: [u8; 9], value: Cell) {
    // `matches` should store the ONLY possible places the `value` can exist
    // within the 3x3 box.
    let mut matches = [false; 9];
    for i in 0..9 {
        let loc = box_group[i];
        let cell = b[loc];
        // No pointing pairs because this value exists outright in the 3x3 Box.
        if cell == value {
            return;
        }
        *unsafe { matches.get_unchecked_mut(i) } = cell.contains(value);
    }

    let 2 = matches.iter().filter(|v| **v).count() else { return };
    let i = matches.iter().position(|v| *v).unwrap();
    let j = i
        + 1
        + unsafe { matches.get_unchecked(i + 1..) }.iter().position(|v| *v).unwrap();

    if i / 3 == j / 3 {
        let r = i - i % 3; // Same row.
        let l1 = *unsafe { box_group.get_unchecked(i) };
        let l2 = *unsafe { box_group.get_unchecked(j) };
        debug_assert!(
            location::row_group(l1).contains(&l2),
            "Column logic error. {:?}, {:?}",
            l1.coords(),
            l2.coords()
        );
        debug_assert!(location::row_group(l1).contains(&box_group[r]));
        debug_assert!(location::row_group(l1).contains(&box_group[r + 1]));
        debug_assert!(location::row_group(l1).contains(&box_group[r + 2]));
        for loc in location::row_group(box_group[i]) {
            if loc == *unsafe { box_group.get_unchecked(r) }
                || loc == *unsafe { box_group.get_unchecked(r + 1) }
                || loc == *unsafe { box_group.get_unchecked(r + 2) }
            {
                continue;
            }
            b[loc].eliminate(value);
            if b[loc].is_solved() {
                b.mark_as_solved(loc);
            }
        }
        return;
    }

    if i % 3 == j % 3 {
        let c = i % 3; // Same column.
        let l1 = *unsafe { box_group.get_unchecked(i) };
        let l2 = *unsafe { box_group.get_unchecked(j) };
        debug_assert!(
            location::column_group(l1).contains(&l2),
            "Column logic error. {:?}, {:?}",
            l1.coords(),
            l2.coords()
        );
        debug_assert!(location::column_group(l1).contains(&box_group[c]));
        debug_assert!(location::column_group(l1).contains(&box_group[c + 3]));
        debug_assert!(location::column_group(l1).contains(&box_group[c + 6]));
        for loc in location::column_group(box_group[i]) {
            if loc == *unsafe { box_group.get_unchecked(c) }
                || loc == *unsafe { box_group.get_unchecked(c + 3) }
                || loc == *unsafe { box_group.get_unchecked(c + 6) }
            {
                continue;
            }
            b[loc].eliminate(value);
            if b[loc].is_solved() {
                b.mark_as_solved(loc);
            }
        }
        return;
    }
}
