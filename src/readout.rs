#[path = "conf.rs"]
pub mod conf;

#[path = "sensor_reading.rs"]
pub mod sensor_reading;

use linux_embedded_hal::{Delay, I2cdev};
use xca9548a::{SlaveAddr, Xca9548a};
use bme280::i2c::BME280;
use sensor_reading::SensorReading;

pub fn data_readout() -> Vec<SensorReading> {
  let mut sensor_readings: Vec<SensorReading> = Vec::new();
  let config = conf::get_system_config();
  
  for channel in 0..8 {
    init_channel(channel);
    let measurements = read_channel();
    let sensor_message = SensorReading {
      device_id: config[0].clone(),
      device_type: config[1].clone(),
      channel: channel,
      temperature: measurements[0],
      humidity: measurements[1],
      pressure: measurements[2]
    };
    sensor_readings.push(sensor_message);
  }
  sensor_readings
}

pub fn read_channel() -> Vec<f32> {
  let mut delay = Delay;
  let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
  let mut bme280 = BME280::new_primary(i2c_bus);
  bme280.init(&mut delay).unwrap();
  let reading = bme280.measure(&mut delay).unwrap();

  let mut measurements: Vec<f32> = Vec::with_capacity(3);
  measurements.push(reading.temperature);
  measurements.push(reading.humidity);
  measurements.push(reading.pressure);
  measurements
}

pub fn init_channel(channel: u8) {
  let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
  let address = SlaveAddr::default();
  let mut mux = Xca9548a::new(i2c_bus, address);
  mux.select_channels(channel).unwrap();
}