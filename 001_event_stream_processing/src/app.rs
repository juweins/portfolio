// WSL2/Ubuntu users: Make sure that you have pkg-config and libssl-dev installed!
mod cli;

use exchange::producer::push_to_kafka;
use exchange::consumer::read_from_kafka;
use exchange::{push_to_azure, request_data, pull_from_azure};
use exchange::cli::{Cli, Command};

use clap::*;
use log::{info, warn, error};

#[tokio::main]
async fn main() {

    // Initialize Logger
    env_logger::init();

    let args = Cli::parse();

    // TODO: Add logging
    // TODO: Add parsing of additional CLI arguments
    match args.subcommand {
        Command::Produce{topic, file} => {
            info!("Producer selected");
            info!("Topic: {}, File: {}", topic, file);

            let result = push_to_kafka(&topic, &file).await;

            match result {
                Ok(_) => {
                    info!("Data pushed to Kafka");
                    info!("Data: {:?}", result.unwrap());                
            },
                Err(e) => warn!("Error while pushing data to Kafka: {}", e)
            }

        },

        Command::Consume{topic, ttl} => {
            info!("Consumer selected");
            info!("Topic: {}, TTL: {}", topic, ttl);

            let result = read_from_kafka(&topic, ttl).await;

            match result {
                Ok(_) => {
                    info!("Data read from Kafka");
                    info!("Data: {:?}", result.unwrap());
                },
                Err(e) => warn!("Error while reading data from Kafka: {}", e)
            }
        },

        Command::Read{container_name, file} => {
            info!("Reader selected");
            info!("Container name: {}, File: {}", container_name, file);
            
            let result = pull_from_azure(&container_name, &file).await;

            match result {
                Ok(_) => info!("Data pulled from Azure Blob Storage {} successfully", &container_name),
                Err(e) => error!("Error while pulling data from Azure Blob Storage {}: {}", &container_name, e)
            }
        },

        Command::Write{container_name, file} => {
            info!("Writer selected");
            info!("Container name: {}, File: {}", container_name, file);
            
            //TODO: blob_name should be optional and default to file
            let result = push_to_azure(&container_name, &file, &file).await;

            match result {
                Ok(_) => info!("Data push to Azure Blob Storage {} successfully", &container_name),
                Err(e) => error!("Error while pushing data to Azure Blob Storage {}: {}", &container_name, e)
            }
        },

        Command::Request{api_name, start_date, end_date} => {
            info!("Reader selected");
            info!("API name: {}, Start date: {}, End date: {}", api_name, start_date, end_date);
            let result = request_data(&api_name, &start_date, &end_date).await;

            match result {
                Ok(_) => info!("Data requested from {} successfully", &api_name),
                Err(e) => error!("Error while requesting data from {}: {}", &api_name, e)
            }
        },

        Command::Version => {
            info!("Version selected");
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
        }

    }
}
