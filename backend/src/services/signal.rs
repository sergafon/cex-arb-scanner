use crate::kernel::enums::exchange::Symbol;
use crate::modules::arbitrage::order_book::OrderBookArbitrage;
use crate::modules::broker::order_book::OrderBookBroker;
use crate::modules::broker::signal::SignalBroker;
use anyhow::Result;
use std::sync::Arc;
use strum::IntoEnumIterator;

pub struct SignalService {
    tick_secs: u64,
    order_book_broker: Arc<OrderBookBroker>,
    signal_broker: Arc<SignalBroker>,
}

impl SignalService {
    pub fn new(
        tick_secs: u64,
        order_book_broker: Arc<OrderBookBroker>,
        signal_broker: Arc<SignalBroker>,
    ) -> Self {
        Self { tick_secs, order_book_broker, signal_broker }
    }

    pub async fn run(&self) -> Result<()> {
        for sym in Symbol::iter() {
            let arbitrage = OrderBookArbitrage::new(
                self.tick_secs,
                Arc::clone(&self.order_book_broker),
                Arc::clone(&self.signal_broker),
            );

            if let Err(error) = arbitrage.run_symbol_task(sym).await {
                tracing::error!("Run arbitrage compare {:?}", error);
                continue;
            };
        }

        Ok(())
    }
}
