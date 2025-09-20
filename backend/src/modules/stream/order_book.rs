use crate::modules::bus::order_book::OrderBookBus;
use crate::modules::decoder::order_book::OrderBookDecoder;
use crate::modules::stream::base_order_book::BaseOrderBookStream;
use async_trait::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait OrderBookStream {
    type Decoder: OrderBookDecoder;

    fn new(broadcast: Arc<OrderBookBus>) -> BaseOrderBookStream<Self::Decoder>;
}
