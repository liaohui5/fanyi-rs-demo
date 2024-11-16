use fanyi::{action, cli, config};
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let args_config = cli::ArgsConfig::new(&args);
    let config = config::Config::new();
    action::Action::new(config).run(args_config).await
}
