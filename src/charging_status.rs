bitflags! {
    /// The Charging Status register reports on the state of the battery
    /// and if the chip itself is overtemperature
    pub struct ChargingStatus: u8 {
        /// Whether the chip is at an unsafe temperature
        const OVERTEMPERATURE = 1 << 7;
        /// Whether the battery is charging
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