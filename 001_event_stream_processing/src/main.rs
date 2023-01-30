// WSL2/Ubuntu users: Make sure that you have pkg-config and libssl-dev installed!

use exchange_stream::{push_data, request_data};

#[tokio::main]
async fn main() {
    // TODO: Make the start and end date dynamic by wrapping in a CLI argument
    let start_date = "2023-01-01";
    let end_date = "2023-01-28";

    let result = push_data();
    println!("Success.");
    // Store the result of the request in variable result
    // let result = request_data(start_date, end_date);

    // Print the result to the console in a readable format :#?
    //println!("Result: {:#?}", result.unwrap());

    // TODO: Write to destination file in Azure Blob Storage
}
