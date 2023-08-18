use super::Filter;
use heapless::{Deque, Vec};

/// Median filter with custom size
/// N should be a odd number
pub struct Median<const N: usize> {
    buffer: Deque<f32, N>,
}

impl<const N: usize> Default for Median<N> {
    fn default() -> Self {
        let mut buffer = Deque::new();
        while !buffer.is_full() {
            buffer.push_back(0.0).ok();
        }
        Self { buffer }
    }
}

impl<const N: usize> Filter for Median<N> {
    fn insert(&mut self, value: f32) {
        self.buffer.pop_front();
        self.buffer.push_back(value).ok();
    }

    fn filtered(&self) -> f32 {
        let mut buffer: Vec<f32, N> = Vec::new();
        buffer.extend(self.buffer.iter().cloned());
        buffer.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        buffer[buffer.len() / 2]
    }
}
