use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

#[tokio::main]
async fn main() {
  let producer = setup_producer();
  send_message(&producer).await;
}

async fn send_message(producer: &FutureProducer) {
  let record = FutureRecord::to("my-topic")
      .payload("Hello from Rust!")
      .key("my-key");

  match producer.send(record, Duration::from_secs(0)).await {
      Ok(delivery) => println!("Sent: {:?}", delivery),
      Err((e, _msg)) => println!("Error sending message: {}", e),
  }
}

fn setup_producer() -> FutureProducer {
  let producer: FutureProducer = ClientConfig::new()
    .set("bootstrap.servers", "localhost:9092")
    .set("message.timeout.ms", "5000")
    .create()
    .expect("Producer creation error");
}