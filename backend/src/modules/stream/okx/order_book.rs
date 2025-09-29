use crate::kernel::enums::exchange::{Exchange, Symbol};
use crate::modules::stream::order_book::{COUNT_PER_SUBSCRIBE, OrderBookStream};
use crate::modules::stream::supported_pair::SupportedPair;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::str::FromStr;

pub struct OkxOrderBookStream;

#[async_trait]
impl OrderBookStream for OkxOrderBookStream {
    const EXCHANGE: Exchange = Exchange::Okx;

    fn get_ws_url() -> String {
        "wss://ws.okx.com:8443/ws/v5/public".to_string()
    }

    async fn get_batches() -> Vec<Value> {
        let mut batches = Vec::new();
        let mut current = Vec::with_capacity(COUNT_PER_SUBSCRIBE);

        let symbols = Self::get_tickers("/data", "baseCcy").await;

        for sym in symbols {
            current.push(json!({
                "channel": "bbo-tbt",
                "instId": format!("{}-USDT", sym)
            }));

            if current.len() == COUNT_PER_SUBSCRIBE {
                batches.push(json!({
                    "op": "subscribe",
                    "args": current
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

impl SupportedPair for OkxOrderBookStream {
    const EXCHANGE: Exchange = Exchange::Okx;

    fn get_rest_url() -> String {
        "https://www.okx.com/api/v5/public/instruments?instType=SPOT".to_string()
    }

    fn filter(value: &Value) -> bool {
        value.get("state").and_then(Value::as_str) == Some("live")
            && value.get("ruleType").and_then(Value::as_str) == Some("normal")
            && value.get("quoteCcy").and_then(Value::as_str) == Some("USDT")
            && value
                .get("baseCcy")
                .and_then(Value::as_str)
                .is_some_and(|base| Symbol::from_str(base).is_ok())
    }
}
