use anyhow::Result;
use arbitrage_scanner::App;

#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<()> {
    App::run().await
}
