use crate::kernel::enums::exchange::{Exchange, Symbol};
use crate::modules::stream::order_book::{COUNT_PER_SUBSCRIBE, OrderBookStream};
use crate::modules::stream::supported_pair::SupportedPair;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::str::FromStr;

pub struct BybitOrderBookStream;

#[async_trait]
impl OrderBookStream for BybitOrderBookStream {
    const EXCHANGE: Exchange = Exchange::Bybit;

    fn get_ws_url() -> String {
        "wss://stream.bybit.com/v5/public/spot".to_string()
    }

    async fn get_batches() -> Vec<Value> {
        let mut batches = Vec::new();
        let mut current = Vec::with_capacity(COUNT_PER_SUBSCRIBE);

        let symbols = Self::get_tickers("/result/list", "baseCoin").await;

        for sym in symbols {
            current.push(format!("orderbook.1.{sym}USDT"));

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

impl SupportedPair for BybitOrderBookStream {
    const EXCHANGE: Exchange = Exchange::Bybit;

    fn get_rest_url() -> String {
        "https://api.bybit.com/v5/market/instruments-info?category=spot".to_string()
    }

    fn filter(value: &Value) -> bool {
        value.get("status").and_then(Value::as_str) == Some("Trading")
            && value.get("quoteCoin").and_then(Value::as_str) == Some("USDT")
            && value
                .get("baseCoin")
                .and_then(Value::as_str)
                .is_some_and(|base| Symbol::from_str(base).is_ok())
    }
}
