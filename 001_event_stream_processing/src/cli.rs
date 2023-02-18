/*
    This file contains the CLI configuration
*/

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
    
    #[clap(subcommand)]
    #[clap(about = "Push data via Kafka Producer to the local broker")]
    Produce,

    #[clap(subcommand)]
    #[clap(about = "Read data via Kafka Consumer from the local broker")]
    Consume,

    #[clap(subcommand)]
    #[clap(about = "Write data to Azure Blob Storage")]
    Write,

    #[clap(subcommand)]
    #[clap(about = "Read data from Azure Blob Storage")]
    Read,

}