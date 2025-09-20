use crate::modules::stream::manager::OrderBookStreamManager;
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
        if let Err(error) = tokio::try_join!(
            self.stream.bybit.run(),
            self.stream.gate.run(),
            self.stream.bitget.run(),
            self.stream.binance.run(),
            self.stream.htx.run(),
            self.stream.okx.run(),
            // self.stream.mexc.run(),
        ) {
            tracing::error!("Run stream {:?}", error);
        };

        Ok(())
    }
}
