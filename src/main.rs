#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_time::{Duration, Timer};

use {defmt_rtt as _, panic_probe as _}; // global logger

#[embassy_executor::task]
async fn blinker(pin: AnyPin, duration: Duration) {
    let mut led = Output::new(pin, Level::Low, Speed::Low);
    loop {
        Timer::after(duration).await;
        led.toggle();
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());
    spawner
        .spawn(blinker(p.PA5.into(), Duration::from_millis(100)))
        .unwrap();
}
