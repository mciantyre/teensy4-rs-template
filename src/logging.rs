//! USB logging support
//!
//! If you don't want USB logging, remove
//!
//! - this module
//! - the `log` dependency in Cargo.toml

use super::bsp::{self, hal::ral::usb::USB1, interrupt, usb};

/// You can specify any USB logging filters here
///
/// See the teensy4_bsp docs for more information on
/// logging filters.
const FILTERS: &[usb::Filter] = &[];

/// Initialize the USB logging system
///
/// You can only call this function once! Do it early in your
/// program.
///
/// # Panics
///
/// Panics if the USB1 peripheral instance is already taken.
pub fn init() {
    assert!(usb::init(
        USB1::take().unwrap(),
        usb::LoggingConfig {
            filters: FILTERS,
            ..Default::default()
        }
    )
    .is_ok());

    // Safety: This call is safe as long as you use the USB
    // interrupt handler supplied by this module. If you add
    // any other state to the handler, you'll need to assess
    // the safety of this call yourself.
    unsafe { cortex_m::peripheral::NVIC::unmask(bsp::interrupt::USB_OTG1) };
}

/// Drive the USB logging system by calling `poll` in the USB
/// interrupt handler. Without this, you might not see a USB
/// device connected to your host.
///
/// # Safety
///
/// Users should not call this function. It will be added to the
/// interrupt vector table, and invoked by the hardware.
#[cortex_m_rt::interrupt]
unsafe fn USB_OTG1() {
    usb::poll();
}
