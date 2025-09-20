use crate::kernel::enums::exchange::Symbol;
use crate::modules::bus::order_book::OrderBookBus;
use crate::modules::decoder::okx::order_book::OkxOrderBookDecoder;
use crate::modules::stream::base_order_book::BaseOrderBookStream;
use crate::modules::stream::order_book::OrderBookStream;
use std::sync::Arc;
use strum::IntoEnumIterator;

pub struct OkxOrderBookStream;

impl OrderBookStream for OkxOrderBookStream {
    type Decoder = OkxOrderBookDecoder;

    fn new(order_book_bus: Arc<OrderBookBus>) -> BaseOrderBookStream<Self::Decoder> {
        let url = "wss://ws.okx.com:8443/ws/v5/public".to_string();
        let mut args = vec![];

        for sym in Symbol::iter() {
            args.push(serde_json::json!({
                "channel": "bbo-tbt",
                "instId": format!("{}-USDT", sym)
            }));
        }

        let subscribe_message = serde_json::json!({
            "op": "subscribe",
            "args": args
        })
        .to_string();

        BaseOrderBookStream { url, subscribe_message, order_book_bus, decoder: OkxOrderBookDecoder }
    }
}
