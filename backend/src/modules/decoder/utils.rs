use crate::kernel::enums::exchange::Symbol;
use anyhow::{anyhow, bail, Result};
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::sync::LazyLock;
use strum::{EnumCount, IntoEnumIterator};

#[inline]
pub fn get_ask<const N: usize>(asks: &[[Decimal; N]]) -> Result<(Decimal, Decimal)> {
    asks.first()
        .map(|lvl| (lvl[0], lvl[1]))
        .ok_or_else(|| anyhow!("Failed to get ask"))
}

#[inline]
pub fn get_bid<const N: usize>(bids: &[[Decimal; N]]) -> Result<(Decimal, Decimal)> {
    bids.first()
        .map(|lvl| (lvl[0], lvl[1]))
        .ok_or_else(|| anyhow!("Failed to get bid"))
}

static SYM_MAP: LazyLock<HashMap<String, Symbol>> = LazyLock::new(|| {
    let mut m = HashMap::with_capacity(Symbol::COUNT);

    for sym in Symbol::iter() {
        m.insert(sym.as_ref().to_string(), sym);
    }

    m
});

static SYM_PREFIX_DESC: LazyLock<Vec<(String, Symbol)>> = LazyLock::new(|| {
    let mut v: Vec<(String, Symbol)> = Symbol::iter()
        .map(|sym| (sym.as_ref().to_string(), sym))
        .collect();

    v.sort_unstable_by(|(a, _), (b, _)| b.len().cmp(&a.len()));
    v
});

const QUOTE_SUFFIXES: &[&str] = &["USDT"];

#[inline]
pub fn normalize_symbol(s: &str) -> Result<Symbol> {
    let s = s.trim().to_ascii_uppercase();

    if let Some(&sym) = SYM_MAP.get(&s) {
        return Ok(sym);
    }

    for q in QUOTE_SUFFIXES {
        if let Some(base) = s.strip_suffix(q) {
            if let Some(&sym) = SYM_MAP.get(base) {
                return Ok(sym);
            }
        }
    }

    for (pat, sym) in SYM_PREFIX_DESC.iter() {
        if s.starts_with(pat) {
            return Ok(*sym);
        }
    }

    bail!("Symbol not found in symbols list, error input: {s}");
}
