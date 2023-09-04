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
mod p9813;

use {defmt_rtt as _, panic_probe as _}; // global logger

#[embassy_executor::task]
async fn temp_printer() {
    loop {
        let temp = lm75b::LM75B_TEMPERATURE.recv().await;

        let temp_cold = 20.0;
        let temp_hot = 30.0;

        let mut r = (temp - temp_cold) / (temp_hot - temp_cold) * 255.0;
        if r < 0.0 {
            r = 0.0;
        }
        if r > 255.0 {
            r = 255.0;
        }

        let r = r as i32 / 2;

        info!("Temperature: {:?}, r: {:?}", temp, r);
        p9813::P9813_SET_COLOR.send((r as u8, 30, 255)).await;
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

    spawner
        .spawn(p9813::handle_p9813(spawner, p.PB10.into(), p.PB4.into()))
        .unwrap();

    spawner.spawn(temp_printer()).unwrap();
}
