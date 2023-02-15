/*
    This file contains the function that reads the API response as an event from Kafka
    It mimics the consumer in the project setup
*/

use std::time::Duration;

use crate::get_kafka_details;

use rdkafka::error::{KafkaError, KafkaResult};
use log::{warn, info};
use rdkafka::{Message};
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer,BaseConsumer};




pub async fn read_from_kafka(topic: &str, test: bool) -> Result<(u8), KafkaError>{

    // TODO: Why does calling new_kafka_consumer() here not work?
    // - It works in the producer.rs file
    // - It works in the client.rs file
    // - It does not work here
    // results in an stack overflow error

    // TODO: remove this and use the function instead

    // read the kafka details from a file and store them in a vector
    let kafka_details = get_kafka_details().unwrap();

    // Assign details to variables (for readability)
    let bootstrap_servers = kafka_details.bootstrap_servers;
    let group_id = kafka_details.group_id;
    let message_timeout_ms = kafka_details.message_timeout_ms;

    // Create a new Kafka consumer (if not already existing)
    let consumer: &BaseConsumer = &ClientConfig::new()
        .set("bootstrap.servers", &bootstrap_servers)
        .set("group.id", &group_id)
        .set("message.timeout.ms", &message_timeout_ms.to_string())
        .set("enable.auto.commit", "true")
        .set("connections.max.idle.ms", "1000")
        .create()
        .expect("Error: Failed to create Kafka consumer");


    // Initialize the message counter
    // - u8 is ok in this case since we only want to process a few messages
    let mut message_counter: u8 = 0;
    let mut retry_counter: u8 = 0;

    // Initialize the message
    let mut event_message: String = "".to_string();

    // Subscribe to the topic
    consumer.subscribe(&[topic]).expect("Error: Failed to subscribe to topic");
    println!("Subscribed to topic: {}", topic);
    // Process the messages
    // (I prefer the for loop since it is more readable than the generic loop) 
    // - identical functionality
   /*  for message in consumer.iter(){
        // increase the message counter
        message_counter += 1;
        match message {
            Ok(message) => {
                // convert the message to a string
                // TODO: remove this, display only
                let message = message.payload().unwrap().to_owned();
                let readable_message = String::from_utf8(message).unwrap();

                // print the message
                println!("Message: {}", readable_message);
                event_message = readable_message;
                break;
            },
            Err(e) => {
                warn!("Error: {:?}", e);
                event_message = "Error".to_string();
            }
        }
        // TODO: Break the loop after a certain amount of messages
        // - this is only for testing purposes
        if test == true {
            break;
        }
    } */


    println!("Listening for messages...");
    loop {
        // poll the stream for a message
        let message = consumer.poll(Duration::from_secs(5));

        // Initialize variable to check idle state
        let msg = message;

        // check if the stream is idle
        if msg.is_none() {
            
            info!("Listening for messages... (retry={})", retry_counter);

            // if the stream is idle, increase the retry counter
            retry_counter += 1;

            // put the thread to sleep for 2 seconds
            tokio::time::sleep(Duration::from_secs(2)).await;

            // if max retries, break the loop to exit the consumer stream
            if retry_counter > 5 {
                break;
            }
        } else {

            // if the stream is not idle, reset the retry counter
            retry_counter = 0;

            // increase the message counter
            message_counter += 1;

            // convert the message to a readable string in stdout
            let message = msg.unwrap().unwrap().payload().unwrap().to_owned();
            let readable_message = String::from_utf8(message).unwrap();

            // print the message
            println!("Message: {}", readable_message);
        }

    }
        println!("Terminate listening for messages... (max retries reached)");
        println!("Message counter: {}", message_counter);
        Ok(message_counter)
}

        

  



   /*      // if the stream is in idle mode, wait for 5 seconds
        let msg = message.as_ref().to_owned().unwrap();
        
        // get current message or None if the stream is idle
        let idle = msg.payload();
        
        // if the stream is idle, wait for 5 seconds
        match idle {
            Some(_) => {
                // if the stream is not idle, reset the retry counter
                if msg.payload().unwrap().is_empty() {
                    message_counter += 1;
                    retry_counter += 1;
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    if retry_counter > 5 {
                        break;
                    }
                } else {
                    retry_counter = 0;
                }
            },
            None => {
                // if the stream is idle, increase the retry counter
                retry_counter += 1;
                // if the retry counter is higher than 5, break the loop
                if retry_counter > 5 {
                    break;
                }
                // wait for 5 seconds
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
 */
        // increase the message counter
        /* match message {
            Ok(message) => {

                // convert the message to a string
                let content = message;
                let message = content.payload().unwrap().to_owned();
                let readable_message = String::from_utf8(message).unwrap();

                if readable_message.is_empty() == true {
                    // if the message is not empty, reset the retry counter
                    message_counter += 1;
                    retry_counter = 0;
                }

                // print the message
                println!("Message: {}", readable_message);

            },
            Err(e) => {
                warn!("Error: {:?}", e);
                event_message = "Error".to_string();
            }
            
        }
    } */


    

    



/*     loop {
        if let Some(ref message) = message {
            // increase the message counter
            match message {
                Result::Ok(msg) => {
                    let message = msg.payload().unwrap().to_owned();
                    let readable_message = String::from_utf8(message).unwrap();

                    // print the message
                    println!("Message: {}", readable_message);
                    event_message = readable_message;
                    break;
                },
                Err(e) => {
                    warn!("Error: {:?}", e);
                    event_message = "Error".to_string();
                },
            }
        }
    }
    Result::Ok(event_message) */
    

    /* WORKS
    
    let message = consumer.poll(Duration::from_millis(0));
    match message {
        Some(message) => {
            // increase the message counter
            message_counter += 1;
            match message {
                Ok(message) => {
                    let message = message.payload().unwrap().to_owned();
                    let readable_message = String::from_utf8(message).unwrap();
        
                    // print the message
                    println!("Message: {}", readable_message);
                    event_message = readable_message;
                    Ok(event_message)
                },
                Err(e) => {
                    warn!("Error: {:?}", e);
                    event_message = "Error".to_string();
                    Err(KafkaError::MessageProduction(rdkafka::types::RDKafkaErrorCode::BadMessage))
                },
            }
        }
        None => {
            warn!("Error: No message received");
            event_message = "Error".to_string();
            Err(KafkaError::MessageProduction(rdkafka::types::RDKafkaErrorCode::BadMessage))
        }
    } */



    // Process only one message
    // - this is only for testing purposes
