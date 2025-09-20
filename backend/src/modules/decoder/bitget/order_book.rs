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
    data: Vec<Data>,
    ts: u64,
}

#[derive(Deserialize, Debug)]
struct Arg<'a> {
    #[serde(rename = "instId")]
    inst_id: &'a str,
}

#[derive(Deserialize, Debug)]
struct Data {
    asks: Vec<[Decimal; 2]>,
    bids: Vec<[Decimal; 2]>,
}

#[derive(Clone)]
pub struct BitgetOrderBookDecoder;

impl OrderBookDecoder for BitgetOrderBookDecoder {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> Result<Arc<OrderBookDto>> {
        let parsed = serde_json::from_slice::<Message>(bytes)?;

        let parsed_data = parsed
            .data
            .first()
            .ok_or_else(|| anyhow!("Bitget empty data array"))?;

        let (ask_price, ask_amount) = get_ask(&parsed_data.asks)?;
        let (bid_price, bid_amount) = get_bid(&parsed_data.bids)?;
        let symbol = normalize_symbol(parsed.arg.inst_id)?;

        Ok(Arc::new(OrderBookDto {
            exchange: Exchange::Bitget,
            symbol,
            ask_price,
            ask_amount,
            bid_price,
            bid_amount,
            pair_link: Exchange::Bitget.pair_url(symbol),
            timestamp: parsed.ts,
        }))
    }
}
