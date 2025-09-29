use crate::kernel::dto::order_book::OrderBookDto;
use crate::kernel::enums::decoder::ActionDecoder;
use crate::kernel::enums::exchange::Exchange;
use crate::modules::decoder::order_book::OrderBookDecoder;
use crate::modules::decoder::utils::{normalize_symbol, val_to_dec};
use anyhow::{Result, anyhow};
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct GateOrderBookDecoder;

impl OrderBookDecoder for GateOrderBookDecoder {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> Result<ActionDecoder> {
        let parsed = serde_json::from_slice::<Value>(bytes)?;

        if parsed.get("channel").and_then(|x| x.as_str()) == Some("spot.pong") {
            return Ok(ActionDecoder::Skip);
        }

        if parsed.pointer("/result/status").and_then(|v| v.as_str()) == Some("success") {
            return Ok(ActionDecoder::Skip);
        }

        if parsed.get("/result/status").and_then(|x| x.as_str()) == Some("fail") {
            tracing::warn!(message=%parsed, "Status error:");

            return Ok(ActionDecoder::Skip);
        }

        if let Some(data) = parsed.get("result") {
            let symbol = normalize_symbol(
                data.get("s")
                    .and_then(|x| x.as_str())
                    .ok_or_else(|| anyhow!("s not found"))?,
            )?;

            let dto = OrderBookDto {
                exchange: Exchange::Gate,
                symbol,
                ask_price: data
                    .get("a")
                    .ok_or_else(|| anyhow!("Error ask_price"))
                    .and_then(val_to_dec)?,
                ask_amount: data
                    .get("A")
                    .ok_or_else(|| anyhow!("Error ask_amount"))
                    .and_then(val_to_dec)?,
                bid_price: data
                    .get("b")
                    .ok_or_else(|| anyhow!("Error bid_price"))
                    .and_then(val_to_dec)?,
                bid_amount: data
                    .get("B")
                    .ok_or_else(|| anyhow!("Error bid_amount"))
                    .and_then(val_to_dec)?,
                pair_link: Exchange::Gate.pair_url(symbol),
                timestamp: parsed
                    .get("time")
                    .and_then(|x| x.as_u64())
                    .ok_or_else(|| anyhow!("timestamp not found"))?,
            };

            return Ok(ActionDecoder::Publish(Arc::new(dto)));
        }

        Ok(ActionDecoder::Skip)
    }
}
