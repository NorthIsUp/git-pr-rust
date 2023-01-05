pub mod args;
pub mod cli;
mod git_commands;
mod prinfo;
mod shell;

use simple_logger::SimpleLogger;

#[tokio::main]

async fn main() {
    let logger = SimpleLogger::new().init().unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    std::process::exit(match cli::main().await {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("error: {:?}", err);
            1
        }
    });
}
