use crate::kernel::dto::signal::SignalDto;
use crate::kernel::enums::exchange::Symbol;
use std::sync::Arc;
use strum::{EnumCount, IntoEnumIterator};
use tokio::sync::watch::{self, Receiver, Sender};

pub struct SignalBroker {
    senders: [Sender<Arc<SignalDto>>; Symbol::COUNT],
    receivers: [Receiver<Arc<SignalDto>>; Symbol::COUNT],
}

impl Default for SignalBroker {
    fn default() -> Self {
        Self::new()
    }
}

impl SignalBroker {
    pub fn new() -> Self {
        let mut senders = Vec::<Sender<Arc<SignalDto>>>::with_capacity(Symbol::COUNT);
        let mut receivers = Vec::<Receiver<Arc<SignalDto>>>::with_capacity(Symbol::COUNT);

        for _ in Symbol::iter() {
            let (transmitter, receiver) = watch::channel::<Arc<SignalDto>>(Default::default());

            senders.push(transmitter);
            receivers.push(receiver);
        }

        Self { senders: senders.try_into().unwrap(), receivers: receivers.try_into().unwrap() }
    }

    pub fn get_receivers(&self) -> [Receiver<Arc<SignalDto>>; Symbol::COUNT] {
        self.receivers.clone()
    }

    #[inline]
    pub fn subscribe(&self, sym: Symbol) -> Receiver<Arc<SignalDto>> {
        self.receivers[sym as usize].clone()
    }

    #[inline]
    pub fn publish(&self, sym: Symbol, event: Arc<SignalDto>) {
        let _ = self.senders[sym as usize].send_replace(event);
    }
}
