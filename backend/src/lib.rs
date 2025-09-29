mod kernel;
mod modules;
pub mod server;
mod services;

use crate::modules::broker::order_book::OrderBookBroker;
use crate::modules::broker::signal::SignalBroker;
use crate::modules::stream::manager::OrderBookStreamManager;
use crate::services::server::ServerService;
use crate::services::signal::SignalService;
use crate::services::stream::StreamService;
use anyhow::Result;
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

const STREAM_INTERVAL: u64 = 5;

pub struct App;

impl App {
    pub async fn run() -> Result<()> {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .with_target(false)
            .with_file(true)
            .with_line_number(true)
            .init();

        let order_book_broker = Arc::new(OrderBookBroker::default());
        let signal_broker = Arc::new(SignalBroker::default());
        let stream = Arc::new(OrderBookStreamManager::new(Arc::clone(&order_book_broker)));
        let order_book_service = StreamService::new(stream);
        let signal_service = SignalService::new(
            STREAM_INTERVAL,
            order_book_broker,
            Arc::clone(&signal_broker),
        );
        let server_service = ServerService::new(STREAM_INTERVAL, signal_broker);

        if let Err(error) = tokio::try_join!(
            order_book_service.run(),
            signal_service.run(),
            server_service.run()
        ) {
            tracing::error!("App run error: {error}");
        };

        Ok(())
    }
}
