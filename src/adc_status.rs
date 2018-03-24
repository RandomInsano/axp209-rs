bitflags! {
	pub struct AdcStatus: u16 {
		const BATTERY_VOLTAGE = 1 << 15;
		const BATTERY_CURRENT = 1 << 14;
		const ACIN_VOLTAGE = 1 << 13;
		const ACIN_CURRENT = 1 << 12;
		const VBUS_VOLTAGE = 1 << 11;
		const VBUS_CURRENT = 1 << 10;
		const APS_VOLTAGE = 1 << 9;
		const TS_FUNCTION = 1 << 8;

		const TEMPERATURE = 1 << 7;
		const GPIO1 = 1 << 3;
		const GPIO2 = 1 << 2;
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
}