use bookmarks::App;
use std::path::PathBuf;
use std::process::exit;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args: Vec<String> = std::env::args().collect();
    let search_engine_config_path = PathBuf::from(&args[1]);

    if let Err(err) = App::new(search_engine_config_path).run().await {
        eprintln!("{}", err);
        exit(1)
    }
}
