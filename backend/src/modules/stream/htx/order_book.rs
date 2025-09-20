use crate::kernel::enums::exchange::Symbol;
use crate::modules::bus::order_book::OrderBookBus;
use crate::modules::decoder::htx::order_book::HtxOrderBookDecoder;
use crate::modules::stream::base_order_book::BaseOrderBookStream;
use crate::modules::stream::order_book::OrderBookStream;
use std::sync::Arc;
use strum::IntoEnumIterator;

pub struct HtxOrderBookStream;

impl OrderBookStream for HtxOrderBookStream {
    type Decoder = HtxOrderBookDecoder;

    fn new(order_book_bus: Arc<OrderBookBus>) -> BaseOrderBookStream<Self::Decoder> {
        let url = "wss://api.huobi.pro/ws".to_string();

        let mut subs = vec![];

        for sym in Symbol::iter() {
            let s = sym.to_string().to_lowercase();

            subs.push(serde_json::json!({
                "sub": format!("market.{s}usdt.bbo"),
                "id": format!("{s}-bbo")
            }));
        }

        let subscribe_message = serde_json::to_string(&subs).expect("serialize subs");

        BaseOrderBookStream { url, subscribe_message, order_book_bus, decoder: HtxOrderBookDecoder }
    }
}
