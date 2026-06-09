use crate::game::{Checkable, State};
use std::fmt;

#[derive(Clone, PartialEq, Eq)]
pub struct Quota {
    pub water: i32,
    pub air: i32,
}

impl<'a> fmt::Debug for Quota {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.water, self.air)
    }
}

impl Quota {
    pub fn vec(water_limit: &Vec<i32>) -> Vec<Self> {
        let size = water_limit.len();
        water_limit.iter().map(|v| Quota::new(*v, size)).collect()
    }

    pub fn new(water_limit: i32, size: usize) -> Self {
        Self { water: water_limit, air: size as i32 - water_limit }
    }

    fn state_mut(&mut self, fluid: &State) -> &mut i32 {
        match fluid {
            State::Air => &mut self.air,
            State::Water => &mut self.water,
            _ => panic!("Use either water or air."),
        }
    }

    pub fn increment(&mut self, fluid: &State) {
        *self.state_mut(fluid) += 1;
    }

    pub fn decrement(&mut self, fluid: &State) {
        *self.state_mut(fluid) -= 1;
    }
}

impl Checkable for Quota {
    fn is_solved(&self) -> bool {
        self.water == 0 && self.air == 0
    }

    fn is_valid(&self) -> bool {
        self.water >= 0 && self.air >= 0
    }
}

impl Checkable for Vec<Quota> {
    fn is_solved(&self) -> bool {
        self.iter().all(|v| v.is_solved())
    }

    fn is_valid(&self) -> bool {
        self.iter().all(|v| v.is_valid())
    }
}
