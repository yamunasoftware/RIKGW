mod data_readout;
mod conf;

use rdkafka::config::ClientConfig;
use rdkafka::producer::future_producer::Delivery;
use rdkafka::producer::{FutureProducer, FutureRecord};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
  let producer: FutureProducer = setup_producer();
  let topic: &str = "imadds";
  let delay: u64 = 2;

  loop {
    send_message(producer, topic).await;
    sleep(Duration::from_secs(delay)).await;
  }
}

async fn send_message(producer: FutureProducer, topic: &str) {
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