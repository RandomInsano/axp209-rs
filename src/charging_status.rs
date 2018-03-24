bitflags! {
	pub struct ChargingStatus: u8 {
		const OVERTEMPERATURE = 1 << 7;
        const CHARGING = 1 << 6;
        const BATTERY_PRESENT = 1 << 5;
        const CELL_ACTIVATION_MODE = 1 << 3;    // "enetered cell activation mode"?!
        const CHARGE_CURRENT_LOW = 1 << 2;
    }
}

impl ChargingStatus {
    pub fn new(value: u8) -> Self {
        Self {
            bits: value
        }
    }
}