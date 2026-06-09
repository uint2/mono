use crate::game::{Checkable, Point, PourPoint, Quota, State};
use std::fmt;

/// An instance of a game is meant to be as lightweight as possible,
/// to be copied as cheaply as possible when backtracking.
#[derive(Clone)]
pub struct Instance<'a> {
    pub state: Vec<Vec<State>>,
    pub qrow: Vec<Quota>,
    pub qcol: Vec<Quota>,
    groups: &'a Vec<Vec<usize>>,
}

const JOIN_UP: &str = "┼│├┤┌┬┐";
const JOIN_RIGHT: &str = "┼├─┌┬└┴";

impl<'a> Instance<'a> {
    pub fn new(
        groups: &'a Vec<Vec<usize>>,
        row_sums: &Vec<i32>,
        col_sums: &Vec<i32>,
    ) -> Self {
        let size = groups.len();
        let state = (0..size)
            .map(|_| (0..size).map(|_| State::None).collect())
            .collect();
        Self {
            state,
            groups,
            qrow: Quota::vec(row_sums),
            qcol: Quota::vec(col_sums),
        }
    }

    pub fn size(&self) -> usize {
        self.groups.len()
    }

    /// Pours a fluid into a list of points, returning a list of
    /// affected points
    pub fn pour<'b>(
        &mut self,
        fluid: State,
        points: &'b Vec<Point>,
    ) -> Vec<&'b Point> {
        let mut affected = vec![];
        for p in points {
            if self.state[p].is_fluid() {
                continue;
            }
            self.state[p] = fluid;
            self.qrow[p.row].decrement(&fluid);
            self.qcol[p.col].decrement(&fluid);
            affected.push(p)
        }
        affected
    }

    /// Un-pours a fluid from a list of points. Meant to be used as a
    /// pair with `self.pour()`.
    pub fn unpour(&mut self, fluid: State, points: Vec<&Point>) {
        for p in points {
            self.state[p] = State::None;
            self.qrow[p.row].increment(&fluid);
            self.qcol[p.col].increment(&fluid);
        }
    }

    /// Tries to pour a certain fluid. If it's valid, the pour is unpourne,
    /// since there is no immediate conclusion.
    ///
    /// If the result is invalid, we know for sure that the first fluid
    /// can't be poured there, so we lock in the second fluid.
    ///
    /// Returns a true if changes were made
    pub fn try_pour(&mut self, pour_point: &PourPoint) -> bool {
        let flow = &pour_point.get_flow(pour_point.fluid);
        let delta = self.pour(pour_point.fluid, flow);

        if self.is_valid() {
            self.unpour(pour_point.fluid, delta);
            false
        } else {
            self.unpour(pour_point.fluid, delta);
            let next = pour_point.fluid.next();
            !self.pour(next, &pour_point.get_flow(next)).is_empty()
        }
    }

    /// Makes all forcing moves based on the current state.
    /// WARNING: may lead to an invalid state. This happens when pouring
    /// both air and water into a particular point leads to an invalid state
    pub fn fast_forward(&mut self, pour_points: &Vec<PourPoint>) {
        loop {
            let mut changed = false;

            for pp in pour_points {
                if self.state[&pp.point].is_fluid() {
                    continue;
                }
                changed |= self.try_pour(pp);
                if !self.is_valid() {
                    return;
                }
            }

            if !changed {
                break;
            }
        }
    }
}

impl<'a> Checkable for Instance<'a> {
    fn is_valid(&self) -> bool {
        self.qcol.is_valid() && self.qrow.is_valid()
    }

    fn is_solved(&self) -> bool {
        self.qcol.is_solved() && self.qrow.is_solved()
    }
}

/// Debugging methods
impl<'a> Instance<'a> {
    /// Get the surrounding groups of a border point.
    /// Bounds belong to group 0.
    /// [<upper-left>, <upper-right>, <lower-left>, <lower-right>]
    fn surrounding_groups(&self, r: usize, c: usize) -> [usize; 4] {
        let (n, g) = (self.size(), &self.groups);
        //           ↖︎  ↗︎  ↙︎  ↘︎
        let mut t = [0, 0, 0, 0];
        if r > 0 {
            if c > 0 {
                t[0] = g[r - 1][c - 1]; // ↖︎
            }
            if c < n {
                t[1] = g[r - 1][c]; // ↗︎
            }
        }
        if r < n {
            if c > 0 {
                t[2] = g[r][c - 1]; // ↙︎
            }
            if c < n {
                t[3] = g[r][c]; // ↘︎
            }
        }
        t
    }

    fn border(&self, r: usize, c: usize) -> char {
        let [ul, ur, ll, lr] = self.surrounding_groups(r, c);
        match (u8::from(ul == ur) << 3)
            + (u8::from(ll == lr) << 2)
            + (u8::from(ul == ll) << 1)
            + u8::from(ur == lr)
        {
            0 => '┼',
            1 => '┤',
            2 => '├',
            3 => '│',
            4 => '┴',
            5 => '┘',
            6 => '└',
            8 => '┬',
            9 => '┐',
            10 => '┌',
            12 => '─',
            _ => ' ',
        }
    }

    fn join_state_line(
        &self,
        borders: &Vec<char>,
        states: &Vec<State>,
    ) -> String {
        let mut result = String::with_capacity(self.size() * 4 + 1);
        result.push('│');
        for i in 0..self.size() {
            result.push(' ');
            result.push(states[i].as_char());
            result.push(' ');
            result.push(match JOIN_UP.contains(borders[i + 1]) {
                true => '│',
                false => ' ',
            });
        }
        result
    }

    fn join_border_line(&self, borders: &Vec<char>) -> String {
        let mut result = String::with_capacity(self.size() * 4 + 1);
        for i in 0..self.size() {
            result.push(borders[i]);
            result.push_str(match JOIN_RIGHT.contains(borders[i]) {
                true => "───",
                false => "   ",
            })
        }
        result.push(borders[borders.len() - 1]);
        result
    }
}

impl<'a> fmt::Debug for Instance<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut stdout = String::new();
        let mut print = |v: String| stdout.push_str(&(v + "\n"));
        let i = || (0..self.size() + 1);
        let borders: Vec<Vec<char>> =
            i().map(|r| i().map(|c| self.border(r, c)).collect()).collect();

        print(format!("         {:?}", self.qcol));

        for i in 0..self.size() + 1 {
            print(format!("         {}", self.join_border_line(&borders[i])));

            if i < self.size() {
                print(format!(
                    "{:>8} {}",
                    format!("{:?}", self.qrow[i]),
                    self.join_state_line(&borders[i], &self.state[i])
                ));
            }
        }

        stdout.pop();
        write!(f, "Instance {{\n{stdout}\n}}")
    }
}
