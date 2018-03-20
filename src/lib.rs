//#![deny(warnings)]
//#![no_std]

#![allow(dead_code)]

extern crate embedded_hal as hal;
extern crate byteorder;

//use byteorder::{ByteOrder, LittleEndian};
use hal::blocking::i2c::{Read, Write, WriteRead};

pub const BATTERY_LEVEL_MISSING: u8 = 0x7f;

enum Registers {
	AcinVoltage = 0x56,
	VbusCurrent = 0x5c,
	Temperature = 0x5e,
	BatteryLevel = 0xb9,
	BatteryVoltage = 0x78,
	BatteryChargeCurrent = 0x7a,
	BatteryDischargeCurrent = 0x7c,
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

	fn write_read_byte(&mut self, send: u8) -> Result<u8, E> {
		let comm: [u8; 1] = [ send ];
		let mut buf: [u8; 1] = [0];
		self.device.write_read(self.address, &comm, &mut buf)?;

		Ok(buf[0])
	}

	/// Many ADC functions on this chip provide their values as a strange
	/// 10bit value that requires some funky shifting
	pub fn get_adc_10bits(&mut self, register: u8) -> Result<u16, E> {
		let comm: [u8; 1] = [ register ];
		let mut recv: [u8; 2] = [ 0, 0 ];
		let mut value: u16;

		self.device.write_read(self.address, &comm, &mut recv)?;

		// Weird way to store a number if ye ask me!
		value = (recv[0] as u16) << 4;
		value |= recv[1] as u16 & 0x0f;

		Ok(value)
	}

	/// In millivolts
	pub fn battery_voltage(&mut self) -> Result<u16, E> {
		let mut value = self.get_adc_10bits(Registers::BatteryVoltage as u8)?;

		// Voltage is in 1.1mV increments, so just add 1/10 the value and
		// avoid those pesky floating point multiplications. :D
		value += value / 10;

		Ok(value)
	}

	/// In milliamps
	pub fn battery_charging_current(&mut self) -> Result<u16, E> {
		let value = self.get_adc_10bits(Registers::BatteryChargeCurrent as u8)?;

		Ok(value / 2)
	}

	/// In milliamps
	pub fn vbus_current(&mut self) -> Result<u16, E> {
		let value = self.get_adc_10bits(Registers::VbusCurrent as u8)?;

		Ok(value / 2)
	}

	/// In millivolts
	pub fn acin_voltage(&mut self) -> Result<u16, E> {
		let mut value = self.get_adc_10bits(Registers::AcinVoltage as u8)?;

		value += value / 7;

		Ok(value)
	}

	/// In celcius
	pub fn temperature(&mut self) -> Result<i16, E> {
		// Check out page 25 of the datasheet for the weird math

		let value = self.get_adc_10bits(Registers::Temperature as u8)?;

		let mut value = value as i16 / 10;
		value -= 145;

		Ok(value)
	}

	/// In milliamps
	pub fn battery_discharging_current(&mut self) -> Result<u16, E> {
		let comm: [u8; 1] = [ Registers::BatteryDischargeCurrent as u8 ];
		let mut recv: [u8; 2] = [ 0, 0 ];
		let mut value: u16;

		self.device.write_read(self.address, &comm, &mut recv)?;

		// Of course one would have 5 least significant bits and
		// ruin my get_adc_10bits function above!
		value = (recv[0] as u16) << 5;
		value |= recv[1] as u16 & 0x1f;

		Ok(value / 2)
	}

	pub fn battery_level(&mut self) -> Result<u8, E> {
		// The MSB for the voltage is a control bit that enables or
		// disables sampling
		match self.write_read_byte(Registers::BatteryLevel as u8) {
			Ok(x) => Ok(x & 0b0111_1111),
			Err(x) => Err(x)
		}
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
        let state = gpio.is_low();

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
