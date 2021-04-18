The [AXP209](http://dl.linux-sunxi.org/AXP/AXP209_Datasheet_v1.0en.pdf)
is a power management chip made by X-Powers for use mainly
in Allwinner ARM system-on-a-chip reference designs. It can be found
in quite a few systems, so if you've got a strange need to control
it from a supported Rust target, this'll get you there.

The machines I know of so far using it:
* [Next Thing Co.'s C.H.I.P computer](https://getchip.com/pages/chip)
* [Retro Games Ltd's C64 Mini](https://thec64.com/about/)
* [Cubietech's early Cubieboards (A10, A20, Cubietruck)](http://cubieboard.org/model/)

The big challenge with interfacing with this chip is that because
it controls power, the kernel has exclusive access to it. That's
probably a good thing if you don't know what you're doing, and
if you brick your little computer, that's on you. 😜

This driver doesn't try to implement every feature yet and only supports
the following:
* Reading various channels of the ADC
  * Internal temperature
  * Battery level (0 - 100%)
  * Battery voltage, charging amperage, draining amperage
  * VBUS voltage, amperage (usually tied to USB input)
  * Power input voltage, amperage (additional external power)
  * TS input (for external temperatures or backup battery)
  * Voltage at GPIO 0 or 1
* Reading the power status
  * Is external power coming in?
  * Where's it coming from?
  * Is there a battery attached?
  * Is that battery (dis)charging?
* Using the internal 127 minute timer (see `timer_control`)
* Turning various output voltages on and off

Here's the output from the example program which runs on the PocketChip:

```text
Battery level: 100%
Voltage: 4184mV
Discharge Current: 0mA
Charge Current:    0mA
ACIN Voltage:      0mV
ACIN Current:      0mA
Vbus Voltage:      0mV
Vbus Current:      0mA
Temperature:       47°C
Temp Sensor Pin:   0mV
Ipsout?!:          4583mV

ADC Control Flags: BATTERY_VOLTAGE | BATTERY_CURRENT | ACIN_VOLTAGE | ACIN_CURRENT | APS_VOLTAGE | TS | TEMPERATURE
Power Control Flags: LDO3 | DCDC2 | LDO4 | LDO2 | DCDC3 | EXTEN
Power Status Flags:  VBUS_PRESENT | VBUS_USABLE | VBUS_ABOVE_HOLD | START_ON_POWER
Charge Status Flags: BATTERY_PRESENT

Timer:
	Expired: false
	Time (minutes): 0
```

If there's a feature you'd like to see implemented, either
[open an issue](https://github.com/RandomInsano/axp209-rs/issues)
or create a pull request if you're feeling helpful. Also feel free
to open an issue if you have general questions.

If you need some examples, check the `examples` folder. They won't work until
if you have a kernel that is accessing to the chip so expect some difficulty
there. Because I was using the chip on a single board computer, I had to
re-compile my kernel with every AXP20X feature disabled.

## License

Cargo is primarily distributed under the terms of both the MIT license
and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.

