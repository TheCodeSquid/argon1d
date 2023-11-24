use std::fs;

use anyhow::Result;

const TEMP_PATH: &str = "/sys/class/thermal/thermal_zone0/temp";

pub fn temp() -> Result<u8> {
    let content = fs::read_to_string(TEMP_PATH)?;
    let temp: f32 = content.trim().parse()?;
    Ok((temp / 1000.0).round() as u8)
}
