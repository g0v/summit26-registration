#[tokio::main]
async fn main() -> anyhow::Result<()> {
    summit26_registration::server::run().await
}
