mod median;
mod movingavg;

pub use median::Median;
pub use movingavg::MovingAvg;

pub trait Filter {
    fn insert(&mut self, value: f32);
    fn filtered(&self) -> f32;
}
