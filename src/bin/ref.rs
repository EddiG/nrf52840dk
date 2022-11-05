#![no_std]
#![no_main]

use blesf as _;

#[cortex_m_rt::entry]
fn main() -> ! {
    let immutable_u8_val: u8 = 200u8;
    let _borrow_immutable_u8_val: &u8 = &immutable_u8_val;
    // cannot borrow `immutable_u8_val` as mutable, as it is not declared as mutable
    // let _borrow_mutable_u8_val: &mut u8 = &mut immutable_u8_val;

    let mut mutable_u8_val: u8 = 200u8;
    let _borrow_immutable_u8_val: &u8 = &mutable_u8_val;
    let borrow_mutable_u8_val: &mut u8 = &mut mutable_u8_val;
    *borrow_mutable_u8_val += 1; // dereference and mutate

    // 0xE000_E010 is a memory address
    // *const means raw pointer of immutable value
    let const_raw_ref: *const u32 = 0xE000_E010 as *const u32;
    // dereference the raw pointer to u32
    // it's unsafe because Rust compiler doesn't know anything about 0xE000_E010 memory address
    let immutable_val: u32 = unsafe { *const_raw_ref };
    let _borrow_immutable_val: &u32 = &immutable_val;
    //
    let mut mutable_val: u32 = unsafe { *const_raw_ref };
    let borrow_mutable_val: &mut u32 = &mut mutable_val;
    *borrow_mutable_val += 1; // dereference and mutate

    // cannot borrow `*const_raw_ref` as mutable, as it is behind a `*const` pointer
    // let borrow_mutable_val: &mut u32 = unsafe { &mut *const_raw_ref };
    // not sure that this is correct, but Rust compiler seems ok with that
    let borrow_mutable_val_from_const_pointer: &mut u32 = &mut unsafe { *const_raw_ref };
    *borrow_mutable_val_from_const_pointer += 1; // dereference and mutate

    // *mut means raw pointer for muttable value
    let mut_raw_ref: *mut u32 = 0xE000_E010 as *mut u32;
    // dereference the raw pointer to u32
    let borrow_mutable_val: &mut u32 = unsafe { &mut *mut_raw_ref };
    *borrow_mutable_val += 1;

    // all in one
    let val: &mut u32 = unsafe { &mut *(0xE000_E010 as *mut u32) };
    *val += 1;

    loop {
        cortex_m::asm::bkpt();
    }
}
