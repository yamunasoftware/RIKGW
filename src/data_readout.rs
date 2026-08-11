use linux_embedded_hal::{Delay, I2cdev};
use xca9548a::{CharacterWidth, SlaveAddr, Xca9548a};
use bme280::i2c::BME280;

fn data_readout() {
  
}

fn read_channel() {
  let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
  let mut bme280 = BME280::new_primary(i2c_bus, Delay);
  bme280.init().unwrap();
  let measurements = bme280.measure().unwrap();
  let temperature = measurements.temperature;
  let humidity = measurements.humidity;
  let pressure = measurements.pressure;
}

fn init_channel() {
  let address = SlaveAddr::default();
  let mut mux = Xca9548a::new(i2c, address);
  // Enable channel 2 (bit 2 = 1 << 2)
  mux.select_channels(&[xca9548a::Channel::Ch2])?;
}