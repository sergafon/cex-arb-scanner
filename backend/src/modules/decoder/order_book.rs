use crate::kernel::enums::decoder::ActionDecoder;
use anyhow::Result;

pub trait OrderBookDecoder: Send + Sync + 'static {
    fn decode(&self, bytes: &[u8]) -> Result<ActionDecoder>;
}
