use crate::kernel::enums::exchange::Symbol;
use crate::kernel::utils::time::get_now_timestamp;
use crate::modules::bus::order_book::OrderBookBus;
use crate::modules::decoder::gate::order_book::GateOrderBookDecoder;
use crate::modules::stream::base_order_book::BaseOrderBookStream;
use crate::modules::stream::order_book::OrderBookStream;
use std::sync::Arc;
use strum::IntoEnumIterator;

pub struct GateOrderBookStream;

impl OrderBookStream for GateOrderBookStream {
    type Decoder = GateOrderBookDecoder;

    fn new(order_book_bus: Arc<OrderBookBus>) -> BaseOrderBookStream<Self::Decoder> {
        let url = "wss://api.gateio.ws/ws/v4/".to_string();
        let mut args = vec![];

        for sym in Symbol::iter() {
            args.push(format!("{sym}_USDT"))
        }

        let subscribe_message = serde_json::json!({
            "time": get_now_timestamp(),
            "channel": "spot.book_ticker",
            "event": "subscribe",
            "payload": args,
        })
        .to_string();

        BaseOrderBookStream {
            url,
            subscribe_message,
            order_book_bus,
            decoder: GateOrderBookDecoder,
        }
    }
}
