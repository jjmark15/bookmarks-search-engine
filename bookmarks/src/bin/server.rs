use bookmarks::{App, AppError};
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let args: Vec<String> = std::env::args().collect();
    let search_engine_config_path = PathBuf::from(&args[1]);

    App::new(search_engine_config_path).run().await
}
