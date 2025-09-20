use crate::kernel::dto::order_book::OrderBookDto;
use anyhow::Result;
use std::sync::Arc;

pub trait OrderBookDecoder {
    fn decode(&self, bytes: &[u8]) -> Result<Arc<OrderBookDto>>;
}
