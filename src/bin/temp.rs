#![no_std]
#![no_main]

use blesf::hal::{pac::Peripherals, temp::Temp};

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Temperature Example");

    let Some(p) = Peripherals::take() else {
        defmt::panic!("The Peripherals cannot be taken");
    };

    let mut temp_sensor = Temp::new(p.TEMP);
    let die_temp_c: i32 = temp_sensor.measure().to_num();
    defmt::println!("Processor temp is {:?}°C", die_temp_c);

    loop {
        cortex_m::asm::bkpt()
    }
}
