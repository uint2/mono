// Find sudoku problems here:
//  * https://sudoku.cba.si/en/
//  * https://huggingface.co/datasets/sapientinc/sudoku-extreme/
//
// Notation:
//   Board := 9x9 cells.
//   Box := 3x3 cells.

mod algo;
mod board;
mod cell;
mod game_data;
mod location;
mod prelude;
mod ringbuf;
mod rules;
mod stats;

use prelude::*;

#[allow(unused)]
fn sudoku() {
    let load_start_t = Instant::now();
    let bin_data = std::fs::read("../data/test.bin").unwrap();

    let mut game_datas = GameData::deserialize_binaries(&bin_data);
    game_datas.sort_by(|a, b| b.difficulty.cmp(&a.difficulty));
    println!("Loaded! ({:?}, {} games)", load_start_t.elapsed(), game_datas.len());

    // Take a slice out of all the game datas.
    let game_datas = &game_datas[0..200.min(game_datas.len())];
    // let game_datas = &game_datas[117..=117];

    let mut stats = Vec::with_capacity(game_datas.len());

    for game in game_datas {
        println!("\x1b[35mStart board with rating {}...\x1b[m", game.difficulty);

        let verbose = false;
        let (solved_board, game_stat) = algo::solve(&game, verbose);
        stats.push(game_stat);

        // Print the board and check if it's solved.
        println!("{}", solved_board);
        solved_board.is_solved().unwrap();
        println!("\x1b[32mSolved!\x1b[m");
    }

    let g = GlobalStats::new(stats);
    println!("Games solved:      {}", g.len());
    println!("Elapsed:           {:?}", g.total_t());
    println!("  - Avg:           {:?}", g.avg_t());
    let (i, m) = g.max_t();
    println!(
        "  - High:          {:?} ({} backtracks, rated {}, #{i})",
        m.duration, m.backtracks_used, m.rated_difficulty
    );
    let (i, m) = g.min_t();
    println!(
        "  - Low:           {:?} ({} backtracks, rated {}, #{i})",
        m.duration, m.backtracks_used, m.rated_difficulty
    );
    println!(
        "Difficulty/Effort: \x1b[31m{:.3}\x1b[m/\x1b[32m{:.3}\x1b[m",
        g.avg_difficulty(),
        g.avg_backtracks()
    );
    println!(
        "Wins/Draws/Losses: \x1b[32m{}\x1b[m / \x1b[33m{}\x1b[m / \x1b[31m{}\x1b[m",
        g.wins, g.draws, g.losses
    );
    if let Some(w) = g.biggest_win() {
        println!(
            "Biggest win:       {} \x1b[32m- {}\x1b[m (= {})",
            w.rated_difficulty,
            w.diff(),
            w.backtracks_used
        );
    }
    if let Some(l) = g.biggest_loss() {
        println!(
            "Biggest loss:      {} \x1b[31m+ {}\x1b[m (= {})",
            l.rated_difficulty,
            l.diff(),
            l.backtracks_used
        );
    }
}

#[allow(unused)]
fn serialize_binary() {
    println!("Checking serialization...");
    let t = Instant::now();
    let reader = csv::Reader::from_path("../data/test.csv").unwrap();
    let mut f = File::create("../data/test.bin").unwrap();
    for row in reader.into_records() {
        // title, problem, solution, difficulty rating.
        let row = row.unwrap();
        let difficulty = row.get(3).unwrap().parse().unwrap();

        let board = row.get(1).unwrap();
        let game_data = GameData::from_str(board, difficulty);
        let bin = game_data.serialize_binary();
        assert_eq!(Some(game_data), GameData::deserialize_binary(&bin));
        f.write_all(&bin).unwrap();
    }
    println!("Serialization complete! ({:?})", t.elapsed());
}

fn main() {
    serialize_binary();
    sudoku();
}
