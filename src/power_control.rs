//! The Power Control register allows turning on and off various voltages
//! supplied by the chip. This allows things like shutting off the WiFi
//! chip or USB/5v rails on the NTC C.H.I.P. computer at their source. 


bitflags! {
    /// Holds the state of the register. Changes will need to be committed manually
    pub struct PowerControl: u8 {
        /// The voltage supplied on the LDO3 pin
        const LDO3 = 1 << 6;
        /// The voltage supplied on the DCDC2 pin
        const DCDC2 = 1 << 4;
        /// The voltage supplied on the LDO4 pin
        const LDO4 = 1 << 3;
        /// The voltage supplied on the LDO2 pin
        const LDO2 = 1 << 2;
        /// The voltage supplied on the DCDC3 pin
        const DCDC3 = 1 << 1;
        /// The state of the EXTEN pin (external power enable)
        /// As an example, the CHIP uses this to enable the
        /// 5v rail via the LP6226 chip that controls the
        /// 5v to USB, pin 3 on the U13 header, and pin 2
        /// on U14.
        const EXTEN = 1 << 0;
    }
}

impl PowerControl {
    /// No checks are made here, and it's expected that it be populated by
    /// the raw value from the axp209 chip.
    pub fn new(value: u8) -> Self {
        Self {
            bits: value
        }
    }
}