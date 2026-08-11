use std::fs;

fn get_kafka_url() -> std::io::Result<String> {
  let content = fs::read_to_string("/ikgw/resources/.conf")?;
  for line in content.lines() {
  
  }
  Ok(())
}

fn get_system_config() -> std::io::Result<()> {
  let content = fs::read_to_string("/ikgw/resources/.conf")?;
  for line in content.lines() {
    
  }
  Ok(())
}