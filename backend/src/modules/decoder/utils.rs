use crate::kernel::enums::exchange::Symbol;
use anyhow::{Result, anyhow, bail};
use rust_decimal::Decimal;
use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::LazyLock;
use strum::{EnumCount, IntoEnumIterator};

#[inline]
pub fn get_price_and_size(side: &Value) -> Result<(Decimal, Decimal)> {
    let row = side
        .as_array()
        .and_then(|a| a.first())
        .and_then(Value::as_array)
        .ok_or_else(|| anyhow!("Row missing or not array"))?;

    let price = row
        .first()
        .ok_or_else(|| anyhow!("[0][0] price missing"))
        .and_then(val_to_dec)?;

    let size = row
        .get(1)
        .ok_or_else(|| anyhow!("[0][1] size missing"))
        .and_then(val_to_dec)?;

    Ok((price, size))
}

#[inline]
pub fn val_to_dec(v: &Value) -> Result<Decimal> {
    match v {
        Value::String(s) => {
            Decimal::from_str(s).map_err(|e| anyhow!("decimal from str `{s}`: {e}"))
        }
        Value::Number(n) => {
            Decimal::from_str(&n.to_string()).map_err(|e| anyhow!("decimal from num `{n}`: {e}"))
        }
        _ => Err(anyhow!("expected string/number, got: {v:?}")),
    }
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
