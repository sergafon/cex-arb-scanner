use crate::modules::bus::order_book::OrderBookBus;
use crate::modules::decoder::binance::order_book::BinanceOrderBookDecoder;
use crate::modules::decoder::bitget::order_book::BitgetOrderBookDecoder;
use crate::modules::decoder::bybit::order_book::BybitOrderBookDecoder;
use crate::modules::decoder::gate::order_book::GateOrderBookDecoder;
use crate::modules::decoder::htx::order_book::HtxOrderBookDecoder;
use crate::modules::decoder::okx::order_book::OkxOrderBookDecoder;
use crate::modules::stream::base_order_book::BaseOrderBookStream;
use crate::modules::stream::binance::order_book::BinanceOrderBookStream;
use crate::modules::stream::bitget::order_book::BitgetOrderBookStream;
use crate::modules::stream::bybit::order_book::BybitOrderBookStream;
use crate::modules::stream::gate::order_book::GateOrderBookStream;
use crate::modules::stream::htx::order_book::HtxOrderBookStream;
use crate::modules::stream::okx::order_book::OkxOrderBookStream;
use crate::modules::stream::order_book::OrderBookStream;
use std::sync::Arc;

pub struct OrderBookStreamManager {
    pub bybit: BaseOrderBookStream<BybitOrderBookDecoder>,
    pub gate: BaseOrderBookStream<GateOrderBookDecoder>,
    pub bitget: BaseOrderBookStream<BitgetOrderBookDecoder>,
    pub binance: BaseOrderBookStream<BinanceOrderBookDecoder>,
    pub htx: BaseOrderBookStream<HtxOrderBookDecoder>,
    pub okx: BaseOrderBookStream<OkxOrderBookDecoder>,
    // pub mexc: BaseOrderBookStream<MexcBookTickerDecoder>,
}

impl OrderBookStreamManager {
    pub fn new(order_book_bus: Arc<OrderBookBus>) -> Self {
        Self {
            bybit: BybitOrderBookStream::new(Arc::clone(&order_book_bus)),
            gate: GateOrderBookStream::new(Arc::clone(&order_book_bus)),
            bitget: BitgetOrderBookStream::new(Arc::clone(&order_book_bus)),
            binance: BinanceOrderBookStream::new(Arc::clone(&order_book_bus)),
            htx: HtxOrderBookStream::new(Arc::clone(&order_book_bus)),
            okx: OkxOrderBookStream::new(Arc::clone(&order_book_bus)),
            // mexc: MexcOrderBookStream::new(Arc::clone(&order_book_bus)),
        }
    }
}
