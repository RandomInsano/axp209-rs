// TODO: Is this much wrapping actually worthwhile? What is the expectation? Check other crates.

bitflags! {
	pub struct AdcStatus: u16 {
		const BATTERY_VOLTAGE = 1 << 15;
		const BATTERY_CURRENT = 1 << 14;
		const ACIN_VOLTAGE = 1 << 13;
		const ACIN_CURRENT = 1 << 12;
		const VBUS_VOLTAGE = 1 << 11;
		const VBUS_CURRENT = 1 << 10;
		const APS_VOLTAGE = 1 << 9; // Not sure what theses do yet
		const TS_FUNCTION = 1 << 8; // so I'm not implementing access

		const TEMPERATURE = 1 << 7;
		const GPIO0 = 1 << 3;
		const GPIO1 = 1 << 2;
	}
}

impl AdcStatus {
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