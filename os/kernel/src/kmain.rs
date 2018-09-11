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
#![feature(panic_info_message)]
#![feature(raw_vec_internals)]
#![feature(alloc, allocator_api, alloc_error_handler)]

extern crate alloc;
#[macro_use]
extern crate core;
extern crate pi;
extern crate stack_vec;

pub mod allocator;
pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

#[cfg(not(test))]
use allocator::Allocator;

#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();

#[no_mangle]
pub unsafe extern "C" fn kmain() {
    ALLOCATOR.initialize();
    // wait until a key is pressed before proceeding to the rest of the program,
    // otherwise things will be printed before you have connected over serial
    console::CONSOLE.lock().read_byte();

    let mut v = vec![];
    for i in 0..1000 {
        v.push(i);
    }
    console::kprintln!("{:?}", v);
}
