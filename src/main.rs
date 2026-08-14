use rdkafka::config::ClientConfig;
use rdkafka::producer::future_producer::Delivery;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

#[tokio::main]
async fn main() {
  let producer = setup_producer();
  let topic = "my-topic";
  let message_payload = "Hello, Kafka from Rust!";
  let message_key = "user_key_123";

  send_message(producer, topic, message_payload, message_key).await;
}

async fn send_message(producer: FutureProducer, topic: &str, message_payload: &str, message_key: &str) {
  let record = FutureRecord::to(topic)
      .payload(message_payload)
      .key(message_key);
    
  match producer.send(record, Duration::from_secs(0)).await {
      Ok(Delivery { partition, offset, timestamp }) => {
          println!(
              "Successfully sent message to partition {} at offset {} at {}",
              partition, offset, timestamp
          );
      }
      Err((error, _original_record)) => {
          eprintln!("Failed to deliver message: {:?}", error);
      }
  }
}

fn setup_producer() -> FutureProducer {
  let producer: FutureProducer = ClientConfig::new()
    .set("bootstrap.servers", "localhost:9092")
    .set("message.timeout.ms", "5000")
    .create()
    .expect("Producer creation failed");
  producer
}