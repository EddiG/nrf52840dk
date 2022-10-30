#![no_std]
#![no_main]

use blesf::hal::{
    clocks::{self, Clocks},
    pac::{interrupt, Interrupt, Peripherals, NVIC},
    timer::Timer,
};

#[allow(non_camel_case_types)]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[interrupt]
fn TIMER1() {
    defmt::println!("TIMER1");
}

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("TIMER1 Example");

    let periph = Peripherals::take().unwrap();

    // Setup clocks
    Clocks::new(periph.CLOCK)
        .set_lfclk_src_external(clocks::LfOscConfiguration::NoExternalNoBypass)
        .start_lfclk();

    // Setup TIMER1
    let mut timer = Timer::new(periph.TIMER1);
    timer.enable_interrupt();
    unsafe { NVIC::unmask(Interrupt::TIMER1) };
    timer.delay(1000000);

    loop {
        // cortex_m::asm::bkpt()
        defmt::println!("Main");
    }
}
