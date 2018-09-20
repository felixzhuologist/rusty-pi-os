use common::IO_BASE;
use volatile::prelude::*;
use volatile::{Volatile, ReadVolatile};

const INT_BASE: usize = IO_BASE + 0xB000 + 0x200;

#[derive(Copy, Clone, PartialEq)]
pub enum Interrupt {
    Timer1 = 1,
    Timer3 = 3,
    Usb = 9,
    Gpio0 = 49,
    Gpio1 = 50,
    Gpio2 = 51,
    Gpio3 = 52,
    Uart = 57,
}

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    _basic_pending: Volatile<u32>,
    irq_pending: [ReadVolatile<u32>; 2],
    _fiq_control: Volatile<u32>,
    enable_irq: [Volatile<u32>; 2],
    _enable_basic: Volatile<u32>,
    disable_irq: [Volatile<u32>; 2],
    _disable_basic: Volatile<u32>
}

/// return index into array and index into the u32
fn get_index(int: Interrupt) -> (usize, usize) {
    let int = int as usize;
    (int / 32, int % 32)
}

/// An interrupt controller. Used to enable and disable interrupts as well as to
/// check if an interrupt is pending.
pub struct Controller {
    registers: &'static mut Registers
}

impl Controller {
    /// Returns a new handle to the interrupt controller.
    pub fn new() -> Controller {
        Controller {
            registers: unsafe { &mut *(INT_BASE as *mut Registers) },
        }
    }

    /// Enables the interrupt `int`.
    pub fn enable(&mut self, int: Interrupt) {
        let (reg, idx) = get_index(int);
        self.registers.enable_irq[reg].or_mask(1 << idx);
    }

    /// Disables the interrupt `int`.
    pub fn disable(&mut self, int: Interrupt) {
        let (reg, idx) = get_index(int);
        self.registers.disable_irq[reg].or_mask(1 << idx);
    }

    /// Returns `true` if `int` is pending. Otherwise, returns `false`.
    pub fn is_pending(&self, int: Interrupt) -> bool {
        let (reg, idx) = get_index(int);
        self.registers.irq_pending[reg].has_mask(1 << idx)
    }
}
