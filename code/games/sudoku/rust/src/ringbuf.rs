#[derive(Clone, Copy)]
pub struct RingBuf<const N: usize> {
    buf: [u8; N],
    /// Points to the first element.
    head: usize,
    /// Points to one past the last element.
    tail: usize,
}

impl<const N: usize> RingBuf<N> {
    pub const fn new() -> Self {
        Self { buf: [0; N], head: 0, tail: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    pub fn push_back(&mut self, value: u8) {
        *unsafe { self.buf.get_unchecked_mut(self.tail) } = value;
        self.tail = (self.tail + 1) % N;
        if self.tail == self.head {
            panic!("Ran out of space on the ring buffer.");
        }
    }

    /// Pop, but from the front.
    pub fn pop_front(&mut self) -> Option<u8> {
        if self.head == self.tail {
            return None;
        }
        let value = unsafe { *self.buf.get_unchecked(self.head) };
        self.head = (self.head + 1) % N;
        Some(value)
    }
}
