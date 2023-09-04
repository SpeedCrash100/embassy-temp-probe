use embassy_executor::Spawner;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};

const FLAG_BITS: u8 = 0b11_00_00_00;

pub static P9813_SET_COLOR: Channel<CriticalSectionRawMutex, (u8, u8, u8), 1> = Channel::new();

static P9813_SEND_DATA: Channel<CriticalSectionRawMutex, u32, 3> = Channel::new();

#[embassy_executor::task]
pub async fn handle_p9813(spawner: Spawner, scl: AnyPin, mosi: AnyPin) {
    spawner.spawn(p9813_send_byte(scl, mosi)).unwrap();

    P9813_SEND_DATA.send(0).await;

    loop {
        let (r, g, b) = P9813_SET_COLOR.recv().await;

        p9813_send_start().await;

        let b_bit = !(b >> 6) & 0b11;
        let g_bit = !(g >> 6) & 0b11;
        let r_bit = !(r >> 6) & 0b11;

        let prefix = FLAG_BITS | (b_bit << 4) | (g_bit << 2) | r_bit;

        let bytes = [prefix, b, g, r];

        let val = u32::from_be_bytes(bytes);

        P9813_SEND_DATA.send(val).await;

        p9813_send_end().await;
    }
}

pub async fn p9813_send_start() {
    P9813_SEND_DATA.send(0).await;
}

pub async fn p9813_send_end() {
    P9813_SEND_DATA.send(0).await;
}

#[embassy_executor::task]
pub async fn p9813_send_byte(scl: AnyPin, mosi: AnyPin) {
    let mut scl = Output::new(scl, Level::High, Speed::VeryHigh);
    let mut mosi = Output::new(mosi, Level::Low, Speed::VeryHigh);

    loop {
        let mut data = P9813_SEND_DATA.recv().await;

        for _ in 0..32 {
            let bit = (0b1 << 31) & data;
            if bit != 0 {
                mosi.set_high();
            } else {
                mosi.set_low();
            }
            data <<= 1;

            // Clock
            scl.set_low();
            Timer::after(Duration::from_micros(1)).await;
            scl.set_high();
            Timer::after(Duration::from_micros(1)).await;
        }
    }
}
