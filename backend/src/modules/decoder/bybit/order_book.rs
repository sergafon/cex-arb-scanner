use crate::kernel::dto::order_book::OrderBookDto;
use crate::kernel::enums::decoder::ActionDecoder;
use crate::kernel::enums::exchange::Exchange;
use crate::modules::decoder::order_book::OrderBookDecoder;
use crate::modules::decoder::utils::{get_price_and_size, normalize_symbol};
use anyhow::{Result, anyhow};
use serde_json::Value;
use std::sync::Arc;

#[derive(Clone)]
pub struct BybitOrderBookDecoder;

impl OrderBookDecoder for BybitOrderBookDecoder {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> Result<ActionDecoder> {
        let parsed = serde_json::from_slice::<Value>(bytes)?;

        if parsed.get("ret_msg").and_then(|x| x.as_str()) == Some("pong") {
            return Ok(ActionDecoder::Skip);
        }

        if parsed.get("success").and_then(|x| x.as_bool()) == Some(true) {
            return Ok(ActionDecoder::Skip);
        }

        if parsed.get("success").and_then(|x| x.as_bool()) == Some(false) {
            tracing::warn!(message=%parsed, "Status error:");

            return Ok(ActionDecoder::Skip);
        }

        if let Some(data) = parsed.get("data") {
            let symbol = normalize_symbol(
                data.get("s")
                    .and_then(|x| x.as_str())
                    .ok_or_else(|| anyhow!("s not found"))?,
            )?;

            let (ask_price, ask_amount) =
                get_price_and_size(data.get("a").ok_or_else(|| anyhow!("Error asks"))?)?;

            let (bid_price, bid_amount) =
                get_price_and_size(data.get("b").ok_or_else(|| anyhow!("Error bids"))?)?;

            let dto = OrderBookDto {
                exchange: Exchange::Bybit,
                symbol,
                ask_price,
                ask_amount,
                bid_price,
                bid_amount,
                pair_link: Exchange::Bybit.pair_url(symbol),
                timestamp: parsed
                    .get("ts")
                    .and_then(|x| x.as_u64())
                    .ok_or_else(|| anyhow!("timestamp not found"))?
                    / 1000,
            };

            return Ok(ActionDecoder::Publish(Arc::new(dto)));
        }

        tracing::debug!(message=?String::from_utf8_lossy(bytes), "Unknown error");

        Ok(ActionDecoder::Skip)

        // let parsed = serde_json::from_slice::<Message>(bytes)?;
        //
        // let (ask_price, ask_amount) = get_ask(&parsed.data.a)?;
        // let (bid_price, bid_amount) = get_bid(&parsed.data.b)?;
        // let symbol = normalize_symbol(parsed.data.s)?;
        //
        // Ok(Arc::new(OrderBookDto {
        //     exchange: Exchange::Bybit,
        //     symbol,
        //     ask_price,
        //     ask_amount,
        //     bid_price,
        //     bid_amount,
        //     pair_link: Exchange::Bybit.pair_url(symbol),
        //     timestamp: parsed.ts,
        // }))
    }
}
