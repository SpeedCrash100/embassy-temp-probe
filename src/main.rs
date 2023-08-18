#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::info;
use embassy_executor::Spawner;
use embassy_stm32::dma::NoDma;
use embassy_stm32::i2c::{Config, I2c};
use embassy_stm32::time::Hertz;
use embassy_time::Duration;

mod filter;
mod i2c;
mod lm75b;

use {defmt_rtt as _, panic_probe as _}; // global logger

#[embassy_executor::task]
async fn temp_printer() {
    loop {
        let temp = lm75b::LM75B_TEMPERATURE.recv().await;
        info!("Temperature: {:?}", temp);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let i2c = I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        i2c::I2CIrqs,
        NoDma,
        NoDma,
        Hertz(100_000),
        Config::default(),
    );

    spawner.spawn(i2c::i2c_handle_commands(i2c)).unwrap();
    spawner
        .spawn(lm75b::lm75b_grab_temperature(Duration::from_secs(1)))
        .unwrap();

    spawner.spawn(temp_printer()).unwrap();
}
