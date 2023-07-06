use anyhow::{self, bail};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_hal::{i2c, peripherals, peripheral::Peripheral};
use log::*;
use distributed_esp_api::I2C_SPEED;

pub static I2C_ADDRESS: u8 = 0;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let i2c = void_setup()?;

    loop {
        if let Err(why) = void_loop(&i2c) {
            error!("a loop iteration failed because: {why}");
        }
    }
    // }
}

fn void_setup() -> anyhow::Result<i2c::I2cDriver> {
    info!("taking peripherals...");
    let peripherals = match peripherals::Peripherals::take() {
        Some(periph) => periph,
        None => bail!("Peripheral could not be taken"),
    };
    info!("peripherals acquired...");

    let i2c = i2c::I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio4,
        peripherals.pins.gpio5,
        &i2c::config::Config::new().baudrate(I2C_SPEED.into()),
    )?;  // raw I2C thing

    Ok(i2c)
}

fn void_loop(i2c: &i2c::I2cDriver) -> anyhow::Result<()> {
    // TODO: write loop stuff
    Ok(())
}