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
    result: Data<'a>,
}

#[derive(Deserialize, Debug)]
struct Data<'a> {
    a: Decimal,
    #[serde(rename = "A")]
    ask_size: Decimal,
    b: Decimal,
    #[serde(rename = "B")]
    bid_size: Decimal,
    s: &'a str,
    t: u64,
}

#[derive(Clone)]
pub struct GateOrderBookDecoder;

impl OrderBookDecoder for GateOrderBookDecoder {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> Result<Arc<OrderBookDto>> {
        let parsed = serde_json::from_slice::<Message>(bytes)?;

        let symbol = normalize_symbol(parsed.result.s)?;

        Ok(Arc::new(OrderBookDto {
            exchange: Exchange::Gate,
            symbol,
            ask_price: parsed.result.a,
            ask_amount: parsed.result.ask_size,
            bid_price: parsed.result.b,
            bid_amount: parsed.result.bid_size,
            pair_link: Exchange::Gate.pair_url(symbol),
            timestamp: parsed.result.t,
        }))
    }
}
