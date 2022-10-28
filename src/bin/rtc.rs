#![no_std]
#![no_main]

use blesf as _;
use nrf52840_hal::{
    clocks::{self, Clocks},
    pac::{interrupt, Interrupt, Peripherals, NVIC},
    rtc::{Rtc, RtcInterrupt},
};

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[interrupt]
fn RTC0() {
    defmt::println!("RTC0");
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("RTC Example");

    let periph = Peripherals::take().unwrap();

    // Setup clocks
    let clocks = Clocks::new(periph.CLOCK);
    let clocks = clocks.enable_ext_hfosc();
    let clocks = clocks.set_lfclk_src_external(clocks::LfOscConfiguration::NoExternalNoBypass);
    let clocks = clocks.start_lfclk();
    let _clocks = clocks.enable_ext_hfosc();
    defmt::debug!("Clocks configured");

    // Setup RTC0 interrupts
    let mut rtc = Rtc::new(periph.RTC0, 0).unwrap();
    rtc.enable_interrupt(RtcInterrupt::Tick, None);
    rtc.enable_counter();
    unsafe { NVIC::unmask(Interrupt::RTC0) };

    loop {
        // cortex_m::asm::bkpt()
    }
}
