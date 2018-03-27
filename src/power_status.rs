bitflags! {
    /// The Power Status register hold information about the power status.
    /// All bits here are read-only.

    pub struct PowerStatus: u8 {
        /// Whether there is power coming in on the ACIN pin or not. Note that
        /// this is not the power coming from USB on the NTC C.H.I.P. but it
        /// may be wired differently in other applications
        const ACIN_PRESENT = 1 << 7;
        /// Whether the voltage coming in the ACIN pin is enough to power the
        /// system.
        const ACIN_USABLE = 1 << 6;
        /// Whether there is power being provided via USB or not
        const VBUS_PRESENT = 1 << 5;
        /// Whether the voltage coming in via USB is enough to power the system or not
        const VBUS_USABLE = 1 << 4;
        /// Whether the incoming voltage is above the configured VHOLD value.
        const VBUS_ABOVE_HOLD = 1 << 3;
        /// State of the battery (Charging = true, Discharging = false)
        const DISCHARGING = 1 << 2;
        /// I'm not quite sure here. The datasheet says a short circuit between VBUS and ACIN
        const SHORT_CIRCUIT = 1 << 1; // A fine movie
        /// If the chip powers on the system when power is appled. Not sure where this can be
        /// set.
        const START_ON_POWER = 1 << 0;
    }
}

impl PowerStatus {
    /// No checks are made here, and it's expected that it be populated by
    /// the raw value from the axp209 chip.
    pub fn new(value: u8) -> Self {
        Self {
            bits: value
        }
    }
}