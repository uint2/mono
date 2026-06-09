use crate::game::{Checkable, Instance, Point, PourPoint};

pub struct Game {
    groups: Vec<Vec<usize>>,
    row_sums: Vec<i32>,
    col_sums: Vec<i32>,
    pour_points: Vec<PourPoint>,
}

impl Game {
    pub fn new(
        row_sums: Vec<i32>,
        col_sums: Vec<i32>,
        groups: Vec<Vec<usize>>,
    ) -> Self {
        let mut points: Vec<Point> = vec![];

        let size = groups.len();

        // load the pouring points
        for r in 0..size {
            for c in 0..size {
                let g = groups[r][c];
                if !points.iter().any(|p| p.row == r && groups[p] == g) {
                    points.push(Point::new(r, c));
                }
            }
        }

        let mut pour_points: Vec<PourPoint> =
            points.into_iter().map(|p| PourPoint::new(p, &groups)).collect();

        // sorted with max damage at the start of the array
        pour_points.sort_by(|a, b| b.max_damage().cmp(&a.max_damage()));

        Self { row_sums, col_sums, groups, pour_points }
    }

    pub fn instance(&self) -> Instance {
        Instance::new(&self.groups, &self.row_sums, &self.col_sums)
    }

    fn backtrack<'a>(&'a self, prev: &Instance<'a>) -> Option<Instance> {
        if !prev.is_valid() {
            return None;
        }

        let mut inst = prev.clone();

        inst.fast_forward(&self.pour_points);

        if !inst.is_valid() {
            return None;
        }

        if inst.is_solved() {
            return Some(inst);
        }

        for pour in &self.pour_points {
            if inst.state[&pour.point].is_fluid() {
                continue;
            }
            let delta = inst.pour(pour.fluid, &pour.get_flow(pour.fluid));

            if let Some(result) = self.backtrack(&inst) {
                return Some(result);
            } else {
                inst.unpour(pour.fluid, delta);
                inst.pour(pour.fluid.next(), &pour.get_flow(pour.fluid.next()));
                if !inst.is_valid() {
                    return None;
                }
            }
        }

        None
    }

    pub fn solve(&self) -> Option<Instance> {
        let inst = self.instance();
        let saved = (inst.qrow.clone(), inst.qcol.clone());
        let mut inst = self.backtrack(&inst)?;
        inst.is_solved().then(|| {
            inst.qrow = saved.0;
            inst.qcol = saved.1;
            inst
        })
    }
}
