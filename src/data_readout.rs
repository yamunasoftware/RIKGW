#[path = "conf.rs"]
pub mod conf;

use linux_embedded_hal::{Delay, I2cdev};
use xca9548a::{CharacterWidth, SlaveAddr, Xca9548a};
use bme280::i2c::BME280;

fn data_readout() -> Vec<SensorReading> {
  let mut sensor_readings: Vec<SensorReading> = Vec::new();
  let config = conf::get_system_config();
  
  loop {
    for channel in 0..8 {
      init_channel(channel);
      let measurements = read_channel();
      let sensor_message = SensorReading::new(
        config[0].clone(),
        config[1].clone(),
        channel,
        measurements[0],
        measurements[1],
        measurements[2]
      );
      sensor_readings.push(sensor_message);
    }
  }
  sensor_readings
}

fn read_channel() -> Vec<f32> {
  let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
  let mut bme280 = BME280::new_primary(i2c_bus);
  bme280.init(&mut delay).unwrap();
  let measurements = bme280.measure(&mut delay).unwrap();

  let mut measurements: Vec<f32> = Vec::with_capacity(3);
  measurements.push(measurements.temperature);
  measurements.push(measurements.humidity);
  measurements.push(measurements.pressure);
  measurements
}

fn init_channel(channel: u8) {
  let address = SlaveAddr::default();
  let mut mux = Xca9548a::new(i2c, address);
  mux.select_channels(&[channel])?;
}

impl SensorReading {
  pub fn new(
    device_id: String, device_type: String, channel: u8,
    temperature: f32, humidity: f32, pressure: f32
  ) -> Self {
    Self {
      device_id,
      device_type,
      channel,
      temperature,
      humidity,
      pressure
    }
  }
}