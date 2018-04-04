//! The AXP209 contains a countdown timer. You won't see the time stored in
//! the register change over time, but once the specified amount of time has
//! passed (give or take) the `expired` field will be set to true.
//! 
//! Writing to the register with the `expired` bit set will (re)start the 
//! countdown. No testing has been done to see how accurate the time is, but
//! it's likely just an estimate.
//! 
//! For this library, to save communication overhead, changes to this struct
//! are not passed to the chip automatically and so commiting the changes is
//! required. Here's an example of how to set the timer for five minute and
//! busywait for it to expire:
//! 
//! ```
//!     // Use the timer on the NTC C.H.I.P. on Linux
//!     let i2c = I2cdev::new("/dev/i2c-0").unwrap();
//!     let mut pmic = Axp209::new(i2c);
//!     let timer = pmic.timer_control();
//!     
//!     timer.set_minutes(5);
//!     timer.set_expired(true);
//!     pmic.set_timer_control(timer);
//! 
//!     while {
//!         // You should do something meaninful here. 😃
//! 
//!         // The expiry will have reset when we set the timer
//!         let timer = pmic.timer_control();
//!         if timer.expired() {
//!             // The timer's done! Hazaa!
//!             break;
//!         }
//!     }
//! ```
//! 
//! The AXP209 has the ability to send an interrupt when the timer expires
//! but that hasn't been implemented here yet.

bitflags! {
    /// Defines the info about the timer. Some implmenetation fun you
    /// don't need to care about: The highest order bit denotes if the
    /// timer has expired (on read) or that you want to reset it
    /// (on write)
    pub struct TimerControl: u8 {
        /// Denotes if the timer has expired or not.
        const TIMER_EXPIRED = 1 << 7;
    }
}

impl TimerControl {
    /// Instantiate a new lovely bit of bits. :D
    /// 
    /// No checks are made here, and it's expected that it be populated by
    /// the raw value from the axp209 chip.
    pub fn new(value: u8) -> Self {
        Self {
            bits: value
        }
    }

    /// Read the number of minutes the timer is counting down from. 
    pub fn minutes(&self) -> u8 {
        let mut value: TimerControl = self.clone();
        value.set(Self::TIMER_EXPIRED, false);

        value.bits
    }

    /// Set the number of minutes. Value can be between 1 and 127.
    /// A value of zero will disable the countdown, so the `expired()`
    /// value will stay false.
    pub fn set_minutes(&mut self, value: u8) {
        assert!(value < 127, "Value can only be between 0 and 127 minutes");

        let expired = self.expired();

        self.bits = value;
        self.set(Self::TIMER_EXPIRED, expired);
    }

    /// Read whether the timer has expired.
    pub fn expired(&self) -> bool {
        self.contains(Self::TIMER_EXPIRED)
    }

    /// Set whether the timer has expired.
    pub fn set_expired(&mut self, value: bool) {
        self.set(Self::TIMER_EXPIRED, value);
    }
}
