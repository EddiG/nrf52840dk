#![no_std]
#![no_main]

use core::mem::MaybeUninit;

use blesf as _;
use nrf52840_hal as _;
pub use nrf_softdevice_s140 as raw;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello world!");

    // Enable SoftDevice
    let clock = raw::nrf_clock_lf_cfg_t {
        source: raw::NRF_CLOCK_LF_SRC_RC as u8,
        rc_ctiv: 4,
        rc_temp_ctiv: 2,
        accuracy: 7,
    };
    let ret = unsafe { raw::sd_softdevice_enable(&clock, Some(fault_handler)) };
    match ret {
        raw::NRF_SUCCESS => {
            defmt::println!("SoftDevice enabled");
        }
        _ => {
            defmt::println!("SoftDevice error {}", ret);
        }
    }

    // Enable BLE
    let mut wanted_app_ram_base = get_app_ram_base();
    let ret = unsafe { raw::sd_ble_enable(&mut wanted_app_ram_base as _) };
    match ret {
        raw::NRF_SUCCESS => {
            defmt::println!("BLE enabled");
            defmt::println!(
                "SoftDevice RAM: {:?} bytes",
                wanted_app_ram_base - 0x20000000
            );
        }
        _ => {
            defmt::println!("BLE error {}", ret);
        }
    }

    // Get BLE addr
    let mut addr: raw::ble_gap_addr_t = unsafe { core::mem::zeroed() };
    let _ret = unsafe { raw::sd_ble_gap_addr_get(&mut addr) };
    defmt::println!("BLE addr: {:02x}", addr.addr);

    // BLE Scan
    const BUF_LEN: usize = 256;
    static mut BUF: [u8; BUF_LEN] = [0u8; BUF_LEN];
    static mut BUF_DATA: raw::ble_data_t = raw::ble_data_t {
        p_data: unsafe { BUF.as_mut_ptr() },
        len: BUF_LEN as u16,
    };
    let mut scan_params: raw::ble_gap_scan_params_t = unsafe { core::mem::zeroed() };
    scan_params.set_extended(1);
    scan_params.set_active(0);
    scan_params.scan_phys = raw::BLE_GAP_PHY_AUTO as _;
    scan_params.timeout = raw::BLE_GAP_SCAN_TIMEOUT_UNLIMITED as u16;
    scan_params.interval = 2732;
    scan_params.window = 500;
    scan_params.set_filter_policy(raw::BLE_GAP_SCAN_FP_ACCEPT_ALL as u8);
    let ret = unsafe { raw::sd_ble_gap_scan_start(&scan_params, &BUF_DATA) };
    match ret {
        raw::NRF_SUCCESS => {
            defmt::println!("BLE scanning");
        }
        _ => {
            defmt::println!("BLE scanning error {}", ret);
        }
    }

    // BLE handle events
    const BLE_EVT_MAX_SIZE: u16 = 128;
    let mut evt: MaybeUninit<[u32; BLE_EVT_MAX_SIZE as usize / 4]> = MaybeUninit::uninit();

    loop {
        let mut len: u16 = BLE_EVT_MAX_SIZE;
        let ret = unsafe { raw::sd_ble_evt_get(evt.as_mut_ptr() as *mut u8, &mut len) };
        match ret {
            raw::NRF_SUCCESS => unsafe {
                let ble_evt = evt.as_ptr() as *const raw::ble_evt_t;
                defmt::println!("BLE event {:?}", (*ble_evt).header.evt_id);
            },
            raw::NRF_ERROR_INVALID_ADDR => {
                defmt::println!("Invalid or not sufficiently aligned pointer supplied.");
            }
            raw::NRF_ERROR_NOT_FOUND => {
                // defmt::println!("No events ready to be pulled.");
            }
            raw::NRF_ERROR_DATA_SIZE => {
                defmt::println!("Event ready but could not fit into the supplied buffer.");
                defmt::println!("Required {:?}", len);
            }
            _ => {
                defmt::println!("BLE event {}", ret);
            }
        }
    }
}

fn get_app_ram_base() -> u32 {
    extern "C" {
        static mut __sdata: u32;
    }

    unsafe { &mut __sdata as *mut u32 as u32 }
}

unsafe extern "C" fn fault_handler(id: u32, pc: u32, info: u32) {
    match (id, info) {
        (raw::NRF_FAULT_ID_SD_ASSERT, _) => panic!(
            "Softdevice assertion failed: an assertion inside the softdevice's code has failed. Most common cause is disabling interrupts for too long. Make sure you're using nrf_softdevice::interrupt::free instead of cortex_m::interrupt::free, which disables non-softdevice interrupts only. PC={:x}",
            pc
        ),
        (raw::NRF_FAULT_ID_APP_MEMACC, 0) => panic!(
            "Softdevice memory access violation. Your program accessed RAM reserved to the softdevice. PC={:x}",
            pc
        ),
        (raw::NRF_FAULT_ID_APP_MEMACC, _) => panic!(
            "Softdevice memory access violation. Your program accessed registers for a peripheral reserved to the softdevice. PC={:x} PREGION={:?}",
            pc, info
        ),
        _ => panic!(
            "Softdevice unknown fault id={:?} pc={:x} info={:?}",
            id, pc, info
        ),
    }
}
