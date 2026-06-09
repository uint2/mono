#![allow(unused)]

use crate::prelude::*;

/// Game statistics.
pub struct GameStat {
    pub rated_difficulty: u16,
    pub backtracks_used: u16,
    pub world: u16,
    pub duration: Duration,
}

impl GameStat {
    pub const fn new(
        difficulty: u16,
        world: u16,
        backtracks: u16,
        duration: Duration,
    ) -> Self {
        Self {
            rated_difficulty: difficulty,
            world,
            backtracks_used: backtracks,
            duration,
        }
    }

    pub fn diff(&self) -> u16 {
        self.rated_difficulty.abs_diff(self.backtracks_used)
    }
}

pub fn mean<I: IntoIterator<Item = usize>>(it: I) -> f64 {
    let vec = it.into_iter().collect::<Vec<_>>();
    vec.iter().map(|v| *v as f64).sum::<f64>() / vec.len() as f64
}

pub fn max<I: IntoIterator<Item = usize>>(it: I) -> usize {
    it.into_iter().max().unwrap()
}

pub fn min<I: IntoIterator<Item = usize>>(it: I) -> usize {
    it.into_iter().min().unwrap()
}

/// Game statistics.
pub struct GlobalStats {
    pub wins: u16,
    pub draws: u16,
    pub losses: u16,
    pub game_stats: Vec<GameStat>,
}

impl GlobalStats {
    pub fn new(game_stats: Vec<GameStat>) -> Self {
        let mut g = Self { game_stats, wins: 0, draws: 0, losses: 0 };
        for s in &g.game_stats {
            match s.backtracks_used.cmp(&s.rated_difficulty) {
                Ordering::Less => g.wins += 1,
                Ordering::Equal => g.draws += 1,
                Ordering::Greater => g.losses += 1,
            }
        }
        g
    }

    pub fn total_t(&self) -> Duration {
        self.game_stats.iter().map(|v| v.duration).sum()
    }

    pub fn len(&self) -> usize {
        self.wins as usize + self.draws as usize + self.losses as usize
    }

    /// Average time per solve.
    pub fn avg_t(&self) -> Duration {
        self.total_t() / self.len() as u32
    }

    pub fn max_t(&self) -> (usize, &GameStat) {
        self.game_stats
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.duration.cmp(&b.1.duration))
            .unwrap()
    }

    pub fn min_t(&self) -> (usize, &GameStat) {
        self.game_stats
            .iter()
            .enumerate()
            .min_by(|a, b| a.1.duration.cmp(&b.1.duration))
            .unwrap()
    }

    pub fn avg_difficulty(&self) -> f64 {
        let t = self.game_stats.iter().map(|v| v.rated_difficulty as f64).sum::<f64>();
        t / self.len() as f64
    }

    pub fn avg_backtracks(&self) -> f64 {
        let t = self.game_stats.iter().map(|v| v.backtracks_used as f64).sum::<f64>();
        t / self.len() as f64
    }

    pub fn biggest_win(&self) -> Option<&GameStat> {
        self.game_stats
            .iter()
            .filter(|v| v.rated_difficulty > v.backtracks_used)
            .max_by(|a, b| a.diff().cmp(&b.diff()))
    }

    pub fn biggest_loss(&self) -> Option<&GameStat> {
        self.game_stats
            .iter()
            .filter(|v| v.rated_difficulty < v.backtracks_used)
            .max_by(|a, b| a.diff().cmp(&b.diff()))
    }
}
