mod conf;
mod sensor_reading;
mod readout;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use tokio::time::{sleep, Duration};
use flexi_logger::{Age, Cleanup, Criterion, FileSpec, Logger, Naming};
use log::{error, info};

#[tokio::main]
async fn main() {
  let topic: &str = "imadds";
  let delay: u64 = 10;
  let kafka_config: Vec<String> = conf::get_kafka_config();
  let producer: FutureProducer = setup_producer(kafka_config);
  setup_logger().expect("Failed to Setup Logger");

  loop {
    send_message(producer.clone(), topic).await;
    sleep(Duration::from_secs(delay)).await;
  }
}

async fn send_message(producer: FutureProducer, topic: &str) {
  let sensor_readings = readout::data_readout();
  let payload = match serde_json::to_string(&sensor_readings) {
    Ok(payload) => payload,
    Err(error) => {
      error!("Failed to Serialize Sensor Readings: {error}");
      return;
    }
  };

  let delivery = producer
    .send(
      FutureRecord::to(topic)
        .key(topic)
        .payload(&payload),
      Duration::from_secs(5),
    )
    .await;

  match delivery {
    Ok(delivery) => info!("Sensor Messages Delivered: {delivery:?}"),
    Err((error, _message)) => error!("Failed to Deliver Sensor Messages: {error}"),
  }
}

fn setup_producer(kafka_config: Vec<String>) -> FutureProducer {
  let kafka_url = &kafka_config[0];
  let kafka_username = &kafka_config[1];
  let kafka_password = &kafka_config[2];

  let jaas_config = format!(
    "org.apache.kafka.common.security.scram.ScramLoginModule required username=\"{}\" password=\"{}\";",
    kafka_username, kafka_password
  );
  
  let producer: FutureProducer = ClientConfig::new()
    .set("bootstrap.servers", kafka_url)
    .set("security.protocol", "SASL_SSL")
    .set("sasl.mechanism", "SCRAM-SHA-512")
    .set("sasl.jaas.config", jaas_config)
    .set("key.serializer", "org.apache.kafka.common.serialization.StringSerializer")
    .set("value.serializer", "org.apache.kafka.common.serialization.StringSerializer")
    .set("acks", "all")
    .set("compression.type", "lz4")
    .set("enable.idempotence", "true")
    .set("retries", "3")
    .set("delivery.timeout.ms", "60000")
    .set("request.timeout.ms", "15000")
    .set("max.in.flight.requests.per.connection", "1")
    .create()
    .expect("RIKGW Producer Creation Failed");
  producer
}

fn setup_logger() -> Result<(), Box<dyn std::error::Error>> {
  Logger::try_with_str("info")?
    .log_to_file(
      FileSpec::default()
        .directory("/main/rikgw/logs")
        .basename("rikgw")
        .suffix("log"),
    )
    .rotate(
        Criterion::Age(Age::Day),
        Naming::Timestamps,
        Cleanup::KeepLogFiles(30),
    )
    .start()?;
  Ok(())
}