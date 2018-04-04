// TODO: Is this much wrapping actually worthwhile? What is the expectation? Check other crates.

bitflags! {
    pub struct AdcControl: u16 {
        /// Register address to read battery voltage from
        const BATTERY_VOLTAGE = 1 << 15;
        /// Register address to read battery current from
        const BATTERY_CURRENT = 1 << 14;
        /// Register address to read inbound power voltage from
        const ACIN_VOLTAGE = 1 << 13;
        /// Register address to read inbound power current from
        const ACIN_CURRENT = 1 << 12;
        /// Register address to read USB power voltage from
        const VBUS_VOLTAGE = 1 << 11;
        /// Register address to read USB power amperage from
        const VBUS_CURRENT = 1 << 10;
        /// Register address to test internal voltage to make sure it's still accurate
        const APS_VOLTAGE = 1 << 9;
        /// Register address for temperature sensor for battery though
        /// it can be used for any voltage between 0 and 3.3v
        const TS_FUNCTION = 1 << 8;
        /// Register address for the internal temperature
        const TEMPERATURE = 1 << 7;
        /// Register address to read voltage from GPIO pin 0 (if configured)
        const GPIO0 = 1 << 3;
        /// Register address to read voltage from GPIO pin 1 (if configured)
        const GPIO1 = 1 << 2;
    }
}

impl AdcControl {
    pub fn new(value: u16) -> Self {
        Self {
            bits: value,
        }
    }

    pub fn battery_voltage(&self) -> bool {
        self.contains(Self::BATTERY_VOLTAGE)
    }

    pub fn set_battery_voltage(&mut self, value: bool) {
        self.set(Self::BATTERY_VOLTAGE, value);
    }

    pub fn battery_current(&self) -> bool {
        self.contains(Self::BATTERY_CURRENT)
    }

    pub fn set_battery_current(&mut self, value: bool) {
        self.set(Self::BATTERY_CURRENT, value);
    }

    pub fn acin_voltage(&self) -> bool {
        self.contains(Self::ACIN_VOLTAGE)
    }

    pub fn set_acin_voltage(&mut self, value: bool) {
        self.set(Self::ACIN_VOLTAGE, value);
    }

    pub fn acin_current(&self) -> bool {
        self.contains(Self::ACIN_CURRENT)
    }

    pub fn set_acin_current(&mut self, value: bool) {
        self.set(Self::ACIN_CURRENT, value);
    }

    pub fn vbus_voltage(&self) -> bool {
        self.contains(Self::VBUS_VOLTAGE)
    }

    pub fn set_vbus_voltage(&mut self, value: bool) {
        self.set(Self::VBUS_VOLTAGE, value);
    }

    pub fn temperature(&self) -> bool {
        self.contains(Self::TEMPERATURE)
    }

    pub fn set_temperature(&mut self, value: bool) {
        self.set(Self::TEMPERATURE, value);
    }

    pub fn gpio0(&self) -> bool {
        self.contains(Self::GPIO0)
    }

    pub fn set_gpio0(&mut self, value: bool) {
        self.set(Self::GPIO0, value);
    }

    pub fn gpio1(&self) -> bool {
        self.contains(Self::GPIO1)
    }

    pub fn set_gpio1(&mut self, value: bool) {
        self.set(Self::GPIO1, value);
    }
}