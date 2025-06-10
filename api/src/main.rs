use shared::config::AppConfig;
use std::path::Path;
use shared::logging::init_logging;
use tracing::info;
fn main() -> anyhow::Result<()>{
    init_logging();
    //can change environment based on context
    let config_path = Path::new("config/dev.toml");
    let config = AppConfig::from_file(config_path)?;
    //println!() -> info!()
    info!("config loaded : {:?}", config);
    Ok(())
}

//test - logging
//RUST_LOG=info cargo run -p api
//APP_ENV=production RUST_LOG=info cargo run -p api