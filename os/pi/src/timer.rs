use common::IO_BASE;
use volatile::prelude::*;
use volatile::{Volatile, ReadVolatile};

/// The base address for the ARM system timer registers.
const TIMER_REG_BASE: usize = IO_BASE + 0x3000;

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    /// system timer control/status: records timer channel comparator matches
    /// which are routed to the interrupt controller
    CS: Volatile<u32>,
    /// system timer counter lower 32 bits
    CLO: ReadVolatile<u32>,
    /// system timer counter higher 32 bits
    CHI: ReadVolatile<u32>,
    /// system timer compare: holds the compare value for each of the four timer
    /// channels, i.e. if CLO matches the value in COMPARE[i], then bit i of CS
    /// is set to high (possibly triggering an interrup)
    COMPARE: [Volatile<u32>; 4]
}

/// The Raspberry Pi ARM system timer.
pub struct Timer {
    registers: &'static mut Registers
}

impl Timer {
    /// Returns a new instance of `Timer`.
    pub fn new() -> Timer {
        Timer {
            registers: unsafe { &mut *(TIMER_REG_BASE as *mut Registers) },
        }
    }

    /// Reads the system timer's counter and returns the 64-bit counter value.
    /// The returned value is the number of elapsed microseconds.
    pub fn read(&self) -> u64 {
        (self.registers.CHI.read() as u64) << 32 |
        (self.registers.CLO.read() as u64)
    }

    /// Sets up a match in timer 1 to occur `us` microseconds from now. If
    /// interrupts for timer 1 are enabled and IRQs are unmasked, then a timer
    /// interrupt will be issued in `us` microseconds.
    pub fn tick_in(&mut self, us: u32) {
        let now_low = self.registers.CLO.read();
        self.registers.CS.write(0b0010);
        self.registers.COMPARE[1].write(now_low.wrapping_add(us));
    }
}

/// Returns the current time in microseconds.
pub fn current_time() -> u64 {
    Timer::new().read()
}

/// Spins until `us` microseconds have passed.
pub fn spin_sleep_us(us: u64) {
    let now = current_time();
    while current_time() < now + us { }
}

/// Spins until `ms` milliseconds have passed.
pub fn spin_sleep_ms(ms: u64) {
    spin_sleep_us(ms * 1000);
}

/// Sets up a match in timer 1 to occur `us` microseconds from now. If
/// interrupts for timer 1 are enabled and IRQs are unmasked, then a timer
/// interrupt will be issued in `us` microseconds.
pub fn tick_in(us: u32) {
    let timer = Timer::new();
    let now = timer.read();
    let target = now + (us as u64);
    timer.registers.CS.or_mask(0b10);
    timer.registers.COMPARE[1].write(target as u32);
}
