use crate::kernel::enums::exchange::Exchange;
use anyhow::{Error, Result, anyhow};
use serde_json::Value;

pub trait SupportedPair {
    const EXCHANGE: Exchange;

    fn get_rest_url() -> String;

    fn filter(value: &Value) -> bool;

    async fn get_data(url: String) -> Result<Value> {
        let client = reqwest::Client::builder()
            // .timeout(Duration::from_secs(5))
            .build()?;

        let result: Value = client
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        Ok(result)
    }

    async fn get_tickers(pointer: &str, base_key: &str) -> Vec<String> {
        let result = async {
            let url = Self::get_rest_url();
            let result = Self::get_data(url).await?;

            if Self::EXCHANGE == Exchange::Binance {
                // tracing::debug!("`result` = {:#?}", &result);
            }

            let data = result
                .pointer(pointer)
                .and_then(Value::as_array)
                .ok_or_else(|| {
                    anyhow!(
                        "{:?} pointer {pointer} is missing or not an array",
                        Self::EXCHANGE
                    )
                })?;

            let out: Vec<String> = data
                .iter()
                .filter(|v| Self::filter(v))
                .filter_map(|it| it.get(base_key).and_then(Value::as_str))
                .map(|s| s.to_string())
                .collect();

            Ok(out)
        }
        .await;

        result.unwrap_or_else(|error: Error| {
            tracing::error!(%error, exchange=?Self::EXCHANGE);
            Vec::new()
        })
    }
}
