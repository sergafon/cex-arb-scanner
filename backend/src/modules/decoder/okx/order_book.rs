use crate::kernel::dto::order_book::OrderBookDto;
use crate::kernel::enums::exchange::Exchange;
use crate::modules::decoder::order_book::OrderBookDecoder;
use crate::modules::decoder::utils::{get_ask, get_bid, normalize_symbol};
use anyhow::{anyhow, Result};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
struct Message<'a> {
    #[serde(borrow)]
    arg: Arg<'a>,
    #[serde(borrow)]
    data: Vec<Data<'a>>,
}

#[derive(Deserialize, Debug)]
struct Arg<'a> {
    #[serde(rename = "instId")]
    inst_id: &'a str,
}

#[derive(Deserialize, Debug)]
struct Data<'a> {
    asks: Vec<[Decimal; 4]>,
    bids: Vec<[Decimal; 4]>,
    ts: &'a str,
}

#[derive(Clone)]
pub struct OkxOrderBookDecoder;

impl OrderBookDecoder for OkxOrderBookDecoder {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> Result<Arc<OrderBookDto>> {
        let parsed = serde_json::from_slice::<Message>(bytes)?;

        let parsed_data = parsed
            .data
            .first()
            .ok_or_else(|| anyhow!("Okx empty data array"))?;

        let (ask_price, ask_amount) = get_ask(&parsed_data.asks)?;
        let (bid_price, bid_amount) = get_bid(&parsed_data.bids)?;
        let symbol = normalize_symbol(parsed.arg.inst_id)?;

        Ok(Arc::new(OrderBookDto {
            exchange: Exchange::Okx,
            symbol,
            ask_price,
            ask_amount,
            bid_price,
            bid_amount,
            pair_link: Exchange::Okx.pair_url(symbol),
            timestamp: parsed_data.ts.parse::<u64>()?,
        }))
    }
}
