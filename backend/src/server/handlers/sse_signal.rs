use crate::kernel::dto::server::ServerState;
use crate::modules::broker::signal::SignalBroker;
use axum::extract::State;
use axum::response::sse::{Event, KeepAlive, Sse};
use futures_util::stream::{Stream, unfold};
use rust_decimal::Decimal;
use std::cmp::Ordering;
use std::convert::Infallible;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{MissedTickBehavior, interval};

const KEEPALIVE_INTERVAL: u64 = 15;

type StreamItem = Result<Event, Infallible>;
type DynStream = Pin<Box<dyn Stream<Item = StreamItem> + Send + 'static>>;

pub struct SseSignalHandler {
    tick_secs: u64,
    signal_broker: Arc<SignalBroker>,
}

impl SseSignalHandler {
    pub fn new(tick_secs: u64, signal_broker: Arc<SignalBroker>) -> Self {
        Self { tick_secs, signal_broker }
    }

    pub async fn signals(&self) -> Sse<DynStream> {
        let receivers = self.signal_broker.get_receivers();
        let mut tick = interval(Duration::from_secs(self.tick_secs));
        tick.set_missed_tick_behavior(MissedTickBehavior::Skip);

        let stream = unfold((receivers, tick), |(mut receivers, mut tick)| async move {
            loop {
                tick.tick().await;

                let mut batch = Vec::new();

                for rx in receivers.iter_mut() {
                    match rx.has_changed() {
                        Ok(true) => {
                            let v = rx.borrow_and_update().clone();
                            batch.push(v);
                        }
                        Ok(false) => continue,
                        Err(error) => {
                            tracing::error!("Sse signal handler error: {}", error);
                            continue;
                        }
                    }
                }

                if !batch.is_empty() {
                    batch.sort_by(|a, b| {
                        let a_profitable = a.net_percent > Decimal::ZERO;
                        let b_profitable = b.net_percent > Decimal::ZERO;

                        match (a_profitable, b_profitable) {
                            (true, false) => Ordering::Less,
                            (false, true) => Ordering::Greater,
                            _ => Ordering::Equal,
                        }
                    });

                    let batch: Vec<_> = batch.iter().map(|arc| &**arc).collect::<Vec<_>>();

                    let payload =
                        serde_json::to_string(&batch).unwrap_or_else(|_| "[]".to_string());

                    let evt = Ok(Event::default().data(payload));

                    return Some((evt, (receivers, tick)));
                }
            }
        });

        let boxed: DynStream = Box::pin(stream);

        Sse::new(boxed)
            .keep_alive(KeepAlive::new().interval(Duration::from_secs(KEEPALIVE_INTERVAL)))
    }
}

pub async fn sse_signal_handler(
    State(state): State<ServerState>,
) -> Sse<impl Stream<Item = anyhow::Result<Event, Infallible>>> {
    state.sse_signal_handler.signals().await
}
