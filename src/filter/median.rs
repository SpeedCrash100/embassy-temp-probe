use super::Filter;
use heapless::Vec;

/// Median filter with custom size
/// N should be a odd number
pub struct Median<const N: usize> {
    buffer: Vec<f32, N>,
}

impl<const N: usize> Default for Median<N> {
    fn default() -> Self {
        let mut buffer = Vec::new();
        buffer
            .resize(N, 0.0)
            .expect("resize bug: resizing to N elements with N-sized buffer failed");
        Self { buffer }
    }
}

impl<const N: usize> Filter for Median<N> {
    fn insert(&mut self, value: f32) {
        self.buffer.remove(0);
        self.buffer.push(value).expect(
            "heapless::Vec bug: after removing 1 element and inserting new one push must succeed",
        );
    }

    fn filtered(&self) -> f32 {
        let mut buffer: Vec<f32, N> = Vec::new();
        buffer.extend(self.buffer.iter().cloned());
        buffer.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

        buffer[buffer.len() / 2]
    }
}
