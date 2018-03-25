bitflags! {
	pub struct PowerStatus: u8 {
		const ACIN_PRESENT = 1 << 7;
        const ACIN_USABLE = 1 << 6;
        const VBUS_PRESENT = 1 << 5;
        const VBUS_USABLE = 1 << 4;
        const VBUS_ABOVE_HOLD = 1 << 3;
        const DISCHARGING = 1 << 2;
        const SHORT_CIRCUIT = 1 << 1; // A fine movie
        const START_ON_POWER = 1 << 0; // If the PMIC boots when ACIN/VBUS connected
    }
}

impl PowerStatus {
    pub fn new(value: u8) -> Self {
        Self {
            bits: value
        }
    }
}