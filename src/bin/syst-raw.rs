#![no_std]
#![no_main]

use blesf as _;
use cortex_m_rt::exception;

// use repr(C) attribute to keep the order of the fields unchangable as it is in C
#[repr(C)]
struct SysTick {
    csr: u32,
    rvr: u32,
    cvr: u32,
    calib: u32,
}

#[exception]
fn SysTick() {
    defmt::println!("SysTick exception");
}

#[cortex_m_rt::entry]
fn main() -> ! {
    // 0xE000_E010 is a first address in the SysTick's set of registers
    let systick = unsafe { &mut *(0xE000_E010 as *mut SysTick) };
    unsafe { core::ptr::write_volatile(&mut systick.rvr, 16_000_000u32) }; // set reload value
    unsafe { core::ptr::write_volatile(&mut systick.cvr, 0u32) }; // clear current value
    unsafe { core::ptr::write_volatile(&mut systick.csr, 7u32) }; // run counter and enable SysTick exception

    let mut count = 0u8;
    // 64 MHz it is 64_000_000 ticks, but the SysTick reload value is limited to 16_777_215
    // that's why we use 16_000_000 that gives us the same 64_000_000 after 4 counts
    let cps = (64_000_000u32 / 16_000_000u32) as u8;
    let timer = 5u8; // 5 seconds

    loop {
        if unsafe { core::ptr::read_volatile(&systick.csr) } & (1 << 16) != 0 {
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
