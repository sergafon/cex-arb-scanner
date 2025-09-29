use crate::modules::broker::order_book::OrderBookBroker;
use crate::modules::stream::binance::order_book::BinanceOrderBookStream;
use crate::modules::stream::bitget::order_book::BitgetOrderBookStream;
use crate::modules::stream::bybit::order_book::BybitOrderBookStream;
use crate::modules::stream::gate::order_book::GateOrderBookStream;
use crate::modules::stream::htx::order_book::HtxOrderBookStream;
use crate::modules::stream::okx::order_book::OkxOrderBookStream;
use std::sync::Arc;

pub struct OrderBookStreamManager {
    pub broker: Arc<OrderBookBroker>,

    pub htx: HtxOrderBookStream,
    pub okx: OkxOrderBookStream,
    pub gate: GateOrderBookStream,
    pub bybit: BybitOrderBookStream,
    pub bitget: BitgetOrderBookStream,
    pub binance: BinanceOrderBookStream,
    // pub mexc: MexcOrderBookStream,
}

impl OrderBookStreamManager {
    pub fn new(order_book_broker: Arc<OrderBookBroker>) -> Self {
        Self {
            broker: order_book_broker,

            htx: HtxOrderBookStream,
            okx: OkxOrderBookStream,
            gate: GateOrderBookStream,
            bybit: BybitOrderBookStream,
            bitget: BitgetOrderBookStream,
            binance: BinanceOrderBookStream,
            // mexc: MexcOrderBookStream,
        }
    }
}
