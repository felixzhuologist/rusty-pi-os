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

#[macro_use]
#[allow(unused_imports)]
extern crate alloc;
extern crate core;
extern crate pi;
extern crate stack_vec;
extern crate fat32;

pub mod allocator;
pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;
pub mod fs;

#[cfg(not(test))]
use allocator::Allocator;
use fs::FileSystem;

#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();

pub static FILE_SYSTEM: FileSystem = FileSystem::uninitialized();

#[no_mangle]
#[cfg(not(test))]
pub unsafe extern "C" fn kmain() {
    ALLOCATOR.initialize();
    FILE_SYSTEM.initialize();
    // wait until a key is pressed before proceeding to the rest of the program,
    // otherwise things will be printed before you have connected over serial
    console::CONSOLE.lock().read_byte();

    shell::shell("❯❯❯ ");
}
