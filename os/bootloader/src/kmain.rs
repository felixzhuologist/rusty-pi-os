#![feature(asm, lang_items)]
#![feature(panic_implementation)]

#[macro_use]
extern crate core;
extern crate xmodem;
extern crate pi;

pub mod lang_items;

/// Start address of the binary to load and of the bootloader.
const BINARY_START_ADDR: usize = 0x80000;
const BOOTLOADER_START_ADDR: usize = 0x4000000;

/// Pointer to where the loaded binary expects to be laoded.
const BINARY_START: *mut u8 = BINARY_START_ADDR as *mut u8;

/// Free space between the bootloader and the loaded binary's start address.
const MAX_BINARY_SIZE: usize = BOOTLOADER_START_ADDR - BINARY_START_ADDR;

/// Branches to the address `addr` unconditionally.
fn jump_to(addr: *mut u8) -> ! {
    unsafe {
        asm!("br $0" : : "r"(addr as usize));
        loop { asm!("nop" :::: "volatile")  }
    }
}

#[no_mangle]
pub extern "C" fn kmain() {
    unsafe {
        let mut uart = pi::uart::MiniUart::new();
        uart.set_read_timeout(750);
        let mut kernel_binary =
            core::slice::from_raw_parts_mut(BINARY_START, MAX_BINARY_SIZE);

        loop {
            let result = xmodem::Xmodem::receive(&mut uart, &mut kernel_binary);
            match result {
                Err(ref e) if e.kind() != std::io::ErrorKind::TimedOut => {
                    // TODO: print message? but printing over serial will screw
                    // with xmodem transfer
                },
                Err(_) => {}
                Ok(_) => { break; }
            };
        }
    }

    jump_to(BINARY_START);
}
