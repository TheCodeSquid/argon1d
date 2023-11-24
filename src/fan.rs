use anyhow::Result;
use rppal::i2c::I2c;

pub fn set_speed(i2c: &I2c, speed: u8) -> Result<()> {
    i2c.smbus_send_byte(speed)?;
    info!("Set fan speed to {speed}");
    Ok(())
}
