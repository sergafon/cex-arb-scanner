use crate::kernel::dto::order_book::OrderBookDto;
use crate::kernel::enums::decoder::ActionDecoder;
use crate::kernel::enums::exchange::Exchange;
use crate::modules::decoder::order_book::OrderBookDecoder;
use crate::modules::decoder::utils::normalize_symbol;
use anyhow::{Result, anyhow};
use flate2::read::GzDecoder;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;
use serde_json::Value;
use std::io::{Cursor, Read};
use std::sync::Arc;

#[derive(Clone)]
pub struct HtxOrderBookDecoder;

impl HtxOrderBookDecoder {
    fn g_unzip(bytes: &[u8]) -> Result<Vec<u8>> {
        let cursor = Cursor::new(bytes);
        let mut gz = GzDecoder::new(cursor);
        let mut bytes = Vec::with_capacity(bytes.len() * 6);

        gz.read_to_end(&mut bytes)?;

        Ok(bytes)
    }
}

impl OrderBookDecoder for HtxOrderBookDecoder {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> Result<ActionDecoder> {
        let bytes = Self::g_unzip(bytes)?;

        if bytes.is_empty() {
            return Ok(ActionDecoder::Skip);
        }

        let parsed = serde_json::from_slice::<Value>(&bytes)?;

        if let Some(ts) = parsed.get("ping").and_then(|x| x.as_u64()) {
            return Ok(ActionDecoder::Pong(ts));
        }

        if parsed.get("status").and_then(|x| x.as_str()) == Some("ok") {
            return Ok(ActionDecoder::Skip);
        }

        if parsed.get("status").and_then(|x| x.as_str()) == Some("error") {
            tracing::warn!(message=%parsed, "Status error:");

            return Ok(ActionDecoder::Skip);
        }

        if let Some(tick) = parsed.get("tick") {
            let symbol = normalize_symbol(
                tick.get("symbol")
                    .and_then(|x| x.as_str())
                    .ok_or_else(|| anyhow!("symbol not found"))?,
            )?;

            let dto = OrderBookDto {
                exchange: Exchange::Htx,
                symbol,
                ask_price: tick
                    .get("ask")
                    .and_then(|x| x.as_f64())
                    .and_then(Decimal::from_f64)
                    .ok_or_else(|| anyhow!("Error ask_price"))?,
                ask_amount: tick
                    .get("askSize")
                    .and_then(|x| x.as_f64())
                    .and_then(Decimal::from_f64)
                    .ok_or_else(|| anyhow!("Error ask_amount"))?,
                bid_price: tick
                    .get("bid")
                    .and_then(|x| x.as_f64())
                    .and_then(Decimal::from_f64)
                    .ok_or_else(|| anyhow!("Error bid_price"))?,
                bid_amount: tick
                    .get("bidSize")
                    .and_then(|x| x.as_f64())
                    .and_then(Decimal::from_f64)
                    .ok_or_else(|| anyhow!("Error bid_amount"))?,
                pair_link: Exchange::Htx.pair_url(symbol),
                timestamp: parsed
                    .get("ts")
                    .and_then(|x| x.as_u64())
                    .ok_or_else(|| anyhow!("timestamp not found"))?
                    / 1000,
            };

            return Ok(ActionDecoder::Publish(Arc::new(dto)));
        }

        Ok(ActionDecoder::Skip)
    }
}
