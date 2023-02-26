// WSL2/Ubuntu users: Make sure that you have pkg-config and libssl-dev installed!
mod cli;

use exchange::producer::push_to_kafka;
use exchange::consumer::read_from_kafka;
use exchange::{push_to_azure, request_data, pull_from_azure};
use exchange::cli::{Cli, Command};

use clap::*;
use log::{info, warn, error};
use serde_json::Value;

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

        Command::Request{api_name} => {
            info!("Requester selected");
            info!("API: {}", &api_name);

            let result = request_data(&api_name).await;

            match result {
                Ok(_) => info!("Data requested from API {} successfully", &api_name),
                Err(e) => error!("Error while requesting data from API {}: {}", &api_name, e)
            }
        },

        Command::Ingest{api_name, topic} => {
            info!("Forwarder selected");
            info!("API: {}, Topic: {}", &api_name, &topic);

            let response = match request_data(&api_name).await {
                Ok(response) => Some(response),
                Err(e) => {
                    error!("Error while requesting data from API {}: {}", &api_name, e);
                    None
                }
            };

            let write = match push_to_kafka(&topic, &response.unwrap().to_string()).await {
                Ok(_) => info!("Data pushed to Kafka"),
                Err(e) => error!("Error while pushing data to Kafka: {}", e)
            };
            info!("Data ingest from API {} complete", &api_name)

        },

        Command::Config { name, url, key } => {
            info!("Configurator selected");
            info!("API: {}, URL: {}, API Key: {}", &name, &url, &key);
            error!("Configurator not implemented yet! No entries have been added.");
            // TODO: Add Configurator logic
        },

        Command::Version => {
            info!("Version selected");
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
        }

    }
}
