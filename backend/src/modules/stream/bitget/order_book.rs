use crate::kernel::enums::exchange::Symbol;
use crate::modules::bus::order_book::OrderBookBus;
use crate::modules::decoder::bitget::order_book::BitgetOrderBookDecoder;
use crate::modules::stream::base_order_book::BaseOrderBookStream;
use crate::modules::stream::order_book::OrderBookStream;
use std::sync::Arc;
use strum::IntoEnumIterator;

pub struct BitgetOrderBookStream;

impl OrderBookStream for BitgetOrderBookStream {
    type Decoder = BitgetOrderBookDecoder;

    fn new(order_book_bus: Arc<OrderBookBus>) -> BaseOrderBookStream<Self::Decoder> {
        let url = "wss://ws.bitget.com/v2/ws/public".to_string();
        let mut args = vec![];

        for sym in Symbol::iter() {
            args.push(serde_json::json!({
                "instType": "SPOT",
                "channel": "books1",
                "instId": format!("{sym}USDT")
            }))
        }

        let subscribe_message = serde_json::json!({
            "op": "subscribe",
            "args": args,
        })
        .to_string();

        BaseOrderBookStream {
            url,
            subscribe_message,
            order_book_bus,
            decoder: BitgetOrderBookDecoder,
        }
    }
}
