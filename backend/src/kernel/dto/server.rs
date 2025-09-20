use crate::server::handlers::sse_signal::SseSignalHandler;
use std::sync::Arc;

#[derive(Clone)]
pub struct ServerState {
    pub sse_signal_handler: Arc<SseSignalHandler>,
}
