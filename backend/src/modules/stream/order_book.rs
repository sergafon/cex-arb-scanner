use crate::kernel::enums::decoder::ActionDecoder;
use crate::kernel::enums::exchange::Exchange;
use crate::kernel::utils::time::get_now_timestamp;
use crate::modules::broker::order_book::OrderBookBroker;
use crate::modules::decoder::order_book::OrderBookDecoder;
use anyhow::Result;
use async_trait::async_trait;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::{Instant, MissedTickBehavior, interval, sleep};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};
use tungstenite::Message;

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub const COUNT_PER_SUBSCRIBE: usize = 10;
pub const SLEEP_DURATION: Duration = Duration::from_millis(500);
pub const PING_DURATION: Duration = Duration::from_secs(15);
pub const PONG_DURATION: Duration = Duration::from_secs(20);

#[async_trait]
pub trait OrderBookStream {
    const EXCHANGE: Exchange;

    fn get_ws_url() -> String;

    async fn get_batches() -> Vec<Value>;

    #[inline]
    async fn subscribe(
        batch: &Value,
    ) -> Result<(SplitSink<WsStream, Message>, SplitStream<WsStream>)> {
        let (ws_stream, _) = connect_async(Self::get_ws_url()).await?;
        let (mut write, read) = ws_stream.split();

        let batches: &[Value] = batch
            .get("batches")
            .and_then(Value::as_array)
            .map(|a| a.as_slice())
            .unwrap_or(std::slice::from_ref(batch));

        for sub in batches {
            let msg = serde_json::to_string(sub)?;
            write.send(Message::Text(msg.into())).await?;

            sleep(SLEEP_DURATION).await;
        }

        Ok((write, read))
    }

    async fn run<D>(&self, decoder: Arc<D>, broker: Arc<OrderBookBroker>)
    where
        Self: Send + Sync + 'static,
        D: OrderBookDecoder,
    {
        for batch in Self::get_batches().await {
            tokio::spawn(Self::execute(
                batch,
                Arc::clone(&decoder),
                Arc::clone(&broker),
            ));

            sleep(SLEEP_DURATION).await;
        }
    }

    #[inline]
    async fn execute<D>(batch: Value, decoder: Arc<D>, broker: Arc<OrderBookBroker>)
    where
        D: OrderBookDecoder,
    {
        loop {
            let (mut write, mut read) = match Self::subscribe(&batch).await {
                Ok(subscribe) => subscribe,
                Err(error) => {
                    tracing::error!(error=%error, batch=%batch, exchange=?Self::EXCHANGE, "Subscribe failed, reconnection...");
                    sleep(SLEEP_DURATION).await;
                    break;
                }
            };

            let mut ping = interval(PING_DURATION);
            ping.set_missed_tick_behavior(MissedTickBehavior::Skip);
            let mut last_message = Instant::now();

            let mut decode_failed_count = 0;

            loop {
                tokio::select! {
                    biased;
                    msg = read.next() => match msg {
                        Some(Ok(message @ (Message::Text(_) | Message::Binary(_)))) => {
                            let bytes = match &message {
                                Message::Text(bytes) => bytes.as_ref(),
                                Message::Binary(bytes) => bytes.as_ref(),
                                _ => {
                                    tracing::error!(%message, exchange=?Self::EXCHANGE, "Invalid extract_bytes");
                                    continue;
                                }
                            };

                            match decoder.decode(bytes) {
                                Ok(ActionDecoder::Publish(dto)) => {
                                    if let Err(error) = broker.publish(dto).await {
                                        tracing::error!(%error, exchange=?Self::EXCHANGE, "Error send to broker");
                                    };
                                }
                                Ok(ActionDecoder::Skip) => {}
                                Ok(ActionDecoder::Pong(ts)) => {
                                    if Self::EXCHANGE == Exchange::Htx {
                                        let message = json!({"pong": ts}).to_string();

                                        if let Err (error) = write.send(Message::Text(message.into())).await {
                                            tracing::error!(error=%error, exchange=?Self::EXCHANGE, "Error sending pong, reconnecting...");
                                            break;
                                        }
                                    }
                                }
                                Err(error) => {
                                    tracing::error!(bytes=%String::from_utf8_lossy(bytes), exchange=?Self::EXCHANGE, "Decode failed bytes");
                                    tracing::error!(%error, exchange=?Self::EXCHANGE, "Decode failed");

                                    if decode_failed_count > 30 {
                                        tracing::error!(exchange=?Self::EXCHANGE, "Decode failed more >30 times, reconnection...");
                                        break;
                                    }

                                    decode_failed_count += 1;
                                }
                            }

                            last_message = Instant::now();
                        }
                        Some(Ok(Message::Ping(payload))) => {
                            if let Err (error) = write.send(Message::Pong(payload)).await {
                                tracing::warn!(error=%error, exchange=?Self::EXCHANGE, "Ping error, reconnecting...");
                                break;
                            }

                            last_message = Instant::now();
                        }
                        Some(Ok(Message::Frame(_))) | Some(Ok(Message::Pong(_))) => {}
                        Some(Ok(Message::Close(frame))) => {
                            tracing::warn!(
                                ?frame,
                                %batch,
                                exchange=?Self::EXCHANGE,
                                "Server connection closed; reconnecting..."
                            );
                            break;
                        }
                        Some(Err(error)) => {
                            tracing::error!(%error, %batch, exchange=?Self::EXCHANGE, "Error receiving message, reconnecting...");
                            break;
                        },
                        None => break,
                    },

                    _ = ping.tick() => {
                        if let Some(msg) = match Self::EXCHANGE {
                            Exchange::Okx | Exchange::Bitget => Some("ping".to_string()),
                            Exchange::Gate => Some(json!({"time": get_now_timestamp(),"channel": "spot.ping"}).to_string()),
                            Exchange::Bybit => Some(json!({"req_id": get_now_timestamp(), "op": "ping"}).to_string()),
                            _ => None,
                        } {
                            if let Err(error) = write.send(Message::Text(msg.into())).await {
                                tracing::error!(%error, exchange=?Self::EXCHANGE, "Ping error, reconnecting...");
                                break;
                            }
                        }

                        if last_message.elapsed() > PONG_DURATION {
                            tracing::warn!(exchange=?Self::EXCHANGE, "No message for >{PONG_DURATION:?}, reconnecting...");
                            break;
                        }
                    }
                }
            }

            sleep(SLEEP_DURATION).await;
        }
    }
}
