#![no_std]
#![no_main]

use blesf as _;
use cortex_m::peripheral::{syst, Peripherals};

#[cortex_m_rt::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut systick = peripherals.SYST;
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(50_000);
    systick.clear_current();
    systick.enable_counter();
    while !systick.has_wrapped() {
        defmt::println!("Tick");
    }

    loop {
        cortex_m::asm::bkpt();
    }
}
