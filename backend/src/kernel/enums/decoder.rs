use crate::kernel::dto::order_book::OrderBookDto;
use std::sync::Arc;

#[derive(Debug)]
pub enum ActionDecoder {
    Pong(u64),
    Publish(Arc<OrderBookDto>),
    Skip,
}
