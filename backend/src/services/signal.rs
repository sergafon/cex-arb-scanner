use crate::kernel::enums::exchange::Symbol;
use crate::modules::arbitrage::order_book::OrderBookArbitrage;
use crate::modules::bus::order_book::OrderBookBus;
use crate::modules::bus::signal::SignalBus;
use anyhow::Result;
use std::sync::Arc;
use strum::IntoEnumIterator;

pub struct SignalService {
    tick_secs: u64,
    order_book_bus: Arc<OrderBookBus>,
    signal_bus: Arc<SignalBus>,
}

impl SignalService {
    pub fn new(
        tick_secs: u64,
        order_book_bus: Arc<OrderBookBus>,
        signal_bus: Arc<SignalBus>,
    ) -> Self {
        Self { tick_secs, order_book_bus, signal_bus }
    }

    pub async fn run(&self) -> Result<()> {
        for sym in Symbol::iter() {
            let arbitrage = OrderBookArbitrage::new(
                self.tick_secs,
                Arc::clone(&self.order_book_bus),
                Arc::clone(&self.signal_bus),
            );

            if let Err(error) = arbitrage.run_symbol_task(sym).await {
                tracing::error!("Run arbitrage compare {:?}", error);
                continue;
            };
        }

        Ok(())
    }
}
