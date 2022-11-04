#![no_std]
#![no_main]

use blesf as _;
use cortex_m::peripheral::{syst, Peripherals};

#[cortex_m_rt::entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut systick = peripherals.SYST;
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(16_000_000u32); // 0x00FFFFFF is maximum possible value
    systick.clear_current();
    systick.enable_counter();

    let mut count = 0u8;
    // 64 MHz it is 64_000_000 ticks, but the SysTick reload value is limited to 16_777_215
    // that's why we use 16_000_000 that gives us the same 64_000_000 after 4 counts
    let cps = (64_000_000u32 / 16_000_000u32) as u8;
    let timer = 5u8; // 5 seconds

    loop {
        if systick.has_wrapped() {
            count += 1;

            if count % cps == 0 {
                defmt::println!("{} sec.", count / cps);
            }
        }

        if count / cps == timer {
            cortex_m::asm::bkpt();
        }
    }
}
