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
pub mod traps;
pub mod aarch64;
pub mod process;
pub mod vm;

#[cfg(not(test))]
use allocator::Allocator;
use fs::FileSystem;
use process::GlobalScheduler;

#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();

pub static FILE_SYSTEM: FileSystem = FileSystem::uninitialized();

pub static SCHEDULER: GlobalScheduler = GlobalScheduler::uninitialized();

/// entrypoint function for the first user process
#[no_mangle]
pub extern fn start_shell() {
    unsafe { asm!("brk 1" :::: "volatile"); }
    unsafe { asm!("brk 2" :::: "volatile"); }
    shell::shell("DeBuG> ", true);
    unsafe { asm!("brk 3" :::: "volatile"); }
    shell::shell("❯❯❯ ", false);
}

#[no_mangle]
#[cfg(not(test))]
pub unsafe extern "C" fn kmain() {
    ALLOCATOR.initialize();
    FILE_SYSTEM.initialize();

    // wait until a key is pressed before proceeding to the rest of the program,
    // otherwise things will be printed before you have connected over serial
    console::CONSOLE.lock().read_byte();

    SCHEDULER.start();
}
