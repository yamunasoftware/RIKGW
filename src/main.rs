mod readout;
mod conf;

use rdkafka::config::ClientConfig;
use rdkafka::producer::future_producer::Delivery;
use rdkafka::producer::{FutureProducer, FutureRecord};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
  let topic: &str = "imadds";
  let delay: u64 = 10;
  let kafka_url: String = conf::get_kafka_url();
  let producer: FutureProducer = setup_producer(kafka_url);

  loop {
    send_message(producer.clone(), topic).await;
    sleep(Duration::from_secs(delay)).await;
  }
}

async fn send_message(producer: FutureProducer, topic: &str) {
  
}

fn setup_producer(kafka_url: String) -> FutureProducer {
  let producer: FutureProducer = ClientConfig::new()
    .set("bootstrap.servers", kafka_url)
    .set("message.timeout.ms", "5000")
    .create()
    .expect("Producer creation failed");
  producer
}