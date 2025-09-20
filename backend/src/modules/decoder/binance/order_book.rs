use crate::kernel::dto::order_book::OrderBookDto;
use crate::kernel::enums::exchange::Exchange;
use crate::kernel::utils::time::get_now_timestamp;
use crate::modules::decoder::order_book::OrderBookDecoder;
use crate::modules::decoder::utils::normalize_symbol;
use anyhow::Result;
use rust_decimal::Decimal;
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize, Debug)]
struct Message<'a> {
    a: Decimal,
    #[serde(rename = "A")]
    ask_size: Decimal,
    b: Decimal,
    #[serde(rename = "B")]
    bid_size: Decimal,
    s: &'a str,
}

#[derive(Clone)]
pub struct BinanceOrderBookDecoder;

impl OrderBookDecoder for BinanceOrderBookDecoder {
    #[inline]
    fn decode(&self, bytes: &[u8]) -> Result<Arc<OrderBookDto>> {
        let parsed = serde_json::from_slice::<Message>(bytes)?;

        let symbol = normalize_symbol(parsed.s)?;

        Ok(Arc::new(OrderBookDto {
            exchange: Exchange::Binance,
            symbol,
            ask_price: parsed.a,
            ask_amount: parsed.ask_size,
            bid_price: parsed.b,
            bid_amount: parsed.bid_size,
            pair_link: Exchange::Binance.pair_url(symbol),
            timestamp: get_now_timestamp(),
        }))
    }
}
