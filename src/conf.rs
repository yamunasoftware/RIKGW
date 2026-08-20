use std::fs;

pub fn get_kafka_url() -> String {
  let mut url = String::new();
  let content = fs::read_to_string("/ikgw/resources/.conf").unwrap();
  for line in content.lines() {
    if line.contains("KAFKA_URL=") {
      url = line.replace("KAFKA_URL=", "").trim().to_string();
      break;
    }
  }
  url
}

pub fn get_system_config() -> Vec<String> {
  let mut config: Vec<String> = Vec::with_capacity(2);
  let content = fs::read_to_string("/ikgw/resources/.conf").unwrap();
  for line in content.lines() {
    if line.contains("SYSTEM_ID=") {
      let system_id = line.replace("SYSTEM_ID=", "").trim().to_string();
      config.push(system_id);
    }

    else if line.contains("SYSTEM_TYPE=") {
      let system_name = line.replace("SYSTEM_TYPE=", "").trim().to_string();
      config.push(system_name);
    }
  }
  config
}