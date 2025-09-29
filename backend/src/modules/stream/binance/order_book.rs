use crate::kernel::enums::exchange::{Exchange, Symbol};
use crate::modules::stream::order_book::{COUNT_PER_SUBSCRIBE, OrderBookStream};
use crate::modules::stream::supported_pair::SupportedPair;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::str::FromStr;

pub struct BinanceOrderBookStream;

#[async_trait]
impl OrderBookStream for BinanceOrderBookStream {
    const EXCHANGE: Exchange = Exchange::Binance;

    fn get_ws_url() -> String {
        "wss://stream.binance.com:9443/ws".to_string()
    }

    async fn get_batches() -> Vec<Value> {
        let mut batches = Vec::new();
        let mut current = Vec::with_capacity(COUNT_PER_SUBSCRIBE);

        let symbols = Self::get_tickers("/symbols", "baseAsset").await;

        for sym in symbols {
            let s = format!("{}usdt@bookTicker", sym.to_lowercase());

            current.push(s);

            if current.len() == COUNT_PER_SUBSCRIBE {
                batches.push(json!({
                    "method": "SUBSCRIBE",
                    "params": current,
                    "id": sym
                }));

                current = Vec::with_capacity(COUNT_PER_SUBSCRIBE);
            }
        }

        if !current.is_empty() {
            batches.push(json!({
                "method": "SUBSCRIBE",
                "params": current,
                "id": 1,
            }));
        }

        batches
    }
}

impl SupportedPair for BinanceOrderBookStream {
    const EXCHANGE: Exchange = Exchange::Binance;

    fn get_rest_url() -> String {
        "https://api.binance.com/api/v3/exchangeInfo".to_string()
    }

    fn filter(value: &Value) -> bool {
        value.get("status").and_then(Value::as_str) == Some("TRADING")
            && value.get("isSpotTradingAllowed").and_then(Value::as_bool) == Some(true)
            && value.get("quoteAsset").and_then(Value::as_str) == Some("USDT")
            && value
                .get("baseAsset")
                .and_then(Value::as_str)
                .is_some_and(|base| Symbol::from_str(base).is_ok())
    }
}
