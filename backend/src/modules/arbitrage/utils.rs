use crate::kernel::enums::exchange::Exchange;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;

#[inline]
pub fn taker_fee(ex: &Exchange) -> Decimal {
    match ex {
        Exchange::Bybit => Decimal::from_f64(0.1 / 100.0).unwrap(),
        Exchange::Gate => Decimal::from_f64(0.09 / 100.0).unwrap(),
        Exchange::Bitget => Decimal::from_f64(0.1 / 100.0).unwrap(),
        Exchange::Binance => Decimal::from_f64(0.1 / 100.0).unwrap(),
        Exchange::Htx => Decimal::from_f64(0.2 / 100.0).unwrap(),
        Exchange::Okx => Decimal::from_f64(0.05 / 100.0).unwrap(),
        Exchange::Mexc => Decimal::from_f64(0.05 / 100.0).unwrap(),
    }
}

#[inline]
pub fn percent(num: Decimal) -> Decimal {
    num * Decimal::from(100)
}
