use crate::kernel::dto::server::ServerState;
use crate::modules::bus::signal::SignalBus;
use crate::server::handlers::sse_signal::SseSignalHandler;
use crate::server::router::build_router;
use anyhow::Result;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct ServerService {
    sse_signal_handler: Arc<SseSignalHandler>,
}

impl ServerService {
    pub fn new(tick_secs: u64, signal_bus: Arc<SignalBus>) -> Self {
        let sse_signal_handler =
            Arc::new(SseSignalHandler::new(tick_secs, Arc::clone(&signal_bus)));

        Self { sse_signal_handler }
    }

    pub async fn run(&self) -> Result<()> {
        let state = ServerState { sse_signal_handler: Arc::clone(&self.sse_signal_handler) };
        let router = build_router(state);
        let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
        let listener = TcpListener::bind(addr).await?;

        tracing::info!("Server start http://{addr}");

        axum::serve(listener, router.into_make_service()).await?;

        Ok(())
    }
}
