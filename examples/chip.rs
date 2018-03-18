extern crate linux_embedded_hal as linux_hal;
extern crate embedded_hal;
extern crate axp209;

use linux_hal::{I2cdev};
use embedded_hal::blocking::i2c::{Read, Write, WriteRead};
use std::fmt::Error;
use axp209::{Axp209, BATTERY_LEVEL_MISSING};

fn main() {
	let i2c = I2cdev::new("/dev/i2c-0").unwrap();
	let address = 0x34;

	let mut pmic = Axp209::new(i2c, address);
	display_battery_info(&mut pmic);
}

fn display_battery_info<I2C,E>(pmic: &mut Axp209<I2C>) where
	I2C: WriteRead<Error = E> + Write<Error = E> + Read<Error = E> {

	let level = match pmic.battery_level() {
		Ok(x) => x,
		_ => { 
			println!("Unable to get battery state. Exiting");
			return;
		}
	};

        if level != BATTERY_LEVEL_MISSING {
		println!("Battery level: {}%", level);
	} else {
		println!("Battery missing");
	}

}

