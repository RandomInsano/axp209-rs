extern crate linux_embedded_hal as linux_hal;
extern crate embedded_hal;
extern crate axp209;

use linux_hal::{I2cdev};
use linux_hal::i2cdev::linux::LinuxI2CError;
use axp209::{Axp209, BATTERY_LEVEL_MISSING};

fn main() {
    let i2c = I2cdev::new("/dev/i2c-0").unwrap();

    let mut pmic = Axp209::new(i2c);
    let level = pmic.battery_level();
    display_battery_info(level);

    let voltage = pmic.battery_voltage().unwrap();
    println!("Voltage: {}mV", voltage);

    let value = pmic.battery_discharging_current().unwrap();
    println!("Discharge Current: {}mA", value);

    let value = pmic.battery_charging_current().unwrap();
    println!("Charge Current:    {}mA", value);

    let value = pmic.acin_voltage().unwrap();
    println!("ACIN Voltage:      {}mV", value);

    let value = pmic.acin_current().unwrap();
    println!("ACIN Current:      {}mA", value);

    let value = pmic.vbus_voltage().unwrap();
    println!("Vbus Voltage:      {}mV", value);

    let value = pmic.vbus_current().unwrap();
    println!("Vbus Current:      {}mA", value);

    let value = pmic.temperature().unwrap();
    println!("Temperature:       {}°C", value);

    let value = pmic.ts_voltage().unwrap();
    println!("Temp Sensor Pin:   {}mV", value);

    let value = pmic.ipsout_voltage().unwrap();
    println!("Ipsout?!:          {}mV", value);

    println!("");

    let value = pmic.adc_control().unwrap();
    println!("Power Control Flags: {:?}", value);

    let value = pmic.power_status().unwrap();
    println!("Power Status Flags:  {:?}", value);

    let value = pmic.charging_status().unwrap();
    println!("Charge Status Flags: {:?}", value);

    println!("");

    let value = pmic.timer_control().unwrap();
    println!("Timer:\n\tExpired: {}\n\tTime (minutes): {}", value.expired(), value.minutes());
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
