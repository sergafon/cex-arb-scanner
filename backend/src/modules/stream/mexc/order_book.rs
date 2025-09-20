use crate::kernel::enums::exchange::Symbol;
use crate::modules::bus::order_book::OrderBookBus;
use crate::modules::decoder::mexc::order_book::MexcBookTickerDecoder;
use crate::modules::stream::base_order_book::BaseOrderBookStream;
use crate::modules::stream::order_book::OrderBookStream;
use serde_json::json;
use std::sync::Arc;
use strum::IntoEnumIterator;

pub struct MexcOrderBookStream;

impl OrderBookStream for MexcOrderBookStream {
    type Decoder = MexcBookTickerDecoder;

    fn new(order_book_bus: Arc<OrderBookBus>) -> BaseOrderBookStream<Self::Decoder> {
        let url = "wss://wbs-api.mexc.com/ws".to_string();

        let mut batches = Vec::new();
        let mut current = Vec::with_capacity(30);

        for sym in Symbol::iter() {
            current.push(format!(
                "spot@public.aggre.bookTicker.v3.api.pb@100ms@{sym}USDT",
            ));

            if current.len() == 30 {
                batches.push(json!({
                    "method": "SUBSCRIPTION",
                    "params": current
                }));
                current = Vec::with_capacity(30);
            }
        }

        if !current.is_empty() {
            batches.push(json!({
                "method": "SUBSCRIPTION",
                "params": current
            }));
        }

        let subscribe_message = serde_json::to_string(&batches).unwrap();

        BaseOrderBookStream {
            url,
            subscribe_message,
            order_book_bus,
            decoder: MexcBookTickerDecoder,
        }
    }
}
