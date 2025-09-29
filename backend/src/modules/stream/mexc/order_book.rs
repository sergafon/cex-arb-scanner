use crate::kernel::enums::exchange::{Exchange, Symbol};
use crate::modules::stream::order_book::{COUNT_PER_SUBSCRIBE, OrderBookStream};
use async_trait::async_trait;
use serde_json::{Value, json};
use strum::IntoEnumIterator;

pub struct MexcOrderBookStream;

#[async_trait]
impl OrderBookStream for MexcOrderBookStream {
    const EXCHANGE: Exchange = Exchange::Mexc;

    fn get_ws_url() -> String {
        "wss://wbs-api.mexc.com/ws".to_string()
    }

    async fn get_batches() -> Vec<Value> {
        let mut batches = Vec::new();
        let mut current = Vec::with_capacity(COUNT_PER_SUBSCRIBE);

        for sym in Symbol::iter() {
            current.push(format!(
                "spot@public.aggre.bookTicker.v3.api.pb@100ms@{sym}USDT",
            ));

            if current.len() == COUNT_PER_SUBSCRIBE {
                batches.push(json!({
                    "method": "SUBSCRIPTION",
                    "params": current
                }));

                current = Vec::with_capacity(COUNT_PER_SUBSCRIBE);
            }
        }

        if !current.is_empty() {
            batches.push(json!({
                "method": "SUBSCRIPTION",
                "params": current
            }));
        }

        batches
    }
}
