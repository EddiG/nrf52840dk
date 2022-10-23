#![no_std]
#![no_main]

use blesf as _;
use nrf52840_hal::{pac::Peripherals, temp::Temp};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Temperature Example");

    let per = Peripherals::take().unwrap();
    let mut temp_sensor = Temp::new(per.TEMP);
    let die_temp_c: i32 = temp_sensor.measure().to_num();
    defmt::println!("Processor temp is {:?}°C", die_temp_c);

    loop {
        cortex_m::asm::bkpt()
    }
}
