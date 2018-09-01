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

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    use std::fmt::Write;

    let mut uart = pi::uart::MiniUart::new();
    loop {
        let byte = uart.read_byte();
        uart.write_byte(byte);
        uart.write_str("<-");
    }
}
