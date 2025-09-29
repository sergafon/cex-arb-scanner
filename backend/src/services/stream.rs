use crate::modules::decoder::binance::order_book::BinanceOrderBookDecoder;
use crate::modules::decoder::bitget::order_book::BitgetOrderBookDecoder;
use crate::modules::decoder::bybit::order_book::BybitOrderBookDecoder;
use crate::modules::decoder::gate::order_book::GateOrderBookDecoder;
use crate::modules::decoder::htx::order_book::HtxOrderBookDecoder;
use crate::modules::decoder::okx::order_book::OkxOrderBookDecoder;
use crate::modules::stream::manager::OrderBookStreamManager;
use crate::modules::stream::order_book::OrderBookStream;
use anyhow::Result;
use std::sync::Arc;

pub struct StreamService {
    stream: Arc<OrderBookStreamManager>,
}

impl StreamService {
    pub fn new(stream: Arc<OrderBookStreamManager>) -> Self {
        Self { stream }
    }

    pub async fn run(&self) -> Result<()> {
        tokio::join!(
            self.stream.htx.run(
                Arc::new(HtxOrderBookDecoder),
                Arc::clone(&self.stream.broker)
            ),
            self.stream.gate.run(
                Arc::new(GateOrderBookDecoder),
                Arc::clone(&self.stream.broker)
            ),
            self.stream.okx.run(
                Arc::new(OkxOrderBookDecoder),
                Arc::clone(&self.stream.broker)
            ),
            self.stream.bybit.run(
                Arc::new(BybitOrderBookDecoder),
                Arc::clone(&self.stream.broker)
            ),
            self.stream.bitget.run(
                Arc::new(BitgetOrderBookDecoder),
                Arc::clone(&self.stream.broker)
            ),
            self.stream.binance.run(
                Arc::new(BinanceOrderBookDecoder),
                Arc::clone(&self.stream.broker)
            ),
            // self.stream.mexc.run(Arc::clone(&self.stream.broker)),
        );

        Ok(())
    }
}
