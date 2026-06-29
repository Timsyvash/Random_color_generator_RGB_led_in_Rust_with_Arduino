#![no_std]
#![no_main]

use panic_halt as _;

use arduino_hal::simple_pwm::IntoPwmPin;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut timer4 = arduino_hal::simple_pwm::Timer4Pwm::new(dp.TC4,
    arduino_hal::simple_pwm::Prescaler::Prescale1024);

    let mut red_led = pins.d6.into_output().into_pwm(&mut timer4);
    let mut green_led = pins.d7.into_output().into_pwm(&mut timer4);
    let mut blue_led = pins.d8.into_output().into_pwm(&mut timer4);

    let hw_seed: u64 = 256;

    let mut rng = fastrand::Rng::with_seed(hw_seed);

    red_led.enable();
    green_led.enable();
    blue_led.enable();

    red_led.set_duty(255);
    green_led.set_duty(255);
    blue_led.set_duty(255);

    loop {
        let rng_for_red_led = rng.u8(0..=255);
        let rng_for_green_led = rng.u8(0..=255);
        let rng_for_blue_led = rng.u8(0..=255);

        red_led.set_duty(rng_for_red_led);
        green_led.set_duty(rng_for_green_led);
        blue_led.set_duty(rng_for_blue_led);
        arduino_hal::delay_ms(1500);
    }
}
