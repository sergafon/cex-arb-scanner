use crate::kernel::enums::exchange::Symbol;
use crate::modules::bus::order_book::OrderBookBus;
use crate::modules::decoder::binance::order_book::BinanceOrderBookDecoder;
use crate::modules::stream::base_order_book::BaseOrderBookStream;
use crate::modules::stream::order_book::OrderBookStream;
use std::sync::Arc;
use strum::IntoEnumIterator;

pub struct BinanceOrderBookStream;

impl OrderBookStream for BinanceOrderBookStream {
    type Decoder = BinanceOrderBookDecoder;

    fn new(order_book_bus: Arc<OrderBookBus>) -> BaseOrderBookStream<Self::Decoder> {
        let url = "wss://stream.binance.com:9443/ws".to_string();

        let mut params = vec![];

        for sym in Symbol::iter() {
            let s = format!("{}usdt@bookTicker", sym.to_string().to_lowercase());
            let s = serde_json::Value::String(s);
            params.push(s);
        }

        let subscribe_message = serde_json::json!({
            "method": "SUBSCRIBE",
            "params": params,
            "id": 1
        })
        .to_string();

        BaseOrderBookStream {
            url,
            subscribe_message,
            order_book_bus,
            decoder: BinanceOrderBookDecoder,
        }
    }
}
