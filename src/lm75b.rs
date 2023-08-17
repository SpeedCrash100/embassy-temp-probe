use defmt::error;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::{Duration, Timer};

use crate::i2c::{I2cCommand, I2cResult, I2C_COMMAND, I2C_RESULT};

const CHANNEL_SIZE: usize = 1;
pub static LM75B_TEMPERATURE: Channel<CriticalSectionRawMutex, f32, CHANNEL_SIZE> = Channel::new();

const LM75B_ADDRESS: u8 = 0x48;
const TEMPERATURE_REGISTER: u8 = 0x00;

fn convert_raw_to_temperature(msb: u8, lsb: u8) -> f32 {
    let mask: u16 = 0b1111_1111_1110_0000;
    let msb = f32::from(msb as i8);
    let decimal = f32::from((lsb & mask as u8) >> 5) * 0.125;
    msb + decimal
}

#[embassy_executor::task]
pub async fn lm75b_grab_temperature(grab_interval: Duration) {
    loop {
        let mut data = heapless::Vec::new();
        data.push(TEMPERATURE_REGISTER).ok(); // Ok -> buffer always have place for 1 byte

        let i2c_cmd = I2cCommand::WriteRead {
            address: LM75B_ADDRESS,
            data,
            read_size: 2,
        };

        I2C_COMMAND.send(i2c_cmd).await;

        let result = I2C_RESULT.recv().await;

        match result {
            I2cResult::Read(recv) => {
                let temperature = convert_raw_to_temperature(recv[0], recv[1]);
                LM75B_TEMPERATURE.send(temperature).await;
            }
            _ => {
                error!("Invalid result recieved");
            }
        }

        Timer::after(grab_interval).await;
    }
}
