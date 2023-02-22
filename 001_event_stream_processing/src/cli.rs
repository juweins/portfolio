/*
    This file contains the CLI configuration
*/

use clap::Args;

/// Command line arguments
/// - subcommand: Action that should be performed
/// - args: Arguments that are processed for the subcommand
#[derive(clap::Parser)]
pub struct Cli {

    /// Action that should be performed
    /// - produce: Push data via Kafka Producer to the local broker
    /// - consume: Read data via Kafka Consumer from the local broker
    /// - write: Write data to Azure Blob Storage
    /// - read: Read data from Azure Blob Storage
    #[clap(subcommand)]
    pub subcommand: Command,

    /// Arguments that are processed for the subcommand:
    /// - produce: topic, message
    /// - consume: topic
    /// - write: container_name, file
    /// - read: container_name, file
    pub args: Vec<String>,

}

/// Subcommands
/// - Produce: Push data via Kafka Producer to the local broker
/// - Consume: Read data via Kafka Consumer from the local broker
/// - Write: Write data to Azure Blob Storage
/// - Read: Read data from Azure Blob Storage
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

    #[clap(about = "Write data to Azure Blob Storage")]
    Write {
        #[clap(short, long, help = "Container name")]
        container_name: String,
        #[clap(short, long, help = "File path")]
        file: String,
    },

    #[clap(about = "Request data from external API")]
    Request {
        #[clap(short, long, help = "Name of the API")]
        api_name: String,
        #[clap(short, long, help = "Start date: YYYY-MM-DD")]
        start_date: String,
        #[clap(short, long, help = "End date: YYYY-MM-DD")]
        end_date: String,
    },

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