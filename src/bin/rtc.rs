#![no_std]
#![no_main]

use blesf::hal::{
    clocks::{self, Clocks},
    pac::Peripherals,
    rtc::{Rtc, RtcCompareReg, RtcInterrupt},
};

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
    rtc.enable_event(RtcInterrupt::Compare0);
    let _ = rtc.set_compare(RtcCompareReg::Compare0, 100);
    rtc.enable_counter();

    loop {
        let counter = rtc.get_counter();
        defmt::println!("RTC0: {}", counter);

        if counter >= 200 {
            cortex_m::asm::bkpt();
        }

        if rtc.is_event_triggered(RtcInterrupt::Compare0) {
            defmt::println!("RTC0 Compare0");
            rtc.reset_event(RtcInterrupt::Compare0);
        }
    }
}
