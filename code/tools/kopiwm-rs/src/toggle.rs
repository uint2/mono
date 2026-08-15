pub struct Toggle<T> {
    now: T,
    prev: Option<T>,
}

impl<T> Toggle<T> {
    pub const fn new(value: T) -> Self {
        Self { now: value, prev: None }
    }

    pub const fn get(&self) -> &T {
        &self.now
    }

    pub fn set(&mut self, mut value: T) {
        core::mem::swap(&mut self.now, &mut value);
        self.prev = Some(value);
    }

    pub fn revert(&mut self) {
        let Some(value) = self.prev.take() else { return };
        self.now = value;
    }
}
