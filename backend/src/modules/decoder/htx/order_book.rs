use crate::kernel::dto::order_book::OrderBookDto;
use crate::kernel::enums::exchange::Exchange;
use crate::modules::decoder::order_book::OrderBookDecoder;
use crate::modules::decoder::utils::normalize_symbol;
use anyhow::Result;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
struct Message<'a> {
    #[serde(borrow)]
    tick: Data<'a>,
    ts: u64,
}

#[derive(Deserialize, Debug)]
struct Data<'a> {
    ask: Decimal,
    #[serde(rename = "askSize")]
    ask_size: Decimal,
    bid: Decimal,
    #[serde(rename = "bidSize")]
    bid_size: Decimal,
    symbol: &'a str,
}

#[derive(Clone)]
pub struct HtxOrderBookDecoder;

impl OrderBookDecoder for HtxOrderBookDecoder {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> Result<Arc<OrderBookDto>> {
        let parsed = serde_json::from_slice::<Message>(bytes)?;

        let symbol = normalize_symbol(parsed.tick.symbol)?;

        Ok(Arc::new(OrderBookDto {
            exchange: Exchange::Htx,
            symbol,
            ask_price: parsed.tick.ask,
            ask_amount: parsed.tick.ask_size,
            bid_price: parsed.tick.bid,
            bid_amount: parsed.tick.bid_size,
            pair_link: Exchange::Htx.pair_url(symbol),
            timestamp: parsed.ts,
        }))
    }
}
