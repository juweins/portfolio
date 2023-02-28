// WSL2/Ubuntu users: Make sure that you have pkg-config and libssl-dev installed!

use exchange::request_data;
use exchange::cli::{Cli, Command};
use exchange::kafka::producer::push_to_kafka;
use exchange::kafka::consumer::read_from_kafka;
use exchange::azure::writer::push_to_azure;
use exchange::azure::reader::pull_from_azure;

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

        Command::Forward{topic, container_name, filename} => {
            info!("Forwarder selected");

            let blob_name = filename; // I find it confusing to call the cli with blob_name directly
            let mut content = String::new();

            let consumer = match read_from_kafka(&topic, 2).await {
                Ok(consumer) => Some(consumer),
                Err(e) => {
                    error!("Error while reading data from Kafka: {}", e);
                    None
                }
            };

            let _result = match consumer {
                Some(consumer) => {
                    // Check if message count is 0
                    if consumer.0 == 0 {
                        // The content should not be pushed to Azure Blob Storage since it results in an overwrite by default
                        warn!("No data received from Kafka: Skipping push to Azure Blob Storage");
                        return;
                    }

                    info!("Message(s) read from Kafka: {}", consumer.0);
                    // Create json from hashmap
                    content = serde_json::to_value(consumer.1).unwrap().to_string();

                },
                
                None => {
                    error!("No data received from Kafka: Check Kafka logs for more information");
                    return;
                }
            };

            let _write = match push_to_azure(&container_name, &blob_name, &content).await {
                Ok(_) => info!("Data pushed to Azure Blob Storage {} successfully", &container_name),
                Err(e) => error!("Error while pushing data to Azure Blob Storage {}: {}", &container_name, e)
            };

        },


        Command::Config { config_file } => {
            info!("Configurator selected");

            // Build path
            let mut path = format!("~/.config/exchange/{}.json", config_file);
            path = shellexpand::tilde(&path).to_string();

            // open file with nano
            subprocess::Exec::cmd("nano")
                .arg(path)
                .join()
                .expect("Error: Failed to run editor");
        },

        Command::Version => {
            info!("Version selected");
            println!("Version: {}", env!("CARGO_PKG_VERSION"));
        }

    }
}
