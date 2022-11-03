#![no_std]
#![no_main]

use blesf::hal::pac::Peripherals;

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = Peripherals::take().unwrap();

    // Initialize CLOCK
    let clock = p.CLOCK;
    clock
        .lfclksrc
        .write(|w| w.src().xtal().bypass().bit(false).external().bit(false));
    clock.tasks_lfclkstart.write(|w| unsafe { w.bits(1) });

    // Initialize RTC
    let rtc = p.RTC0;
    let cmp = 0; // select Compare0
    rtc.evtenset.write(|w| w.compare0().set()); // enable event
    rtc.cc[cmp].write(|w| unsafe { w.bits(100) }); // set the required counter value to emit an event
    rtc.tasks_start.write(|w| unsafe { w.bits(1) }); // start counter

    loop {
        // print current counter value
        let counter = rtc.counter.read().bits();
        defmt::println!("RTC0: {}", counter);

        if counter >= 200 {
            cortex_m::asm::bkpt();
        }

        // wait for the event from compare 0
        if rtc.events_compare[cmp].read().bits() == 1 {
            defmt::println!("RTC Compare0");
            rtc.events_compare[cmp].write(|w| unsafe { w.bits(0) });
        }
    }
}
