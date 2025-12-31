use anyhow::Result;
use yaru::run;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
