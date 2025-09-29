use crate::kernel::enums::exchange::{Exchange, Symbol};
use crate::modules::stream::order_book::{COUNT_PER_SUBSCRIBE, OrderBookStream};
use crate::modules::stream::supported_pair::SupportedPair;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::str::FromStr;

pub struct BitgetOrderBookStream;

#[async_trait]
impl OrderBookStream for BitgetOrderBookStream {
    const EXCHANGE: Exchange = Exchange::Bitget;

    fn get_ws_url() -> String {
        "wss://ws.bitget.com/v2/ws/public".to_string()
    }

    async fn get_batches() -> Vec<Value> {
        let mut batches = Vec::new();
        let mut current = Vec::with_capacity(COUNT_PER_SUBSCRIBE);

        let symbols = Self::get_tickers("/data", "baseCoin").await;

        for sym in symbols {
            current.push(json!({
                "instType": "SPOT",
                "channel": "books1",
                "instId": format!("{sym}USDT")
            }));

            if current.len() == COUNT_PER_SUBSCRIBE {
                batches.push(json!({
                    "op": "subscribe",
                    "args": current,
                }));

                current = Vec::with_capacity(COUNT_PER_SUBSCRIBE);
            }
        }

        if !current.is_empty() {
            batches.push(json!({
                "op": "subscribe",
                "args": current
            }));
        }

        batches
    }
}

impl SupportedPair for BitgetOrderBookStream {
    const EXCHANGE: Exchange = Exchange::Bitget;

    fn get_rest_url() -> String {
        "https://api.bitget.com/api/v2/spot/public/symbols".to_string()
    }

    fn filter(value: &Value) -> bool {
        value.get("state").and_then(Value::as_str) == Some("online")
            && value.get("quoteCoin").and_then(Value::as_str) == Some("USDT")
            && value
                .get("baseCoin")
                .and_then(Value::as_str)
                .is_some_and(|base| Symbol::from_str(base).is_ok())
    }
}
