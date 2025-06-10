pub mod router;

use std::net::SocketAddr;
use shared::config::AppConfig;
use std::path::Path;
use shared::logging::init_logging;
use tracing::info;

use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    init_logging();
    //can change environment based on context
    let config_path = Path::new("config/dev.toml");
    let config = AppConfig::from_file(config_path)?;
    //println!() -> info!()
    info!("config loaded : {:?}", config);
    
    let app = router::build_router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("starting server at http://{}", addr);

    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    info!("server stopped");
    Ok(())
}

//test - logging
//RUST_LOG=info cargo run -p api
//APP_ENV=production RUST_LOG=info cargo run -p api