// WSL2/Ubuntu users: Make sure that you have pkg-config and libssl-dev installed!
use exchange::producer::push_to_kafka;
use exchange::consumer::read_from_kafka;
use exchange::{push_data, request_data};
use exchange::cli::{Cli, Command};


use clap::*;
use log::{info, warn};

#[tokio::main]
async fn main() {

    let args = Cli::parse();

    // set_term_width(0) in clap
    // https://docs.rs/clap/2.33.3/clap/struct.App.html#method.set_term_width

    

    // TODO: Add logging
    // TODO: Add parsing of additional CLI arguments
    match args.subcommand {
        Command::Produce => {
            info!("Producer selected");
            push_to_kafka("test", true).await;
        },
        Command::Consume => {
            info!("Consumer selected");
            read_from_kafka("test", true).await;
        },
        Command::Write => {
            println!("Writer selected");
            push_data().await;
        },
        Command::Read => {
            println!("Reader selected");
            request_data("1", "2", "3").await;  
        }
    }
}
