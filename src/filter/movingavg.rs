use super::Filter;
use heapless::Deque;

pub struct MovingAvg<const N: usize> {
    buffer: Deque<f32, N>,
    avg: f32,
}

impl<const N: usize> Default for MovingAvg<N> {
    fn default() -> Self {
        Self {
            buffer: Deque::new(),
            avg: 0.0,
        }
    }
}

impl<const N: usize> Filter for MovingAvg<N> {
    fn insert(&mut self, value: f32) {
        if self.buffer.is_full() {
            let to_remove = self
                .buffer
                .pop_front()
                .expect("Deque bug: pop from full buffer return None");
            self.avg -= to_remove;
        }

        self.buffer
            .push_back(value)
            .expect("Deque bug: cannot push into not full buffer");
        self.avg += value;
    }

    fn filtered(&self) -> f32 {
        self.avg / (self.buffer.len() as f32)
    }
}
