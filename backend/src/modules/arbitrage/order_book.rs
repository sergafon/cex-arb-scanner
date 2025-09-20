use crate::kernel::dto::order_book::OrderBookDto;
use crate::kernel::dto::signal::SignalDto;
use crate::kernel::enums::exchange::Symbol;
use crate::kernel::utils::time::get_now_timestamp;
use crate::modules::arbitrage::utils::{percent, taker_fee};
use crate::modules::bus::order_book::OrderBookBus;
use crate::modules::bus::signal::SignalBus;
use anyhow::Result;
use rust_decimal::Decimal;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::watch::Receiver;
use tokio::time::{interval, MissedTickBehavior};

pub struct OrderBookArbitrage {
    tick_secs: u64,
    order_book_bus: Arc<OrderBookBus>,
    signal_bus: Arc<SignalBus>,
}

impl OrderBookArbitrage {
    pub fn new(
        tick_secs: u64,
        order_book_bus: Arc<OrderBookBus>,
        signal_bus: Arc<SignalBus>,
    ) -> Self {
        Self { tick_secs, order_book_bus, signal_bus }
    }

    pub async fn run_symbol_task(&self, symbol: Symbol) -> Result<()> {
        let mut receivers = self.order_book_bus.subscribe(symbol).await?;
        let sender = Arc::clone(&self.signal_bus);
        let tick_secs = self.tick_secs;

        tokio::spawn(async move {
            let mut tick = interval(Duration::from_secs(tick_secs));
            tick.set_missed_tick_behavior(MissedTickBehavior::Skip);

            loop {
                tick.tick().await;

                let books = Self::prepare_books(&mut receivers);

                let signal = match Self::compare(symbol, &books) {
                    Some(signal) => signal,
                    None => {
                        // tracing::error!("can't compare books for symbol {symbol}");

                        continue;
                    }
                };

                sender.publish(symbol, signal);
            }
        });

        Ok(())
    }

    #[inline]
    fn compare(symbol: Symbol, books: &[Arc<OrderBookDto>]) -> Option<Arc<SignalDto>> {
        let mut best_buy = None::<(Arc<OrderBookDto>, Decimal)>;
        let mut best_sell = None::<(Arc<OrderBookDto>, Decimal)>;

        for book in books.iter() {
            let fee = taker_fee(&book.exchange);
            let ask_eff = book.ask_price * (Decimal::ONE + fee);

            if best_buy.as_ref().map(|(_, a)| ask_eff < *a).unwrap_or(true) {
                best_buy = Some((Arc::clone(book), ask_eff));
            }
        }

        let (buy, buy_eff) = best_buy?;

        for book in books.iter() {
            if book.exchange == buy.exchange {
                continue;
            }

            let fee = taker_fee(&book.exchange);
            let bid_eff = book.bid_price * (Decimal::ONE - fee);

            if best_sell
                .as_ref()
                .map(|(_, b)| bid_eff > *b)
                .unwrap_or(true)
            {
                best_sell = Some((Arc::clone(book), bid_eff));
            }
        }

        let (sell, sell_eff) = best_sell?;

        // if sell_eff <= buy_eff {
        //     continue;
        // }

        let volume = buy.ask_amount.min(sell.bid_amount);

        // let notional = volume * buy.ask_price;

        // if notional < min_notional {
        //     continue;
        // }

        let gross = (sell.bid_price - buy.ask_price) / buy.ask_price;

        if gross <= Decimal::ZERO {
            return None;
        }

        let net = (sell_eff - buy_eff) / buy_eff;

        Some(Arc::new(SignalDto {
            symbol,
            buy_exchange: buy.exchange,
            sell_exchange: sell.exchange,
            buy_price: buy.ask_price,
            sell_price: sell.bid_price,
            volume,
            gross_percent: percent(gross),
            net_percent: percent(net),
            buy_link: buy.pair_link.to_string(),
            sell_link: sell.pair_link.to_string(),
        }))
    }

    #[inline]
    fn prepare_books(receiver: &mut [Receiver<Arc<OrderBookDto>>]) -> Vec<Arc<OrderBookDto>> {
        let mut books = Vec::with_capacity(receiver.len());

        let timestamp = get_now_timestamp();
        let staleness = Duration::from_secs(3);

        for rx in receiver.iter_mut() {
            let book = rx.borrow();

            if timestamp.saturating_sub(book.timestamp) > staleness.as_millis() as u64 {
                continue;
            }

            books.push(Arc::clone(&*book));
        }

        books
    }
}
