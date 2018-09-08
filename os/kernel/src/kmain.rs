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
    console::CONSOLE.lock().read_byte();
    panic!("testing!");
    // shell::shell("❯❯❯ ");
}
