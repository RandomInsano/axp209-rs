//#![deny(warnings)]
//#![no_std]

#![allow(dead_code)]

extern crate embedded_hal as hal;

use hal::blocking::delay::DelayMs;
use hal::blocking::i2c::{Read, Write, WriteRead};

pub const BATTERY_LEVEL_MISSING: u8 = 0x7f;

enum Registers {
	BatteryLevel = 0xb9,
}

pub struct Axp209<I2C> {
	device: I2C,
	address: u8,
}

impl<I2C, E> Axp209<I2C>
where
	I2C: WriteRead<Error = E> + Write<Error = E> + Read<Error = E>,
{
	pub fn new(dev: I2C, address: u8) -> Self {
		Axp209 {
			device: dev,
			address: address,
		}
	}

	pub fn battery_level(&mut self) -> Result<u8, E> {
		let comm: [u8; 1] = [ Registers::BatteryLevel as u8 ];
		let mut buf: [u8; 1] = [0];
		self.device.write_read(self.address, &comm, &mut buf)?;

		Ok(buf[0])
	}

	fn battery_present(&mut self) -> Result<bool, E> {
		let level = self.battery_level()?;

		Ok(level == BATTERY_LEVEL_MISSING)
	}
}

#[cfg(test)]
mod tests {
    extern crate linux_embedded_hal as linux_hal;

    use super::*;

    use self::linux_hal::{Pin, I2cdev};
    use hal::digital::OutputPin;

    #[test]
    fn permissions() {
        let mut gpio = Pin::new(135);
        //let state = gpio.is_low();
        let state = true;

        if state {
            gpio.set_high();
        } else {
            gpio.set_low();
        }
    }

    #[test]
    fn battery_level() {
        let i2c = I2cdev::new("/dev/i2c-0").unwrap();
        let address = 0x34;

        let mut pmic = Axp209::new(i2c, address);
        let level = pmic.battery_level().unwrap();
        // TODO: no_std makes this kinda hard:
        println!("Battery level: {}%", level);

	// Values for 'level' can be either the percentage, or
        // 0x7F if the battery is missing
    }
}
