use crate::kernel::dto::order_book::OrderBookDto;
use crate::kernel::enums::decoder::ActionDecoder;
use crate::kernel::enums::exchange::Exchange;
use crate::kernel::utils::time::get_now_timestamp;
use crate::modules::decoder::order_book::OrderBookDecoder;
use crate::modules::decoder::utils::{normalize_symbol, val_to_dec};
use anyhow::{Result, anyhow};
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct BinanceOrderBookDecoder;

impl OrderBookDecoder for BinanceOrderBookDecoder {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> Result<ActionDecoder> {
        let parsed = serde_json::from_slice::<Value>(bytes)?;

        if parsed.get("result").is_some_and(Value::is_null) {
            return Ok(ActionDecoder::Skip);
        }

        if parsed.get("code").and_then(Value::as_u64) >= Some(400) {
            tracing::warn!(message=%parsed, "Status error:");

            return Ok(ActionDecoder::Skip);
        }

        let symbol = normalize_symbol(
            parsed
                .get("s")
                .and_then(|x| x.as_str())
                .ok_or_else(|| anyhow!("s not found"))?,
        )?;

        let dto = OrderBookDto {
            exchange: Exchange::Binance,
            symbol,
            ask_price: parsed
                .get("a")
                .ok_or_else(|| anyhow!("Error ask_price"))
                .and_then(val_to_dec)?,
            ask_amount: parsed
                .get("A")
                .ok_or_else(|| anyhow!("Error ask_amount"))
                .and_then(val_to_dec)?,
            bid_price: parsed
                .get("b")
                .ok_or_else(|| anyhow!("Error bid_price"))
                .and_then(val_to_dec)?,
            bid_amount: parsed
                .get("B")
                .ok_or_else(|| anyhow!("Error bid_amount"))
                .and_then(val_to_dec)?,
            pair_link: Exchange::Binance.pair_url(symbol),
            timestamp: get_now_timestamp(),
        };

        Ok(ActionDecoder::Publish(Arc::new(dto)))
    }
}
