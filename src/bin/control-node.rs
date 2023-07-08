use anyhow::{self, bail};
use distributed_esp_api::{world::World, I2cAdr, I2C_SPEED};
use esp_idf_hal::{i2c, peripherals, units::Hertz};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use log::*;
use shared_bus::{BusManagerSimple, I2cProxy};

pub static I2C_ADDRESS: u8 = 0;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("taking peripherals...");
    let peripherals = match peripherals::Peripherals::take() {
        Some(periph) => periph,
        None => bail!("Peripheral could not be taken"),
    };
    info!("peripherals acquired...");

    let i2c_driver = i2c::I2cDriver::new(
        peripherals.i2c0,
        peripherals.pins.gpio4,
        peripherals.pins.gpio5,
        &i2c::config::Config::new().baudrate(Hertz(I2C_SPEED)),
    )?;

    let bus = BusManagerSimple::new(i2c_driver);
    let mut i2c = bus.acquire_i2c();

    let world: World = void_setup(&i2c)?;

    loop {
        if let Err(why) = void_loop(&i2c) {
            error!("a loop iteration failed because: {why}");
        }
    }
}

fn void_setup(i2c: &I2cProxy) -> anyhow::Result<()> {
    // let workers: Vec<I2cAdr> = Vec::new();

    // Ok(World::new(&workers))
    Ok(())
}

fn void_loop(i2c: &I2cProxy) -> anyhow::Result<()> {
    // TODO: write loop stuff

    Ok(())
}

