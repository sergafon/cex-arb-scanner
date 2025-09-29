use crate::kernel::dto::order_book::OrderBookDto;
use crate::kernel::enums::decoder::ActionDecoder;
use crate::kernel::enums::exchange::Exchange;
use crate::modules::decoder::order_book::OrderBookDecoder;
use crate::modules::decoder::utils::{get_price_and_size, normalize_symbol};
use anyhow::{Result, anyhow};
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct BitgetOrderBookDecoder;

impl OrderBookDecoder for BitgetOrderBookDecoder {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> Result<ActionDecoder> {
        if bytes.eq_ignore_ascii_case(b"pong") {
            return Ok(ActionDecoder::Skip);
        }

        let parsed = serde_json::from_slice::<Value>(bytes)?;

        if parsed.get("event").and_then(|x| x.as_str()) == Some("subscribe") {
            return Ok(ActionDecoder::Skip);
        }

        if parsed.get("event").and_then(|x| x.as_str()) == Some("error") {
            tracing::warn!(message=%parsed, "Status error:");

            return Ok(ActionDecoder::Skip);
        }

        if let Some(arg) = parsed.get("arg") {
            let symbol = normalize_symbol(
                arg.get("instId")
                    .and_then(|x| x.as_str())
                    .ok_or_else(|| anyhow!("instId not found"))?,
            )?;

            let data: &Value = parsed
                .get("data")
                .and_then(Value::as_array)
                .and_then(|a| a.first())
                .ok_or_else(|| anyhow!("Okx empty data array"))?;

            let (ask_price, ask_amount) =
                get_price_and_size(data.get("asks").ok_or_else(|| anyhow!("Error asks"))?)?;

            let (bid_price, bid_amount) =
                get_price_and_size(data.get("bids").ok_or_else(|| anyhow!("Error bids"))?)?;

            let dto = OrderBookDto {
                exchange: Exchange::Bitget,
                symbol,
                ask_price,
                ask_amount,
                bid_price,
                bid_amount,
                pair_link: Exchange::Bitget.pair_url(symbol),
                timestamp: data
                    .get("ts")
                    .and_then(|x| x.as_str())
                    .ok_or_else(|| anyhow!("timestamp not found"))?
                    .parse::<u64>()?
                    / 1000,
            };

            return Ok(ActionDecoder::Publish(Arc::new(dto)));
        }

        Ok(ActionDecoder::Skip)
    }
}
