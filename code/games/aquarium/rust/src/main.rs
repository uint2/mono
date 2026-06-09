use aquarium::Game;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

fn get_problem_filenames<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    let mut files = vec![];
    if let Ok(v) = std::fs::read_dir(dir) {
        for file in v.filter_map(|v| v.ok()) {
            files.push(file.path());
        }
    }
    files
}

fn run_all(games: &Vec<Game>) -> Duration {
    let start = Instant::now();
    for game in games {
        match game.solve() {
            Some(_) => {}
            None => panic!("UNSOLVED"),
        }
    }
    Instant::elapsed(&start)
}

#[tokio::main]
async fn main() {
    let games: Vec<Game> = get_problem_filenames("../problems/problem-db")
        .into_iter()
        .filter(|v| v.to_string_lossy().contains("15x15_hard"))
        .filter_map(Game::from_filename)
        .collect();

    let mut timings: Vec<Duration> = vec![];
    let ignore = 10;
    let runs = 10;

    println!("warming up...");

    for r in 0..ignore {
        run_all(&games);
        println!("warm up {}/{}", r + 1, ignore);
    }

    println!("running...");

    for r in 0..runs {
        timings.push(run_all(&games));
        println!("complete run {}/{}", r + 1, runs);
    }

    let avg = timings.iter().fold(Duration::ZERO, |a, c| a + *c);
    let avg = avg / timings.len() as u32;

    println!("aquarium-rust: done execution! ({:?})", avg);
    println!("({:?} per game)", avg / games.len() as u32);
}
