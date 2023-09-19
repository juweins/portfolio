/*
    This file contains the CLI configuration
*/

use clap::Args;

/// Command line arguments
/// - subcommand: Action that should be performed
/// - args: Arguments that are processed for the subcommand
#[derive(clap::Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Command
}

/// Subcommands
/// - Produce: Push data via Kafka Producer to the local broker
/// - Consume: Read data via Kafka Consumer from the local broker
/// - Write: Write data to Azure Blob Storage
/// - Read: Read data from Azure Blob Storage
/// - Request: Request data from external API
/// - Ingest: Run the applications pipeline (Request -> Produce)
/// - Forward: Forward data from one broker through an intermediate appicaltion to azure  (Consume -> Write)  
/// - Config: Configure the application (API for Ingest)
/// - Version: Get version information
#[derive(clap::Subcommand)]
pub enum Command {

    #[clap(about = "Push data via Kafka Producer to the local broker")]
    Produce {
        #[clap(short, long, help = "Topic name")]
        topic: String,
        #[clap(short, long, help = "Message")]
        file: String,
    },

    #[clap(about = "Read data via Kafka Consumer from the local broker")]
    Consume {
        #[clap(short, long, help = "Topic name")]
        topic: String,
        #[clap(long, help="Time to live (in seconds)", default_value = "30")]
        ttl: u8,
    },

    #[clap(about = "Read data from Azure Blob Storage")]
    Read {
        #[clap(short, long, help = "Container name")]
        container_name: String,
        #[clap(short, long, help = "File path")]
        file: String,
    },

    #[clap(about = "Write data to Azure Blob Storage")]
    Write {
        #[clap(short, long, help = "Container name")]
        container_name: String,
        #[clap(short, long, help = "File path")]
        file: String,
    },

    #[clap(about = "Request most recent data from external API")]
    Request {
        #[clap(short, long, help = "Name of the API to request")]
        api_name: String,
    },

    #[clap(about = "Run the applications pipeline")]
    Ingest {
        #[clap(short, long, help = "Name of the API to request")]
        api_name: String,
        #[clap(short, long, help = "Topic name for Producer")]
        topic: String,
    },

    #[clap(about = "Forward data from one broker through an intermediate appicaltion to azure")]
    Forward {
        #[clap(short, long, help = "Topic to read from")]
        topic: String,
        #[clap(short, long, help = "Container name for Azure Blob Storage")]
        container_name: String,
        #[clap(short, long, help = "File (or path) for Azure Blob Storage")]
        filename: String,
    },

    #[clap(about = "Configure the application")]
    Config {
        #[clap(short, long, help = "Name of the API")]
        config_file: String,
/*         #[clap(short, long, help = "URL of the API")]
        url: String,
        #[clap(short, long, help = "API key")]
        key: String, */
    },

    #[clap(about = "Get version information")]
    Version,

}

// Arguments for the subcommands
// IMPORTANT: !!! Not in use !!!
// - Produce: topic, message
// - Consume: topic, time to live
// - Write: container_name, file
// - Read: container_name, file
#[derive(Debug, Args)]
pub struct Produce {
    pub topic: String,
    pub message: String,
}

#[derive(clap::Parser)]
pub struct Consume {
    #[clap(short, long, help = "Topic name")]
    pub topic: String,
    #[clap(long, help="Time to live (in seconds)", default_value = "30")]
    pub ttl: u8,
}

#[derive(clap::Parser)]
pub struct Write {
    #[clap(short, long, help = "Container name")]
    pub container_name: String,
    #[clap(short, long, help = "File path")]
    pub file: String,
}

#[derive(clap::Parser)]
pub struct Read {
    #[clap(short, long, help = "Container name")]
    pub container_name: String,
    #[clap(short, long, help = "File path")]
    pub file: String,
}