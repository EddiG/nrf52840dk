#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    loop {
        defmt::println!("Hello world!");
        cortex_m::asm::bkpt();
    }
}

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
