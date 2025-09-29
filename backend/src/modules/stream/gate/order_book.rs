use crate::kernel::enums::exchange::{Exchange, Symbol};
use crate::kernel::utils::time::get_now_timestamp;
use crate::modules::stream::order_book::{COUNT_PER_SUBSCRIBE, OrderBookStream};
use crate::modules::stream::supported_pair::SupportedPair;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::str::FromStr;

pub struct GateOrderBookStream;

#[async_trait]
impl OrderBookStream for GateOrderBookStream {
    const EXCHANGE: Exchange = Exchange::Gate;

    fn get_ws_url() -> String {
        "wss://api.gateio.ws/ws/v4/".to_string()
    }

    async fn get_batches() -> Vec<Value> {
        let mut batches = Vec::new();
        let mut current = Vec::with_capacity(COUNT_PER_SUBSCRIBE);

        let symbols = Self::get_tickers("", "base").await;

        for sym in symbols {
            current.push(format!("{sym}_USDT"));

            if current.len() == COUNT_PER_SUBSCRIBE {
                batches.push(json!({
                    "time": get_now_timestamp(),
                    "channel": "spot.book_ticker",
                    "event": "subscribe",
                    "payload": current
                }));

                current = Vec::with_capacity(COUNT_PER_SUBSCRIBE);
            }
        }

        if !current.is_empty() {
            batches.push(json!({
                "time": get_now_timestamp(),
                "channel": "spot.book_ticker",
                "event": "subscribe",
                "payload": current
            }));
        }

        batches
    }
}

impl SupportedPair for GateOrderBookStream {
    const EXCHANGE: Exchange = Exchange::Gate;

    fn get_rest_url() -> String {
        "https://api.gateio.ws/api/v4/spot/currency_pairs".to_string()
    }

    fn filter(value: &Value) -> bool {
        value.get("trade_status").and_then(Value::as_str) == Some("tradable")
            && value.get("quote").and_then(Value::as_str) == Some("USDT")
            && value
                .get("base")
                .and_then(Value::as_str)
                .is_some_and(|base| Symbol::from_str(base).is_ok())
    }
}
