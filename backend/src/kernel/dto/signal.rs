use crate::kernel::enums::exchange::{Exchange, Symbol};
use rust_decimal::Decimal;
use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct SignalDto {
    pub symbol: Symbol,
    pub buy_exchange: Exchange,
    pub sell_exchange: Exchange,
    pub buy_price: Decimal,
    pub sell_price: Decimal,
    pub volume: Decimal,
    pub gross_percent: Decimal,
    pub net_percent: Decimal,
    pub buy_link: String,
    pub sell_link: String,
}
