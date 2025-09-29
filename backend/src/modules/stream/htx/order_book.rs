use crate::kernel::enums::exchange::{Exchange, Symbol};
use crate::modules::stream::order_book::{COUNT_PER_SUBSCRIBE, OrderBookStream};
use crate::modules::stream::supported_pair::SupportedPair;
use async_trait::async_trait;
use serde_json::{Value, json};
use std::str::FromStr;

pub struct HtxOrderBookStream;

#[async_trait]
impl OrderBookStream for HtxOrderBookStream {
    const EXCHANGE: Exchange = Exchange::Htx;

    fn get_ws_url() -> String {
        "wss://api.huobi.pro/ws".to_string()
    }

    async fn get_batches() -> Vec<Value> {
        let mut batches = Vec::new();
        let mut current = Vec::with_capacity(COUNT_PER_SUBSCRIBE);

        let symbols = Self::get_tickers("/data", "bcdn").await;

        for sym in symbols {
            let sym = sym.to_lowercase();

            current.push(json!({
                "sub": format!("market.{sym}usdt.bbo"),
                "id": format!("{sym}-bbo")
            }));

            if current.len() == COUNT_PER_SUBSCRIBE {
                batches.push(json!({
                    "batches": current
                }));

                current = Vec::with_capacity(COUNT_PER_SUBSCRIBE);
            }
        }

        if !current.is_empty() {
            batches.push(json!({
                "batches": current
            }));
        }

        batches
    }
}

impl SupportedPair for HtxOrderBookStream {
    const EXCHANGE: Exchange = Exchange::Htx;

    fn get_rest_url() -> String {
        "https://api.huobi.pro/v2/settings/common/symbols".to_string()
    }

    fn filter(value: &Value) -> bool {
        value.get("state").and_then(Value::as_str) == Some("online")
            && value.get("te").and_then(Value::as_bool) == Some(true)
            && value.get("qcdn").and_then(Value::as_str) == Some("USDT")
            && value
                .get("bcdn")
                .and_then(Value::as_str)
                .is_some_and(|base| Symbol::from_str(base).is_ok())
    }
}
