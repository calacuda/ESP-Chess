use std::sync::Arc;

pub const I2C_SPEED: usize = 800;  // 400

pub trait I2cCommunication<T> {
    /// this function will turn the struct into sendable data that can be sent over I2C
    fn encode(&self) -> anyhow::Result<Arc<[u8]>>;
    
    /// this function is the inverse func of encode. it takes the I2C data and converts it back into a rust struct 
    fn decode_from(&self) -> anyhow::Result<T>;
}