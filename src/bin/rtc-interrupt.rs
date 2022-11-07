#![no_std]
#![no_main]

use blesf::hal::{
    clocks::{self, Clocks},
    pac::{interrupt, Interrupt, Peripherals, NVIC},
    rtc::{Rtc, RtcInterrupt},
};

static mut COUNT: u8 = 0;

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[interrupt]
fn RTC0() {
    defmt::println!("RTC0 Interrupt {}", unsafe {
        COUNT += 1;
        COUNT
    });
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("RTC Example");

    let Some(p) = Peripherals::take() else {
        defmt::panic!("The Peripherals cannot be taken");
    };

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
