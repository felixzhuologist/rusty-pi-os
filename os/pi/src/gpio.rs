use core::marker::PhantomData;

use common::{IO_BASE, states};
use volatile::prelude::*;
use volatile::{Volatile, WriteVolatile, ReadVolatile, Reserved};

/// An alternative GPIO function.
#[repr(u8)]
pub enum Function {
    Input = 0b000,
    Output = 0b001,
    Alt0 = 0b100,
    Alt1 = 0b101,
    Alt2 = 0b110,
    Alt3 = 0b111,
    Alt4 = 0b011,
    Alt5 = 0b010
}

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    FSEL: [Volatile<u32>; 6],
    __r0: Reserved<u32>,
    SET: [WriteVolatile<u32>; 2],
    __r1: Reserved<u32>,
    CLR: [WriteVolatile<u32>; 2],
    __r2: Reserved<u32>,
    LEV: [ReadVolatile<u32>; 2],
    __r3: Reserved<u32>,
    EDS: [Volatile<u32>; 2],
    __r4: Reserved<u32>,
    REN: [Volatile<u32>; 2],
    __r5: Reserved<u32>,
    FEN: [Volatile<u32>; 2],
    __r6: Reserved<u32>,
    HEN: [Volatile<u32>; 2],
    __r7: Reserved<u32>,
    LEN: [Volatile<u32>; 2],
    __r8: Reserved<u32>,
    AREN: [Volatile<u32>; 2],
    __r9: Reserved<u32>,
    AFEN: [Volatile<u32>; 2],
    __r10: Reserved<u32>,
    PUD: Volatile<u32>,
    PUDCLK: [Volatile<u32>; 2],
}

/// Possible states for a GPIO pin.
states! {
    Uninitialized, Input, Output, Alt
}

/// A GPIP pin in state `State`.
///
/// The `State` generic always corresponds to an uninstantiatable type that is
/// use solely to mark and track the state of a given GPIO pin. A `Gpio`
/// structure starts in the `Uninitialized` state and must be transitions into
/// one of `Input`, `Output`, or `Alt` via the `into_input`, `into_output`, and
/// `into_alt` methods before it can be used.
pub struct Gpio<State> {
    pin: u8,
    registers: &'static mut Registers,
    _state: PhantomData<State>
}

/// The base address of the `GPIO` registers.
const GPIO_BASE: usize = IO_BASE + 0x200000;

impl<T> Gpio<T> {
    /// Transitions `self` to state `S`, consuming `self` and returning a new
    /// `Gpio` instance in state `S`. This method should _never_ be exposed to
    /// the public!
    #[inline(always)]
    fn transition<S>(self) -> Gpio<S> {
        Gpio {
            pin: self.pin,
            registers: self.registers,
            _state: PhantomData
        }
    }
}

impl Gpio<Uninitialized> {
    /// Returns a new `GPIO` structure for pin number `pin`.
    ///
    /// # Panics
    ///
    /// Panics if `pin` > `53`.
    pub fn new(pin: u8) -> Gpio<Uninitialized> {
        if pin > 53 {
            panic!("Gpio::new(): pin {} exceeds maximum of 53", pin);
        }

        Gpio {
            registers: unsafe { &mut *(GPIO_BASE as *mut Registers) },
            pin: pin,
            _state: PhantomData
        }
    }

    pub fn set_function(&mut self, function: Function) {
        let pin = self.pin;
        let register = &mut self.registers.FSEL[(pin / 10) as usize];
        register.or_mask((function as u32) << ((pin % 10) * 3));
    }

    /// Enables the alternative function `function` for `self`. Consumes self
    /// and returns a `Gpio` structure in the `Alt` state.
    pub fn into_alt(mut self, function: Function) -> Gpio<Alt> {
        self.set_function(function);
        self.transition()
    }

    /// Sets this pin to be an _output_ pin. Consumes self and returns a `Gpio`
    /// structure in the `Output` state.
    pub fn into_output(mut self) -> Gpio<Output> {
        self.set_function(Function::Output);
        self.transition()
    }

    /// Sets this pin to be an _input_ pin. Consumes self and returns a `Gpio`
    /// structure in the `Input` state.
    pub fn into_input(mut self) -> Gpio<Input> {
        self.set_function(Function::Input);
        self.transition()
    }
}

impl Gpio<Output> {
    /// Sets (turns on) the pin.
    pub fn set(&mut self) {
        let pin = self.pin;
        let register = &mut self.registers.SET[(pin / 32) as usize];
        register.write(1 << (pin % 32));
    }

    /// Clears (turns off) the pin.
    pub fn clear(&mut self) {
        let pin = self.pin;
        let register = &mut self.registers.CLR[(pin / 32) as usize];
        register.write(1 << (pin % 32));
    }
}

impl Gpio<Input> {
    /// Reads the pin's value. Returns `true` if the level is high and `false`
    /// if the level is low.
    pub fn level(&mut self) -> bool {
        let pin = self.pin;
        let register = &mut self.registers.LEV[(pin / 32) as usize];
        (register.read() >> (pin % 32)) == 1
    }
}
