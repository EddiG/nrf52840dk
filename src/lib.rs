#![no_std]

use defmt_rtt as _;
pub use nrf52840_hal as hal;
use panic_probe as _;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
