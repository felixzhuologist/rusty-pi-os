use pi::interrupt::Interrupt;
use pi::timer;
use process::TICK;

use traps::TrapFrame;

pub fn handle_irq(interrupt: Interrupt, tf: &mut TrapFrame) {
    timer::tick_in(TICK);
}
