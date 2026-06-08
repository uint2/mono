use winit::dpi::PhysicalPosition;

pub trait PositionTrait {
    fn distance(&self, rhs: &Self) -> f32;
}

impl PositionTrait for PhysicalPosition<f32> {
    fn distance(&self, rhs: &Self) -> f32 {
        let a = self.x - rhs.x;
        let b = self.y - rhs.y;
        (a * a + b * b).sqrt()
    }
}
