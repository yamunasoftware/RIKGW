use std::fs;

pub fn get_kafka_config() -> Vec<String> {
  let mut config: Vec<String> = Vec::with_capacity(2);
  let content = fs::read_to_string("/ikgw/resources/.conf").unwrap();

  for line in content.lines() {
    if line.contains("KAFKA_URL=") {
      config.push(line.replace("KAFKA_URL=", "").trim().to_string());
    }

    else if line.contains("KAFKA_USERNAME=") {
      config.push(line.replace("KAFKA_USERNAME=", "").trim().to_string());
    }

    else if line.contains("KAFKA_PASSWORD=") {
      config.push(line.replace("KAFKA_PASSWORD=", "").trim().to_string());
    }
  }
  config
}

pub fn get_system_config() -> Vec<String> {
  let mut config: Vec<String> = Vec::with_capacity(2);
  let content = fs::read_to_string("/ikgw/resources/.conf").unwrap();

  for line in content.lines() {
    if line.contains("SYSTEM_ID=") {
      config.push(line.replace("SYSTEM_ID=", "").trim().to_string());
    }

    else if line.contains("SYSTEM_TYPE=") {
      config.push(line.replace("SYSTEM_TYPE=", "").trim().to_string());
    }
  }
  config
}