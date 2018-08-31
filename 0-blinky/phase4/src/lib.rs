#![feature(panic_implementation, compiler_builtins_lib, lang_items, asm, pointer_methods)]
#![no_builtins]
#![no_std]

pub mod lang_items;

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

#[inline(never)]
fn spin_sleep_ms(ms: usize) {
    for _ in 0..(ms * 600) {
        unsafe { asm!("nop" :::: "volatile"); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    *GPIO_FSEL1 |= 1 << 18;
    loop {
      *GPIO_SET0 |= 1 << 16;
      spin_sleep_ms(1000);
      *GPIO_CLR0 |= 1 << 16;
      spin_sleep_ms(1000);
    }
}
