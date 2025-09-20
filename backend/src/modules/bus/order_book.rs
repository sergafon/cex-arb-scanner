use crate::kernel::dto::order_book::OrderBookDto;
use crate::kernel::enums::exchange::{Exchange, Symbol};
use anyhow::{anyhow, bail, Result};
use std::collections::HashMap;
use std::sync::Arc;
use strum::IntoEnumIterator;
use tokio::sync::watch::{self, Receiver, Sender};

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
struct Key {
    exchange: Exchange,
    symbol: Symbol,
}

#[derive(Debug)]
pub struct OrderBookBus {
    senders: HashMap<Key, Sender<Arc<OrderBookDto>>>,
    receivers: HashMap<Symbol, Vec<Receiver<Arc<OrderBookDto>>>>,
}

impl Default for OrderBookBus {
    fn default() -> Self {
        Self::new()
    }
}

impl OrderBookBus {
    pub fn new() -> Self {
        let mut senders = HashMap::new();
        let mut receivers: HashMap<_, Vec<_>> = HashMap::new();

        for exchange in Exchange::iter() {
            for symbol in Symbol::iter() {
                let (transmitter, receiver) =
                    watch::channel::<Arc<OrderBookDto>>(Default::default());

                senders.insert(Key { symbol, exchange }, transmitter);
                receivers.entry(symbol).or_default().push(receiver);
            }
        }

        Self { senders, receivers }
    }

    pub async fn subscribe(&self, symbol: Symbol) -> Result<Vec<Receiver<Arc<OrderBookDto>>>> {
        match self.receivers.get(&symbol) {
            Some(receiver) => Ok(receiver.clone()),
            None => bail!("Receiver for {symbol:#?} not found"),
        }
    }

    #[inline]
    pub async fn publish(&self, ob: Arc<OrderBookDto>) -> Result<()> {
        let key = Key { exchange: ob.exchange, symbol: ob.symbol };

        self.senders
            .get(&key)
            .ok_or_else(|| anyhow!("Sender for {key:#?} not found"))?
            .send_replace(ob);

        Ok(())
    }
}
