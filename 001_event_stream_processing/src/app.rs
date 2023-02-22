// WSL2/Ubuntu users: Make sure that you have pkg-config and libssl-dev installed!
use exchange::producer::push_to_kafka;
use exchange::consumer::read_from_kafka;
use exchange::{push_to_azure, request_data};
use exchange::cli::{Cli, Command};


use clap::*;
use log::{info, warn};

#[tokio::main]
async fn main() {

    // Initialize Logger
    env_logger::init();

    let args = Cli::parse();

    // set_term_width(0) in clap
    // https://docs.rs/clap/2.33.3/clap/struct.App.html#method.set_term_width

    // TODO: Add logging
    // TODO: Add parsing of additional CLI arguments
    match args.subcommand {
        Command::Produce => {
            info!("Producer selected");

            // Parse arguments
            let topic = args.args[0].clone();
            let message = args.args[1].clone();

            let result = push_to_kafka(&topic, &message).await;

            match result {
                Ok(_) => {
                    info!("Data pushed to Kafka");
                    info!("Data: {:?}", result.unwrap());
            },
                Err(e) => warn!("Error while pushing data to Kafka: {}", e)
            }

        },
        Command::Consume => {
            info!("Consumer selected");

            // Parse arguments
            let topic = args.args[0].clone();
            let ttl: u8;

            // Parse ttl as u8 or set default value
            match args.args[1].parse::<u8>() {
                Ok(t) => {
                    info!("Received TTL: {}", t);
                    ttl = t;
                }
                Err(_) => {
                    info!("Error parsing TTL. Setting default TTL: 30 seconds");
                    ttl = 30
                }
            }


            let result = read_from_kafka(&topic, ttl).await;

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
            let result = push_to_azure().await;

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
