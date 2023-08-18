use defmt::error;
use embassy_stm32::bind_interrupts;
use embassy_stm32::dma::NoDma;
use embassy_stm32::i2c::{I2c, InterruptHandler};
use embassy_stm32::peripherals;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;

// I2C bus defs
type I2CBus = I2c<'static, peripherals::I2C1, NoDma, NoDma>;
bind_interrupts!(pub struct I2CIrqs {
    I2C1_EV => InterruptHandler<peripherals::I2C1>;
});

const MAX_I2C_COMMAND_SIZE: usize = 64;
type I2cDataBuffer = heapless::Vec<u8, MAX_I2C_COMMAND_SIZE>;

#[warn(dead_code)]
pub enum I2cCommand {
    Write {
        address: u8,
        data: I2cDataBuffer,
    },
    Read {
        address: u8,
        read_size: usize,
    },
    WriteRead {
        address: u8,
        data: I2cDataBuffer,
        read_size: usize,
    },
}

impl I2cCommand {}

pub enum I2cResult {
    Writen,
    Read(I2cDataBuffer),
    Error,
}

pub static I2C_COMMAND: Channel<CriticalSectionRawMutex, I2cCommand, 1> = Channel::new();
pub static I2C_RESULT: Channel<CriticalSectionRawMutex, I2cResult, 1> = Channel::new();

#[embassy_executor::task]
pub async fn i2c_handle_commands(mut bus: I2CBus) {
    loop {
        let command = I2C_COMMAND.recv().await;
        match command {
            I2cCommand::Write { address, data } => {
                handle_write(&mut bus, address, data).await;
            }
            I2cCommand::Read { address, read_size } => {
                handle_read(&mut bus, address, read_size).await;
            }
            I2cCommand::WriteRead {
                address,
                data,
                read_size,
            } => handle_write_read(&mut bus, address, data, read_size).await,
        }
    }
}

async fn handle_write(bus: &mut I2CBus, address: u8, data: I2cDataBuffer) {
    match bus.blocking_write(address, data.as_slice()) {
        Ok(()) => I2C_RESULT.send(I2cResult::Writen).await,
        Err(e) => {
            error!("Failed to write into I2C bus: {}", e);
            I2C_RESULT.send(I2cResult::Error).await
        }
    }
}

async fn handle_read(bus: &mut I2CBus, address: u8, read_size: usize) {
    let mut data = heapless::Vec::new();

    if data.resize(read_size, 0).is_err() {
        I2C_RESULT.send(I2cResult::Error).await;
        return;
    }

    match bus.blocking_read(address, data.as_mut()) {
        Ok(()) => I2C_RESULT.send(I2cResult::Read(data)).await,
        Err(e) => {
            error!("Failed to read drom I2C bus: {}", e);
            I2C_RESULT.send(I2cResult::Error).await
        }
    }
}

async fn handle_write_read(bus: &mut I2CBus, address: u8, data: I2cDataBuffer, read_size: usize) {
    let mut data_to_recv = heapless::Vec::new();
    if data_to_recv.resize(read_size, 0).is_err() {
        I2C_RESULT.send(I2cResult::Error).await;
        return;
    }

    match bus.blocking_write_read(address, data.as_slice(), data_to_recv.as_mut()) {
        Ok(()) => I2C_RESULT.send(I2cResult::Read(data_to_recv)).await,
        Err(e) => {
            error!("Failed to write into I2C bus and read: {}", e);
            I2C_RESULT.send(I2cResult::Error).await
        }
    }
}
