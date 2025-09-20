use crate::modules::bus::order_book::OrderBookBus;
use crate::modules::decoder::order_book::OrderBookDecoder;
use anyhow::Result;
use flate2::read::GzDecoder;
use futures::{SinkExt, StreamExt};
use serde_json::{json, Value};
use std::io::Read;
use std::sync::Arc;
use tokio_tungstenite::connect_async;
use tungstenite::Message;

pub struct BaseOrderBookStream<D>
where
    D: OrderBookDecoder,
{
    pub url: String,
    pub subscribe_message: String,
    pub order_book_bus: Arc<OrderBookBus>,
    pub decoder: D,
}

impl<D> BaseOrderBookStream<D>
where
    D: OrderBookDecoder,
{
    pub async fn run(&self) -> Result<()> {
        loop {
            let (ws_stream, _) = connect_async(&self.url).await?;

            let (mut write, mut read) = ws_stream.split();

            if self.subscribe_message.trim_start().starts_with('[') {
                let subs: Vec<Value> = serde_json::from_str(&self.subscribe_message)?;
                for sub in subs {
                    write.send(Message::Text(sub.to_string().into())).await?;
                }
            } else {
                write
                    .send(Message::Text(self.subscribe_message.clone().into()))
                    .await?;
            }

            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(bytes)) => {
                        if let Err(error) = self.handle_text(&mut write, bytes.as_ref()).await {
                            tracing::warn!(
                                "{} handle_text decode failed: {error}",
                                std::any::type_name::<D>(),
                            );
                            continue;
                        }
                    }
                    Ok(Message::Binary(bytes)) => {
                        let mut d = GzDecoder::new(&bytes[..]);
                        let mut s = String::new();

                        if let Err(error) = d.read_to_string(&mut s) {
                            tracing::warn!(
                                "{} GZIP decode failed: {error}",
                                std::any::type_name::<D>(),
                            );
                            continue;
                        }

                        if let Err(error) = self.handle_text(&mut write, &s).await {
                            tracing::warn!(
                                "{} handle_text decode failed: {error}",
                                std::any::type_name::<D>(),
                            );
                            continue;
                        }
                    }
                    Ok(Message::Ping(payload)) => {
                        if let Err(error) = write.send(Message::Pong(payload)).await {
                            tracing::warn!(
                                "{} Pong send failed: {error}. Reconnecting…",
                                std::any::type_name::<D>(),
                            );
                            break;
                        }
                    }
                    Ok(Message::Close(frame)) => {
                        tracing::warn!(
                            "{} WS closed by server: {frame:?}. Reconnecting…",
                            std::any::type_name::<D>(),
                        );
                        break;
                    }
                    Err(error) => {
                        tracing::warn!(
                            "{} WS read error: {error}. Reconnecting…",
                            std::any::type_name::<D>(),
                        );
                        break;
                    }
                    _ => {}
                }
            }
        }
    }

    async fn handle_text<S>(&self, write: &mut S, s: &str) -> Result<(), tungstenite::Error>
    where
        S: futures::Sink<Message, Error = tungstenite::Error> + Unpin,
    {
        if let Ok(val) = serde_json::from_str::<Value>(s) {
            if let Some(ts) = val.get("ping") {
                let pong = json!({ "pong": ts });

                write.send(Message::Text(pong.to_string().into())).await?;

                return Ok(());
            }
        }

        // let start = Instant::now();

        let parsed = match self.decoder.decode(s.as_bytes()) {
            Ok(parsed) => parsed,
            Err(error) => {
                tracing::error!(
                    "{} decode error: {error}\nmessage: {}",
                    std::any::type_name::<D>(),
                    String::from_utf8_lossy(s.as_bytes())
                );

                return Ok(());
            }
        };

        // println!(
        //     "{:?} parsed order book {} µs",
        //     parsed.exchange,
        //     start.elapsed().as_nanos() as f64 / 1_000.0
        // );
        //
        // let start = Instant::now();

        if let Err(error) = self.order_book_bus.publish(parsed).await {
            tracing::error!("Publish error: {error}");
        };

        // println!(
        //     "{:?} insert order book {} µs",
        //     parsed.exchange,
        //     start.elapsed().as_nanos() as f64 / 1_000.0
        // );

        Ok(())
    }
}
