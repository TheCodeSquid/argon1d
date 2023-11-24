mod config;

use std::{thread, time::Duration};

use anyhow::{Context, Result};
use rppal::i2c::I2c;

const ADDR_FAN: u16 = 0x1a;

fn main() -> Result<()> {
    let mut bus = I2c::new().with_context(|| "I2C disabled or permission denied")?;
    bus.set_slave_address(ADDR_FAN)?;

    bus.smbus_send_byte(100)?;
    thread::sleep(Duration::from_secs(5));
    bus.smbus_send_byte(0)?;

    println!("Hello, world!");
    Ok(())
}
