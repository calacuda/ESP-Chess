use anyhow::bail;
use enemies::Enemy;
use esp_idf_hal::{delay::BLOCK, i2c::I2cDriver};
use items::Item;
use player::Player;

pub mod battle_logic;
pub mod enemies;
pub mod game;
pub mod items;
pub mod level;
pub mod player;
pub mod world;
pub mod zone;

pub static I2C_SPEED: u32 = 1000; // 400
pub type I2cAdr = u8;
pub const LEVEL_MIN: u8 = 5;
pub const LEVEL_MAX: u8 = 15;
pub const ZONE_DIM_MIN: u8 = 7;
pub const ZONE_DIM_MAX: u8 = 10;
pub const PACKET_SIZE: usize = u8::MAX as usize;

pub trait I2cCom<T> {
    // : erased_serde::Serialize {
    /// this function will turn the struct into sendable data that can be sent over I2C
    fn encode(&self) -> anyhow::Result<Vec<u8>>;

    /// this function is the inverse func of encode. it takes the I2C data and converts it back into a rust struct
    fn decode_from(&self, data: &[u8]) -> anyhow::Result<T>;
}

/// helper function for msg_len
fn _get_len(k: u16, target: usize) -> anyhow::Result<u16> {
    if (k as usize) < target {
        _get_len(k + (k / 2), target)
    } else if (k as usize) > target {
        _get_len(k / 2, target)
    } else {
        Ok(k)
    }
}

/// just in case
fn _msg_len(mesg: &[u8]) -> anyhow::Result<u16> {
    if (u16::max as usize) < mesg.len() {
        bail!("message too long. its length was greater then could be stored in a u16")
    } else {
        _get_len(u16::MAX, mesg.len())
    }
}

fn i2c_write(i2c: &mut I2cDriver, addr: I2cAdr, mesage: &[u8]) -> anyhow::Result<()> {
    let mut mesg = Vec::from(mesage);
    let mut n_packets = mesg.len() / PACKET_SIZE;

    if mesg.len() % PACKET_SIZE != 0 {
        n_packets += 1;
        while mesg.len() < n_packets * PACKET_SIZE {
            mesg.push(0)
        }
    }

    for packet_i in 0..n_packets - 1 {
        i2c.write(
            addr,
            &mesg[(PACKET_SIZE * packet_i)..(PACKET_SIZE * (packet_i + 1))],
            BLOCK,
        )?;
    }

    Ok(())
}

fn i2c_read(i2c: &mut I2cDriver, addr: I2cAdr) -> anyhow::Result<Vec<u8>> {
    let mut res = Vec::with_capacity(u16::MAX as usize);
    let mut cr_seen = false; // carage return seen

    while !cr_seen {
        let mut buf: [u8; PACKET_SIZE as usize] = [0; PACKET_SIZE as usize];

        i2c.read(addr, &mut buf, BLOCK)?;
        cr_seen = buf.contains(&"\n".as_bytes()[0]);

        res.append(&mut Vec::from(buf));
    }

    Ok(res)
}

fn send_cmd(i2c: &mut I2cDriver, addr: I2cAdr, byte_code: u8, mesg: &[u8]) -> anyhow::Result<()> {
    i2c.write(addr, &[byte_code], BLOCK)?;
    i2c_write(i2c, addr, mesg)
}

fn send_cmd_read(
    i2c: &mut I2cDriver,
    addr: I2cAdr,
    byte_code: u8,
    mesg: &[u8],
) -> anyhow::Result<Vec<u8>> {
    send_cmd(i2c, addr, byte_code, mesg)?;

    let mut buf: [u8; 1] = [0];
    i2c.read(addr, &mut buf, BLOCK)?;

    if buf[0] == 1 {
        Ok(Vec::new())
    } else {
        i2c_read(i2c, addr)
    }
}
