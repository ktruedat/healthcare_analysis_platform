mod cfg;
mod db;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = cfg::parse::parse_yaml_config("config.yaml")?;

    let db = db::surreal::connect(&config).await?;

    Ok(())
}
