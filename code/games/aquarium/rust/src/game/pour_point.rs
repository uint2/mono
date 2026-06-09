use crate::game::{Point, State};
use std::fmt;

pub struct PourPoint {
    pub point: Point,
    pub fluid: State,
    water_flow: Vec<Point>,
    air_flow: Vec<Point>,
}

impl fmt::Debug for PourPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{:?}", self.point)
    }
}

impl PourPoint {
    pub fn new(point: Point, groups: &Vec<Vec<usize>>) -> Self {
        let size = groups.len();
        let group = groups[&point];
        let (mut water_flow, mut air_flow) =
            (vec![point.clone()], vec![point.clone()]);

        for r in 0..size {
            for c in 0..size {
                // skip different groups
                if groups[r][c] != group {
                    continue;
                }
                // skip the starting point
                if point.row == r && point.col == c {
                    continue;
                }
                if r >= point.row {
                    water_flow.push(Point::new(r, c));
                }
                if r <= point.row {
                    air_flow.push(Point::new(r, c));
                }
            }
        }

        let fluid = match water_flow.len() > air_flow.len() {
            true => State::Water,
            false => State::Air,
        };

        Self { point, water_flow, air_flow, fluid }
    }

    pub fn max_damage(&self) -> usize {
        self.water_flow.len().max(self.air_flow.len())
    }

    pub fn get_flow(&self, state: State) -> &Vec<Point> {
        match state {
            State::Water => &self.water_flow,
            State::Air => &self.air_flow,
            _ => panic!("Use either water or air only"),
        }
    }
}
