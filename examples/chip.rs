extern crate linux_embedded_hal as linux_hal;
extern crate embedded_hal;
extern crate axp209;

use linux_hal::{I2cdev};
use linux_hal::i2cdev::linux::LinuxI2CError;
use axp209::{Axp209, BATTERY_LEVEL_MISSING};

fn main() {
	let i2c = I2cdev::new("/dev/i2c-0").unwrap();
	let address = 0x34;

	let mut pmic = Axp209::new(i2c, address);
	let level = pmic.battery_level();
	display_battery_info(level);

	let voltage = pmic.battery_voltage().unwrap();
	println!("Voltage: {}v", voltage);
}

fn display_battery_info(level: Result<u8, LinuxI2CError>) {
	let level = match level {
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

