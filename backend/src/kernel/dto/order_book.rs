use crate::kernel::enums::exchange::{Exchange, Symbol};
use rust_decimal::Decimal;

#[derive(Debug, Default)]
pub struct OrderBookDto {
    pub exchange: Exchange,
    pub symbol: Symbol,
    pub ask_price: Decimal,
    pub ask_amount: Decimal,
    pub bid_price: Decimal,
    pub bid_amount: Decimal,
    pub pair_link: String,
    pub timestamp: u64,
}
