//! The starter code slowly blinks the LED, and sets up
//! USB logging.

#![no_std]
#![no_main]

use teensy4_bsp as bsp;
use teensy4_panic as _;

mod logging;

const LED_PERIOD_MS: u32 = 1_000;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = bsp::Peripherals::take().unwrap();
    let mut systick = bsp::SysTick::new(cortex_m::Peripherals::take().unwrap().SYST);
    let pins = bsp::t40::into_pins(p.iomuxc);
    let mut led = bsp::configure_led(pins.p13);

    // See the `logging` module docs for more info.
    assert!(logging::init().is_ok());

    loop {
        led.toggle();
        systick.delay(LED_PERIOD_MS);
        log::info!("Hello world");
    }
}
