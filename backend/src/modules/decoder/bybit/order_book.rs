use crate::kernel::dto::order_book::OrderBookDto;
use crate::kernel::enums::exchange::Exchange;
use crate::modules::decoder::order_book::OrderBookDecoder;
use crate::modules::decoder::utils::{get_ask, get_bid, normalize_symbol};
use anyhow::Result;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
struct Message<'a> {
    #[serde(borrow)]
    data: Data<'a>,
    ts: u64,
}

#[derive(Deserialize, Debug)]
struct Data<'a> {
    a: Vec<[Decimal; 2]>,
    b: Vec<[Decimal; 2]>,
    s: &'a str,
}

#[derive(Clone)]
pub struct BybitOrderBookDecoder;

impl OrderBookDecoder for BybitOrderBookDecoder {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> Result<Arc<OrderBookDto>> {
        let parsed = serde_json::from_slice::<Message>(bytes)?;

        let (ask_price, ask_amount) = get_ask(&parsed.data.a)?;
        let (bid_price, bid_amount) = get_bid(&parsed.data.b)?;
        let symbol = normalize_symbol(parsed.data.s)?;

        Ok(Arc::new(OrderBookDto {
            exchange: Exchange::Bybit,
            symbol,
            ask_price,
            ask_amount,
            bid_price,
            bid_amount,
            pair_link: Exchange::Bybit.pair_url(symbol),
            timestamp: parsed.ts,
        }))
    }
}
