use crate::kernel::dto::server::ServerState;
use crate::server::handlers::sse_signal::sse_signal_handler;
use axum::Router;
use axum::routing::get;

pub fn build_router(state: ServerState) -> Router {
    let signals = Router::new()
        .route("/signals", get(sse_signal_handler))
        .with_state(state);

    Router::new().nest("/api/v1", signals)
}
