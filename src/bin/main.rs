use anyhow::{self, bail};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::AnyIOPin;
use esp_idf_hal::i2c::*;
use esp_idf_hal::prelude::*;
use esp_idf_hal::{i2c::I2cDriver, peripherals};
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use esp_idf_sys::{esp, esp_vfs_dev_uart_use_driver, uart_driver_install};
use log::*;
use rogue_xork::biome;
use rogue_xork::level::Biome;
use rogue_xork::{game::Game, I2cAdr, I2C_SPEED};
use std::io::stdin;
use std::ptr::null_mut;

// pub static I2C_ADDRESS: u8 = 0;

fn main() -> anyhow::Result<()> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();
    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    info!("taking peripherals...");
    let peripherals = match peripherals::Peripherals::take() {
        Some(periph) => periph,
        None => {
            error!("peripherals not taken");
            bail!("Peripheral could not be taken");
        }
    };
    info!("peripherals acquired...");
    let pins = peripherals.pins;

    info!("installing UART driver...");
    unsafe {
        esp!(uart_driver_install(0, 512, 512, 10, null_mut(), 0)).unwrap();
        esp_vfs_dev_uart_use_driver(0);
    }
    info!("UART driver installed.");

    info!("Biome: {}", biome!());

    let i2c_conf = I2cConfig::new().baudrate(I2C_SPEED.kHz().into());
    let mut i2c = I2cDriver::new(
        peripherals.i2c0,
        Into::<AnyIOPin>::into(pins.gpio21),
        Into::<AnyIOPin>::into(pins.gpio22),
        &i2c_conf,
    )?;

    let mut game = void_setup()?;

    loop {
        if let Err(why) = void_loop(&mut i2c, &mut game) {
            error!("a loop iteration failed because: {why}");
        }

        FreeRtos::delay_us(10)
    }
}

fn void_setup() -> anyhow::Result<Game> {
    let mut game = Game::new();
    game.world.generate()?;
    info!("world created");

    Ok(game)
}

fn void_loop(i2c: &mut I2cDriver, game: &mut Game) -> anyhow::Result<()> {
    // TODO: get player input
    print!("~ >>> ");
    let mut buffer = String::new();

    match stdin().read_line(&mut buffer) {
        Ok(n_bytes) => info!("got {n_bytes} bytes from stdin. message was \"{buffer}\""),
        Err(e) => bail!("reading from UART in failed with error: \"{e}\""),
    }

    // TODO: do player input

    Ok(())
}

/// scans all i2c addresses for worker nodes.
fn i2c_scan(i2c: &mut I2cDriver) -> anyhow::Result<Vec<I2cAdr>> {
    Ok((0..127)
        .into_iter()
        .filter(|adr| is_host(*adr, i2c))
        .collect())
}

fn is_host(adr: I2cAdr, i2c: &mut I2cDriver) -> bool {
    i2c.write(adr, &[3], 10).is_ok()
}
