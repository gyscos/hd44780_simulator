use std::cell::Cell;
use std::rc::Rc;

use lcd_hd44780;

#[derive(Copy, Clone, Debug)]
pub enum PinState {
    Low,
    High,
}

impl PinState {
    pub fn new() -> Rc<Cell<Self>> {
        Rc::new(Cell::new(PinState::Low))
    }
}

pub struct Pin {
    state: Rc<Cell<PinState>>,
}

impl Pin {
    pub fn new(state: Rc<Cell<PinState>>) -> Self {
        Pin { state: state }
    }
}

impl lcd_hd44780::gpio::Pin for Pin {
    fn high(&mut self) {
        self.state.set(PinState::High);
    }

    fn low(&mut self) {
        self.state.set(PinState::Low);
    }
}
