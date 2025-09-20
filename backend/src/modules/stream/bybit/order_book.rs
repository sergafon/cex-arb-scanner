use crate::kernel::enums::exchange::Symbol;
use crate::modules::bus::order_book::OrderBookBus;
use crate::modules::decoder::bybit::order_book::BybitOrderBookDecoder;
use crate::modules::stream::base_order_book::BaseOrderBookStream;
use crate::modules::stream::order_book::OrderBookStream;
use serde_json::json;
use std::sync::Arc;
use strum::IntoEnumIterator;

pub struct BybitOrderBookStream;

impl OrderBookStream for BybitOrderBookStream {
    type Decoder = BybitOrderBookDecoder;

    fn new(order_book_bus: Arc<OrderBookBus>) -> BaseOrderBookStream<Self::Decoder> {
        let url = "wss://stream.bybit.com/v5/public/spot".to_string();

        let mut batches = Vec::new();
        let mut current = Vec::with_capacity(10);

        for sym in Symbol::iter() {
            current.push(format!("orderbook.1.{sym}USDT"));

            if current.len() == 10 {
                batches.push(json!({
                    "op": "subscribe",
                    "args": current
                }));
                current = Vec::with_capacity(10);
            }
        }

        if !current.is_empty() {
            batches.push(json!({
                "op": "subscribe",
                "args": current
            }));
        }

        let subscribe_message = serde_json::to_string(&batches).unwrap();

        BaseOrderBookStream {
            url,
            subscribe_message,
            order_book_bus,
            decoder: BybitOrderBookDecoder,
        }
    }
}
