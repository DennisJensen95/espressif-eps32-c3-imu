#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::{print, println};
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, Delay, IO};

use core::fmt::Write;

use esp_backtrace as _;
use hal::{i2c::I2C, timer::TimerGroup, Rtc, UsbSerialJtag};
use icm42670::{prelude::*, Address, Icm42670};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    println!("Hello world!");

    // Set GPIO7 as an output, and set its state high initially.
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = io.pins.gpio7.into_push_pull_output();

    led.set_high().unwrap();

    // Initialize the Delay peripheral, and use it to toggle the LED state in a
    // loop.
    let mut delay = Delay::new(&clocks);

    let timer_group0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(peripherals.TIMG1, &clocks);
    let mut wdt1 = timer_group1.wdt;

    wdt0.disable();
    wdt1.disable();

    let mut delay = Delay::new(&clocks);
    // let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio10,
        io.pins.gpio8,
        400u32.kHz(),
        &clocks,
    );

    let mut icm = Icm42670::new(i2c, Address::Primary).unwrap();

    loop {
        // led.toggle().unwrap();
        // delay.delay_ms(500u32);

        let accel_norm = icm.accel_norm().unwrap();
        let gyro_norm = icm.gyro_norm().unwrap();

        print!(
            "ACCEL=X: {:+.04} Y: {:+.04} Z: {:+.04}\t\tGYRO  =  X: {:+.04} Y: {:+.04} Z: {:+.04}\r",
            accel_norm.x, accel_norm.y, accel_norm.z, gyro_norm.x, gyro_norm.y, gyro_norm.z
        );

        delay.delay_ms(100u32);
    }
}
