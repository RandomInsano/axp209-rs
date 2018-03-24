bitflags! {
    /// The AXP209 contains a countdown timer. Setting the time, then resetting
    /// the expired field will set an internal countdown that doesn't change the
    /// time in the register. Once the specified amount of time has passed (give
    /// or take) the 'expired' field will be set to true.
    /// 
    /// Setting 'expired' will reset the timer to zero, so you can re-use the
    /// the value contained in the register and reset at any time by setting
    /// expired to true and writing the register.
	pub struct TimerControl: u8 {
		const TIMER_EXPIRED = 1 << 7;
    }
}

impl TimerControl {
    pub fn new(value: u8) -> Self {
        Self {
            bits: value
        }
    }

    pub fn minutes(&self) -> u8 {
        let mut value: TimerControl = self.clone();
        value.set(Self::TIMER_EXPIRED, false);

        value.bits
    }

    pub fn set_minutes(&mut self, value: u8) {
        let expired = self.expired();

        self.bits = value;
        self.set(Self::TIMER_EXPIRED, expired);
    }

    pub fn expired(&self) -> bool {
        self.contains(Self::TIMER_EXPIRED)
    }

    pub fn set_expired(&self, value: bool) {
        self.set(Self::TIMER_EXPIRED, value);
    }
}