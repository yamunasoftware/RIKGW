use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

#[tokio::main]
async fn main() {
  // 1. Configure and build the Kafka producer
  let producer: FutureProducer = ClientConfig::new()
      .set("bootstrap.servers", "localhost:9092")
      .set("message.timeout.ms", "5000")
      .create()
      .expect("Producer creation failed");

  // 2. Define message destination and contents
  let topic = "my-topic";
  let message_payload = "Hello, Kafka from Rust!";
  let message_key = "user_key_123";

  println!("Sending message to topic: {}...", topic);

  // 3. Construct and send the record asynchronously
  let record = FutureRecord::to(topic)
      .payload(message_payload)
      .key(message_key);

  // 4. Await delivery acknowledgment from Kafka
  match producer.send(record, Duration::from_secs(0)).await {
      Ok((partition, offset)) => {
          println!(
              "Successfully sent message to partition {} at offset {}",
              partition, offset
          );
      }
      Err((error, _original_record)) => {
          eprintln!("Failed to deliver message: {:?}", error);
      }
  }
}