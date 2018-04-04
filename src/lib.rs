//! The [AXP209](http://dl.linux-sunxi.org/AXP/AXP209_Datasheet_v1.0en.pdf)
//! is a power management chip made by X-Powers for use mainly
//! in Allwinner ARM system-on-a-chip reference designs. It can be found
//! in quite a few systems, so if you've got a strange need to control
//! it from a supported Rust target, this'll get you there.
//! 
//! The machines I know of so far using it:
//! * [Next Thing Co.'s C.H.I.P computer](https://getchip.com/pages/chip)
//! * [Retro Games Ltd's C64 Mini](https://thec64.com/about/)
//! * [Cubietech's early Cubieboards (A10, A20, Cubietruck)](http://cubieboard.org/model/)
//!
//! The big challenge with interfacing with this chip is that because
//! it controls power, the kernel has exclusive access to it. That's
//! probably a good thing if you don't know what you're doing, and
//! if you brick your little computer, that's on you. 😜
//! 
//! This driver doesn't go that far yet and only supports the following
//! features:
//! * Reading various channels of the ADC:
//!   * Internal temperature
//!   * Battery level (0 - 100%)
//!   * Battery voltage, charging amperage, draining amperage
//!   * VBUS voltage, amperage (usually tied to USB input)
//!   * Power input voltage, amperage (additional external power)
//!   * TS input (for external temperatures or backup battery)
//!   * Voltage at GPIO 0 or 1
//! * Reading the power status
//!   * Is external power coming in?
//!   * Where's it coming from?
//!   * Is there a battery attached?
//! * Using the internal 127 minute timer (see `timer_control`)
//! * Turning various output voltages on and off
//! 
//! If there's a feature you'd like to see implemented, either
//! [open an issue](https://github.com/RandomInsano/axp209-rs/issues)
//! or create a pull request if you're feeling helpful. Also feel free
//! to open an issue if you have general questions.
//! 
//! If you need some examples, check the `examples` folder. They won't work until
//! if you have a kernel that is accessing to the chip so expect some difficulty
//! there. Because I was using the chip on a single board computer, I had to
//! re-compile my kernel with every AXP20X feature disabled.

#![no_std]
#![feature(rustc_private)]
#![deny(warnings)]

extern crate embedded_hal as hal;
#[macro_use]
extern crate bitflags;
extern crate byteorder;

pub mod adc_control;
pub mod power_status;
pub mod power_control;
pub mod charging_status;
pub mod timer_control;

pub use self::adc_control::AdcControl;
pub use self::power_status::PowerStatus;
pub use self::power_control::PowerControl;
pub use self::charging_status::ChargingStatus;
pub use self::timer_control::TimerControl;

use byteorder::{ByteOrder, BigEndian};
use hal::blocking::i2c::{Read, Write, WriteRead};

pub const BATTERY_LEVEL_MISSING: u8 = 0x7f;
/// The address can't be changed
const ADDRESS: u8 = 0x34;

enum Registers {
    /// Power status and control registers
    PowerStatus = 0x00,
    ChargingStatus = 0x01,
    PowerControl = 0x12,
    TimerControl = 0x8a,

    /// ADC Control
    AdcControl = 0x82,    

    /// ADC Value registers
    AcinVoltage = 0x56,
    AcinCurrent = 0x58,
    VbusVoltage = 0x5a,
    VbusCurrent = 0x5c,
    Temperature = 0x5e,
    BatteryTemperature = 0x62,
    Gpio0Voltage = 0x64,
    Gpio1Voltage = 0x66,
    //InstantaniousBatteryPower = 0x70, // Three bytes?!
    BatteryVoltage = 0x78,
    BatteryChargeCurrent = 0x7a,
    BatteryDischargeCurrent = 0x7c,
    SystemIpsout = 0x7e,

    //CoulombBattery = 0xb0,
    //CoulombBatteryDischarge = 0xb4,
    //CoulombBatteryEncryption = 0xb8,
    BatteryLevel = 0xb9,
}

pub struct Axp209<I2C> {
    device: I2C,
}

impl<I2C, E> Axp209<I2C>
where
    I2C: WriteRead<Error = E> + Write<Error = E> + Read<Error = E>,
{
    pub fn new(dev: I2C) -> Self {
        Axp209 {
            device: dev,
        }
    }

    fn write_read_byte(&mut self, send: u8) -> Result<u8, E> {
        let comm: [u8; 1] = [ send ];
        let mut buf: [u8; 1] = [0];
        self.device.write_read(ADDRESS, &comm, &mut buf)?;

        Ok(buf[0])
    }

    /// Many ADC functions on this chip provide their values as a strange
    /// 10bit value that requires some funky shifting
    fn get_adc_12bits(&mut self, register: u8) -> Result<u16, E> {
        let comm: [u8; 1] = [ register ];
        let mut recv: [u8; 2] = [ 0, 0 ];
        let mut value: u16;

        self.device.write_read(ADDRESS, &comm, &mut recv)?;

        // Weird way to store a number if ye ask me!
        value = (recv[0] as u16) << 4;
        value |= recv[1] as u16 & 0x0f;

        Ok(value)
    }

    fn get_8bit_register(&mut self, register: u8) -> Result<u8, E> {
        let comm: [u8; 1] = [ register ];
        let mut buf: [u8; 1] = [0];

        self.device.write_read(ADDRESS, &comm, &mut buf)?;

        Ok(buf[0])
    }

    fn set_8bit_register(&mut self, register: u8, value: u8) -> Result<(), E> {
        let comm: [u8; 2] = [ register, value ];

        self.device.write(ADDRESS, &comm)?;

        Ok(())
    }

    fn get_16bit_register(&mut self, register: u8) -> Result<u16, E> {
        let comm: [u8; 1] = [ register ];
        let mut buf: [u8; 2] = [0, 0];

        self.device.write_read(ADDRESS, &comm, &mut buf)?;

        Ok(BigEndian::read_u16(&buf))
    }

    pub fn adc_control(&mut self) -> Result<AdcControl, E> {
        Ok(AdcControl::new(self.get_16bit_register(Registers::AdcControl as u8)?))
    }

    pub fn power_status(&mut self) -> Result<PowerStatus, E> {
        Ok(PowerStatus::new(self.get_8bit_register(Registers::PowerStatus as u8)?))
    }

    pub fn power_control(&mut self) -> Result<PowerControl, E> {
        Ok(PowerControl::new(self.get_8bit_register(Registers::PowerControl as u8)?))
    }

    pub fn set_power_control(&mut self, value: PowerControl) -> Result<(), E> {
        Ok(self.set_8bit_register(Registers::PowerControl as u8, value.bits())?)
    }

    pub fn charging_status(&mut self) -> Result<ChargingStatus, E> {
        Ok(ChargingStatus::new(self.get_8bit_register(Registers::ChargingStatus as u8)?))
    }

    pub fn timer_control(&mut self) -> Result<TimerControl, E> {
        Ok(TimerControl::new(self.get_8bit_register(Registers::TimerControl as u8)?))
    }

    pub fn set_timer_control(&mut self, value: TimerControl) -> Result<(), E> {
        Ok(self.set_8bit_register(Registers::TimerControl as u8, value.bits())?)
    }

    /// In milliamps
    pub fn battery_discharging_current(&mut self) -> Result<u16, E> {
        let comm: [u8; 1] = [ Registers::BatteryDischargeCurrent as u8 ];
        let mut recv: [u8; 2] = [ 0, 0 ];
        let mut value: u16;

        self.device.write_read(ADDRESS, &comm, &mut recv)?;

        // Of course one would have 5 least significant bits and
        // ruin my get_adc_12bits function above!
        value = (recv[0] as u16) << 5;
        value |= recv[1] as u16 & 0x1f;

        Ok(value / 2)
    }    

    /// In millivolts
    pub fn battery_voltage(&mut self) -> Result<u16, E> {
        let mut value = self.get_adc_12bits(Registers::BatteryVoltage as u8)?;

        // Voltage is in 1.1mV increments, so just add 1/10 the value and
        // avoid those pesky floating point multiplications. :D
        value += value / 10;

        Ok(value)
    }

    /// In milliamps
    pub fn battery_charging_current(&mut self) -> Result<u16, E> {
        let value = self.get_adc_12bits(Registers::BatteryChargeCurrent as u8)?;

        Ok(value / 2)
    }

    /// In millivolts
    pub fn acin_voltage(&mut self) -> Result<u16, E> {
        let mut value = self.get_adc_12bits(Registers::AcinVoltage as u8)?;

        value += value / 7;

        Ok(value)
    }

    /// In milliamps
    pub fn acin_current(&mut self) -> Result<u16, E> {
        let mut value = self.get_adc_12bits(Registers::AcinCurrent as u8)?;

        // Trying to avoid too much rounding as it's multiples of 0.625 milliamps.
        // For similar odd math with explination, check out vbus_current()
        value = ((value * 16) / 10) / 16;

        Ok(value)
    }

    /// In milliamps
    pub fn vbus_voltage(&mut self) -> Result<u16, E> {
        let mut value = self.get_adc_12bits(Registers::VbusVoltage as u8)?;

        value += value / 7;

        Ok(value)
    }

    /// In milliamps
    pub fn vbus_current(&mut self) -> Result<u16, E> {
        let mut value = self.get_adc_12bits(Registers::VbusCurrent as u8)?;

        // Trying to avoid too much rounding as it's multiples of 0.375 milliamps
        // The max this register will return is 4096, so we have enough headroom
        // to multiply by 16, and 0.375*16 (probably by design) comes out as 6.
        value = ((value * 16) / 6) / 16;

        Ok(value / 2)
    }

    /// In celcius
    pub fn temperature(&mut self) -> Result<i16, E> {
        // Check out page 25 of the datasheet for the weird math

        let value = self.get_adc_12bits(Registers::Temperature as u8)?;

        let mut value = value as i16 / 10;
        value -= 145;

        Ok(value)
    }

    /// In millivolts. Battery temperature sensor
    pub fn ts_voltage(&mut self) -> Result<u16, E> {
        let value = self.get_adc_12bits(Registers::BatteryTemperature as u8)?;

        // Increments of 0.8
        Ok((value * 8) / 10)
    }

    /// In millivolts. I'm assuming power division is 1.4 as defined in APS, but
    /// as there is nothing in the datasheet specifically for Ipsout's settings
    /// and there is no register defined for ipsout.
    pub fn ipsout_voltage(&mut self) -> Result<u16, E> {
        let value = self.get_adc_12bits(Registers::SystemIpsout as u8)?;

        // Increments of 1.4
        Ok((value * 14) / 10)
    }

    /// In millivolts. Unconfirmed
    pub fn gpio0_voltage(&mut self) -> Result<u16, E> {
        let value = self.get_adc_12bits(Registers::Gpio0Voltage as u8)?;

        Ok(value / 2)
    }

    /// In millivolts. Unconfirmed
    pub fn gpio1_voltage(&mut self) -> Result<u16, E> {
        let value = self.get_adc_12bits(Registers::Gpio1Voltage as u8)?;

        Ok(value / 2)
    }

    // In percentage.
    pub fn battery_level(&mut self) -> Result<u8, E> {
        // The MSB for the voltage is a control bit that enables or
        // disables sampling
        match self.write_read_byte(Registers::BatteryLevel as u8) {
            Ok(x) => Ok(x & 0b0111_1111),
            Err(x) => Err(x)
        }
    }

    pub fn battery_present(&mut self) -> Result<bool, E> {
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
        let _level = pmic.battery_level().unwrap();

        // Values for 'level' can be either the percentage, or
        // 0x7F if the battery is missing
    }
}

