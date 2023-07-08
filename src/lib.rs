pub mod enemies;
pub mod items;
pub mod level;
pub mod player;
pub mod world;
pub mod zone;

pub const I2C_SPEED: u32 = 115200; // 400
pub type I2cAdr = u8;

pub trait I2cCommunication<T> {
    /// this function will turn the struct into sendable data that can be sent over I2C
    fn encode(&self) -> anyhow::Result<Vec<u8>>;

    /// this function is the inverse func of encode. it takes the I2C data and converts it back into a rust struct
    fn decode_from(&self, data: &[u8]) -> anyhow::Result<T>;
}
