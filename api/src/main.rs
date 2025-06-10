use shared::config::AppConfig;
use std::path::Path;
fn main() -> anyhow::Result<()>{
    //can change environment based on context
    let config_path = Path::new("config/dev.toml");
    let config = AppConfig::from_file(config_path)?;
    
    println!("config loaded : {:?}", config);
    Ok(())
}