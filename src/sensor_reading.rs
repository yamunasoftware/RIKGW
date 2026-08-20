pub struct SensorReading {
  pub device_id: String,
  pub device_type: String,
  pub channel: u8,
  pub temperature: f32,
  pub humidity: f32,
  pub pressure: f32
}