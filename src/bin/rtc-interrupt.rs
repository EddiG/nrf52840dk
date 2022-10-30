#![no_std]
#![no_main]

use blesf::hal::{
    clocks::{self, Clocks},
    pac::{interrupt, Interrupt, Peripherals, NVIC},
    rtc::{Rtc, RtcInterrupt},
};

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[interrupt]
fn RTC0() {
    defmt::println!("RTC0 Interrupt");
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("RTC Example");

    let p = Peripherals::take().unwrap();

    // Setup clocks
    Clocks::new(p.CLOCK)
        .set_lfclk_src_external(clocks::LfOscConfiguration::NoExternalNoBypass)
        .start_lfclk();

    // Setup RTC0 interrupts
    let mut rtc = Rtc::new(p.RTC0, 0).unwrap();
    rtc.enable_interrupt(RtcInterrupt::Tick, None);
    unsafe { NVIC::unmask(Interrupt::RTC0) };
    rtc.enable_counter();

    loop {}
}
