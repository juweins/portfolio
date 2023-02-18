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
            let result = push_to_kafka("test", "example_response.json").await;

            match result {
                Ok(_) => {
                    info!("Data pushed to Kafka");
                    info!("Data: {}", result.unwrap());
            },
                Err(e) => warn!("Error while pushing data to Kafka: {}", e)
            }

        },
        Command::Consume => {
            info!("Consumer selected");
            let result = read_from_kafka("test", true).await;

            match result {
                Ok(_) => {
                    info!("Data read from Kafka");
                    info!("Data: {:?}", result.unwrap());
                },
                Err(e) => warn!("Error while reading data from Kafka: {}", e)
            }
        },
        Command::Write => {
            println!("Writer selected");
            let result = push_data().await;

            match result {
                Ok(_) => info!("Data pushed to Azure Blob Storage"),
                Err(e) => warn!("Error while pushing data to Azure Blob Storage: {}", e)
            }
        },
        Command::Read => {
            println!("Reader selected");
            let result = request_data("1", "2", "3").await;

            match result {
                Ok(_) => info!("Data read from Azure Blob Storage"),
                Err(e) => warn!("Error while reading data from Azure Blob Storage: {}", e)
            }
        }
    }
}
