use crate::prelude::*;

/// Returns the solved board, and the number of backtracks.
pub fn solve(game: &GameData, verbose: bool) -> (Board, GameStat) {
    let start_t = Instant::now();
    let mut stack = Vec::with_capacity(512);
    stack.push(game.to_board());

    macro_rules! vprintln {
        ($($arg:tt)*) => {{
            if verbose {
                std::println!($($arg)*)
            }
        }};
    }

    let mut backtracks = 0;

    let mut sort_buf = Vec::with_capacity(4);

    for world in 0.. {
        let mut board = stack.pop().expect("Ran out of boards to try.");
        if verbose {
            vprintln!("world[{world}], stack length: {}, starting board:", stack.len());
            vprintln!("{board}");
        }
        if let Err(_) = board.fast_forward() {
            vprintln!("\x1b[33mworld[{world}] led to an invalid state.\x1b[m");
            vprintln!("{board}");
            // Board is invalid. Try the next one.
            backtracks += 1;
            continue;
        }
        // At this point, the fast-forwarded board is valid. No more
        // heuristically known moves to execute.
        vprintln!("\x1b[35mworld[{world}] has gone as far as it could.\x1b[m");
        vprintln!("{board}");

        let next = board.first_unsolved_cell_location();

        // Find the first cell that's unsolved, and try all the candidates.
        let Some((unsolved_loc, popularity)) = next else {
            // Done!
            let stats =
                GameStat::new(game.difficulty, world, backtracks, start_t.elapsed());
            return (board, stats);
        };
        vprintln!(
            "\x1b[36mworld[{world}] Investigating at {:?}.\x1b[m",
            unsolved_loc.coords()
        );

        let cell = board[unsolved_loc];
        for (i, suggest) in NINE_CELLS.into_iter().enumerate() {
            if cell.contains(suggest) {
                let mut b2 = board.clone();
                b2.set_solved_cell(unsolved_loc, suggest);
                sort_buf.push((popularity[i], b2));
            }
        }
        assert!(sort_buf.len() < 4);
        sort_buf.sort_by(|a, b| a.0.cmp(&b.0));
        stack.extend(sort_buf.drain(..).map(|v| v.1));
    }
    panic!("Can't solve");
}
