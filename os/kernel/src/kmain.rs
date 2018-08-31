#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(panic_implementation)]
#![feature(compiler_builtins_lib)]
#![feature(pointer_methods)]

#[macro_use]
extern crate core;
extern crate pi;
extern crate stack_vec;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    let mut gpio = pi::gpio::Gpio::new(16).into_output();
    loop {
      gpio.set();
      pi::timer::spin_sleep_ms(1000);
      gpio.clear();
      pi::timer::spin_sleep_ms(1000);
    }
}
